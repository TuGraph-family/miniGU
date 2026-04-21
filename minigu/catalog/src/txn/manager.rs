use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};

use minigu_common::{
    IsolationLevel, Timestamp, global_timestamp_generator, global_transaction_id_generator,
};
use parking_lot::Mutex;

use crate::txn::error::{CatalogTxnError, CatalogTxnResult};
use crate::txn::transaction::CatalogTxn;

#[derive(Debug)]
pub struct CatalogTxnManager {
    active_txns: RwLock<HashMap<u64, Timestamp>>, // txn_id.raw() -> start_ts
    watermark: AtomicU64,
    commit_lock: Arc<Mutex<()>>,
}

impl CatalogTxnManager {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            active_txns: RwLock::new(HashMap::new()),
            watermark: AtomicU64::new(0),
            commit_lock: Arc::new(Mutex::new(())),
        })
    }

    pub(crate) fn commit_lock(&self) -> Arc<Mutex<()>> {
        Arc::clone(&self.commit_lock)
    }

    fn update_watermark(&self, guard: &HashMap<u64, Timestamp>) {
        let lw = guard
            .values()
            .map(|ts| ts.raw())
            .min()
            .unwrap_or_else(|| global_timestamp_generator().current().raw());
        self.watermark.store(lw, Ordering::SeqCst);
    }

    pub fn finish_transaction(&self, txn: &CatalogTxn) -> CatalogTxnResult<()> {
        let mut guard = self.active_txns.write().expect("poisoned active set");
        guard.remove(&txn.txn_id().raw());
        self.update_watermark(&guard);
        Ok(())
    }

    pub fn begin_transaction(
        self: &Arc<Self>,
        isolation_level: IsolationLevel,
    ) -> Result<Arc<CatalogTxn>, CatalogTxnError> {
        self.begin_transaction_at(None, None, isolation_level)
    }

    pub fn begin_transaction_at(
        self: &Arc<Self>,
        txn_id: Option<Timestamp>,
        start_ts: Option<Timestamp>,
        isolation_level: IsolationLevel,
    ) -> Result<Arc<CatalogTxn>, CatalogTxnError> {
        let txn_id = if let Some(id) = txn_id {
            global_transaction_id_generator().update_if_greater(id)?;
            id
        } else {
            global_transaction_id_generator().next()?
        };
        let start_ts = if let Some(ts) = start_ts {
            global_timestamp_generator().update_if_greater(ts)?;
            ts
        } else {
            global_timestamp_generator().next()?
        };

        let txn = Arc::new(CatalogTxn::new(
            txn_id,
            start_ts,
            isolation_level,
            Arc::downgrade(self),
        ));

        let mut guard = self.active_txns.write().expect("poisoned active set");
        guard.insert(txn_id.raw(), start_ts);
        self.update_watermark(&guard);

        Ok(txn)
    }

    /// TODO: implement garbage collection for catalog transactions
    pub fn garbage_collect(&self) -> Result<(), CatalogTxnError> {
        Ok(())
    }

    pub fn low_watermark(&self) -> Timestamp {
        Timestamp::with_ts(self.watermark.load(Ordering::SeqCst))
    }
}
