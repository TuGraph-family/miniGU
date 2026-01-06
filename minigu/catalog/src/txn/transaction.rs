use std::hash::Hash;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, Weak};

use minigu_common::{IsolationLevel, Timestamp, global_timestamp_generator};
use parking_lot::RawMutex;
use parking_lot::lock_api::ArcMutexGuard;

use crate::txn::error::{CatalogTxnError, CatalogTxnResult};
use crate::txn::manager::CatalogTxnManager;
use crate::txn::versioned::{CatalogVersionNode, CommitPlan, Entry, VersionedMap, WriteOp};

/// Unified abstraction of "the write sets touched by this transaction" - two-phase interface.
///
/// This is closer to a transaction "write set / write intents" than a generic access list:
/// the implementor is expected to validate and then apply its buffered writes at commit time.
trait TxnWriteSet: Send + Sync + std::fmt::Debug {
    fn validate(&self) -> CatalogTxnResult<()>;
    fn apply(&self, commit_ts: Timestamp) -> CatalogTxnResult<()>;
    fn abort(&self) -> CatalogTxnResult<()>;
}

/// Per-map write-set implementation for `VersionedMap<K, V>`.
#[derive(Debug)]
struct VersionedMapWriteSet<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static + std::fmt::Debug,
    V: Send + Sync + 'static + std::fmt::Debug,
{
    map: Weak<VersionedMap<K, V>>,
    entries: Vec<Entry<K, V>>,
    plans: Mutex<Option<Vec<CommitPlan<K, V>>>>,
}

