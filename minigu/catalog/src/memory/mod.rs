pub mod directory;
pub mod graph_type;
pub mod schema;

use crate::error::CatalogResult;
use crate::provider::{CatalogProvider, DirectoryOrSchema};
use crate::txn::CatalogTxnManager;

/// Global catalog transaction manager used for auto-commit compatibility paths.
/// TODO: Remove the global singleton and use api with `_txn` suffixes only.
pub fn txn_manager() -> &'static std::sync::Arc<CatalogTxnManager> {
    use std::sync::OnceLock;

    static MANAGER: OnceLock<std::sync::Arc<CatalogTxnManager>> = OnceLock::new();
    MANAGER.get_or_init(CatalogTxnManager::new)
}

#[derive(Debug)]
pub struct MemoryCatalog {
    root: DirectoryOrSchema,
}

impl MemoryCatalog {
    pub fn new(root: DirectoryOrSchema) -> Self {
        Self { root }
    }
}

impl CatalogProvider for MemoryCatalog {
    #[inline]
    fn get_root(&self) -> CatalogResult<DirectoryOrSchema> {
        Ok(self.root.clone())
    }
}
