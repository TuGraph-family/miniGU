use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};

use minigu_transaction::Timestamp;

use crate::txn::error::CatalogTxnError;

/// Encode `Option<Timestamp>` into `u64`; `0` represents `None`.
#[inline]
fn encode_commit_ts(opt: Option<Timestamp>) -> u64 {
    match opt {
        Some(ts) => ts.raw(),
        None => 0,
    }
}

/// Decode `u64` into `Option<Timestamp>`; `0` represents `None`.
#[inline]
fn decode_commit_ts(raw: u64) -> Option<Timestamp> {
    if raw == 0 {
        None
    } else {
        Some(Timestamp::with_ts(raw))
    }
}

/// Version node: stores value or tombstone, creator txn, and commit timestamp.
#[derive(Debug)]
pub struct CatalogVersionNode<V> {
    // None means tombstone
    value: RwLock<Option<Arc<V>>>,
    tombstone: RwLock<bool>,
    creator_txn: Timestamp,
    commit_ts_raw: AtomicU64, // 0 means not committed yet
    prev: Option<Arc<CatalogVersionNode<V>>>,
}

impl<V> CatalogVersionNode<V> {
    #[inline]
    pub fn new_uncommitted(
        value: Option<Arc<V>>,
        tombstone: bool,
        creator_txn: Timestamp,
        prev: Option<Arc<CatalogVersionNode<V>>>,
    ) -> Arc<Self> {
        Arc::new(Self {
            value: RwLock::new(value),
            tombstone: RwLock::new(tombstone),
            creator_txn,
            commit_ts_raw: AtomicU64::new(0),
            prev,
        })
    }

    #[inline]
    pub fn creator_txn(&self) -> Timestamp {
        self.creator_txn
    }

    #[inline]
    pub fn commit_ts(&self) -> Option<Timestamp> {
        decode_commit_ts(self.commit_ts_raw.load(Ordering::SeqCst))
    }

    #[inline]
    pub fn is_committed(&self) -> bool {
        self.commit_ts().is_some()
    }

    #[inline]
    pub fn is_tombstone(&self) -> bool {
        *self.tombstone.read().unwrap()
    }

    #[inline]
    pub fn value(&self) -> Option<Arc<V>> {
        self.value.read().unwrap().as_ref().cloned()
    }

    #[inline]
    pub fn prev(&self) -> Option<Arc<CatalogVersionNode<V>>> {
        self.prev.as_ref().cloned()
    }

    /// Overwrite the current uncommitted node.
    ///
    /// Invariants for arguments:
    /// - When `tombstone == false`, `new_value` must be `Some(Arc<V>)`.
    /// - When `tombstone == true`, `new_value` must be `None`.
    ///
    /// Violations of the above invariants return an error instead of panicking.
    pub fn overwrite_uncommitted(
        &self,
        creator_txn: Timestamp,
        new_value: Option<Arc<V>>,
        tombstone: bool,
    ) -> Result<(), CatalogTxnError> {
        if self.is_committed() {
            return Err(CatalogTxnError::IllegalState {
                reason: "cannot overwrite committed node".into(),
            });
        }
        if self.creator_txn != creator_txn {
            return Err(CatalogTxnError::IllegalState {
                reason: "cannot overwrite node of another txn".into(),
            });
        }
        match (tombstone, new_value) {
            // Clear tombstone and set a concrete value
            (false, Some(val)) => {
                {
                    let mut v = self.value.write().unwrap();
                    *v = Some(val);
                }
                {
                    let mut t = self.tombstone.write().unwrap();
                    *t = false;
                }
            }
            // Set tombstone and clear any value
            (true, None) => {
                {
                    let mut v = self.value.write().unwrap();
                    *v = None;
                }
                {
                    let mut t = self.tombstone.write().unwrap();
                    *t = true;
                }
            }
            // Reject invalid combinations to avoid panic and silent misuse
            (false, None) => {
                return Err(CatalogTxnError::IllegalState {
                    reason: "overwrite_uncommitted: expected Some(value) when tombstone=false"
                        .into(),
                });
            }
            (true, Some(_)) => {
                return Err(CatalogTxnError::IllegalState {
                    reason: "overwrite_uncommitted: expected None when tombstone=true".into(),
                });
            }
        }
        Ok(())
    }