impl<K, V> TxnWriteSet for VersionedMapWriteSet<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static + std::fmt::Debug,
    V: Send + Sync + 'static + std::fmt::Debug,
{
    fn validate(&self) -> CatalogTxnResult<()> {
        if let Some(map) = self.map.upgrade() {
            let plans = map.validate_batch(&self.entries)?;
            let mut slot = self.plans.lock().expect("plans mutex poisoned");
            *slot = Some(plans);
            Ok(())
        } else {
            Ok(())
        }
    }

    fn apply(&self, commit_ts: Timestamp) -> CatalogTxnResult<()> {
        if let Some(map) = self.map.upgrade() {
            if let Some(plans) = self.plans.lock().expect("plans mutex poisoned").as_ref() {
                map.apply_batch(plans, commit_ts)
            } else {
                Err(CatalogTxnError::IllegalState {
                    reason: "apply without prior validate".into(),
                })
            }
        } else {
            Ok(())
        }
    }

    fn abort(&self) -> CatalogTxnResult<()> {
        if let Some(map) = self.map.upgrade() {
            map.abort_batch(&self.entries)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
/// Catalog transaction object.
pub struct CatalogTxn {
    txn_id: Timestamp,
    start_ts: Timestamp,
    commit_ts: AtomicU64, // 0 means not committed yet.
    isolation_level: IsolationLevel,
    write_sets: Mutex<Vec<Box<dyn TxnWriteSet>>>,
    hooks: Mutex<Vec<Box<dyn TxnHook>>>, // Record the hooks for pre-commit validation.
    mgr: Weak<CatalogTxnManager>,
    op_mutex: Mutex<()>, // commit/abort mutex.
}

impl CatalogTxn {
    pub(crate) fn new(
        txn_id: Timestamp,
        start_ts: Timestamp,
        isolation: IsolationLevel,
        mgr: Weak<CatalogTxnManager>,
    ) -> Self {
        Self {
            txn_id,
            start_ts,
            commit_ts: AtomicU64::new(0),
            isolation_level: isolation,
            write_sets: Mutex::new(Vec::new()),
            hooks: Mutex::new(Vec::new()),
            mgr,
            op_mutex: Mutex::new(()),
        }
    }

    /// Record a set of writes to a `VersionedMap` for subsequent batch commit/abort.
    pub fn record_versioned_map_writes<K, V>(
        &self,
        map: &Arc<VersionedMap<K, V>>,
        entries: Vec<Entry<K, V>>,
    ) where
        K: Eq + Hash + Clone + Send + Sync + 'static + std::fmt::Debug,
        V: Send + Sync + 'static + std::fmt::Debug,
    {
        let write_set = VersionedMapWriteSet {
            map: Arc::downgrade(map),
            entries,
            plans: Mutex::new(None),
        };
        self.write_sets
            .lock()
            .expect("poisoned write_sets mutex")
            .push(Box::new(write_set));
    }

    /// Transaction-level hook registration (e.g., consistency checks before commit).
    pub fn add_hook(&self, hook: Box<dyn TxnHook>) {
        self.hooks.lock().expect("poisoned hooks mutex").push(hook);
    }

    /// Whether this transaction runs under Serializable isolation
    #[inline]
    pub fn is_serializable(&self) -> bool {
        matches!(self.isolation_level, IsolationLevel::Serializable)
    }

    /// Construct and record a single write entry.
    pub fn record_write<K, V>(
        &self,
        map: &Arc<VersionedMap<K, V>>,
        key: K,
        node: Arc<CatalogVersionNode<V>>,
        op: WriteOp,
    ) where
        K: Eq + Hash + Clone + Send + Sync + 'static + std::fmt::Debug,
        V: Send + Sync + 'static + std::fmt::Debug,
    {
        let entry = Entry { key, node, op };
        self.record_versioned_map_writes(map, vec![entry]);
    }

    pub fn txn_id(&self) -> Timestamp {
        self.txn_id
    }

    pub fn start_ts(&self) -> Timestamp {
        self.start_ts
    }

    pub fn commit_ts(&self) -> Option<Timestamp> {
        let raw = self.commit_ts.load(Ordering::SeqCst);
        if raw == 0 {
            None
        } else {
            Some(Timestamp::with_ts(raw))
        }
    }

    pub fn isolation_level(&self) -> &IsolationLevel {
        &self.isolation_level
    }

    /// Prepare this transaction for commit by running precommit hooks and validating write sets.
    ///
    /// This method acquires a global commit lock to prevent other catalog transactions from
    /// committing between validation and apply. The returned guard must be consumed via
    /// [`PreparedCatalogTxn::apply`] to finalize the commit.
    pub fn prepare(&self) -> Result<PreparedCatalogTxn<'_>, CatalogTxnError> {
        if self.commit_ts().is_some() {
            return Err(CatalogTxnError::IllegalState {
                reason: "transaction already committed".into(),
            });
        }

        let mgr = self
            .mgr
            .upgrade()
            .ok_or_else(|| CatalogTxnError::IllegalState {
                reason: "transaction manager dropped".into(),
            })?;
        let commit_guard = mgr.commit_lock().lock_arc();
        let op_guard = self.op_mutex.lock().expect("op mutex poisoned");

        {
            let hooks = self.hooks.lock().expect("poisoned hooks mutex");
            for h in hooks.iter() {
                h.precommit(self)?;
            }
        }

        {
            let write_sets = self.write_sets.lock().expect("poisoned write_sets mutex");
            for set in write_sets.iter() {
                set.validate()?;
            }
        }

        Ok(PreparedCatalogTxn {
            txn: self,
            _commit_guard: commit_guard,
            _op_guard: op_guard,
        })
    }

    pub fn commit_at(&self, commit_ts: Timestamp) -> Result<Timestamp, CatalogTxnError> {
        global_timestamp_generator().update_if_greater(commit_ts)?;
        self.prepare()?.apply(commit_ts)
    }

    pub fn commit(&self) -> Result<Timestamp, CatalogTxnError> {
        let prepared = self.prepare()?;
        let commit_ts = global_timestamp_generator().next()?;
        prepared.apply(commit_ts)
    }

    pub fn abort(&self) -> Result<(), CatalogTxnError> {
        let _guard = self.op_mutex.lock().unwrap_or_else(|e| e.into_inner());

        {
            let write_sets = self.write_sets.lock().unwrap_or_else(|e| e.into_inner());
            for set in write_sets.iter().rev() {
                set.abort()?;
            }
        }

        if let Some(mgr) = self.mgr.upgrade() {
            mgr.finish_transaction(self)?;
        }

        Ok(())
    }
}

impl Drop for CatalogTxn {
    fn drop(&mut self) {
        let commit_ts = self.commit_ts();
        if commit_ts.is_some() {
            return;
        }
        {
            let _guard = self.op_mutex.lock().unwrap_or_else(|e| e.into_inner());

            let write_sets = self.write_sets.lock().unwrap_or_else(|e| e.into_inner());
            for set in write_sets.iter().rev() {
                let _ = set.abort();
            }

            if let Some(mgr) = self.mgr.upgrade() {
                let _ = mgr.finish_transaction(self);
            }
        }
    }
}

/// Transaction hook interface exposed to external users (e.g., pre-commit checks).
pub trait TxnHook: Send + Sync + std::fmt::Debug {
    fn precommit(&self, txn: &CatalogTxn) -> CatalogTxnResult<()>;
}

/// RAII guard representing a catalog transaction that has been successfully validated for commit.
///
/// Holding this guard prevents other catalog transactions from committing, ensuring that
/// serializable read-validation hooks remain valid until the changes are applied.
pub struct PreparedCatalogTxn<'a> {
    txn: &'a CatalogTxn,
    _commit_guard: ArcMutexGuard<RawMutex, ()>,
    _op_guard: MutexGuard<'a, ()>,
}

impl PreparedCatalogTxn<'_> {
    pub fn apply(self, commit_ts: Timestamp) -> Result<Timestamp, CatalogTxnError> {
        {
            let write_sets = self
                .txn
                .write_sets
                .lock()
                .expect("poisoned write_sets mutex");
            for set in write_sets.iter() {
                set.apply(commit_ts)?;
            }
        }

        self.txn.commit_ts.store(commit_ts.raw(), Ordering::SeqCst);

        if let Some(mgr) = self.txn.mgr.upgrade() {
            mgr.finish_transaction(self.txn)?;
        }

        Ok(commit_ts)
    }
}

