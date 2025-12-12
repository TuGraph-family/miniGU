use std::fmt;
use std::sync::{Arc, Weak};

use minigu_transaction::Transaction;

use crate::error::CatalogResult;
use crate::provider::{
    DirectoryProvider, DirectoryRef, GraphRef, GraphTypeRef, ProcedureRef, SchemaProvider,
};
use crate::txn::catalog_txn::CatalogTxn;
use crate::txn::versioned_map::{VersionedMap, WriteOp};

pub struct MemorySchemaCatalog {
    parent: Option<Weak<dyn DirectoryProvider>>,
    graph_map: Arc<VersionedMap<String, GraphRef>>,
    graph_type_map: Arc<VersionedMap<String, GraphTypeRef>>,
    procedure_map: Arc<VersionedMap<String, ProcedureRef>>,
}

impl fmt::Debug for MemorySchemaCatalog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MemorySchemaCatalog").finish()
    }
}

impl MemorySchemaCatalog {
    #[inline]
    pub fn new(parent: Option<Weak<dyn DirectoryProvider>>) -> Self {
        Self {
            parent,
            graph_map: Arc::new(VersionedMap::new()),
            graph_type_map: Arc::new(VersionedMap::new()),
            procedure_map: Arc::new(VersionedMap::new()),
        }
    }

    // ===== Transactional write APIs =====
    #[inline]
    pub fn add_graph_txn(
        &self,
        name: String,
        graph: GraphRef,
        txn: &CatalogTxn,
    ) -> Result<(), crate::txn::error::CatalogTxnError> {
        if self.graph_map.get(&name, txn).is_some() {
            return Err(crate::txn::error::CatalogTxnError::AlreadyExists { key: name });
        }
        let node = self.graph_map.put(name.clone(), Arc::new(graph), txn)?;
        txn.record_write(&self.graph_map, name, node, WriteOp::Create);
        Ok(())
    }

    #[inline]
    pub fn remove_graph_txn(
        &self,
        name: &str,
        txn: &CatalogTxn,
    ) -> Result<(), crate::txn::error::CatalogTxnError> {
        let key = name.to_string();
        let _base = self
            .graph_map
            .get_node_visible(&key, txn.start_ts(), txn.txn_id())
            .ok_or_else(|| crate::txn::error::CatalogTxnError::NotFound { key: key.clone() })?;
        let node = self.graph_map.delete(&key, txn)?;
        txn.record_write(&self.graph_map, key, node, WriteOp::Delete);
        Ok(())
    }

    #[inline]
    pub fn add_graph_type_txn(
        &self,
        name: String,
        graph_type: GraphTypeRef,
        txn: &CatalogTxn,
    ) -> Result<(), crate::txn::error::CatalogTxnError> {
        if self.graph_type_map.get(&name, txn).is_some() {
            return Err(crate::txn::error::CatalogTxnError::AlreadyExists { key: name });
        }
        let node = self
            .graph_type_map
            .put(name.clone(), Arc::new(graph_type), txn)?;
        txn.record_write(&self.graph_type_map, name, node, WriteOp::Create);
        Ok(())
    }

    #[inline]
    pub fn remove_graph_type_txn(
        &self,
        name: &str,
        txn: &CatalogTxn,
    ) -> Result<(), crate::txn::error::CatalogTxnError> {
        let key = name.to_string();
        let _base = self
            .graph_type_map
            .get_node_visible(&key, txn.start_ts(), txn.txn_id())
            .ok_or_else(|| crate::txn::error::CatalogTxnError::NotFound { key: key.clone() })?;
        let node = self.graph_type_map.delete(&key, txn)?;
        txn.record_write(&self.graph_type_map, key, node, WriteOp::Delete);
        Ok(())
    }

    #[inline]
    pub fn add_procedure_txn(
        &self,
        name: String,
        procedure: ProcedureRef,
        txn: &CatalogTxn,
    ) -> Result<(), crate::txn::error::CatalogTxnError> {
        if self.procedure_map.get(&name, txn).is_some() {
            return Err(crate::txn::error::CatalogTxnError::AlreadyExists { key: name });
        }
        let node = self
            .procedure_map
            .put(name.clone(), Arc::new(procedure), txn)?;
        txn.record_write(&self.procedure_map, name, node, WriteOp::Create);
        Ok(())
    }