    /// Set the commit timestamp (only allowed from uncommitted -> committed).
    pub fn set_commit_ts(&self, commit_ts: Timestamp) -> Result<(), CatalogTxnError> {
        let old = self.commit_ts_raw.compare_exchange(
            0,
            encode_commit_ts(Some(commit_ts)),
            Ordering::SeqCst,
            Ordering::SeqCst,
        );
        match old {
            Ok(_) => Ok(()),
            Err(cur) => Err(CatalogTxnError::IllegalState {
                reason: format!(
                    "node already committed (cur={})",
                    encode_commit_ts(decode_commit_ts(cur))
                ),
            }),
        }
    }

    /// Check whether this node is visible under the given read view.
    #[inline]
    pub fn visible_for(&self, start_ts: Timestamp, txn_id: Timestamp) -> bool {
        if let Some(cts) = self.commit_ts() {
            // Committed and committed time is not later than the read snapshot.
            cts <= start_ts
        } else {
            // Uncommitted: only visible to the creator transaction.
            self.creator_txn == txn_id
        }
    }
}

/// Version chain: a singly-linked list from newest to oldest (`head` points to the latest).
#[derive(Debug)]
pub struct CatalogVersionChain<V> {
    head: RwLock<Option<Arc<CatalogVersionNode<V>>>>,
}

impl<V> Default for CatalogVersionChain<V> {
    fn default() -> Self {
        Self {
            head: RwLock::new(None),
        }
    }
}

impl<V> CatalogVersionChain<V> {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Read the current head of the chain.
    #[inline]
    pub fn head(&self) -> Option<Arc<CatalogVersionNode<V>>> {
        self.head.read().unwrap().as_ref().cloned()
    }

    /// Read the last committed node of the chain.
    #[inline]
    pub fn last_committed(&self) -> Option<Arc<CatalogVersionNode<V>>> {
        let mut cur = self.head();
        while let Some(node) = cur {
            if node.is_committed() {
                return Some(node);
            }
            cur = node.prev();
        }
        None
    }

    /// Append an uncommitted version.
    pub fn append_uncommitted(
        &self,
        value: Option<Arc<V>>,
        tombstone: bool,
        creator_txn: Timestamp,
    ) -> Arc<CatalogVersionNode<V>> {
        let prev = self.head();
        let node = CatalogVersionNode::new_uncommitted(value, tombstone, creator_txn, prev);
        let mut guard = self.head.write().unwrap();
        *guard = Some(node.clone());
        node
    }

    /// Find the visible version node under the given read view.
    pub fn visible_at(
        &self,
        start_ts: Timestamp,
        txn_id: Timestamp,
    ) -> Option<Arc<CatalogVersionNode<V>>> {
        let mut cur = self.head();
        while let Some(node) = cur {
            if node.visible_for(start_ts, txn_id) {
                return Some(node);
            }
            cur = node.prev();
        }
        None
    }

    /// Commit the specified node (ensuring it belongs to this chain).
    /// Returns an error if the node is already committed.
    pub fn commit_node(
        &self,
        target: &Arc<CatalogVersionNode<V>>,
        commit_ts: Timestamp,
    ) -> Result<(), CatalogTxnError> {
        // Verify the node is on this chain (traverse from head to older).
        let mut cur = self.head();
        while let Some(node) = cur {
            if Arc::ptr_eq(&node, target) {
                return target.set_commit_ts(commit_ts);
            }
            cur = node.prev();
        }
        Err(CatalogTxnError::IllegalState {
            reason: "commit target not in chain".to_string(),
        })
    }

