use std::fmt;
use std::sync::Arc;

use minigu_catalog::memory::graph_type::MemoryGraphTypeCatalog;
use minigu_catalog::txn::{CatalogTxn, CatalogTxnError, CatalogTxnView};
use minigu_common::types::{EdgeId, VectorIndexKey, VertexId};
use minigu_common::value::ScalarValue;
use minigu_common::{IsolationLevel, Timestamp, TimestampError, global_timestamp_generator};
use minigu_storage::error::StorageError;
use minigu_storage::model::edge::{Edge, Neighbor};
use minigu_storage::model::vertex::Vertex;
use minigu_storage::tp::{GraphTxnView, MemTransaction};
use thiserror::Error;

/// Result alias for transaction operations.
pub type TxnResult<T> = Result<T, TxnError>;

/// Global transaction that coordinates graph and catalog updates.
#[derive(Debug)]
pub struct Transaction {
    pub core: TransactionCore,
    pub graph: GraphTxnState,
    pub catalog: Option<CatalogTxnState>,
}

impl Transaction {
    pub fn new(
        core: TransactionCore,
        graph: GraphTxnState,
        catalog: Option<CatalogTxnState>,
    ) -> Self {
        Self {
            core,
            graph,
            catalog,
        }
    }

    /// Commit graph and catalog updates as a single global transaction.
    ///
    /// The catalog sub-transaction is validated first (prepare), then the graph is committed,
    /// and finally the catalog changes are published (apply) using the same commit timestamp.
    pub fn commit(&mut self) -> TxnResult<Timestamp> {
        if self.core.state != TxnState::Active {
            return Err(TxnError::InvalidState(
                "transaction already finished or not active",
            ));
        }

        // Phase 1: validate catalog writes (no publish)
        let catalog_txn = self.catalog.as_ref().map(|c| Arc::clone(c.txn()));
        let prepared_catalog = if let Some(txn) = catalog_txn.as_deref() {
            Some(txn.prepare()?)
        } else {
            None
        };

        let commit_ts = global_timestamp_generator().next()?;

        // Phase 2: publish graph first; if this fails, we can still abort catalog safely.
        match self.graph.commit_at(commit_ts) {
            Ok(ts) => {
                // Publish catalog after graph commit. `prepare()` holds the catalog commit lock,
                // so apply is expected to be non-failing.
                if let Some(prepared) = prepared_catalog {
                    prepared.apply(commit_ts)?;
                }
                self.core.mark_committed(ts);
                Ok(ts)
            }
            Err(err) => {
                // Drop the prepared guard before aborting.
                drop(prepared_catalog);
                if let Some(catalog) = self.catalog.as_ref() {
                    let _ = catalog.abort();
                }
                Err(TxnError::from(err))
            }
        }
    }

    /// Abort graph and catalog sub-transactions.
    pub fn abort(&mut self) -> TxnResult<()> {
        if self.core.state != TxnState::Active {
            return Ok(());
        }

        if let Some(catalog) = self.catalog.as_ref() {
            let _ = catalog.abort();
        }

        self.graph.abort()?;
        self.core.mark_aborted();
        Ok(())
    }

    pub fn state(&self) -> TxnState {
        self.core.state
    }

    pub fn graph(&self) -> &GraphTxnState {
        &self.graph
    }

    pub fn catalog(&self) -> Option<&CatalogTxnState> {
        self.catalog.as_ref()
    }

    pub fn core(&self) -> &TransactionCore {
        &self.core
    }

    pub fn get_vertex(&self, vid: minigu_common::types::VertexId) -> TxnResult<Vertex> {
        let graph = self.graph.mem().graph();
        graph.get_vertex(self, vid).map_err(TxnError::from)
    }

    pub fn get_edge(&self, eid: EdgeId) -> TxnResult<Edge> {
        let graph = self.graph.mem().graph();
        graph.get_edge(self, eid).map_err(TxnError::from)
    }