    #[inline]
    pub fn remove_procedure_txn(
        &self,
        name: &str,
        txn: &CatalogTxn,
    ) -> Result<(), crate::txn::error::CatalogTxnError> {
        let key = name.to_string();
        let _base = self
            .procedure_map
            .get_node_visible(&key, txn.start_ts(), txn.txn_id())
            .ok_or_else(|| crate::txn::error::CatalogTxnError::NotFound { key: key.clone() })?;
        let node = self.procedure_map.delete(&key, txn)?;
        txn.record_write(&self.procedure_map, key, node, WriteOp::Delete);
        Ok(())
    }
}

impl SchemaProvider for MemorySchemaCatalog {
    #[inline]
    fn parent(&self) -> Option<DirectoryRef> {
        self.parent.clone().and_then(|p| p.upgrade())
    }

    #[inline]
    fn get_graph(&self, name: &str, txn: &CatalogTxn) -> CatalogResult<Option<GraphRef>> {
        Ok(self
            .graph_map
            .get(&name.to_string(), txn)
            .map(|arc| (*arc).clone()))
    }

    #[inline]
    fn graph_names(&self, txn: &CatalogTxn) -> Vec<String> {
        self.graph_map.visible_keys(txn.start_ts(), txn.txn_id())
    }

    #[inline]
    fn get_graph_type(&self, name: &str, txn: &CatalogTxn) -> CatalogResult<Option<GraphTypeRef>> {
        Ok(self
            .graph_type_map
            .get(&name.to_string(), txn)
            .map(|arc| (*arc).clone()))
    }

    #[inline]
    fn graph_type_names(&self, txn: &CatalogTxn) -> Vec<String> {
        self.graph_type_map
            .visible_keys(txn.start_ts(), txn.txn_id())
    }

    #[inline]
    fn get_procedure(&self, name: &str, txn: &CatalogTxn) -> CatalogResult<Option<ProcedureRef>> {
        Ok(self
            .procedure_map
            .get(&name.to_string(), txn)
            .map(|arc| (*arc).clone()))
    }

    #[inline]
    fn procedure_names(&self, txn: &CatalogTxn) -> Vec<String> {
        self.procedure_map
            .visible_keys(txn.start_ts(), txn.txn_id())
    }

    #[inline]
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use std::any::Any;

    use minigu_transaction::{GraphTxnManager, IsolationLevel, Transaction};

    use super::*;
    use crate::memory::graph_type::MemoryGraphTypeCatalog as GT;
    use crate::provider::{GraphProvider, ProcedureProvider};
    use crate::txn::manager::CatalogTxnManager;

