use std::collections::HashMap;
use std::sync::Arc;

use minigu_catalog::memory::graph_type::{
    MemoryEdgeTypeCatalog, MemoryGraphTypeCatalog, MemoryVertexTypeCatalog,
};
use minigu_catalog::provider::{
    CatalogProvider, EdgeTypeProvider, GraphTypeProvider, SchemaProvider,
};
use minigu_catalog::txn::catalog_txn::CatalogTxn;
use minigu_common::data_chunk::DataChunk;
use minigu_context::graph::{GraphContainer, GraphStorage};
use minigu_context::session::SessionContext;
use minigu_planner::bound::{
    BoundCatalogModifyingStatement, BoundCreateGraphStatement, BoundCreateGraphTypeStatement,
    BoundDropGraphStatement, BoundDropGraphTypeStatement, BoundDropSchemaStatement, BoundGraphType,
    CreateKind,
};
use minigu_storage::tp::MemoryGraph;
use minigu_transaction::{GraphTxnManager, Transaction};

use crate::error::{ExecutionError, ExecutionResult};
use crate::executor::Executor;

pub struct CatalogDdlBuilder {
    session: SessionContext,
    stmt: BoundCatalogModifyingStatement,
}

impl CatalogDdlBuilder {
    pub fn new(session: SessionContext, stmt: BoundCatalogModifyingStatement) -> Self {
        Self { session, stmt }
    }

    pub fn into_executor(self) -> crate::executor::BoxedExecutor {
        Box::new(CatalogDdlExec {
            session: self.session,
            stmt: self.stmt,
            done: false,
        })
    }
}

struct CatalogDdlExec {
    session: SessionContext,
    stmt: BoundCatalogModifyingStatement,
    done: bool,
}

impl CatalogDdlExec {
    fn run(&mut self) -> ExecutionResult<()> {
        match self.stmt.clone() {
            BoundCatalogModifyingStatement::CreateGraphType(s) => self.exec_create_graph_type(&s),
            BoundCatalogModifyingStatement::DropGraphType(s) => self.exec_drop_graph_type(&s),
            BoundCatalogModifyingStatement::CreateGraph(s) => self.exec_create_graph(&s),
            BoundCatalogModifyingStatement::DropGraph(s) => self.exec_drop_graph(&s),
            BoundCatalogModifyingStatement::CreateSchema(s) => self.exec_create_schema(&s),
            BoundCatalogModifyingStatement::DropSchema(s) => self.exec_drop_schema(&s),
            BoundCatalogModifyingStatement::Call(_) => unreachable!(),
        }
    }

    fn current_schema(
        &self,
    ) -> ExecutionResult<Arc<minigu_catalog::memory::schema::MemorySchemaCatalog>> {
        self.session
            .current_schema
            .clone()
            .ok_or_else(|| ExecutionError::Custom("current schema not set".into()))
    }

    fn exec_create_graph_type(&mut self, s: &BoundCreateGraphTypeStatement) -> ExecutionResult<()> {
        let schema = self.current_schema()?;
        self.session
            .with_statement_txn(|txn| {
                // Resolve source graph type provider and deep copy to a new MemoryGraphTypeCatalog
                let (gt, _id_map) = match &s.source {
                    BoundGraphType::Ref(named) => {
                        Self::deep_copy_graph_type(named.object().clone(), txn).map_err(|e| {
                            minigu_catalog::txn::error::CatalogTxnError::External(Box::new(e))
                        })?
                    }
                    BoundGraphType::Nested(_elems) => {
                        (Arc::new(MemoryGraphTypeCatalog::new()), HashMap::new())
                    }
                };
                // Handle create kind
                let exists = schema
                    .get_graph_type(&s.name, txn)
                    .map_err(|e| {
                        minigu_catalog::txn::error::CatalogTxnError::External(Box::new(e))
                    })?
                    .is_some();
                match s.kind {
                    CreateKind::Create => {
                        if exists {
                            return Err(
                                minigu_catalog::txn::error::CatalogTxnError::IllegalState {
                                    reason: "graph type already exists".into(),
                                },
                            );
                        }
                    }
                    CreateKind::CreateIfNotExists => {
                        if exists {
                            return Ok(());
                        }
                    }
                    CreateKind::CreateOrReplace => {
                        if exists {
                            schema.remove_graph_type_txn(&s.name, txn).map_err(|e| {
                                minigu_catalog::txn::error::CatalogTxnError::External(Box::new(e))
                            })?;
                        }
                    }
                }
                schema
                    .add_graph_type_txn(s.name.to_string(), gt as _, txn)
                    .map_err(|e| {
                        minigu_catalog::txn::error::CatalogTxnError::External(Box::new(e))
                    })?;
                Ok(())
            })
            .map_err(|e| ExecutionError::Custom(Box::new(e)))
    }

