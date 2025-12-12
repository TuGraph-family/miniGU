use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};

use minigu_transaction::timestamp::Timestamp;
use minigu_transaction::{
    GraphTxnManager, IsolationLevel, Transaction, global_timestamp_generator,
    global_transaction_id_generator,
};

use crate::txn::catalog_txn::CatalogTxn;
use crate::txn::error::{CatalogTxnError, CatalogTxnResult};
use crate::txn::runtime::CatalogRuntime;

#[derive(Debug)]
pub struct CatalogTxnManagerInner {
    active: RwLock<HashMap<u64, Timestamp>>, // txn_id.raw() -> start_ts
    low_watermark_raw: AtomicU64,
}

impl CatalogTxnManagerInner {
    fn new() -> Self {
        Self {
            active: RwLock::new(HashMap::new()),
            low_watermark_raw: AtomicU64::new(0),
        }
    }

    fn update_low_watermark_locked(&self, guard: &HashMap<u64, Timestamp>) {
        let lw = guard
            .values()
            .map(|ts| ts.raw())
            .min()
            .unwrap_or_else(|| global_timestamp_generator().current().raw());
        self.low_watermark_raw.store(lw, Ordering::SeqCst);
    }

    pub(crate) fn finish_transaction(&self, txn: &CatalogTxn) -> CatalogTxnResult<()> {
        let mut guard = self.active.write().expect("poisoned active set");
        guard.remove(&txn.txn_id().raw());
        self.update_low_watermark_locked(&guard);
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct CatalogTxnManager {
    inner: Arc<CatalogTxnManagerInner>,
}

impl Default for CatalogTxnManager {
    fn default() -> Self {
        Self::new()
    }
}

impl CatalogTxnManager {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(CatalogTxnManagerInner::new()),
        }
    }

    pub fn inner(&self) -> Arc<CatalogTxnManagerInner> {
        self.inner.clone()
    }
}

impl GraphTxnManager for CatalogTxnManager {
    type Error = CatalogTxnError;
    type GraphContext = CatalogRuntime;
    type Transaction = CatalogTxn;

    fn begin_transaction(
        &self,
        isolation_level: IsolationLevel,
    ) -> Result<Arc<Self::Transaction>, Self::Error> {
        let txn_id = global_transaction_id_generator().next()?;
        let start_ts = global_timestamp_generator().next()?;

        let txn = Arc::new(CatalogTxn::new(
            txn_id,
            start_ts,
            isolation_level,
            Arc::downgrade(&self.inner),
        ));

        let mut guard = self.inner.active.write().expect("poisoned active set");
        guard.insert(txn_id.raw(), start_ts);
        self.inner.update_low_watermark_locked(&guard);

        Ok(txn)
    }

    fn finish_transaction(&self, txn: &Self::Transaction) -> Result<(), Self::Error> {
        self.inner.finish_transaction(txn)
    }

    fn garbage_collect(&self, _graph: &Self::GraphContext) -> Result<(), Self::Error> {
        // TODO: Not yet implemented.
        Ok(())
    }

    fn low_watermark(&self) -> Timestamp {
        Timestamp::with_ts(self.inner.low_watermark_raw.load(Ordering::SeqCst))
    }
}
