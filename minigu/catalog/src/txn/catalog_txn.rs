use std::hash::Hash;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, Weak};

use minigu_transaction::timestamp::Timestamp;
use minigu_transaction::{IsolationLevel, Transaction, global_timestamp_generator};

use crate::txn::error::{CatalogTxnError, CatalogTxnResult};
use crate::txn::manager::CatalogTxnManagerInner;
use crate::txn::versioned::CatalogVersionNode;
use crate::txn::versioned_map::{CommitPlan, TouchedItem, VersionedMap, WriteOp};

fn encode_commit_ts(opt: Option<Timestamp>) -> u64 {
    match opt {
        Some(ts) => ts.raw(),
        None => 0,
    }
}

fn decode_commit_ts(raw: u64) -> Option<Timestamp> {
    if raw == 0 {
        None
    } else {
        Some(Timestamp::with_ts(raw))
    }
}

/// Unified abstraction of "the containers touched by this transaction" - two-phase interface.
trait TxnTouchedSet: Send + Sync + std::fmt::Debug {
    fn validate(&self) -> CatalogTxnResult<()>;
    fn apply(&self, commit_ts: Timestamp) -> CatalogTxnResult<()>;
    fn abort(&self) -> CatalogTxnResult<()>;
}

/// Touched-set implementation for `VersionedMap<K, V>`.
#[derive(Debug)]
struct VersionedMapTouched<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static + std::fmt::Debug,
    V: Send + Sync + 'static + std::fmt::Debug,
{
    map: Weak<VersionedMap<K, V>>,
    items: Vec<TouchedItem<K, V>>,
    plans: Mutex<Option<Vec<CommitPlan<K, V>>>>,
}