    fn exec_drop_graph_type(&mut self, s: &BoundDropGraphTypeStatement) -> ExecutionResult<()> {
        let schema = self.current_schema()?;
        self.session
            .with_statement_txn(|txn| {
                let exists = schema
                    .get_graph_type(&s.name, txn)
                    .map_err(|e| {
                        minigu_catalog::txn::error::CatalogTxnError::External(Box::new(e))
                    })?
                    .is_some();
                if !exists {
                    if s.if_exists {
                        return Ok(());
                    }
                    return Err(minigu_catalog::txn::error::CatalogTxnError::IllegalState {
                        reason: "graph type not found".into(),
                    });
                }
                schema.remove_graph_type_txn(&s.name, txn).map_err(|e| {
                    minigu_catalog::txn::error::CatalogTxnError::External(Box::new(e))
                })?;
                Ok(())
            })
            .map_err(|e| ExecutionError::Custom(Box::new(e)))
    }

    fn exec_create_graph(&mut self, s: &BoundCreateGraphStatement) -> ExecutionResult<()> {
        let schema = self.current_schema()?;
        let (target_graph, id_map) = self
            .session
            .with_statement_txn(|txn| {
                // Decide graph type: deep copy to ensure independence
                let (gt, id_map) = match &s.graph_type {
                    BoundGraphType::Ref(named) => {
                        Self::deep_copy_graph_type(named.object().clone(), txn).map_err(|e| {
                            minigu_catalog::txn::error::CatalogTxnError::External(Box::new(e))
                        })?
                    }
                    BoundGraphType::Nested(_elems) => {
                        (Arc::new(MemoryGraphTypeCatalog::new()), HashMap::new())
                    }
                };
                // Handle kind
                let exists = schema
                    .get_graph(&s.name, txn)
                    .map_err(|e| {
                        minigu_catalog::txn::error::CatalogTxnError::External(Box::new(e))
                    })?
                    .is_some();
                match s.kind {
                    CreateKind::Create => {
                        if exists {
                            return Err(
                                minigu_catalog::txn::error::CatalogTxnError::IllegalState {
                                    reason: "graph already exists".into(),
                                },
                            );
                        }
                    }
                    CreateKind::CreateIfNotExists => {
                        if exists {
                            return Ok((None, HashMap::new()));
                        }
                    }
                    CreateKind::CreateOrReplace => {
                        if exists {
                            schema.remove_graph_txn(&s.name, txn).map_err(|e| {
                                minigu_catalog::txn::error::CatalogTxnError::External(Box::new(e))
                            })?;
                        }
                    }
                }
                let memory_graph = MemoryGraph::new();
                let target_graph = memory_graph.clone();
                let container = GraphContainer::new(gt.clone(), GraphStorage::Memory(memory_graph));
                schema
                    .add_graph_txn(s.name.to_string(), Arc::new(container) as _, txn)
                    .map_err(|e| {
                        minigu_catalog::txn::error::CatalogTxnError::External(Box::new(e))
                    })?;
                Ok((Some(target_graph), id_map))
            })
            .map_err(|e| ExecutionError::Custom(Box::new(e)))?;

        if let (Some(target_graph), Some(src_named_graph)) = (
            target_graph,
            s.source.as_ref().map(|ng| ng.object().clone()),
        ) {
            self.copy_graph_data(src_named_graph, &target_graph, &id_map)?;
        }
        Ok(())
    }

    fn exec_drop_graph(&mut self, s: &BoundDropGraphStatement) -> ExecutionResult<()> {
        let schema = self.current_schema()?;
        self.session
            .with_statement_txn(|txn| {
                let exists = schema
                    .get_graph(&s.name, txn)
                    .map_err(|e| {
                        minigu_catalog::txn::error::CatalogTxnError::External(Box::new(e))
                    })?
                    .is_some();
                if !exists {
                    if s.if_exists {
                        return Ok(());
                    }
                    return Err(minigu_catalog::txn::error::CatalogTxnError::IllegalState {
                        reason: "graph not found".into(),
                    });
                }
                schema.remove_graph_txn(&s.name, txn).map_err(|e| {
                    minigu_catalog::txn::error::CatalogTxnError::External(Box::new(e))
                })?;
                Ok(())
            })
            .map_err(|e| ExecutionError::Custom(Box::new(e)))
    }