/// View trait to allow accepting either `CatalogTxn` or higher-level `Transaction`.
pub trait CatalogTxnView {
    fn catalog_txn(&self) -> &CatalogTxn;
}

impl CatalogTxnView for CatalogTxn {
    fn catalog_txn(&self) -> &CatalogTxn {
        self
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex, mpsc};
    use std::thread;
    use std::time::Duration;

    use minigu_common::timestamp::global_timestamp_generator;
    use minigu_common::{IsolationLevel, Timestamp};

    use super::{CatalogTxn, TxnHook, TxnWriteSet, VersionedMapWriteSet};
    use crate::txn::manager::CatalogTxnManager;
    use crate::txn::versioned::{Entry, VersionedMap, WriteOp};
    use crate::txn::{CatalogTxnError, CatalogTxnResult};

    fn mk_txn_id(raw: u64) -> Timestamp {
        Timestamp::with_ts(Timestamp::TXN_ID_START + raw)
    }

    fn mk_start_ts(raw: u64) -> Timestamp {
        // commit-ts domain
        Timestamp::with_ts(raw)
    }

    #[test]
    fn test_transaction_basic_invariants() {
        let mgr = CatalogTxnManager::new();
        let txn_id = mk_txn_id(42);
        let start_ts = mk_start_ts(100);
        let txn = CatalogTxn::new(
            txn_id,
            start_ts,
            IsolationLevel::Serializable,
            Arc::downgrade(&mgr),
        );

        assert_eq!(txn.txn_id(), txn_id);
        assert_eq!(txn.start_ts(), start_ts);
        assert!(txn.commit_ts().is_none());
        assert!(matches!(
            txn.isolation_level(),
            IsolationLevel::Serializable
        ));
        assert!(txn.is_serializable());

        let txn2 = CatalogTxn::new(
            mk_txn_id(43),
            mk_start_ts(101),
            IsolationLevel::Snapshot,
            Arc::downgrade(&mgr),
        );
        assert!(!txn2.is_serializable());
    }

