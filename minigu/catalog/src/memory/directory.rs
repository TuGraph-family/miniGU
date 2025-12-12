use std::sync::{Arc, Weak};

use minigu_transaction::Transaction;

use crate::error::CatalogResult;
use crate::provider::{DirectoryOrSchema, DirectoryProvider, DirectoryRef};
use crate::txn::catalog_txn::CatalogTxn;
use crate::txn::versioned_map::{VersionedMap, WriteOp};

#[derive(Debug)]
pub struct MemoryDirectoryCatalog {
    parent: Option<Weak<dyn DirectoryProvider>>,
    children: Arc<VersionedMap<String, DirectoryOrSchema>>,
}

impl MemoryDirectoryCatalog {
    #[inline]
    pub fn new(parent: Option<Weak<dyn DirectoryProvider>>) -> Self {
        Self {
            parent,
            children: Arc::new(VersionedMap::new()),
        }
    }

    /// Add child with transaction. If the child is visible in the txn, return
    /// `AlreadyExists` error; otherwise, add a uncommitted version and record it in the
    /// transaction write set
    #[inline]
    pub fn add_child(
        &self,
        name: String,
        child: DirectoryOrSchema,
        txn: &CatalogTxn,
    ) -> Result<(), crate::txn::error::CatalogTxnError> {
        if self.children.get(&name, txn).is_some() {
            return Err(crate::txn::error::CatalogTxnError::AlreadyExists { key: name });
        }
        let node = self.children.put(name.clone(), Arc::new(child), txn)?;
        txn.record_write(&self.children, name, node, WriteOp::Create);
        Ok(())
    }

    /// Remove child with transaction. If the child is not visible in the txn, return
    /// `NotFound` error; otherwise, add a tombstone and record it in the transaction write set
    #[inline]
    pub fn remove_child(
        &self,
        name: &str,
        txn: &CatalogTxn,
    ) -> Result<(), crate::txn::error::CatalogTxnError> {
        let key = name.to_string();
        let _base = self
            .children
            .get_node_visible(&key, txn.start_ts(), txn.txn_id())
            .ok_or_else(|| crate::txn::error::CatalogTxnError::NotFound { key: key.clone() })?;
        let node = self.children.delete(&key, txn)?;
        txn.record_write(&self.children, key, node, WriteOp::Delete);
        Ok(())
    }
}

impl DirectoryProvider for MemoryDirectoryCatalog {
    #[inline]
    fn parent(&self) -> Option<DirectoryRef> {
        self.parent.clone().and_then(|p| p.upgrade())
    }

    #[inline]
    fn get_child(&self, name: &str, txn: &CatalogTxn) -> CatalogResult<Option<DirectoryOrSchema>> {
        Ok(self
            .children
            .get(&name.to_string(), txn)
            .map(|arc| arc.as_ref().clone()))
    }

    #[inline]
    fn children_names(&self, txn: &CatalogTxn) -> Vec<String> {
        self.children.visible_keys(txn.start_ts(), txn.txn_id())
    }

    #[inline]
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use minigu_transaction::{GraphTxnManager, IsolationLevel, Transaction};

    use super::*;
    use crate::txn::manager::CatalogTxnManager;

    #[test]
    fn directory_add_and_read_visibility_with_commit() {
        let mgr = CatalogTxnManager::new();
        let root = MemoryDirectoryCatalog::new(None);
        let root_ref: Arc<dyn DirectoryProvider> = Arc::new(root);

        // T1 add child but not commit yet
        let t1 = mgr.begin_transaction(IsolationLevel::Serializable).unwrap();
        let child = Arc::new(MemoryDirectoryCatalog::new(Some(Arc::downgrade(&root_ref))));
        let child_name = "sub".to_string();
        let root_impl = root_ref
            .as_any()
            .downcast_ref::<MemoryDirectoryCatalog>()
            .unwrap();
        root_impl
            .add_child(
                child_name.clone(),
                DirectoryOrSchema::Directory(child.clone() as _),
                &t1,
            )
            .unwrap();

        // T1 can see
        assert!(matches!(
            root_impl.get_child(&child_name, &t1).unwrap(),
            Some(DirectoryOrSchema::Directory(_))
        ));

        // T2 should not see uncommitted write
        let t2 = mgr.begin_transaction(IsolationLevel::Serializable).unwrap();
        assert!(root_impl.get_child(&child_name, &t2).unwrap().is_none());

        // Commit T1, T2 still should not see, a new txn T3 sees it
        t1.commit().unwrap();
        assert!(root_impl.get_child(&child_name, &t2).unwrap().is_none());
        let t3 = mgr.begin_transaction(IsolationLevel::Serializable).unwrap();
        assert!(root_impl.get_child(&child_name, &t3).unwrap().is_some());
    }

    #[test]
    fn directory_remove_and_read_visibility_with_commit() {
        let mgr = CatalogTxnManager::new();
        let root = MemoryDirectoryCatalog::new(None);
        let root_ref: Arc<dyn DirectoryProvider> = Arc::new(root);
        let root_impl = root_ref
            .as_any()
            .downcast_ref::<MemoryDirectoryCatalog>()
            .unwrap();

        // Seed: create a child and commit
        let seed = mgr.begin_transaction(IsolationLevel::Serializable).unwrap();
        let child_name = "to_del".to_string();
        let child = Arc::new(MemoryDirectoryCatalog::new(Some(Arc::downgrade(&root_ref))));
        root_impl
            .add_child(
                child_name.clone(),
                DirectoryOrSchema::Directory(child.clone() as _),
                &seed,
            )
            .unwrap();
        seed.commit().unwrap();

        // T1 delete but not commit yet; T1 cannot see it; T2 still sees it
        let t1 = mgr.begin_transaction(IsolationLevel::Serializable).unwrap();
        root_impl.remove_child(&child_name, &t1).unwrap();
        assert!(root_impl.get_child(&child_name, &t1).unwrap().is_none());
        let t2 = mgr.begin_transaction(IsolationLevel::Serializable).unwrap();
        assert!(root_impl.get_child(&child_name, &t2).unwrap().is_some());

        // Commit T1; T2 still sees; T3 not see
        t1.commit().unwrap();
        assert!(root_impl.get_child(&child_name, &t2).unwrap().is_some());
        let t3 = mgr.begin_transaction(IsolationLevel::Serializable).unwrap();
        assert!(root_impl.get_child(&child_name, &t3).unwrap().is_none());
    }
}
