use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::sync::{Arc, RwLock, Weak};

use minigu_transaction::{IsolationLevel, Timestamp, Transaction};

use crate::txn::catalog_txn::{CatalogTxn, TxnHook};
use crate::txn::error::{CatalogTxnError, CatalogTxnResult};
use crate::txn::versioned::{CatalogVersionChain, CatalogVersionNode};

/// Type of write operation (used for commit validation)
#[derive(Clone, Copy, Debug)]
pub enum WriteOp {
    Create,
    Delete,
    Replace,
}

/// Touched item: records the written node, operation, and expected base version
#[derive(Debug)]
pub struct TouchedItem<K, V> {
    pub key: K,
    pub node: Arc<CatalogVersionNode<V>>,
    pub op: WriteOp,
}

#[derive(Debug)]
pub struct VersionedMap<K, V>
where
    K: Eq + Hash + Clone + std::fmt::Debug,
    V: Send + Sync + 'static,
{
    inner: RwLock<HashMap<K, CatalogVersionChain<V>>>,
}

impl<K, V> Default for VersionedMap<K, V>
where
    K: Eq + Hash + Clone + std::fmt::Debug,
    V: Send + Sync + 'static,
{
    fn default() -> Self {
        Self {
            inner: RwLock::new(HashMap::new()),
        }
    }
}

