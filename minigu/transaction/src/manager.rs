use std::sync::Arc;

use minigu_catalog::memory::graph_type::MemoryGraphTypeCatalog;
use minigu_catalog::txn::CatalogTxnManager;
use minigu_common::{
    IsolationLevel, Timestamp, global_timestamp_generator, global_transaction_id_generator,
};
use minigu_storage::tp::MemoryGraph;

use crate::transaction::{CatalogTxnState, GraphTxnState, Transaction, TransactionCore, TxnError};

/// Coordinates creation of a global transaction spanning catalog + graph.
///
/// Phase 1 implementation: only unifies `begin` / `begin_at` and constructs the existing
/// `Transaction` object. Commit/abort logic remains on `Transaction`.
#[derive(Clone)]
pub struct TransactionManager {
    graph: Arc<MemoryGraph>,
    graph_type: Arc<MemoryGraphTypeCatalog>,
    catalog_txn_mgr: Arc<CatalogTxnManager>,
}

impl TransactionManager {
    pub fn new(
        graph: Arc<MemoryGraph>,
        graph_type: Arc<MemoryGraphTypeCatalog>,
        catalog_txn_mgr: Arc<CatalogTxnManager>,
    ) -> Self {
        Self {
            graph,
            graph_type,
            catalog_txn_mgr,
        }
    }

    pub fn begin_transaction(
        &self,
        isolation_level: IsolationLevel,
    ) -> Result<Transaction, TxnError> {
        let txn_id = global_transaction_id_generator().next()?;
        let start_ts = global_timestamp_generator().next()?;
        self.begin_transaction_at(txn_id, start_ts, isolation_level, false)
    }

    pub fn begin_transaction_at(
        &self,
        txn_id: Timestamp,
        start_ts: Timestamp,
        isolation_level: IsolationLevel,
        graph_skip_wal: bool,
    ) -> Result<Transaction, TxnError> {
        let mem_txn = self.graph.txn_manager().begin_transaction_at(
            Some(txn_id),
            Some(start_ts),
            isolation_level,
            graph_skip_wal,
        )?;
        let graph_state = GraphTxnState::new(mem_txn);

        let catalog_txn = self.catalog_txn_mgr.begin_transaction_at(
            Some(txn_id),
            Some(start_ts),
            isolation_level,
        )?;
        let catalog_state = Some(CatalogTxnState::new(
            Arc::clone(&self.graph_type),
            catalog_txn,
        ));

        let core = TransactionCore::new(txn_id, start_ts, isolation_level);
        Ok(Transaction::new(core, graph_state, catalog_state))
    }
}