    pub fn iter_vertices(
        &self,
    ) -> TxnResult<Box<dyn Iterator<Item = minigu_storage::error::StorageResult<Vertex>> + '_>>
    {
        let graph = self.graph.mem().graph();
        graph.iter_vertices(self).map_err(TxnError::from)
    }

    pub fn iter_edges(
        &self,
    ) -> TxnResult<Box<dyn Iterator<Item = minigu_storage::error::StorageResult<Edge>> + '_>> {
        let graph = self.graph.mem().graph();
        graph.iter_edges(self).map_err(TxnError::from)
    }

    pub fn iter_adjacency(
        &self,
        vid: VertexId,
    ) -> TxnResult<Box<dyn Iterator<Item = minigu_storage::error::StorageResult<Neighbor>> + '_>>
    {
        let graph = self.graph.mem().graph();
        graph.iter_adjacency(self, vid).map_err(TxnError::from)
    }

    pub fn create_vertex(&self, vertex: Vertex) -> TxnResult<VertexId> {
        let graph = self.graph.mem().graph();
        graph.create_vertex(self, vertex).map_err(TxnError::from)
    }

    pub fn create_edge(&self, edge: Edge) -> TxnResult<EdgeId> {
        let graph = self.graph.mem().graph();
        graph.create_edge(self, edge).map_err(TxnError::from)
    }

    pub fn delete_vertex(&self, vid: VertexId) -> TxnResult<()> {
        let graph = self.graph.mem().graph();
        graph.delete_vertex(self, vid).map_err(TxnError::from)
    }

    pub fn delete_edge(&self, eid: EdgeId) -> TxnResult<()> {
        let graph = self.graph.mem().graph();
        graph.delete_edge(self, eid).map_err(TxnError::from)
    }

    pub fn set_vertex_property(
        &self,
        vid: VertexId,
        indices: Vec<usize>,
        props: Vec<ScalarValue>,
    ) -> TxnResult<()> {
        let graph = self.graph.mem().graph();
        graph
            .set_vertex_property(self, vid, indices, props)
            .map_err(TxnError::from)
    }

    pub fn set_edge_property(
        &self,
        eid: EdgeId,
        indices: Vec<usize>,
        props: Vec<ScalarValue>,
    ) -> TxnResult<()> {
        let graph = self.graph.mem().graph();
        graph
            .set_edge_property(self, eid, indices, props)
            .map_err(TxnError::from)
    }

    pub fn build_vector_index(&self, index_key: VectorIndexKey) -> TxnResult<()> {
        let graph = self.graph.mem().graph();
        graph
            .build_vector_index(self, index_key)
            .map_err(TxnError::from)
    }

    pub fn insert_into_vector_index(
        &self,
        index_key: VectorIndexKey,
        node_ids: &[u64],
    ) -> TxnResult<()> {
        let graph = self.graph.mem().graph();
        graph
            .insert_into_vector_index(self, index_key, node_ids)
            .map_err(TxnError::from)
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        if self.core.state == TxnState::Active {
            let _ = self.abort();
        }
    }
}

/// The lifecycle state of a transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TxnState {
    Active,
    Committed,
    Aborted,
}

/// Shared metadata used by all sub-transactions.
#[derive(Debug, Clone)]
pub struct TransactionCore {
    pub txn_id: Timestamp,
    pub start_ts: Timestamp,
    pub isolation_level: IsolationLevel,
    pub state: TxnState,
    pub commit_ts: Option<Timestamp>,
}

impl TransactionCore {
    pub fn new(txn_id: Timestamp, start_ts: Timestamp, isolation_level: IsolationLevel) -> Self {
        Self {
            txn_id,
            start_ts,
            isolation_level,
            state: TxnState::Active,
            commit_ts: None,
        }
    }

    pub fn mark_committed(&mut self, commit_ts: Timestamp) {
        self.state = TxnState::Committed;
        self.commit_ts = Some(commit_ts);
    }