impl<K, V> VersionedMap<K, V>
where
    K: Eq + Hash + Clone + std::fmt::Debug + Send + Sync + 'static,
    V: Send + Sync + 'static + std::fmt::Debug,
{
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the visible node of the key (including commit/creator information)
    pub fn get_node_visible(
        &self,
        key: &K,
        start_ts: Timestamp,
        txn_id: Timestamp,
    ) -> Option<Arc<CatalogVersionNode<V>>> {
        let guard = self.inner.read().unwrap();
        let chain = guard.get(key)?;
        let node = chain.visible_at(start_ts, txn_id)?;

        if node.is_tombstone() {
            return None;
        }

        Some(node)
    }

    /// Get the visible value; when txn is Serializable, register a read-validation hook.
    pub fn get(self: &Arc<Self>, key: &K, txn: &CatalogTxn) -> Option<Arc<V>> {
        let start_ts = txn.start_ts();
        let txn_id = txn.txn_id();

        let guard = self.inner.read().unwrap();
        let chain: &CatalogVersionChain<V> = guard.get(key)?;
        let node = chain.visible_at(start_ts, txn_id)?;
        if node.is_tombstone() {
            return None;
        }
        let val = node.value();

        // When the transaction is Serializable, register a read-validation hook.
        if matches!(txn.isolation_level(), IsolationLevel::Serializable) {
            let hook = ReadValidateHook {
                map: Arc::downgrade(self),
                key: key.clone(),
                start_ts,
            };
            txn.add_hook(Box::new(hook));
        }
        val
    }

    /// Whether the key was modified by OTHER transactions after `start_ts`.
    /// This checks the head node and returns true if:
    /// - it exists, and
    /// - its commit_ts > start_ts, and
    /// - its creator_txn != `txn_id` (ignore self-modifications)
    pub fn was_modified_after(
        &self,
        key: &K,
        start_ts: Timestamp,
        txn_id: Timestamp,
    ) -> CatalogTxnResult<bool> {
        let guard = self
            .inner
            .read()
            .map_err(|_| CatalogTxnError::IllegalState {
                reason: "map rwlock poisoned".into(),
            })?;

        if let Some(chain) = guard.get(key)
            && let Some(head) = chain.head()
            && let Some(commit_ts) = head.commit_ts()
            && commit_ts > start_ts
            && head.creator_txn() != txn_id
        {
            return Ok(true);
        }
        Ok(false)
    }

    /// Return the set of keys that are visible and non-tombstone under the given read view
    pub fn visible_keys(&self, start_ts: Timestamp, txn_id: Timestamp) -> Vec<K> {
        let guard = self.inner.read().unwrap();
        guard
            .iter()
            .filter_map(|(k, chain)| {
                chain
                    .visible_at(start_ts, txn_id)
                    .filter(|node| !node.is_tombstone())
                    .map(|_| k.clone())
            })
            .collect()
    }

    // Get or create the chain for the key
    fn get_or_create_chain<'a>(
        inner: &'a mut HashMap<K, CatalogVersionChain<V>>,
        key: &K,
    ) -> &'a mut CatalogVersionChain<V> {
        inner.entry(key.clone()).or_default()
    }

    /// Append an uncommitted value version and return the created version node handle.
    /// Precondition: the current head (if any) must be visible for the caller's read view.
    pub fn put(
        &self,
        key: K,
        value: Arc<V>,
        txn: &CatalogTxn,
    ) -> Result<Arc<CatalogVersionNode<V>>, CatalogTxnError> {
        let mut guard = self.inner.write().unwrap();
        let chain = Self::get_or_create_chain(&mut guard, &key);
        let start_ts = txn.start_ts();
        let txn_id = txn.txn_id();
        // If there is a head, check if it is visible for the caller's read view
        if let Some(head) = chain.head() {
            if !head.visible_for(start_ts, txn_id) {
                return Err(CatalogTxnError::WriteConflict {
                    key: format!("{:?}", &key),
                });
            }
            // If head is our own uncommitted, overwrite in place instead of creating a new node
            if head.commit_ts().is_none() && head.creator_txn() == txn_id {
                head.overwrite_uncommitted(txn_id, Some(value), false)?;
                return Ok(head);
            }
        }
        Ok(chain.append_uncommitted(Some(value), false, txn_id))
    }

    /// Append an uncommitted tombstone version and return the created version node handle.
    /// Precondition: the current head (if any) must be visible for the caller's read view.
    pub fn delete(
        &self,
        key: &K,
        txn: &CatalogTxn,
    ) -> Result<Arc<CatalogVersionNode<V>>, CatalogTxnError> {
        let mut guard = self.inner.write().unwrap();
        let chain = Self::get_or_create_chain(&mut guard, key);
        let start_ts = txn.start_ts();
        let txn_id = txn.txn_id();
        // If there is a head, check if it is visible for the caller's read view
        if let Some(head) = chain.head() {
            if !head.visible_for(start_ts, txn_id) {
                return Err(CatalogTxnError::WriteConflict {
                    key: format!("{:?}", &key),
                });
            }
            // If head is our own uncommitted, overwrite in place instead of creating a new node
            if head.commit_ts().is_none() && head.creator_txn() == txn_id {
                head.overwrite_uncommitted(txn_id, None, true)?;
                return Ok(head);
            }
        }
        Ok(chain.append_uncommitted(None, true, txn_id))
    }

    /// (Phase A) Validate only — checks ALL items in a single critical section.
    /// Returns a vector of plans that can be applied atomically later.
    pub fn validate_batch(
        &self,
        items: &[TouchedItem<K, V>],
    ) -> Result<Vec<CommitPlan<K, V>>, CatalogTxnError> {
        use WriteOp::*;

        let guard = self
            .inner
            .write()
            .map_err(|_| CatalogTxnError::IllegalState {
                reason: "poisoned lock in VersionedMap::validate_batch".into(),
            })?;

        // 1) Deduplicate.
        let mut seen: HashSet<&K> = HashSet::new();
        for it in items {
            if !seen.insert(&it.key) {
                return Err(CatalogTxnError::WriteConflict {
                    key: format!("duplicate key in the same batch: {:?}", &it.key),
                });
            }
        }

        // 2) Validate and generate plans.
        let mut plans: Vec<CommitPlan<K, V>> = Vec::with_capacity(items.len());
        for it in items {
            let chain = guard
                .get(&it.key)
                .ok_or_else(|| CatalogTxnError::NotFound {
                    key: format!("key not found: {:?}", &it.key),
                })?;

            // Confirm the uncommitted head is the node of this transaction.
            let Some(head_uncommitted) = chain.head() else {
                return Err(CatalogTxnError::IllegalState {
                    reason: format!("no uncommitted head to commit for key {:?}", &it.key),
                });
            };
            if !Arc::ptr_eq(&head_uncommitted, &it.node) {
                return Err(CatalogTxnError::WriteConflict {
                    key: format!(
                        "uncommitted head is not from this txn for key {:?}",
                        &it.key
                    ),
                });
            }

            // Check the committed head.
            let committed_head = chain.last_committed();
            match it.op {
                Create => {
                    if committed_head.is_some() {
                        return Err(CatalogTxnError::AlreadyExists {
                            key: format!(
                                "cannot Create: key already exists (committed) {:?}",
                                &it.key
                            ),
                        });
                    }
                }
                Replace => {
                    if committed_head.is_none() {
                        return Err(CatalogTxnError::NotFound {
                            key: format!(
                                "cannot Replace: no committed value for key {:?}",
                                &it.key
                            ),
                        });
                    }
                }
                Delete => {
                    if committed_head.is_none() {
                        return Err(CatalogTxnError::NotFound {
                            key: format!("cannot Delete: no committed value for key {:?}", &it.key),
                        });
                    }
                }
            }

            plans.push(CommitPlan {
                key: it.key.clone(),
                node: it.node.clone(),
            });
        }

        Ok(plans)
    }

    /// (Phase B) Apply only — mark ALL planned nodes committed with `commit_ts`.
    pub fn apply_batch(
        &self,
        plans: &[CommitPlan<K, V>],
        commit_ts: Timestamp,
    ) -> Result<(), CatalogTxnError> {
        let mut guard = self
            .inner
            .write()
            .map_err(|_| CatalogTxnError::IllegalState {
                reason: "poisoned lock in VersionedMap::apply_batch".into(),
            })?;
        for p in plans {
            let chain = guard
                .get_mut(&p.key)
                .ok_or_else(|| CatalogTxnError::IllegalState {
                    reason: format!("chain not found for key {:?}", p.key),
                })?;
            chain
                .commit_node(&p.node, commit_ts)
                .map_err(|e| CatalogTxnError::IllegalState {
                    reason: format!("commit apply failed for key {:?}: {}", p.key, e),
                })?;
        }
        Ok(())
    }

    /// Backward compatibility: validate → apply wrapper.
    pub fn commit_batch(
        &self,
        items: &[TouchedItem<K, V>],
        commit_ts: Timestamp,
    ) -> Result<(), CatalogTxnError> {
        let plans = self.validate_batch(items)?;
        self.apply_batch(&plans, commit_ts)
    }

    /// Batch rollback nodes created by the same transaction.
    ///
    /// Semantics:
    /// - If the node is the current head AND is UNCOMMITTED, we physically remove it by moving the
    ///   head pointer to its predecessor.
    /// - Otherwise (already committed or not the head), we do nothing for safety (no-op).
    /// - Items are processed in reverse append order to maximize the chance that older uncommitted
    ///   nodes eventually become the head and can be removed.
    pub fn abort_batch(&self, items: &[TouchedItem<K, V>]) -> Result<(), CatalogTxnError> {
        // Use a WRITE lock because we potentially mutate chains (remove head nodes).
        let mut guard = self
            .inner
            .write()
            .map_err(|_| CatalogTxnError::IllegalState {
                reason: "poisoned lock in VersionedMap::abort_batch".into(),
            })?;

        // Process in reverse order so that the most recently appended nodes are removed first.
        for it in items.iter().rev() {
            let chain = guard
                .get_mut(&it.key)
                .ok_or_else(|| CatalogTxnError::IllegalState {
                    reason: "abort key not found: (exists node, missing chain)".to_string(),
                })?;

            // Best-effort rollback of this node; no-op if it's not removable under the rules.
            chain.abort_node(&it.node)?;
        }
        Ok(())
    }

    /// Reserved GC interface: will reclaim obsolete versions based on low watermark in later stages
    pub fn gc(&self, _low_watermark: Timestamp) -> Result<(), CatalogTxnError> {
        // TODO: Not yet implemented.
        Ok(())
    }
}