    #[derive(Debug)]
    struct DummyGraph {
        gt: Arc<GT>,
    }
    impl GraphProvider for DummyGraph {
        fn graph_type(&self) -> GraphTypeRef {
            self.gt.clone() as _
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    #[derive(Debug)]
    struct DummyProcedure;
    impl ProcedureProvider for DummyProcedure {
        fn parameters(&self) -> &[minigu_common::data_type::LogicalType] {
            &[]
        }

        fn schema(&self) -> Option<minigu_common::data_type::DataSchemaRef> {
            None
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    #[test]
    fn add_remove_graph_and_type_visibility() {
        let mgr = CatalogTxnManager::new();
        let schema = MemorySchemaCatalog::new(None);
        let gt = Arc::new(GT::new());
        let graph = Arc::new(DummyGraph { gt: gt.clone() });

        let t1 = mgr.begin_transaction(IsolationLevel::Serializable).unwrap();
        schema
            .add_graph_type_txn("GT1".to_string(), gt.clone() as _, &t1)
            .unwrap();
        schema
            .add_graph_txn("G1".to_string(), graph.clone() as _, &t1)
            .unwrap();
        assert!(schema.get_graph_type("GT1", &t1).unwrap().is_some());
        let t2 = mgr.begin_transaction(IsolationLevel::Serializable).unwrap();
        assert!(schema.get_graph_type("GT1", &t2).unwrap().is_none());
        t1.commit().unwrap();
        let t3 = mgr.begin_transaction(IsolationLevel::Serializable).unwrap();
        assert!(schema.get_graph("G1", &t3).unwrap().is_some());

        let t4 = mgr.begin_transaction(IsolationLevel::Serializable).unwrap();
        schema.remove_graph_txn("G1", &t4).unwrap();
        assert!(schema.get_graph("G1", &t4).unwrap().is_none());
        t4.commit().unwrap();
        let t5 = mgr.begin_transaction(IsolationLevel::Serializable).unwrap();
        assert!(schema.get_graph("G1", &t5).unwrap().is_none());

        let t6 = mgr.begin_transaction(IsolationLevel::Serializable).unwrap();
        schema.remove_graph_type_txn("GT1", &t6).unwrap();
        t6.commit().unwrap();
        let t7 = mgr.begin_transaction(IsolationLevel::Serializable).unwrap();
        assert!(schema.get_graph_type("GT1", &t7).unwrap().is_none());
    }

    #[test]
    fn add_remove_procedure_visibility() {
        let mgr = CatalogTxnManager::new();
        let schema = MemorySchemaCatalog::new(None);
        let proc_ref: ProcedureRef = Arc::new(DummyProcedure) as _;

        let t1 = mgr.begin_transaction(IsolationLevel::Serializable).unwrap();
        schema
            .add_procedure_txn("P1".to_string(), proc_ref.clone(), &t1)
            .unwrap();
        assert!(schema.get_procedure("P1", &t1).unwrap().is_some());
        let t2 = mgr.begin_transaction(IsolationLevel::Serializable).unwrap();
        assert!(schema.get_procedure("P1", &t2).unwrap().is_none());
        t1.commit().unwrap();

        let t3 = mgr.begin_transaction(IsolationLevel::Serializable).unwrap();
        assert!(schema.get_procedure("P1", &t3).unwrap().is_some());

        let t4 = mgr.begin_transaction(IsolationLevel::Serializable).unwrap();
        schema.remove_procedure_txn("P1", &t4).unwrap();
        t4.commit().unwrap();
        let t5 = mgr.begin_transaction(IsolationLevel::Serializable).unwrap();
        assert!(schema.get_procedure("P1", &t5).unwrap().is_none());
    }

    #[test]
    fn add_memory_graph_and_basic_storage_ops() {
        use minigu_common::value::ScalarValue;
        use minigu_storage::common::model::properties::PropertyRecord;
        use minigu_storage::common::model::vertex::Vertex;
        use minigu_storage::tp::MemoryGraph;

        // Prepare schema and graph type
        let mgr = CatalogTxnManager::new();
        let schema = MemorySchemaCatalog::new(None);
        let gt = Arc::new(GT::new());

        // Create a label in graph type under a catalog txn
        let t0 = mgr
            .begin_transaction(minigu_transaction::IsolationLevel::Serializable)
            .unwrap();
        let person = gt.add_label("Person".to_string(), &t0).unwrap();
        t0.commit().unwrap();

        // Define a GraphProvider wrapping MemoryGraph to insert into catalog
        struct DummyMemGraph {
            gt: Arc<GT>,
            mem: Arc<MemoryGraph>,
        }
        impl std::fmt::Debug for DummyMemGraph {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("DummyMemGraph").finish()
            }
        }
        impl GraphProvider for DummyMemGraph {
            fn graph_type(&self) -> GraphTypeRef {
                self.gt.clone() as _
            }

            fn as_any(&self) -> &dyn Any {
                self
            }
        }

        // Add MemoryGraph wrapped by our DummyMemGraph into catalog schema
        let mem_graph = MemoryGraph::new();
        let container: Arc<dyn GraphProvider> = Arc::new(DummyMemGraph {
            gt: gt.clone(),
            mem: mem_graph.clone(),
        });

        let t1 = mgr
            .begin_transaction(minigu_transaction::IsolationLevel::Serializable)
            .unwrap();
        schema
            .add_graph_txn("G1".to_string(), container.clone(), &t1)
            .unwrap();
        t1.commit().unwrap();

        // Read back the graph via catalog and operate storage-layer txn
        let t2 = mgr
            .begin_transaction(minigu_transaction::IsolationLevel::Serializable)
            .unwrap();
        let gref = schema.get_graph("G1", &t2).unwrap().expect("graph exists");
        let d = gref
            .as_ref()
            .downcast_ref::<DummyMemGraph>()
            .expect("dummy mem graph");
        let mem = d.mem.clone();

        // storage-layer: create a vertex and read it back
        let stx = mem
            .txn_manager()
            .begin_transaction(minigu_transaction::IsolationLevel::Serializable)
            .unwrap();
        let v = Vertex::new(
            100u64,
            person,
            PropertyRecord::new(vec![ScalarValue::String(Some("Alice".to_string()))]),
        );
        mem.create_vertex(&stx, v).unwrap();
        stx.commit().unwrap();

        let stx2 = mem
            .txn_manager()
            .begin_transaction(minigu_transaction::IsolationLevel::Serializable)
            .unwrap();
        let rv = mem.get_vertex(&stx2, 100u64).unwrap();
        assert_eq!(rv.vid(), 100u64);
        assert_eq!(rv.label_id, person);
        stx2.abort().ok();

        // catalog read still sees the graph via the same t2 snapshot
        assert!(schema.get_graph("G1", &t2).unwrap().is_some());
    }
}
