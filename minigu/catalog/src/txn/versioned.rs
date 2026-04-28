use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex, RwLock, Weak};

use minigu_common::{IsolationLevel, Timestamp};

use crate::txn::error::{CatalogTxnError, CatalogTxnResult};
use crate::txn::transaction::{CatalogTxn, CatalogTxnView, TxnHook};

/// Type of write operation (used for commit validation)
#[derive(Clone, Copy, Debug)]
pub enum WriteOp {
    Create,
    Delete,
    Replace,
}

/// Entry: records the written node, operation and key
#[derive(Debug)]
pub struct Entry<K, V> {
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
    pub fn get(self: &Arc<Self>, key: &K, view: &impl CatalogTxnView) -> Option<Arc<V>> {
        let txn = view.catalog_txn();
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
    pub fn put(
        &self,
        key: K,
        value: Arc<V>,
        view: &impl CatalogTxnView,
    ) -> Result<Arc<CatalogVersionNode<V>>, CatalogTxnError> {
        let txn = view.catalog_txn();
        let mut guard = self.inner.write().unwrap();
        let chain = Self::get_or_create_chain(&mut guard, &key);
        let start_ts = txn.start_ts();
        let txn_id = txn.txn_id();
        if let Some(head) = chain.head() {
            if !head.visible_for(start_ts, txn_id) {
                return Err(CatalogTxnError::WriteConflict {
                    key: format!("{:?}", &key),
                });
            }
            if head.commit_ts().is_none() && head.creator_txn() == txn_id {
                head.overwrite_uncommitted(txn_id, Some(value), false)?;
                return Ok(head);
            }
        }
        Ok(chain.append_uncommitted(Some(value), false, txn_id))
    }

    /// Append an uncommitted tombstone version and return the created version node handle.
    pub fn delete(
        &self,
        key: &K,
        view: &impl CatalogTxnView,
    ) -> Result<Arc<CatalogVersionNode<V>>, CatalogTxnError> {
        let txn = view.catalog_txn();
        let mut guard = self.inner.write().unwrap();
        let chain = Self::get_or_create_chain(&mut guard, key);
        let start_ts = txn.start_ts();
        let txn_id = txn.txn_id();
        if let Some(head) = chain.head() {
            if !head.visible_for(start_ts, txn_id) {
                return Err(CatalogTxnError::WriteConflict {
                    key: format!("{:?}", &key),
                });
            }
            if head.commit_ts().is_none() && head.creator_txn() == txn_id {
                head.overwrite_uncommitted(txn_id, None, true)?;
                return Ok(head);
            }
        }
        Ok(chain.append_uncommitted(None, true, txn_id))
    }

    /// Validate a batch of write-intent entries and produce commit plans.
    pub fn validate_batch(
        &self,
        entries: &[Entry<K, V>],
    ) -> CatalogTxnResult<Vec<CommitPlan<K, V>>> {
        let mut plans = Vec::new();
        let guard = self.inner.read().unwrap();
        for entry in entries.iter() {
            let chain = guard
                .get(&entry.key)
                .ok_or_else(|| CatalogTxnError::IllegalState {
                    reason: "missing chain during validate".into(),
                })?;
            let head = chain.head().ok_or_else(|| CatalogTxnError::IllegalState {
                reason: "missing head during validate".into(),
            })?;
            // Ensure the head is the same node we wrote.
            if !Arc::ptr_eq(&head, &entry.node) {
                return Err(CatalogTxnError::WriteConflict {
                    key: format!("{:?}", &entry.key),
                });
            }
            plans.push(CommitPlan {
                key: entry.key.clone(),
                node: entry.node.clone(),
                op: entry.op,
            });
        }
        Ok(plans)
    }

    /// Apply a batch of commit plans.
    pub fn apply_batch(
        &self,
        plans: &[CommitPlan<K, V>],
        commit_ts: Timestamp,
    ) -> CatalogTxnResult<()> {
        let guard = self.inner.read().unwrap();
        for plan in plans.iter() {
            let chain = guard
                .get(&plan.key)
                .ok_or_else(|| CatalogTxnError::IllegalState {
                    reason: "missing chain during apply".into(),
                })?;
            let head = chain.head().ok_or_else(|| CatalogTxnError::IllegalState {
                reason: "missing head during apply".into(),
            })?;
            if !Arc::ptr_eq(&head, &plan.node) {
                return Err(CatalogTxnError::WriteConflict {
                    key: format!("{:?}", &plan.key),
                });
            }
            head.set_committed(commit_ts);
        }
        Ok(())
    }

    /// Abort a batch of writes by removing uncommitted heads when they belong to the txn.
    pub fn abort_batch(&self, entries: &[Entry<K, V>]) -> CatalogTxnResult<()> {
        let mut guard = self.inner.write().unwrap();
        for entry in entries.iter() {
            let chain = guard
                .get_mut(&entry.key)
                .ok_or_else(|| CatalogTxnError::IllegalState {
                    reason: "missing chain during abort".into(),
                })?;
            let head = chain.head().ok_or_else(|| CatalogTxnError::IllegalState {
                reason: "missing head during abort".into(),
            })?;
            if Arc::ptr_eq(&head, &entry.node) && head.commit_ts().is_none() {
                chain.head = head.next();
            }
        }
        Ok(())
    }
}

/// Commit plan for a write-intent entry
#[derive(Debug)]
pub struct CommitPlan<K, V> {
    pub key: K,
    pub node: Arc<CatalogVersionNode<V>>,
    pub op: WriteOp,
}

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
        if let Some(map) = self.map.upgrade()
            && map.was_modified_after(&self.key, self.start_ts, txn.txn_id())?
        {
            return Err(CatalogTxnError::WriteConflict {
                key: format!("{:?}", &self.key),
            });
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct CatalogVersionNode<V> {
    value: Mutex<Option<Arc<V>>>,
    tombstone: Mutex<bool>,
    creator_txn: Timestamp,
    commit_ts: Mutex<Option<Timestamp>>,
    next: Mutex<Option<Arc<CatalogVersionNode<V>>>>,
}

impl<V> CatalogVersionNode<V> {
    pub fn new_uncommitted(value: Option<Arc<V>>, tombstone: bool, creator_txn: Timestamp) -> Self {
        Self {
            value: Mutex::new(value),
            tombstone: Mutex::new(tombstone),
            creator_txn,
            commit_ts: Mutex::new(None),
            next: Mutex::new(None),
        }
    }

    pub fn overwrite_uncommitted(
        &self,
        txn_id: Timestamp,
        value: Option<Arc<V>>,
        tombstone: bool,
    ) -> CatalogTxnResult<()> {
        if self.creator_txn != txn_id || self.commit_ts().is_some() {
            return Err(CatalogTxnError::IllegalState {
                reason: "overwrite_uncommitted on node not owned by txn or already committed"
                    .to_string(),
            });
        }
        *self.value.lock().unwrap() = value;
        *self.tombstone.lock().unwrap() = tombstone;
        Ok(())
    }

    pub fn set_committed(&self, commit_ts: Timestamp) {
        *self.commit_ts.lock().unwrap() = Some(commit_ts);
    }

    pub fn commit_ts(&self) -> Option<Timestamp> {
        *self.commit_ts.lock().unwrap()
    }

    pub fn creator_txn(&self) -> Timestamp {
        self.creator_txn
    }

    pub fn value(&self) -> Option<Arc<V>> {
        self.value.lock().unwrap().clone()
    }

    pub fn is_tombstone(&self) -> bool {
        *self.tombstone.lock().unwrap()
    }

    pub fn next(&self) -> Option<Arc<CatalogVersionNode<V>>> {
        self.next.lock().unwrap().clone()
    }

    pub fn set_next(&self, next: Option<Arc<CatalogVersionNode<V>>>) {
        *self.next.lock().unwrap() = next;
    }

    pub fn visible_for(&self, start_ts: Timestamp, txn_id: Timestamp) -> bool {
        if let Some(commit_ts) = self.commit_ts() {
            commit_ts <= start_ts
        } else {
            self.creator_txn == txn_id
        }
    }
}

#[derive(Debug)]
pub struct CatalogVersionChain<V> {
    pub(crate) head: Option<Arc<CatalogVersionNode<V>>>,
}

impl<V> Default for CatalogVersionChain<V> {
    fn default() -> Self {
        Self { head: None }
    }
}

impl<V> CatalogVersionChain<V> {
    pub fn head(&self) -> Option<Arc<CatalogVersionNode<V>>> {
        self.head.clone()
    }

    pub fn visible_at(
        &self,
        start_ts: Timestamp,
        txn_id: Timestamp,
    ) -> Option<Arc<CatalogVersionNode<V>>> {
        let mut cursor = self.head.clone();
        while let Some(node) = cursor {
            if node.visible_for(start_ts, txn_id) {
                return Some(node);
            }
            cursor = node.next();
        }
        None
    }

    pub fn append_uncommitted(
        &mut self,
        value: Option<Arc<V>>,
        tombstone: bool,
        creator_txn: Timestamp,
    ) -> Arc<CatalogVersionNode<V>> {
        let node = Arc::new(CatalogVersionNode::new_uncommitted(
            value,
            tombstone,
            creator_txn,
        ));
        node.set_next(self.head.clone());
        self.head = Some(node.clone());
        node
    }
}