    pub fn mark_aborted(&mut self) {
        self.state = TxnState::Aborted;
    }
}

/// Graph-side transaction state. Wraps the existing MemTransaction to avoid duplicating fields.
#[derive(Clone)]
pub struct GraphTxnState {
    mem: Arc<MemTransaction>,
}

impl GraphTxnState {
    pub fn new(mem: Arc<MemTransaction>) -> Self {
        Self { mem }
    }

    pub fn commit_at(&self, commit_ts: Timestamp) -> Result<Timestamp, StorageError> {
        self.mem.commit_at(Some(commit_ts), false)
    }

    pub fn abort(&self) -> Result<(), StorageError> {
        self.mem.abort_at(false)
    }

    pub fn mem(&self) -> &Arc<MemTransaction> {
        &self.mem
    }
}

impl GraphTxnView for GraphTxnState {
    fn mem_txn(&self) -> &Arc<MemTransaction> {
        &self.mem
    }
}

impl GraphTxnView for Transaction {
    fn mem_txn(&self) -> &Arc<MemTransaction> {
        self.graph.mem()
    }
}

impl CatalogTxnView for Transaction {
    fn catalog_txn(&self) -> &CatalogTxn {
        self.catalog
            .as_ref()
            .expect("catalog transaction missing")
            .txn()
    }
}

impl fmt::Debug for GraphTxnState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GraphTxnState")
            .field("txn_id", &self.mem.txn_id())
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct CatalogTxnState {
    graph_type: Arc<MemoryGraphTypeCatalog>,
    txn: Arc<CatalogTxn>,
}

impl CatalogTxnState {
    pub fn new(graph_type: Arc<MemoryGraphTypeCatalog>, txn: Arc<CatalogTxn>) -> Self {
        Self { graph_type, txn }
    }

    pub fn graph_type(&self) -> &Arc<MemoryGraphTypeCatalog> {
        &self.graph_type
    }

    pub fn txn(&self) -> &Arc<CatalogTxn> {
        &self.txn
    }

    pub fn commit_at(&self, commit_ts: Timestamp) -> TxnResult<Timestamp> {
        self.txn.commit_at(commit_ts).map_err(TxnError::from)
    }

    pub fn abort(&self) -> TxnResult<()> {
        self.txn.abort().map_err(TxnError::from)
    }
}