    fn deep_copy_graph_type(
        src: Arc<dyn GraphTypeProvider>,
        txn: &CatalogTxn,
    ) -> ExecutionResult<(
        Arc<MemoryGraphTypeCatalog>,
        HashMap<minigu_common::types::LabelId, minigu_common::types::LabelId>,
    )> {
        // 1) Copy labels and build id mapping
        let dst = MemoryGraphTypeCatalog::new();
        use minigu_catalog::label_set::LabelSet;
        use minigu_common::types::LabelId;
        let mut id_map: HashMap<LabelId, LabelId> = HashMap::new();
        for name in src.label_names(txn) {
            let old_id = src
                .get_label_id(&name, txn)
                .map_err(|e| ExecutionError::Custom(Box::new(e)))?
                .expect("label id exists");
            let new_id = dst
                .add_label(name, txn)
                .map_err(|e| ExecutionError::Custom(Box::new(e)))?;
            id_map.insert(old_id, new_id);
        }
        // 2) Copy vertex types
        let mut new_vertex_types: HashMap<LabelSet, Arc<MemoryVertexTypeCatalog>> = HashMap::new();
        for key in src.vertex_type_keys(txn) {
            let mapped: LabelSet = key.iter().map(|lid| *id_map.get(&lid).unwrap()).collect();
            let vt = src
                .get_vertex_type(&key, txn)
                .map_err(|e| ExecutionError::Custom(Box::new(e)))?
                .expect("vertex type present");
            let props = vt.properties().into_iter().map(|(_, p)| p).collect();
            let new_vt = Arc::new(MemoryVertexTypeCatalog::new(mapped.clone(), props));
            new_vertex_types.insert(mapped, new_vt);
        }
        // 3) Copy edge types
        for key in src.edge_type_keys(txn) {
            let mapped: LabelSet = key.iter().map(|lid| *id_map.get(&lid).unwrap()).collect();
            let et = src
                .get_edge_type(&key, txn)
                .map_err(|e| ExecutionError::Custom(Box::new(e)))?
                .expect("edge type present");
            let props = et.properties().into_iter().map(|(_, p)| p).collect();
            let src_v_ls = et.src().label_set();
            let dst_v_ls = et.dst().label_set();
            let new_src_ls: LabelSet = src_v_ls
                .iter()
                .map(|lid| *id_map.get(&lid).unwrap())
                .collect();
            let new_dst_ls: LabelSet = dst_v_ls
                .iter()
                .map(|lid| *id_map.get(&lid).unwrap())
                .collect();
            let new_src = new_vertex_types
                .get(&new_src_ls)
                .expect("new src vertex exists")
                .clone() as _;
            let new_dst = new_vertex_types
                .get(&new_dst_ls)
                .expect("new dst vertex exists")
                .clone() as _;
            let new_et = Arc::new(MemoryEdgeTypeCatalog::new(mapped, new_src, new_dst, props));
            let et_ref: minigu_catalog::provider::EdgeTypeRef = new_et.clone() as _;
            dst.add_edge_type(new_et.label_set(), et_ref, txn)
                .map_err(|e| ExecutionError::Custom(Box::new(e)))?;
        }
        // 4) Register vertex types into dst
        for (k, vt) in new_vertex_types.into_iter() {
            let vt_ref: minigu_catalog::provider::VertexTypeRef = vt as _;
            dst.add_vertex_type(k, vt_ref, txn)
                .map_err(|e| ExecutionError::Custom(Box::new(e)))?;
        }
        Ok((Arc::new(dst), id_map))
    }