impl<K, V> TxnTouchedSet for VersionedMapTouched<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static + std::fmt::Debug,
    V: Send + Sync + 'static + std::fmt::Debug,
{
    fn validate(&self) -> CatalogTxnResult<()> {
        if let Some(map) = self.map.upgrade() {
            let plans = map.validate_batch(&self.items)?;
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
            map.abort_batch(&self.items)
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
    commit_ts_raw: AtomicU64, // 0 means not committed yet.
    isolation: IsolationLevel,
    touched: Mutex<Vec<Box<dyn TxnTouchedSet>>>, // Record the touched containers.
    hooks: Mutex<Vec<Box<dyn TxnHook>>>,         // Record the hooks for pre-commit validation.
    mgr: Weak<CatalogTxnManagerInner>,
    op_mutex: Mutex<()>, // commit/abort mutex.
}

impl CatalogTxn {
    pub(crate) fn new(
        txn_id: Timestamp,
        start_ts: Timestamp,
        isolation: IsolationLevel,
        mgr: Weak<CatalogTxnManagerInner>,
    ) -> Self {
        Self {
            txn_id,
            start_ts,
            commit_ts_raw: AtomicU64::new(0),
            isolation,
            touched: Mutex::new(Vec::new()),
            hooks: Mutex::new(Vec::new()),
            mgr,
            op_mutex: Mutex::new(()),
        }
    }

    /// Record a set of writes to a `VersionedMap` for subsequent batch commit/abort.
    pub fn record_versioned_map_writes<K, V>(
        &self,
        map: &Arc<VersionedMap<K, V>>,
        items: Vec<TouchedItem<K, V>>,
    ) where
        K: Eq + Hash + Clone + Send + Sync + 'static + std::fmt::Debug,
        V: Send + Sync + 'static + std::fmt::Debug,
    {
        let touched = VersionedMapTouched {
            map: Arc::downgrade(map),
            items,
            plans: Mutex::new(None),
        };
        self.touched
            .lock()
            .expect("poisoned touched mutex")
            .push(Box::new(touched));
    }

    /// Transaction-level hook registration (e.g., consistency checks before commit).
    pub fn add_hook(&self, hook: Box<dyn TxnHook>) {
        self.hooks.lock().expect("poisoned hooks mutex").push(hook);
    }

    /// Convenience: whether this transaction runs under Serializable isolation
    #[inline]
    pub fn is_serializable(&self) -> bool {
        matches!(self.isolation, IsolationLevel::Serializable)
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
        let item = TouchedItem { key, node, op };
        self.record_versioned_map_writes(map, vec![item]);
    }
}

impl Transaction for CatalogTxn {
    type Error = CatalogTxnError;

    fn txn_id(&self) -> Timestamp {
        self.txn_id
    }

    fn start_ts(&self) -> Timestamp {
        self.start_ts
    }

    fn commit_ts(&self) -> Option<Timestamp> {
        decode_commit_ts(self.commit_ts_raw.load(Ordering::SeqCst))
    }

    fn isolation_level(&self) -> &IsolationLevel {
        &self.isolation
    }

    fn commit(&self) -> Result<Timestamp, Self::Error> {
        let _guard = self.op_mutex.lock().expect("op mutex poisoned");

        // Pre-commit hooks.
        {
            let hooks = self.hooks.lock().expect("poisoned hooks mutex");
            for h in hooks.iter() {
                h.precommit(self)?;
            }
        }

        // Assign commit_ts (only write after successful application).
        let commit_ts = global_timestamp_generator().next()?;

        // Phase one: validate all containers.
        {
            let touched = self.touched.lock().expect("poisoned touched mutex");
            for set in touched.iter() {
                set.validate()?;
            }
        }

        // Phase two: apply all containers.
        {
            let touched = self.touched.lock().expect("poisoned touched mutex");
            for set in touched.iter() {
                set.apply(commit_ts)?;
            }
        }

        // Write commit_ts and state.
        self.commit_ts_raw
            .store(encode_commit_ts(Some(commit_ts)), Ordering::SeqCst);

        // Remove from active set.
        if let Some(mgr) = self.mgr.upgrade() {
            mgr.finish_transaction(self)?;
        }

        Ok(commit_ts)
    }

    fn abort(&self) -> Result<(), Self::Error> {
        let _guard = self.op_mutex.lock().expect("op mutex poisoned");

        // Rollback each container (in reverse order).
        {
            let touched = self.touched.lock().expect("poisoned touched mutex");
            for set in touched.iter().rev() {
                set.abort()?;
            }
        }

        if let Some(mgr) = self.mgr.upgrade() {
            mgr.finish_transaction(self)?;
        }

        Ok(())
    }
}

/// Transaction hook interface exposed to external users (e.g., pre-commit checks).
pub trait TxnHook: Send + Sync + std::fmt::Debug {
    fn precommit(&self, txn: &CatalogTxn) -> CatalogTxnResult<()>;
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use minigu_transaction::GraphTxnManager;

    use super::*;
    use crate::txn::manager::CatalogTxnManager;
    use crate::txn::versioned_map::{VersionedMap, WriteOp};

    fn begin_txn(mgr: &CatalogTxnManager) -> Arc<CatalogTxn> {
        mgr.begin_transaction(minigu_transaction::IsolationLevel::Snapshot)
            .expect("begin txn")
    }

    #[test]
    fn commit_should_make_data_visible() {
        let mgr = CatalogTxnManager::new();
        let map: Arc<VersionedMap<String, i32>> = Arc::new(VersionedMap::new());

        // txn1 writes a record
        let txn1 = begin_txn(&mgr);
        let key = "foo".to_string();
        assert!(map.get(&key, &txn1).is_none());

        // write node (simulate put)
        let node = map.put(key.clone(), Arc::new(10), &txn1).expect("put ok");
        txn1.record_write(&map, key.clone(), node, WriteOp::Create);

        // commit transaction
        let commit_ts = txn1.commit().expect("commit ok");
        assert!(commit_ts.raw() > 0);

        // new transaction should see the committed value
        let txn2 = begin_txn(&mgr);
        let val = map.get(&key, &txn2).unwrap();
        assert_eq!(*val, 10);
    }

    #[test]
    fn abort_should_discard_uncommitted_data() {
        let mgr = CatalogTxnManager::new();
        let map: Arc<VersionedMap<String, i32>> = Arc::new(VersionedMap::new());

        // txn1 writes but does not commit
        let txn1 = begin_txn(&mgr);
        let key = "bar".to_string();
        let node = map.put(key.clone(), Arc::new(99), &txn1).expect("put ok");
        txn1.record_write(&map, key.clone(), node, WriteOp::Create);

        // abort transaction
        txn1.abort().expect("abort ok");

        // new transaction should not see the aborted value
        let txn2 = begin_txn(&mgr);
        assert!(map.get(&key, &txn2).is_none());
    }

    #[test]
    fn cross_container_commit_should_be_atomic() {
        // simulate two independent VersionedMaps
        let mgr = CatalogTxnManager::new();
        let map_a: Arc<VersionedMap<String, i32>> = Arc::new(VersionedMap::new());
        let map_b: Arc<VersionedMap<String, i32>> = Arc::new(VersionedMap::new());

        let txn = begin_txn(&mgr);
        let node_a = map_a
            .put("a".to_string(), Arc::new(1), &txn)
            .expect("put A");
        txn.record_write(&map_a, "a".to_string(), node_a, WriteOp::Create);

        let node_b = map_b
            .put("b".to_string(), Arc::new(2), &txn)
            .expect("put B");
        txn.record_write(&map_b, "b".to_string(), node_b, WriteOp::Create);

        // commit should succeed and both containers should see committed values
        txn.commit().expect("commit ok");

        let check_txn = begin_txn(&mgr);
        assert_eq!(map_a.get(&"a".to_string(), &check_txn).map(|v| *v), Some(1));
        assert_eq!(map_b.get(&"b".to_string(), &check_txn).map(|v| *v), Some(2));
    }

    #[test]
    fn concurrent_txns_should_isolate_uncommitted_changes() {
        let mgr = CatalogTxnManager::new();
        let map: Arc<VersionedMap<String, i32>> = Arc::new(VersionedMap::new());
        let key = "k1".to_string();

        // txn1 starts first
        let txn1 = begin_txn(&mgr);
        let node1 = map
            .put(key.clone(), Arc::new(100), &txn1)
            .expect("txn1 put ok");
        txn1.record_write(&map, key.clone(), node1, WriteOp::Create);

        // txn2 starts while txn1 is still uncommitted
        let txn2 = begin_txn(&mgr);
        // txn2 should not see txn1's uncommitted write
        assert!(map.get(&key, &txn2).is_none());

        // commit txn1
        txn1.commit().expect("txn1 commit ok");

        // txn2 should still not see txn1's commit (snapshot isolation)
        assert!(map.get(&key, &txn2).is_none());

        // a new transaction should see txn1's committed value
        let txn3 = begin_txn(&mgr);
        let val = map.get(&key, &txn3).unwrap();
        assert_eq!(*val, 100);
    }

    #[test]
    fn two_txns_writing_same_key_should_conflict() {
        let mgr = CatalogTxnManager::new();
        let map: Arc<VersionedMap<String, i32>> = Arc::new(VersionedMap::new());
        let key = "x".to_string();

        // txn1 writes first
        let txn1 = begin_txn(&mgr);
        let node1 = map
            .put(key.clone(), Arc::new(1), &txn1)
            .expect("txn1 put ok");
        txn1.record_write(&map, key.clone(), node1, WriteOp::Create);

        // txn2 tries to write the same key before txn1 commits
        let txn2 = begin_txn(&mgr);
        let node2 = map.put(key.clone(), Arc::new(2), &txn2);
        assert!(
            node2.is_err(),
            "txn2 should be blocked by txn1 uncommitted version"
        );

        // txn1 commits successfully
        txn1.commit().expect("txn1 commit ok");
    }

    #[test]
    fn interleaved_commit_and_abort_should_keep_map_consistent() {
        let mgr = CatalogTxnManager::new();
        let map: Arc<VersionedMap<String, i32>> = Arc::new(VersionedMap::new());
        let key1 = "key1".to_string();
        let key2 = "key2".to_string();

        // txn1 writes to key1
        let txn1 = begin_txn(&mgr);
        let node1 = map
            .put(key1.clone(), Arc::new(10), &txn1)
            .expect("txn1 put ok");
        txn1.record_write(&map, key1.clone(), node1, WriteOp::Create);

        // txn2 writes to key2
        let txn2 = begin_txn(&mgr);
        let node2 = map
            .put(key2.clone(), Arc::new(20), &txn2)
            .expect("txn2 put ok");
        txn2.record_write(&map, key2.clone(), node2, WriteOp::Create);

        // txn1 aborts, txn2 commits
        txn1.abort().expect("txn1 abort ok");
        txn2.commit().expect("txn2 commit ok");

        // a new transaction should only see key2
        let txn3 = begin_txn(&mgr);
        assert!(map.get(&key1, &txn3).is_none());
        assert_eq!(map.get(&key2, &txn3).map(|v| *v), Some(20));
    }

    #[test]
    fn multiple_txns_commit_should_create_correct_versions() {
        let mgr = CatalogTxnManager::new();
        let map: Arc<VersionedMap<String, i32>> = Arc::new(VersionedMap::new());
        let key = "ver".to_string();

        // txn1 creates version 1
        let txn1 = begin_txn(&mgr);
        let node1 = map
            .put(key.clone(), Arc::new(1), &txn1)
            .expect("txn1 put ok");
        txn1.record_write(&map, key.clone(), node1, WriteOp::Create);
        txn1.commit().expect("txn1 commit ok");

        // txn2 creates version 2
        let txn2 = begin_txn(&mgr);
        let node2 = map
            .put(key.clone(), Arc::new(2), &txn2)
            .expect("txn2 put ok");
        txn2.record_write(&map, key.clone(), node2, WriteOp::Replace);
        txn2.commit().expect("txn2 commit ok");

        // txn3 creates version 3
        let txn3 = begin_txn(&mgr);
        let node3 = map
            .put(key.clone(), Arc::new(3), &txn3)
            .expect("txn3 put ok");
        txn3.record_write(&map, key.clone(), node3, WriteOp::Replace);
        txn3.commit().expect("txn3 commit ok");

        // a fresh transaction should see latest committed value = 3
        let check_txn = begin_txn(&mgr);
        let val = map.get(&key, &check_txn).unwrap();
        assert_eq!(*val, 3);
    }

    #[test]
    fn serializable_read_conflict_on_commit() {
        let mgr = CatalogTxnManager::new();
        let map: Arc<VersionedMap<String, i32>> = Arc::new(VersionedMap::new());
        let key = "s1".to_string();

        // Seed an initial committed value = 1
        let seed = mgr
            .begin_transaction(minigu_transaction::IsolationLevel::Snapshot)
            .expect("begin seed txn");
        let n1 = map
            .put(key.clone(), Arc::new(1), &seed)
            .expect("seed put ok");
        seed.record_write(&map, key.clone(), n1, WriteOp::Create);
        seed.commit().expect("seed commit ok");

        // T1 (Serializable) reads value 1
        let t1 = mgr
            .begin_transaction(minigu_transaction::IsolationLevel::Serializable)
            .expect("begin t1");
        let v_read = map.get(&key, &t1).expect("t1 should read value");
        assert_eq!(*v_read, 1);

        // T2 writes a new version (=2) and commits
        let t2 = mgr
            .begin_transaction(minigu_transaction::IsolationLevel::Serializable)
            .expect("begin t2");
        let n2 = map.put(key.clone(), Arc::new(2), &t2).expect("t2 put ok");
        t2.record_write(&map, key.clone(), n2, WriteOp::Replace);
        t2.commit().expect("t2 commit ok");

        // T1 tries to commit and should hit serialization conflict
        let r = t1.commit();
        match r {
            Err(CatalogTxnError::SerializationConflict { key: k }) => {
                assert!(k.contains("s1"), "conflict key should be s1, got {}", k);
            }
            other => panic!("expected serialization conflict, got {:?}", other),
        }
    }
}