/// Commit plan: the "applicable unit" produced by the validation phase, avoiding lookup and
/// determination when applying.
#[derive(Debug)]
pub struct CommitPlan<K, V> {
    pub key: K,
    pub node: Arc<CatalogVersionNode<V>>,
}

// =============== Serializable read validation hook ===============
#[derive(Debug)]
struct ReadValidateHook<K, V>
where
    K: Eq + Hash + Clone + std::fmt::Debug + Send + Sync + 'static,
    V: Send + Sync + 'static + std::fmt::Debug,
{
    map: Weak<VersionedMap<K, V>>,
    key: K,
    start_ts: Timestamp,
}

impl<K, V> TxnHook for ReadValidateHook<K, V>
where
    K: Eq + Hash + Clone + std::fmt::Debug + Send + Sync + 'static,
    V: Send + Sync + 'static + std::fmt::Debug,
{
    fn precommit(&self, txn: &CatalogTxn) -> CatalogTxnResult<()> {
        if let Some(map) = self.map.upgrade() {
            if map.was_modified_after(&self.key, self.start_ts, txn.txn_id())? {
                return Err(CatalogTxnError::SerializationConflict {
                    key: format!("{:?}", &self.key),
                });
            }
            Ok(())
        } else {
            Err(CatalogTxnError::IllegalState {
                reason: "map weak reference expired".into(),
            })
        }
    }
}

#[cfg(test)]
mod tests {

    use minigu_transaction::{GraphTxnManager, IsolationLevel, Transaction};

    use super::*;
    use crate::txn::manager::CatalogTxnManager;

