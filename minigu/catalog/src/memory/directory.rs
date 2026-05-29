use std::sync::{Arc, Weak};

use minigu_common::IsolationLevel;

use crate::error::{CatalogError, CatalogResult};
use crate::memory::txn_manager;
use crate::provider::{DirectoryOrSchema, DirectoryProvider, DirectoryRef};
use crate::txn::versioned::{VersionedMap, WriteOp};
use crate::txn::{CatalogTxn, CatalogTxnError};

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

    #[inline]
    pub fn add_child_txn(
        &self,
        name: String,
        child: DirectoryOrSchema,
        txn: &CatalogTxn,
    ) -> Result<(), CatalogTxnError> {
        if self.children.get(&name, txn).is_some() {
            return Err(CatalogTxnError::AlreadyExists { key: name });
        }
        let node = self.children.put(name.clone(), Arc::new(child), txn)?;
        txn.record_write(&self.children, name, node, WriteOp::Create);
        Ok(())
    }

    #[inline]
    pub fn remove_child_txn(&self, name: &str, txn: &CatalogTxn) -> Result<(), CatalogTxnError> {
        let key = name.to_string();
        let _base = self
            .children
            .get_node_visible(&key, txn.start_ts(), txn.txn_id())
            .ok_or_else(|| CatalogTxnError::NotFound { key: key.clone() })?;
        let node = self.children.delete(&key, txn)?;
        txn.record_write(&self.children, key, node, WriteOp::Delete);
        Ok(())
    }

    /// **Legacy API**: Automatically wraps the operation in a standalone transaction.
    ///
    /// This method is deprecated because it does not compose with external transactions.
    /// Use [`add_child_txn`](Self::add_child_txn) instead for transactional correctness.
    #[inline]
    #[deprecated(
        note = "Use the `_txn` variant for transactional contexts. This method uses an internal auto-commit transaction."
    )]
    pub fn add_child(&self, name: String, child: DirectoryOrSchema) -> bool {
        let txn = match txn_manager().begin_transaction(IsolationLevel::Serializable) {
            Ok(txn) => txn,
            Err(_) => return false,
        };
        let res = self.add_child_txn(name, child, txn.as_ref());
        match res {
            Ok(_) => txn.commit().is_ok(),
            Err(_) => {
                txn.abort().ok();
                false
            }
        }
    }

    /// **Legacy API**: Automatically wraps the operation in a standalone transaction.
    ///
    /// This method is deprecated because it does not compose with external transactions.
    /// Use [`remove_child_txn`](Self::remove_child_txn) instead for transactional correctness.
    #[inline]
    #[deprecated(
        note = "Use the `_txn` variant for transactional contexts. This method uses an internal auto-commit transaction."
    )]
    pub fn remove_child(&self, name: &str) -> bool {
        let txn = match txn_manager().begin_transaction(IsolationLevel::Serializable) {
            Ok(txn) => txn,
            Err(_) => return false,
        };
        let res = self.remove_child_txn(name, txn.as_ref());
        match res {
            Ok(_) => txn.commit().is_ok(),
            Err(_) => {
                txn.abort().ok();
                false
            }
        }
    }
}

impl DirectoryProvider for MemoryDirectoryCatalog {
    #[inline]
    fn parent(&self) -> Option<DirectoryRef> {
        self.parent.clone().and_then(|p| p.upgrade())
    }

    #[inline]
    fn get_child(&self, name: &str) -> CatalogResult<Option<DirectoryOrSchema>> {
        let txn = txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .map_err(|e| CatalogError::External(Box::new(e)))?;
        let res = self.get_child_txn(name, txn.as_ref());
        txn.abort().ok();
        res
    }

    #[inline]
    fn get_child_txn(
        &self,
        name: &str,
        txn: &CatalogTxn,
    ) -> CatalogResult<Option<DirectoryOrSchema>> {
        Ok(self
            .children
            .get(&name.to_string(), txn)
            .map(|arc| arc.as_ref().clone()))
    }

    #[inline]
    fn children_names(&self) -> Vec<String> {
        let txn = match txn_manager().begin_transaction(IsolationLevel::Serializable) {
            Ok(txn) => txn,
            Err(_) => return Vec::new(),
        };
        let res = self.children_names_txn(txn.as_ref());
        txn.abort().ok();
        res
    }

    #[inline]
    fn children_names_txn(&self, txn: &CatalogTxn) -> Vec<String> {
        self.children.visible_keys(txn.start_ts(), txn.txn_id())
    }
}