/// Errors surfaced by the transaction layer.
#[derive(Error, Debug)]
pub enum TxnError {
    #[error("storage error: {0}")]
    Storage(#[from] StorageError),
    #[error("timestamp error: {0}")]
    Timestamp(#[from] TimestampError),
    #[error("invalid transaction state: {0}")]
    InvalidState(&'static str),
    #[error("catalog error: {0}")]
    Catalog(#[from] CatalogTxnError),
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use minigu_catalog::memory::graph_type::MemoryGraphTypeCatalog;
    use minigu_catalog::provider::GraphTypeProvider;
    use minigu_catalog::txn::transaction::TxnHook;
    use minigu_catalog::txn::{CatalogTxnManager, CatalogTxnView};
    use minigu_common::types::LabelId;
    use minigu_common::{IsolationLevel, Timestamp};
    use minigu_storage::common::model::properties::PropertyRecord;
    use minigu_storage::tp::{MemoryGraph, memory_graph};

    use super::{GraphTxnState, Transaction, TransactionCore, TxnError, TxnState};
    use crate::TransactionManager;

    fn label_id_1() -> LabelId {
        LabelId::new(1).expect("label id should be non-zero")
    }

    fn create_txn_mgr(
        graph: Arc<MemoryGraph>,
    ) -> (
        TransactionManager,
        Arc<MemoryGraphTypeCatalog>,
        Arc<CatalogTxnManager>,
    ) {
        let graph_type = Arc::new(MemoryGraphTypeCatalog::new());
        let catalog_txn_mgr = CatalogTxnManager::new();
        let mgr =
            TransactionManager::new(graph, Arc::clone(&graph_type), Arc::clone(&catalog_txn_mgr));
        (mgr, graph_type, catalog_txn_mgr)
    }

    fn create_test_vertex(id: u64) -> minigu_storage::model::vertex::Vertex {
        minigu_storage::model::vertex::Vertex::new(id, label_id_1(), PropertyRecord::new(vec![]))
    }

    #[test]
    fn test_transaction_basic_invariants() {
        let txn_id = Timestamp::with_ts(Timestamp::TXN_ID_START + 123);
        let start_ts = Timestamp::with_ts(10);

        let (graph, _cleaner) = memory_graph::tests::mock_empty_graph();
        let mem_txn = graph
            .txn_manager()
            .begin_transaction_at(
                Some(txn_id),
                Some(start_ts),
                IsolationLevel::Serializable,
                true,
            )
            .expect("begin graph txn");
        let graph_state = GraphTxnState::new(mem_txn);
        let core = TransactionCore::new(txn_id, start_ts, IsolationLevel::Serializable);
        let txn = Transaction::new(core.clone(), graph_state, None);

        assert_eq!(txn.state(), TxnState::Active);
        assert!(txn.core.commit_ts.is_none());
        assert_eq!(txn.core.txn_id, txn_id);
        assert_eq!(txn.core.start_ts, start_ts);
        assert!(matches!(
            txn.core.isolation_level,
            IsolationLevel::Serializable
        ));
    }

    #[test]
    fn test_commit_success_graph_and_catalog_both_visible_and_same_commit_ts() {
        let (graph, _cleaner) = memory_graph::tests::mock_empty_graph();
        let (mgr, graph_type, _catalog_mgr) = create_txn_mgr(Arc::clone(&graph));

        let mut txn = mgr
            .begin_transaction(IsolationLevel::Serializable)
            .expect("begin global txn");

        graph_type
            .add_label_txn("A".to_string(), txn.catalog_txn())
            .expect("catalog write");
        txn.create_vertex(create_test_vertex(1))
            .expect("graph write");

        let commit_ts = txn.commit().expect("commit should succeed");
        assert_eq!(txn.state(), TxnState::Committed);
        assert_eq!(txn.core.commit_ts, Some(commit_ts));
        assert_eq!(txn.graph.mem().commit_ts(), Some(commit_ts));
        assert_eq!(
            txn.catalog.as_ref().unwrap().txn().commit_ts(),
            Some(commit_ts)
        );

        // Verify both writes are visible under new transactions.
        let read_txn_g = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .expect("begin read graph txn");
        let v = graph.get_vertex(&read_txn_g, 1).expect("get vertex");
        assert_eq!(v.vid(), 1);
        read_txn_g.abort().ok();

        let read_txn_c = _catalog_mgr
            .begin_transaction(IsolationLevel::Serializable)
            .expect("begin read catalog txn");
        let label = graph_type
            .get_label_id_txn("A", read_txn_c.as_ref())
            .expect("get label");
        assert!(label.is_some());
        read_txn_c.abort().ok();
    }

    #[derive(Debug)]
    struct FailHook;

    impl TxnHook for FailHook {
        fn precommit(
            &self,
            _txn: &minigu_catalog::txn::transaction::CatalogTxn,
        ) -> minigu_catalog::txn::CatalogTxnResult<()> {
            Err(minigu_catalog::txn::CatalogTxnError::ReferentialIntegrity {
                reason: "fail".to_string(),
            })
        }
    }

    #[test]
    fn test_catalog_prepare_fails_graph_not_committed() {
        let (graph, _cleaner) = memory_graph::tests::mock_empty_graph();
        let (mgr, graph_type, catalog_mgr) = create_txn_mgr(Arc::clone(&graph));

        let mut txn = mgr
            .begin_transaction(IsolationLevel::Serializable)
            .expect("begin global txn");

        txn.catalog
            .as_ref()
            .unwrap()
            .txn()
            .add_hook(Box::new(FailHook));

        graph_type
            .add_label_txn("A".to_string(), txn.catalog_txn())
            .expect("catalog write");
        txn.create_vertex(create_test_vertex(1))
            .expect("graph write");

        let err = txn.commit().expect_err("commit should fail");
        assert!(matches!(err, TxnError::Catalog(_)));

        // Explicit abort to clean up; should be safe and idempotent.
        txn.abort().expect("abort should succeed");
        assert_eq!(txn.state(), TxnState::Aborted);

        let read_txn_g = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .expect("begin read graph txn");
        assert!(graph.get_vertex(&read_txn_g, 1).is_err());
        read_txn_g.abort().ok();

        let read_txn_c = catalog_mgr
            .begin_transaction(IsolationLevel::Serializable)
            .expect("begin read catalog txn");
        let label = graph_type
            .get_label_id_txn("A", read_txn_c.as_ref())
            .expect("get label");
        assert!(label.is_none());
        read_txn_c.abort().ok();
    }

    #[test]
    fn test_graph_commit_fails_catalog_is_aborted() {
        let (graph, _cleaner) = memory_graph::tests::mock_empty_graph();
        let (mgr, graph_type, catalog_mgr) = create_txn_mgr(Arc::clone(&graph));

        // Base committed vertex so that read-set validation can conflict later.
        let base_txn = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .expect("base txn");
        graph
            .create_vertex(&base_txn, create_test_vertex(10))
            .expect("create base vertex");
        base_txn.commit().expect("commit base");

        let mut txn = mgr
            .begin_transaction(IsolationLevel::Serializable)
            .expect("begin global txn");

        // Register a read so that graph commit does serializable validation.
        let _ = txn.get_vertex(10).expect("read base vertex");

        // Catalog write that would otherwise be committed.
        graph_type
            .add_label_txn("A".to_string(), txn.catalog_txn())
            .expect("catalog write");

        // Conflicting writer modifies the read vertex after this txn started.
        let writer = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .expect("writer txn");
        graph.delete_vertex(&writer, 10).expect("delete");
        writer.commit().expect("writer commit");

        let err = txn.commit().expect_err("commit should fail");
        assert!(matches!(err, TxnError::Storage(_)));

        // Catalog write should not be visible (abort path executed).
        let read_txn_c = catalog_mgr
            .begin_transaction(IsolationLevel::Serializable)
            .expect("begin read catalog txn");
        let label = graph_type
            .get_label_id_txn("A", read_txn_c.as_ref())
            .expect("get label");
        assert!(label.is_none());
        read_txn_c.abort().ok();

        // Ensure we can create the same label afterwards (no leaked head).
        let txn2 = catalog_mgr
            .begin_transaction(IsolationLevel::Serializable)
            .expect("begin txn2");
        graph_type
            .add_label_txn("A".to_string(), txn2.as_ref())
            .expect("add_label_txn should succeed after abort");
        txn2.commit().expect("commit txn2");
    }

    #[test]
    fn test_abort_is_global_and_idempotent() {
        let (graph, _cleaner) = memory_graph::tests::mock_empty_graph();
        let (mgr, graph_type, catalog_mgr) = create_txn_mgr(Arc::clone(&graph));

        let mut txn = mgr
            .begin_transaction(IsolationLevel::Serializable)
            .expect("begin global txn");
        graph_type
            .add_label_txn("A".to_string(), txn.catalog_txn())
            .expect("catalog write");
        txn.create_vertex(create_test_vertex(1))
            .expect("graph write");

        txn.abort().expect("abort should succeed");
        assert_eq!(txn.state(), TxnState::Aborted);
        txn.abort().expect("abort should be idempotent");
        assert_eq!(txn.state(), TxnState::Aborted);

        let read_txn_g = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .expect("read txn");
        assert!(graph.get_vertex(&read_txn_g, 1).is_err());
        read_txn_g.abort().ok();

        let read_txn_c = catalog_mgr
            .begin_transaction(IsolationLevel::Serializable)
            .expect("read txn");
        let label = graph_type
            .get_label_id_txn("A", read_txn_c.as_ref())
            .expect("get label");
        assert!(label.is_none());
        read_txn_c.abort().ok();
    }

    #[test]
    fn test_drop_is_implicit_abort() {
        let (graph, _cleaner) = memory_graph::tests::mock_empty_graph();
        let (mgr, graph_type, catalog_mgr) = create_txn_mgr(Arc::clone(&graph));

        {
            let txn = mgr
                .begin_transaction(IsolationLevel::Serializable)
                .expect("begin global txn");
            graph_type
                .add_label_txn("A".to_string(), txn.catalog_txn())
                .expect("catalog write");
            txn.create_vertex(create_test_vertex(1))
                .expect("graph write");
            // drop without commit/abort
        }

        let read_txn_g = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .expect("read txn");
        assert!(graph.get_vertex(&read_txn_g, 1).is_err());
        read_txn_g.abort().ok();

        let read_txn_c = catalog_mgr
            .begin_transaction(IsolationLevel::Serializable)
            .expect("read txn");
        let label = graph_type
            .get_label_id_txn("A", read_txn_c.as_ref())
            .expect("get label");
        assert!(label.is_none());
        read_txn_c.abort().ok();
    }

    #[test]
    fn test_finished_txn_cannot_commit_again() {
        let (graph, _cleaner) = memory_graph::tests::mock_empty_graph();
        let (mgr, _graph_type, _catalog_mgr) = create_txn_mgr(Arc::clone(&graph));

        let mut txn = mgr
            .begin_transaction(IsolationLevel::Serializable)
            .expect("begin global txn");
        let ts1 = txn.commit().expect("commit should succeed");
        assert_eq!(txn.state(), TxnState::Committed);
        assert_eq!(txn.core.commit_ts, Some(ts1));

        let err = txn.commit().expect_err("second commit should fail");
        assert!(matches!(err, TxnError::InvalidState(_)));
        assert_eq!(txn.core.commit_ts, Some(ts1));
    }

    #[test]
    fn test_graph_only_transaction_commit_and_abort() {
        let (graph, _cleaner) = memory_graph::tests::mock_empty_graph();

        // graph-only transaction
        let mem_txn = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .expect("begin graph txn");
        let core = TransactionCore::new(
            mem_txn.txn_id(),
            mem_txn.start_ts(),
            IsolationLevel::Serializable,
        );
        let graph_state = GraphTxnState::new(mem_txn);
        let mut txn = Transaction::new(core, graph_state, None);

        txn.create_vertex(create_test_vertex(1))
            .expect("graph write");
        let ts = txn.commit().expect("commit should succeed");
        assert_eq!(txn.state(), TxnState::Committed);
        assert_eq!(txn.core.commit_ts, Some(ts));
        assert_eq!(txn.graph.mem().commit_ts(), Some(ts));

        // abort on committed should be no-op, not panic
        txn.abort().expect("abort should be ok for non-active");
    }

    #[test]
    fn test_commit_ts_global_consistency() {
        let (graph, _cleaner) = memory_graph::tests::mock_empty_graph();
        let (mgr, graph_type, _catalog_mgr) = create_txn_mgr(Arc::clone(&graph));

        let mut txn = mgr
            .begin_transaction(IsolationLevel::Serializable)
            .expect("begin global txn");
        graph_type
            .add_label_txn("A".to_string(), txn.catalog_txn())
            .expect("catalog write");
        txn.create_vertex(create_test_vertex(1))
            .expect("graph write");

        let commit_ts = txn.commit().expect("commit should succeed");
        assert_eq!(txn.core.commit_ts, Some(commit_ts));
        assert_eq!(txn.graph.mem().commit_ts(), Some(commit_ts));
        assert_eq!(
            txn.catalog.as_ref().unwrap().txn().commit_ts(),
            Some(commit_ts)
        );
    }
}
