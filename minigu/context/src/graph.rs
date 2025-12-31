use std::any::Any;
use std::fmt::{self, Debug};
use std::sync::Arc;

use minigu_catalog::memory::graph_type::MemoryGraphTypeCatalog;
use minigu_catalog::memory::txn_manager;
use minigu_catalog::provider::{GraphProvider, GraphTypeRef};
use minigu_common::IsolationLevel;
use minigu_common::types::{LabelId, VertexIdArray};
use minigu_storage::error::StorageResult;
use minigu_storage::tp::MemoryGraph;
use minigu_transaction::{Transaction, TransactionManager, TxnError};

pub enum GraphStorage {
    Memory(Arc<MemoryGraph>),
}

pub struct GraphContainer {
    graph_type: Arc<MemoryGraphTypeCatalog>,
    graph_storage: GraphStorage,
    txn_mgr: TransactionManager,
}

impl GraphContainer {
    pub fn new(graph_type: Arc<MemoryGraphTypeCatalog>, graph_storage: GraphStorage) -> Self {
        let catalog_txn_mgr = Arc::clone(txn_manager());
        let graph = match &graph_storage {
            GraphStorage::Memory(mem) => Arc::clone(mem),
        };
        let txn_mgr =
            TransactionManager::new(graph, Arc::clone(&graph_type), Arc::clone(&catalog_txn_mgr));
        Self {
            graph_type,
            graph_storage,
            txn_mgr,
        }
    }

    pub fn begin_transaction(
        &self,
        isolation_level: IsolationLevel,
    ) -> Result<Transaction, TxnError> {
        self.txn_mgr.begin_transaction(isolation_level)
    }

    #[inline]
    pub fn graph_storage(&self) -> &GraphStorage {
        &self.graph_storage
    }

    #[inline]
    pub fn graph_type(&self) -> Arc<MemoryGraphTypeCatalog> {
        self.graph_type.clone()
    }
}

// TODO: Remove and use a checker.
fn vertex_has_all_labels(
    _mem: &Arc<MemoryGraph>,
    _txn: &Arc<minigu_storage::tp::transaction::MemTransaction>,
    _vid: u64,
    _label_ids: &Option<Vec<Vec<LabelId>>>,
) -> StorageResult<bool> {
    let Some(label_specs) = _label_ids else {
        return Ok(true);
    };

    let vertex = _mem.get_vertex(_txn, _vid)?;
    let vertex_label = vertex.label_id;

    for and_labels in label_specs {
        if and_labels.is_empty() {
            return Ok(true);
        }
        if and_labels.contains(&vertex_label) {
            return Ok(true);
        }
    }
    Ok(false)
}

impl GraphContainer {
    pub fn vertex_source(
        &self,
        label_ids: &Option<Vec<Vec<LabelId>>>,
        batch_size: usize,
    ) -> StorageResult<Box<dyn Iterator<Item = Arc<VertexIdArray>> + Send + 'static>> {
        let mem = match self.graph_storage() {
            GraphStorage::Memory(m) => Arc::clone(m),
        };
        let txn = mem
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)?;
        let mut ids: Vec<u64> = Vec::new();
        {
            let it = mem.iter_vertices(&txn)?;
            for v in it {
                let v = v?;
                let vid = v.vid();
                if vertex_has_all_labels(&mem, &txn, vid, label_ids)? {
                    ids.push(vid);
                }
            }
        }

        // TODO(Colin): Sort IDs to ensure deterministic output in tests.
        // Remove once ORDER BY is supported.
        ids.sort_unstable();

        let mut pos = 0usize;
        let iter = std::iter::from_fn(move || {
            if pos >= ids.len() {
                return None;
            }
            let end = (pos + batch_size).min(ids.len());
            let slice = &ids[pos..end];
            pos = end;
            Some(Arc::new(VertexIdArray::from_iter(slice.iter().copied())))
        });

        Ok(Box::new(iter))
    }
}

impl Debug for GraphContainer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GraphContainer")
            .field("graph_type", &self.graph_type)
            .finish()
    }
}