    fn exec_create_schema(
        &mut self,
        s: &minigu_planner::bound::BoundCreateSchemaStatement,
    ) -> ExecutionResult<()> {
        let root = self
            .session
            .database()
            .catalog()
            .get_root()
            .map_err(|e| ExecutionError::Custom(Box::new(e)))?;
        self.session.with_statement_txn(|txn| {
            let mut current_dir = root
                .as_directory()
                .ok_or_else(|| minigu_catalog::txn::error::CatalogTxnError::IllegalState { reason: "root is not a directory".into() })?
                .clone();
            for (i, seg) in s.schema_path.iter().enumerate() {
                let is_last = i + 1 == s.schema_path.len();
                if seg.as_str() == ".." {
                    if let Some(p) = current_dir.parent() {
                        current_dir = p;
                    }
                    continue;
                }
                let existing = current_dir
                    .get_child(seg, txn)
                    .map_err(|e| minigu_catalog::txn::error::CatalogTxnError::External(Box::new(e)))?;
                match (existing, is_last) {
                    (Some(child), false) => {
                        if let Some(dir) = child.into_directory() {
                            current_dir = dir;
                        } else {
                            return Err(minigu_catalog::txn::error::CatalogTxnError::IllegalState { reason: format!("path segment '{}' is a schema", seg) });
                        }
                    }
                    (Some(child), true) => {
                        if child.is_schema() {
                            if s.if_not_exists {
                                return Ok(());
                            }
                            return Err(minigu_catalog::txn::error::CatalogTxnError::IllegalState { reason: format!("schema '{}' already exists", seg) });
                        } else {
                            return Err(minigu_catalog::txn::error::CatalogTxnError::IllegalState { reason: format!("name '{}' already used by a directory", seg) });
                        }
                    }
                    (None, false) => {
                        let mem_dir = current_dir
                            .as_any()
                            .downcast_ref::<minigu_catalog::memory::directory::MemoryDirectoryCatalog>()
                            .ok_or_else(|| minigu_catalog::txn::error::CatalogTxnError::IllegalState { reason: "not a memory directory".into() })?;
                        let new_dir = Arc::new(
                            minigu_catalog::memory::directory::MemoryDirectoryCatalog::new(Some(
                                Arc::downgrade(&current_dir),
                            )),
                        );
                        mem_dir
                            .add_child(
                                seg.to_string(),
                                minigu_catalog::provider::DirectoryOrSchema::Directory(
                                    new_dir.clone() as _
                                ),
                                txn,
                            )
                            .map_err(|e| minigu_catalog::txn::error::CatalogTxnError::External(Box::new(e)))?;
                        current_dir = new_dir as _;
                    }
                    (None, true) => {
                        let mem_dir = current_dir
                            .as_any()
                            .downcast_ref::<minigu_catalog::memory::directory::MemoryDirectoryCatalog>()
                            .ok_or_else(|| minigu_catalog::txn::error::CatalogTxnError::IllegalState { reason: "not a memory directory".into() })?;
                        let new_schema = Arc::new(minigu_catalog::memory::schema::MemorySchemaCatalog::new(
                            Some(Arc::downgrade(&current_dir)),
                        ));
                        mem_dir
                            .add_child(
                                seg.to_string(),
                                minigu_catalog::provider::DirectoryOrSchema::Schema(new_schema as _),
                                txn,
                            )
                            .map_err(|e| minigu_catalog::txn::error::CatalogTxnError::External(Box::new(e)))?;
                    }
                }
            }
            Ok(())
        }).map_err(|e| ExecutionError::Custom(Box::new(e)))
    }

    fn exec_drop_schema(&mut self, s: &BoundDropSchemaStatement) -> ExecutionResult<()> {
        use minigu_catalog::provider::DirectoryOrSchema;
        if s.schema_path.is_empty() {
            return Err(ExecutionError::Custom("empty schema path".into()));
        }
        let root = self
            .session
            .database()
            .catalog()
            .get_root()
            .map_err(|e| ExecutionError::Custom(Box::new(e)))?;
        self.session
            .with_statement_txn(|txn| {
                let mut current = root;
                for seg in &s.schema_path[..s.schema_path.len() - 1] {
                    if seg.as_str() == ".." {
                        if let Some(p) = current.parent() {
                            current = DirectoryOrSchema::Directory(p);
                        }
                        continue;
                    }
                    let dir = current.as_directory().ok_or_else(|| {
                        minigu_catalog::txn::error::CatalogTxnError::IllegalState {
                            reason: "not a directory in path".into(),
                        }
                    })?;
                    let child = dir.get_child(seg, txn).map_err(|e| {
                        minigu_catalog::txn::error::CatalogTxnError::External(Box::new(e))
                    })?;
                    current = match child {
                        Some(c) => c,
                        None => {
                            if s.if_exists {
                                return Ok(());
                            }
                            return Err(
                                minigu_catalog::txn::error::CatalogTxnError::IllegalState {
                                    reason: format!("path segment '{}' not found", seg),
                                },
                            );
                        }
                    };
                }
                let last = s.schema_path.last().unwrap();
                if last.as_str() == ".." {
                    return Err(minigu_catalog::txn::error::CatalogTxnError::IllegalState {
                        reason: "invalid path: ends with '..'".into(),
                    });
                }
                let parent_dir = current.as_directory().ok_or_else(|| {
                    minigu_catalog::txn::error::CatalogTxnError::IllegalState {
                        reason: "parent is not a directory".into(),
                    }
                })?;
                let child = parent_dir.get_child(last, txn).map_err(|e| {
                    minigu_catalog::txn::error::CatalogTxnError::External(Box::new(e))
                })?;
                let Some(child) = child else {
                    if s.if_exists {
                        return Ok(());
                    }
                    return Err(minigu_catalog::txn::error::CatalogTxnError::IllegalState {
                        reason: format!("schema '{}' not found", last),
                    });
                };
                let Some(schema) = child.as_schema() else {
                    return Err(minigu_catalog::txn::error::CatalogTxnError::IllegalState {
                        reason: format!("'{}' is not a schema", last),
                    });
                };
                if !schema.graph_names(txn).is_empty()
                    || !schema.graph_type_names(txn).is_empty()
                    || !schema.procedure_names(txn).is_empty()
                {
                    return Err(minigu_catalog::txn::error::CatalogTxnError::IllegalState {
                        reason: "schema is not empty".into(),
                    });
                }
                let mem_dir = parent_dir
                    .as_any()
                    .downcast_ref::<minigu_catalog::memory::directory::MemoryDirectoryCatalog>()
                    .ok_or_else(
                        || minigu_catalog::txn::error::CatalogTxnError::IllegalState {
                            reason: "not a memory directory".into(),
                        },
                    )?;
                mem_dir.remove_child(last, txn).map_err(|e| {
                    minigu_catalog::txn::error::CatalogTxnError::External(Box::new(e))
                })?;
                Ok(())
            })
            .map_err(|e| ExecutionError::Custom(Box::new(e)))
    }