    fn begin_txn(mgr: &CatalogTxnManager) -> std::sync::Arc<crate::txn::catalog_txn::CatalogTxn> {
        mgr.begin_transaction(IsolationLevel::Snapshot)
            .expect("begin txn")
    }

    #[test]
    fn mvcc_visibility_read_writes_and_commit() {
        let mgr = CatalogTxnManager::new();
        let map: std::sync::Arc<VersionedMap<String, i32>> =
            std::sync::Arc::new(VersionedMap::new());

        let txn1 = begin_txn(&mgr);
        let key = "g1".to_string();

        assert!(map.get(&key, &txn1).is_none());

        // Uncommitted write: only visible to txn1
        let node = map
            .put(key.clone(), std::sync::Arc::new(1), &txn1)
            .expect("put ok");
        txn1.record_write(&map, key.clone(), node, WriteOp::Create);

        assert_eq!(map.get(&key, &txn1).map(|v| *v), Some(1));

        // After commit, the latest view is visible to txn2
        txn1.commit().expect("commit ok");
        let txn2 = begin_txn(&mgr);
        assert_eq!(map.get(&key, &txn2).map(|v| *v), Some(1));
    }

    #[test]
    fn mvcc_abort_discard_head_uncommitted() {
        let mgr = CatalogTxnManager::new();
        let map: std::sync::Arc<VersionedMap<String, i32>> =
            std::sync::Arc::new(VersionedMap::new());

        let key = "k_abort".to_string();

        let txn1 = begin_txn(&mgr);
        let node1 = map
            .put(key.clone(), std::sync::Arc::new(21), &txn1)
            .expect("put ok");
        txn1.record_write(&map, key.clone(), node1, WriteOp::Create);
        assert_eq!(map.get(&key, &txn1).map(|v| *v), Some(21));
        txn1.commit().expect("commit ok");

        let txn2 = begin_txn(&mgr);
        let node2 = map
            .put(key.clone(), std::sync::Arc::new(42), &txn2)
            .expect("put ok");
        txn2.record_write(&map, key.clone(), node2, WriteOp::Create);
        assert_eq!(map.get(&key, &txn2).map(|v| *v), Some(42));

        txn2.abort().expect("abort ok");

        let txn2 = begin_txn(&mgr);
        assert_eq!(map.get(&key, &txn2).map(|v| *v), Some(21));
    }

    #[test]
    fn mvcc_create_conflict() {
        let mgr = CatalogTxnManager::new();
        let map: std::sync::Arc<VersionedMap<String, i32>> =
            std::sync::Arc::new(VersionedMap::new());
        let key = "dup".to_string();

        let txn1 = begin_txn(&mgr);
        let node1 = map
            .put(key.clone(), std::sync::Arc::new(1), &txn1)
            .expect("put ok");
        txn1.record_write(&map, key.clone(), node1, WriteOp::Create);

        let txn2 = begin_txn(&mgr);
        let r2 = map.put(key.clone(), std::sync::Arc::new(2), &txn2);
        assert!(r2.is_err());

        let r1 = txn1.commit();
        assert!(r1.is_ok());
        let txn3 = begin_txn(&mgr);
        assert_eq!(map.get(&key, &txn3).map(|v| *v), Some(1));
    }

    #[test]
    fn mvcc_delete_conflict() {
        let mgr = CatalogTxnManager::new();
        let map: std::sync::Arc<VersionedMap<String, i32>> =
            std::sync::Arc::new(VersionedMap::new());
        let key = "del".to_string();

        // Create and commit first
        let txn1 = begin_txn(&mgr);
        let node1 = map
            .put(key.clone(), std::sync::Arc::new(7), &txn1)
            .expect("put ok");
        txn1.record_write(&map, key.clone(), node1, WriteOp::Create);
        txn1.commit().expect("seed commit");

        let txn_check = begin_txn(&mgr);
        assert_eq!(map.get(&key, &txn_check).map(|v| *v), Some(7));

        // txn2 delete the node without commit
        let txn2 = begin_txn(&mgr);
        let node2 = map.delete(&key, &txn2).expect("delete ok");
        txn2.record_write(&map, key.clone(), node2, WriteOp::Delete);

        // txn3 delete the node without commit
        let txn3 = begin_txn(&mgr);
        let r3 = map.delete(&key, &txn3);
        assert!(r3.is_err());

        let _ = txn2.commit().expect("first delete ok");
        let txn_check = begin_txn(&mgr);
        assert!(map.get(&key, &txn_check).is_none());
    }
}