impl GraphProvider for GraphContainer {
    #[inline]
    fn graph_type(&self) -> GraphTypeRef {
        self.graph_type.clone()
    }

    #[inline]
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use minigu_catalog::memory::graph_type::MemoryGraphTypeCatalog;
    use minigu_catalog::provider::{GraphProvider, GraphTypeProvider};
    use minigu_catalog::txn::{CatalogTxnManager, CatalogTxnView};
    use minigu_common::IsolationLevel;
    use minigu_common::types::LabelId;
    use minigu_storage::common::model::properties::PropertyRecord;
    use minigu_storage::model::vertex::Vertex;
    use minigu_storage::tp::memory_graph;
    use minigu_transaction::TxnState;

    use super::{GraphContainer, GraphStorage};

    fn label_id_1() -> LabelId {
        LabelId::new(1).expect("label id should be non-zero")
    }

    #[test]
    fn test_graph_container_new_invariants() {
        let (graph, _cleaner) = memory_graph::tests::mock_empty_graph();
        let graph_type = Arc::new(MemoryGraphTypeCatalog::new());
        let container = GraphContainer::new(
            Arc::clone(&graph_type),
            GraphStorage::Memory(Arc::clone(&graph)),
        );

        assert!(Arc::ptr_eq(&container.graph_type(), &graph_type));

        match container.graph_storage() {
            GraphStorage::Memory(mem) => assert!(Arc::ptr_eq(mem, &graph)),
        }
    }