    #[test]
    fn test_record_write_commit_visible() {
        let mgr = CatalogTxnManager::new();
        let map: Arc<VersionedMap<String, u64>> = Arc::new(VersionedMap::new());

        let txn = mgr
            .begin_transaction(IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");
        let node = map
            .put("k".to_string(), Arc::new(1), txn.as_ref())
            .expect("put should succeed");
        txn.record_write(&map, "k".to_string(), node, WriteOp::Create);

        let commit_ts = txn.commit().expect("commit should succeed");
        assert_eq!(txn.commit_ts(), Some(commit_ts));

        let read_txn = mgr
            .begin_transaction(IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");
        let val = map.get(&"k".to_string(), read_txn.as_ref());
        assert_eq!(val.map(|v| *v), Some(1));
        read_txn.abort().ok();
    }

    #[test]
    fn test_record_write_abort_invisible() {
        let mgr = CatalogTxnManager::new();
        let map: Arc<VersionedMap<String, u64>> = Arc::new(VersionedMap::new());

        let txn = mgr
            .begin_transaction(IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");
        let node = map
            .put("k".to_string(), Arc::new(1), txn.as_ref())
            .expect("put should succeed");
        txn.record_write(&map, "k".to_string(), node, WriteOp::Create);

        txn.abort().expect("abort should succeed");
        assert!(txn.commit_ts().is_none());

        let read_txn = mgr
            .begin_transaction(IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");
        let val = map.get(&"k".to_string(), read_txn.as_ref());
        assert!(val.is_none());
        read_txn.abort().ok();
    }

    #[test]
    fn test_drop_is_implicit_abort_and_advances_low_watermark() {
        let mgr = CatalogTxnManager::new();
        let map: Arc<VersionedMap<String, u64>> = Arc::new(VersionedMap::new());

        let txn = mgr
            .begin_transaction(IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");
        let start_ts = txn.start_ts();
        assert_eq!(mgr.low_watermark(), start_ts);

        let node = map
            .put("k".to_string(), Arc::new(1), txn.as_ref())
            .expect("put should succeed");
        txn.record_write(&map, "k".to_string(), node, WriteOp::Create);

        drop(txn);

        assert!(
            mgr.low_watermark().raw() > start_ts.raw(),
            "low watermark should advance after dropping the last active txn"
        );

        let read_txn = mgr
            .begin_transaction(IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");
        let val = map.get(&"k".to_string(), read_txn.as_ref());
        assert!(val.is_none());
        read_txn.abort().ok();
    }

    #[test]
    fn test_prepare_without_apply_is_not_visible_and_commit_ts_stays_none() {
        let mgr = CatalogTxnManager::new();
        let map: Arc<VersionedMap<String, u64>> = Arc::new(VersionedMap::new());

        let txn = mgr
            .begin_transaction(IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");
        let node = map
            .put("k".to_string(), Arc::new(1), txn.as_ref())
            .expect("put should succeed");
        txn.record_write(&map, "k".to_string(), node, WriteOp::Create);

        let prepared = txn.prepare().expect("prepare should succeed");
        drop(prepared);

        assert!(
            txn.commit_ts().is_none(),
            "commit_ts must remain None until apply"
        );

        let read_txn = mgr
            .begin_transaction(IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");
        let val = map.get(&"k".to_string(), read_txn.as_ref());
        assert!(
            val.is_none(),
            "prepared-but-not-applied writes must be invisible"
        );
        read_txn.abort().ok();

        txn.abort().ok();
    }

    #[test]
    fn test_write_set_apply_without_validate_is_illegal_state() {
        let map: Arc<VersionedMap<String, u64>> = Arc::new(VersionedMap::new());
        let write_set = VersionedMapWriteSet::<String, u64> {
            map: Arc::downgrade(&map),
            entries: Vec::new(),
            plans: Mutex::new(None),
        };

        let err = write_set
            .apply(mk_start_ts(1))
            .expect_err("apply without validate should fail");
        assert!(matches!(
            err,
            CatalogTxnError::IllegalState { ref reason } if reason == "apply without prior validate"
        ));
    }

    #[derive(Debug)]
    struct AlwaysFailHook;

    impl TxnHook for AlwaysFailHook {
        fn precommit(&self, _txn: &CatalogTxn) -> CatalogTxnResult<()> {
            Err(CatalogTxnError::ReferentialIntegrity {
                reason: "hook failed".to_string(),
            })
        }
    }

    #[test]
    fn test_precommit_hook_failure_aborts_commit_and_no_apply() {
        let mgr = CatalogTxnManager::new();
        let map: Arc<VersionedMap<String, u64>> = Arc::new(VersionedMap::new());

        let txn = mgr
            .begin_transaction(IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");
        txn.add_hook(Box::new(AlwaysFailHook));

        let node = map
            .put("k".to_string(), Arc::new(1), txn.as_ref())
            .expect("put should succeed");
        txn.record_write(&map, "k".to_string(), node, WriteOp::Create);

        let res = txn.commit();
        assert!(matches!(
            res,
            Err(CatalogTxnError::ReferentialIntegrity { .. })
        ));
        drop(txn);

        let read_txn = mgr
            .begin_transaction(IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");
        let val = map.get(&"k".to_string(), read_txn.as_ref());
        assert!(val.is_none(), "failed commit must not publish writes");
        read_txn.abort().ok();
    }

    #[test]
    fn test_commit_then_commit_or_prepare_is_illegal_state() {
        let mgr = CatalogTxnManager::new();
        let txn = mgr
            .begin_transaction(IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");

        let commit_ts_1 = txn.commit().expect("commit should succeed");
        assert_eq!(txn.commit_ts(), Some(commit_ts_1));

        let commit_again = txn.commit();
        assert!(matches!(
            commit_again,
            Err(CatalogTxnError::IllegalState { .. })
        ));
        assert_eq!(txn.commit_ts(), Some(commit_ts_1));

        let prepare_again = txn.prepare();
        assert!(matches!(
            prepare_again,
            Err(CatalogTxnError::IllegalState { .. })
        ));
        assert_eq!(txn.commit_ts(), Some(commit_ts_1));
    }

    #[derive(Debug)]
    struct ProbeWriteSet {
        id: &'static str,
        log: Arc<Mutex<Vec<String>>>,
    }

    impl ProbeWriteSet {
        fn new(id: &'static str, log: Arc<Mutex<Vec<String>>>) -> Self {
            Self { id, log }
        }
    }

    impl TxnWriteSet for ProbeWriteSet {
        fn validate(&self) -> CatalogTxnResult<()> {
            self.log
                .lock()
                .unwrap()
                .push(format!("validate:{}", self.id));
            Ok(())
        }

        fn apply(&self, _commit_ts: Timestamp) -> CatalogTxnResult<()> {
            self.log.lock().unwrap().push(format!("apply:{}", self.id));
            Ok(())
        }

        fn abort(&self) -> CatalogTxnResult<()> {
            self.log.lock().unwrap().push(format!("abort:{}", self.id));
            Ok(())
        }
    }

    #[test]
    fn test_write_set_ordering_validate_apply_and_abort_reverse() {
        let mgr = CatalogTxnManager::new();

        // validate/apply ordering
        let log = Arc::new(Mutex::new(Vec::<String>::new()));
        let txn = mgr
            .begin_transaction(IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");
        {
            let mut ws = txn.write_sets.lock().unwrap();
            ws.push(Box::new(ProbeWriteSet::new("A", Arc::clone(&log))));
            ws.push(Box::new(ProbeWriteSet::new("B", Arc::clone(&log))));
        }

        let prepared = txn.prepare().expect("prepare should succeed");
        let commit_ts = global_timestamp_generator().next().expect("commit ts");
        prepared.apply(commit_ts).expect("apply should succeed");

        let got = log.lock().unwrap().clone();
        assert_eq!(got, vec!["validate:A", "validate:B", "apply:A", "apply:B"]);

        // abort ordering (reverse)
        let log2 = Arc::new(Mutex::new(Vec::<String>::new()));
        let txn2 = mgr
            .begin_transaction(IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");
        {
            let mut ws = txn2.write_sets.lock().unwrap();
            ws.push(Box::new(ProbeWriteSet::new("A", Arc::clone(&log2))));
            ws.push(Box::new(ProbeWriteSet::new("B", Arc::clone(&log2))));
        }
        txn2.abort().expect("abort should succeed");

        let got2 = log2.lock().unwrap().clone();
        assert_eq!(got2, vec!["abort:B", "abort:A"]);
    }

    #[test]
    fn test_commit_lock_serializes_concurrent_commits() {
        let mgr = CatalogTxnManager::new();

        let txn1 = mgr
            .begin_transaction(IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");
        let prepared = txn1.prepare().expect("prepare should succeed");

        let (ready_tx, ready_rx) = mpsc::channel::<()>();
        let (done_tx, done_rx) = mpsc::channel::<Timestamp>();
        let mgr2 = Arc::clone(&mgr);

        let handle = thread::spawn(move || {
            ready_tx.send(()).unwrap();
            let txn2 = mgr2
                .begin_transaction(IsolationLevel::Serializable)
                .expect("begin_transaction should succeed");
            let ts = txn2.commit().expect("commit should succeed");
            done_tx.send(ts).unwrap();
        });

        ready_rx
            .recv_timeout(Duration::from_secs(1))
            .expect("worker should be ready");

        // While txn1 holds the commit lock via `prepared`, txn2 should be blocked in commit().
        assert!(
            done_rx.recv_timeout(Duration::from_millis(50)).is_err(),
            "txn2 commit should be blocked until txn1 apply releases commit lock"
        );

        let commit_ts1 = global_timestamp_generator().next().expect("commit ts");
        prepared.apply(commit_ts1).expect("apply should succeed");

        let commit_ts2 = done_rx
            .recv_timeout(Duration::from_secs(2))
            .expect("txn2 should complete after txn1 apply");

        assert!(
            commit_ts2 > commit_ts1,
            "commit timestamps should reflect serialized commit order"
        );

        handle.join().expect("worker thread join");
    }

    // Ensure `VersionedMapWriteSet` can still be constructed with entries and validated/applied.
    #[test]
    fn test_versioned_map_write_set_validate_apply_roundtrip() {
        let mgr = CatalogTxnManager::new();
        let map: Arc<VersionedMap<String, u64>> = Arc::new(VersionedMap::new());
        let txn = mgr
            .begin_transaction(IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");
        let node = map
            .put("k".to_string(), Arc::new(1), txn.as_ref())
            .expect("put should succeed");
        let entries = vec![Entry {
            key: "k".to_string(),
            node,
            op: WriteOp::Create,
        }];
        let ws = VersionedMapWriteSet::<String, u64> {
            map: Arc::downgrade(&map),
            entries,
            plans: Mutex::new(None),
        };
        ws.validate().expect("validate should succeed");
        ws.apply(mk_start_ts(10)).expect("apply should succeed");
    }
}