    /// Abort (rollback) an uncommitted node that was appended by the current transaction.
    ///
    /// Rules:
    /// - Only remove the node if it is EXACTLY the current chain head AND is UNCOMMITTED.
    /// - If the node is already committed or is not the head, do nothing (no-op).
    /// - This function is idempotent and safe to call multiple times.
    pub fn abort_node(&mut self, node: &Arc<CatalogVersionNode<V>>) -> Result<(), CatalogTxnError> {
        // Fetch the current head (if any).
        let Some(head) = self.head() else {
            // Empty chain: nothing to do.
            return Ok(());
        };

        // If the current head is already committed, we cannot remove it. No-op.
        if head.is_committed() {
            return Ok(());
        }

        // Only when the uncommitted head is EXACTLY the same node, we pop it.
        if Arc::ptr_eq(&head, node) {
            // Move head back to its predecessor (physically dropping this uncommitted node).
            // NOTE: `prev()` should return the previous node in the version chain (if any).
            let new_head = head.prev();
            // We have acquired a write lock, so it's safe to mutate the inner value.
            *self.head.get_mut().unwrap() = new_head;
        }

        // If it's not the head (e.g., older uncommitted node), we do nothing to avoid
        // removing nodes that are not at the top of the chain. No-op.
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use minigu_transaction::{Timestamp, global_timestamp_generator};

    use super::*;

    fn collect_values<V: Clone>(chain: &CatalogVersionChain<V>) -> Vec<V> {
        let mut vals = Vec::new();
        let mut cur = chain.head();
        while let Some(node) = cur {
            if let Some(v) = node.value() {
                vals.push((*v).clone());
            }
            cur = node.prev();
        }
        vals
    }

    #[test]
    fn append_and_iter_should_form_chain() {
        let chain: CatalogVersionChain<i32> = CatalogVersionChain::new();
        let ts_gen = global_timestamp_generator();

        let _ = chain.append_uncommitted(Some(Arc::new(1)), false, ts_gen.next().unwrap());
        let _ = chain.append_uncommitted(Some(Arc::new(2)), false, ts_gen.next().unwrap());
        let n3 = chain.append_uncommitted(Some(Arc::new(3)), false, ts_gen.next().unwrap());

        assert!(Arc::ptr_eq(&chain.head().unwrap(), &n3));

        let values: Vec<i32> = collect_values(&chain);
        assert_eq!(values, vec![3, 2, 1]);
    }

    #[test]
    fn commit_node_should_set_commit_ts() {
        let chain: CatalogVersionChain<i32> = CatalogVersionChain::new();
        let ts_gen = global_timestamp_generator();

        let n1 = chain.append_uncommitted(Some(Arc::new(100)), false, ts_gen.next().unwrap());
        let commit_ts = ts_gen.next().unwrap();

        chain.commit_node(&n1, commit_ts).expect("commit ok");

        assert_eq!(n1.commit_ts(), Some(commit_ts));
        assert!(n1.is_committed());
    }

    #[test]
    fn abort_should_remove_uncommitted_head() {
        let mut chain: CatalogVersionChain<i32> = CatalogVersionChain::new();
        let ts_gen = global_timestamp_generator();

        let n1 = chain.append_uncommitted(Some(Arc::new(10)), false, ts_gen.next().unwrap());
        let n2 = chain.append_uncommitted(Some(Arc::new(20)), false, ts_gen.next().unwrap());

        chain.abort_node(&n2).expect("abort ok");

        assert!(Arc::ptr_eq(&chain.head().unwrap(), &n1));
        let vals: Vec<i32> = collect_values(&chain);
        assert_eq!(vals, vec![10]);
    }

    #[test]
    fn visible_to_should_respect_snapshot_isolation() {
        let chain: CatalogVersionChain<i32> = CatalogVersionChain::new();
        let ts_gen = global_timestamp_generator();

        let n1 = chain.append_uncommitted(Some(Arc::new(1)), false, ts_gen.next().unwrap());
        let _n2 = chain.append_uncommitted(Some(Arc::new(2)), false, ts_gen.next().unwrap());

        // commit n1, leave n2 uncommitted
        let commit_ts1 = ts_gen.next().unwrap();
        chain.commit_node(&n1, commit_ts1).expect("commit ok");

        // txn snapshot that is not the creator of n2
        let read_ts = ts_gen.next().unwrap();
        let visible = chain.visible_at(read_ts, Timestamp::with_ts(0)).unwrap();

        assert_eq!(visible.value().map(|v| *v), Some(1));
    }

    #[test]
    fn commit_then_append_new_version_should_keep_chain_order() {
        let chain: CatalogVersionChain<i32> = CatalogVersionChain::new();
        let ts_gen = global_timestamp_generator();

        // first version committed
        let v1 = chain.append_uncommitted(Some(Arc::new(1)), false, ts_gen.next().unwrap());
        let commit_ts1 = ts_gen.next().unwrap();
        chain.commit_node(&v1, commit_ts1).expect("commit ok");

        // new uncommitted version
        let _v2 = chain.append_uncommitted(Some(Arc::new(2)), false, ts_gen.next().unwrap());

        let vals: Vec<i32> = collect_values(&chain);
        assert_eq!(vals, vec![2, 1]);
    }
}