    #[test]
    fn test_begin_transaction_creates_global_transaction() {
        let (graph, _cleaner) = memory_graph::tests::mock_empty_graph();
        let graph_type = Arc::new(MemoryGraphTypeCatalog::new());
        let container = GraphContainer::new(
            Arc::clone(&graph_type),
            GraphStorage::Memory(Arc::clone(&graph)),
        );

        let txn = container
            .begin_transaction(IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");

        assert_eq!(txn.state(), TxnState::Active);
        assert!(Arc::ptr_eq(txn.graph.mem().graph(), &graph));
        assert!(txn.catalog().is_some());
    }

    #[test]
    fn test_graph_provider_graph_type_consistency() {
        let (graph, _cleaner) = memory_graph::tests::mock_empty_graph();
        let graph_type = Arc::new(MemoryGraphTypeCatalog::new());
        let container = GraphContainer::new(
            Arc::clone(&graph_type),
            GraphStorage::Memory(Arc::clone(&graph)),
        );

        let provider_graph_type = container.graph_type();

        // Commit a catalog change using the concrete type...
        let mgr = CatalogTxnManager::new();
        let txn = mgr
            .begin_transaction(IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");
        let id = graph_type
            .add_label_txn("A".to_string(), txn.as_ref())
            .expect("add_label_txn should succeed");
        txn.commit().expect("commit should succeed");

        // ...and observe it via both access paths.
        let seen_via_container = graph_type
            .get_label_id("A")
            .expect("get_label_id should succeed");
        let seen_via_provider = provider_graph_type
            .get_label_id("A")
            .expect("get_label_id should succeed");
        assert_eq!(seen_via_container, Some(id));
        assert_eq!(seen_via_provider, Some(id));
    }

    #[test]
    fn test_as_any_downcast_ref_graph_container() {
        let (graph, _cleaner) = memory_graph::tests::mock_empty_graph();
        let graph_type = Arc::new(MemoryGraphTypeCatalog::new());
        let container = GraphContainer::new(graph_type, GraphStorage::Memory(graph));

        let provider: &dyn GraphProvider = &container;
        let downcasted = GraphProvider::as_any(provider).downcast_ref::<GraphContainer>();
        assert!(downcasted.is_some());
    }

    #[test]
    fn test_global_commit_makes_graph_and_catalog_visible() {
        let (graph, _cleaner) = memory_graph::tests::mock_empty_graph();
        let graph_type = Arc::new(MemoryGraphTypeCatalog::new());
        let container = GraphContainer::new(
            Arc::clone(&graph_type),
            GraphStorage::Memory(Arc::clone(&graph)),
        );

        let mut txn = container
            .begin_transaction(IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");

        graph_type
            .add_label_txn("A".to_string(), txn.catalog_txn())
            .expect("add_label_txn should succeed");
        txn.create_vertex(Vertex::new(1, label_id_1(), PropertyRecord::new(vec![])))
            .expect("create_vertex should succeed");

        txn.commit().expect("commit should succeed");
        assert_eq!(txn.state(), TxnState::Committed);

        // Graph visibility.
        let read_txn_g = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .expect("begin graph txn");
        assert!(graph.get_vertex(&read_txn_g, 1).is_ok());
        read_txn_g.abort().ok();

        // Catalog visibility.
        let read_mgr = CatalogTxnManager::new();
        let read_txn_c = read_mgr
            .begin_transaction(IsolationLevel::Serializable)
            .expect("begin catalog txn");
        let label = graph_type
            .get_label_id_txn("A", read_txn_c.as_ref())
            .expect("get_label_id_txn should succeed");
        assert!(label.is_some());
        read_txn_c.abort().ok();
    }

    #[test]
    fn test_global_abort_makes_graph_and_catalog_invisible_and_idempotent() {
        let (graph, _cleaner) = memory_graph::tests::mock_empty_graph();
        let graph_type = Arc::new(MemoryGraphTypeCatalog::new());
        let container = GraphContainer::new(
            Arc::clone(&graph_type),
            GraphStorage::Memory(Arc::clone(&graph)),
        );

        let mut txn = container
            .begin_transaction(IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");
        graph_type
            .add_label_txn("A".to_string(), txn.catalog_txn())
            .expect("add_label_txn should succeed");
        txn.create_vertex(Vertex::new(1, label_id_1(), PropertyRecord::new(vec![])))
            .expect("create_vertex should succeed");

        txn.abort().expect("abort should succeed");
        assert_eq!(txn.state(), TxnState::Aborted);
        txn.abort().expect("abort should be idempotent");

        let read_txn_g = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .expect("begin graph txn");
        assert!(graph.get_vertex(&read_txn_g, 1).is_err());
        read_txn_g.abort().ok();

        let read_mgr = CatalogTxnManager::new();
        let read_txn_c = read_mgr
            .begin_transaction(IsolationLevel::Serializable)
            .expect("begin catalog txn");
        let label = graph_type
            .get_label_id_txn("A", read_txn_c.as_ref())
            .expect("get_label_id_txn should succeed");
        assert!(label.is_none());
        read_txn_c.abort().ok();
    }

    #[test]
    fn test_drop_is_implicit_abort_container_level() {
        let (graph, _cleaner) = memory_graph::tests::mock_empty_graph();
        let graph_type = Arc::new(MemoryGraphTypeCatalog::new());
        let container = GraphContainer::new(
            Arc::clone(&graph_type),
            GraphStorage::Memory(Arc::clone(&graph)),
        );

        {
            let txn = container
                .begin_transaction(IsolationLevel::Serializable)
                .expect("begin_transaction should succeed");
            graph_type
                .add_label_txn("A".to_string(), txn.catalog_txn())
                .expect("add_label_txn should succeed");
            txn.create_vertex(Vertex::new(1, label_id_1(), PropertyRecord::new(vec![])))
                .expect("create_vertex should succeed");
        }

        let read_txn_g = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .expect("begin graph txn");
        assert!(graph.get_vertex(&read_txn_g, 1).is_err());
        read_txn_g.abort().ok();

        let read_mgr = CatalogTxnManager::new();
        let read_txn_c = read_mgr
            .begin_transaction(IsolationLevel::Serializable)
            .expect("begin catalog txn");
        let label = graph_type
            .get_label_id_txn("A", read_txn_c.as_ref())
            .expect("get_label_id_txn should succeed");
        assert!(label.is_none());
        read_txn_c.abort().ok();
    }
}