    fn copy_graph_data(
        &self,
        src_graph: Arc<dyn minigu_catalog::provider::GraphProvider>,
        dst_graph: &Arc<MemoryGraph>,
        id_map: &HashMap<minigu_common::types::LabelId, minigu_common::types::LabelId>,
    ) -> ExecutionResult<()> {
        use minigu_storage::common::model::edge::Edge;
        use minigu_storage::common::model::properties::PropertyRecord;
        use minigu_storage::common::model::vertex::Vertex;
        use minigu_transaction::IsolationLevel;

        let src_container = src_graph
            .as_ref()
            .downcast_ref::<GraphContainer>()
            .ok_or_else(|| {
                ExecutionError::Custom("source is not a memory graph container".into())
            })?;
        let src_graph_arc = match src_container.graph_storage() {
            GraphStorage::Memory(g) => g.clone(),
        };

        let src_txn = src_graph_arc
            .txn_manager()
            .begin_transaction(IsolationLevel::Snapshot)
            .map_err(|e| ExecutionError::Custom(Box::new(e)))?;
        let dst_txn = dst_graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Snapshot)
            .map_err(|e| ExecutionError::Custom(Box::new(e)))?;

        let it_v = src_graph_arc
            .iter_vertices(&src_txn)
            .map_err(|e| ExecutionError::Custom(Box::new(e)))?;
        for v_res in it_v {
            let v = v_res.map_err(|e| ExecutionError::Custom(Box::new(e)))?;
            let new_label = *id_map.get(&v.label_id).unwrap_or(&v.label_id);
            let props = PropertyRecord::new(v.properties().clone());
            let new_v = Vertex::new(v.vid(), new_label, props);
            dst_graph
                .create_vertex(&dst_txn, new_v)
                .map_err(|e| ExecutionError::Custom(Box::new(e)))?;
        }

        let it_e = src_graph_arc
            .iter_edges(&src_txn)
            .map_err(|e| ExecutionError::Custom(Box::new(e)))?;
        for e_res in it_e {
            let e = e_res.map_err(|e| ExecutionError::Custom(Box::new(e)))?;
            let new_label = *id_map.get(&e.label_id()).unwrap_or(&e.label_id());
            let props = PropertyRecord::new(e.properties().clone());
            let new_e = Edge::new(e.eid(), e.src_id(), e.dst_id(), new_label, props);
            dst_graph
                .create_edge(&dst_txn, new_e)
                .map_err(|e| ExecutionError::Custom(Box::new(e)))?;
        }

        dst_txn
            .commit()
            .map_err(|e| ExecutionError::Custom(Box::new(e)))?;
        Ok(())
    }
}

impl Executor for CatalogDdlExec {
    fn next_chunk(&mut self) -> Option<ExecutionResult<DataChunk>> {
        if self.done {
            return None;
        }
        self.done = true;
        if let Err(e) = self.run() {
            return Some(Err(e));
        }
        // DDL produces no data; signal completion with None on next call
        None
    }
}
