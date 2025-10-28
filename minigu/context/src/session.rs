use std::sync::Arc;

use minigu_catalog::memory::schema::MemorySchemaCatalog;
use minigu_catalog::named_ref::NamedGraphRef;
use minigu_catalog::txn::catalog_txn::CatalogTxn;
use minigu_catalog::txn::error::CatalogTxnResult;
use minigu_catalog::txn::manager::CatalogTxnManager;
use minigu_transaction::{GraphTxnManager, IsolationLevel};

use crate::database::DatabaseContext;

#[derive(Clone)]
pub struct SessionContext {
    database: Arc<DatabaseContext>,
    pub home_schema: Option<Arc<MemorySchemaCatalog>>,
    pub current_schema: Option<Arc<MemorySchemaCatalog>>,
    pub home_graph: Option<NamedGraphRef>,
    pub current_graph: Option<NamedGraphRef>,
    pub current_txn: Option<Arc<CatalogTxn>>,
    pub catalog_txn_mgr: CatalogTxnManager,
    isolation_level: Option<IsolationLevel>,
}

impl SessionContext {
    pub fn new(database: Arc<DatabaseContext>) -> Self {
        Self {
            database,
            home_schema: None,
            current_schema: None,
            home_graph: None,
            current_graph: None,
            current_txn: None,
            catalog_txn_mgr: CatalogTxnManager::new(),
            isolation_level: None,
        }
    }

    pub fn database(&self) -> &DatabaseContext {
        &self.database
    }

    /// Set default isolation level for implicitly created transactions.
    #[inline]
    pub fn set_default_isolation(&mut self, iso: IsolationLevel) {
        self.isolation_level = Some(iso);
    }

    /// Begin a new transaction using the default isolation level.
    fn begin_txn(&self) -> CatalogTxnResult<Arc<CatalogTxn>> {
        if let Some(iso) = self.isolation_level {
            let txn_arc = self.catalog_txn_mgr.begin_transaction(iso)?;
            return Ok(txn_arc);
        }
        let txn_arc = self
            .catalog_txn_mgr
            .begin_transaction(IsolationLevel::Serializable)?;
        Ok(txn_arc)
    }

    /// Get the current transaction if present; otherwise begin one and store it.
    pub fn get_or_begin_txn(&mut self) -> CatalogTxnResult<Arc<CatalogTxn>> {
        if let Some(txn) = &self.current_txn {
            return Ok(txn.clone());
        }

        let txn = self.begin_txn()?;
        self.current_txn = Some(txn.clone());
        Ok(txn)
    }

    /// Clear the current transaction reference (without committing or rolling back).
    /// This is used in auto-commit mode, where the transaction is committed successfully,
    /// and the `current_txn` is removed to avoid mistakenly reusing the committed transaction.
    /// Note: This method only clears the session-level reference, does not affect the lifecycle of
    /// the transaction itself.
    #[inline]
    pub fn clear_current_txn(&mut self) {
        self.current_txn = None;
    }
}
