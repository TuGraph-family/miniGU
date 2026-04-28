use std::fmt;
use std::sync::{Arc, Weak};

use minigu_common::IsolationLevel;

use super::graph_type::MemoryGraphTypeCatalog;
use crate::error::{CatalogError, CatalogResult};
use crate::memory::txn_manager;
use crate::provider::{
    DirectoryProvider, DirectoryRef, GraphRef, GraphTypeRef, ProcedureRef, SchemaProvider,
};
use crate::txn::versioned::{VersionedMap, WriteOp};
use crate::txn::{CatalogTxn, CatalogTxnError};

/// Result of a create graph operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreateGraphResult {
    Created,
    AlreadyExists,
    Replaced,
}

/// Kind of create operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreateKind {
    Create,
    CreateIfNotExists,
    CreateOrReplace,
}

/// Result of a drop graph operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DropGraphResult {
    Dropped,
    NotFound,
}

pub struct MemorySchemaCatalog {
    parent: Option<Weak<dyn DirectoryProvider>>,
    graph_map: Arc<VersionedMap<String, GraphRef>>,
    graph_type_map: Arc<VersionedMap<String, GraphTypeRef>>,
    procedure_map: Arc<VersionedMap<String, ProcedureRef>>,
}

impl fmt::Debug for MemorySchemaCatalog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MemorySchemaCatalog").finish()
    }
}

impl MemorySchemaCatalog {
    #[inline]
    pub fn new(parent: Option<Weak<dyn DirectoryProvider>>) -> Self {
        Self {
            parent,
            graph_map: Arc::new(VersionedMap::new()),
            graph_type_map: Arc::new(VersionedMap::new()),
            procedure_map: Arc::new(VersionedMap::new()),
        }
    }

    #[inline]
    pub fn add_graph_txn(
        &self,
        name: String,
        graph: GraphRef,
        txn: &CatalogTxn,
    ) -> Result<(), CatalogTxnError> {
        if self.graph_map.get(&name, txn).is_some() {
            return Err(CatalogTxnError::AlreadyExists { key: name });
        }
        let node = self.graph_map.put(name.clone(), Arc::new(graph), txn)?;
        txn.record_write(&self.graph_map, name, node, WriteOp::Create);
        Ok(())
    }

    #[inline]
    pub fn remove_graph_txn(&self, name: &str, txn: &CatalogTxn) -> Result<(), CatalogTxnError> {
        let key = name.to_string();
        let _base = self
            .graph_map
            .get_node_visible(&key, txn.start_ts(), txn.txn_id())
            .ok_or_else(|| CatalogTxnError::NotFound { key: key.clone() })?;
        let node = self.graph_map.delete(&key, txn)?;
        txn.record_write(&self.graph_map, key, node, WriteOp::Delete);
        Ok(())
    }

    #[inline]
    pub fn add_graph_type_txn(
        &self,
        name: String,
        graph_type: Arc<MemoryGraphTypeCatalog>,
        txn: &CatalogTxn,
    ) -> Result<(), CatalogTxnError> {
        if self.graph_type_map.get(&name, txn).is_some() {
            return Err(CatalogTxnError::AlreadyExists { key: name });
        }
        let gt_ref: GraphTypeRef = graph_type;
        let node = self
            .graph_type_map
            .put(name.clone(), Arc::new(gt_ref), txn)?;
        txn.record_write(&self.graph_type_map, name, node, WriteOp::Create);
        Ok(())
    }

    #[inline]
    pub fn remove_graph_type_txn(
        &self,
        name: &str,
        txn: &CatalogTxn,
    ) -> Result<(), CatalogTxnError> {
        let key = name.to_string();
        let _base = self
            .graph_type_map
            .get_node_visible(&key, txn.start_ts(), txn.txn_id())
            .ok_or_else(|| CatalogTxnError::NotFound { key: key.clone() })?;
        let node = self.graph_type_map.delete(&key, txn)?;
        txn.record_write(&self.graph_type_map, key, node, WriteOp::Delete);
        Ok(())
    }

    #[inline]
    pub fn add_procedure_txn(
        &self,
        name: String,
        procedure: ProcedureRef,
        txn: &CatalogTxn,
    ) -> Result<(), CatalogTxnError> {
        if self.procedure_map.get(&name, txn).is_some() {
            return Err(CatalogTxnError::AlreadyExists { key: name });
        }
        let node = self
            .procedure_map
            .put(name.clone(), Arc::new(procedure), txn)?;
        txn.record_write(&self.procedure_map, name, node, WriteOp::Create);
        Ok(())
    }

    #[inline]
    pub fn remove_procedure_txn(
        &self,
        name: &str,
        txn: &CatalogTxn,
    ) -> Result<(), CatalogTxnError> {
        let key = name.to_string();
        let _base = self
            .procedure_map
            .get_node_visible(&key, txn.start_ts(), txn.txn_id())
            .ok_or_else(|| CatalogTxnError::NotFound { key: key.clone() })?;
        let node = self.procedure_map.delete(&key, txn)?;
        txn.record_write(&self.procedure_map, key, node, WriteOp::Delete);
        Ok(())
    }

    /// **Legacy API**: Automatically wraps the operation in a standalone transaction.
    ///
    /// This method is deprecated because it does not compose with external transactions.
    /// Use [`add_graph_txn`](Self::add_graph_txn) instead for transactional correctness.
    #[inline]
    #[deprecated(
        note = "Use the `_txn` variant for transactional contexts. This method uses an internal auto-commit transaction."
    )]
    pub fn add_graph(&self, name: String, graph: GraphRef) -> bool {
        let txn = match txn_manager().begin_transaction(IsolationLevel::Serializable) {
            Ok(txn) => txn,
            Err(_) => return false,
        };
        let res = self.add_graph_txn(name, graph, txn.as_ref());
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
    /// Use [`drop_graph`](Self::drop_graph) or [`remove_graph_txn`](Self::remove_graph_txn)
    /// instead for transactional correctness.
    #[inline]
    #[deprecated(
        note = "Use the `_txn` variant for transactional contexts. This method uses an internal auto-commit transaction."
    )]
    pub fn remove_graph(&self, name: &str) -> bool {
        matches!(self.drop_graph(name), DropGraphResult::Dropped)
    }

    /// Unified Graph Deletion Interface.
    pub fn drop_graph(&self, name: &str) -> DropGraphResult {
        let txn = match txn_manager().begin_transaction(IsolationLevel::Serializable) {
            Ok(txn) => txn,
            Err(_) => return DropGraphResult::NotFound,
        };
        let res = self.remove_graph_txn(name, txn.as_ref());
        match res {
            Ok(_) if txn.commit().is_ok() => DropGraphResult::Dropped,
            Err(_) => {
                txn.abort().ok();
                DropGraphResult::NotFound
            }
            _ => DropGraphResult::NotFound,
        }
    }

    /// **Legacy API**: Automatically wraps the operation in a standalone transaction.
    ///
    /// This method is deprecated because it does not compose with external transactions.
    /// Use [`add_graph_type_txn`](Self::add_graph_type_txn) instead for transactional correctness.
    #[inline]
    #[deprecated(
        note = "Use the `_txn` variant for transactional contexts. This method uses an internal auto-commit transaction."
    )]
    pub fn add_graph_type(&self, name: String, graph_type: Arc<MemoryGraphTypeCatalog>) -> bool {
        let txn = match txn_manager().begin_transaction(IsolationLevel::Serializable) {
            Ok(txn) => txn,
            Err(_) => return false,
        };
        let res = self.add_graph_type_txn(name, graph_type, txn.as_ref());
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
    /// Use [`remove_graph_type_txn`](Self::remove_graph_type_txn) instead for transactional
    /// correctness.
    #[inline]
    #[deprecated(
        note = "Use the `_txn` variant for transactional contexts. This method uses an internal auto-commit transaction."
    )]
    pub fn remove_graph_type(&self, name: &str) -> bool {
        let txn = match txn_manager().begin_transaction(IsolationLevel::Serializable) {
            Ok(txn) => txn,
            Err(_) => return false,
        };
        let res = self.remove_graph_type_txn(name, txn.as_ref());
        match res {
            Ok(_) => txn.commit().is_ok(),
            Err(_) => {
                txn.abort().ok();
                false
            }
        }
    }

    pub fn create_graph_txn(
        &self,
        name: String,
        graph: GraphRef,
        kind: CreateKind,
        txn: &CatalogTxn,
    ) -> Result<CreateGraphResult, CatalogTxnError> {
        let existed = self.graph_map.get(&name, txn).is_some();
        match (kind, existed) {
            (CreateKind::Create, true) | (CreateKind::CreateIfNotExists, true) => {
                Ok(CreateGraphResult::AlreadyExists)
            }
            (CreateKind::CreateOrReplace, true) => {
                let node = self.graph_map.put(name.clone(), Arc::new(graph), txn)?;
                txn.record_write(&self.graph_map, name, node, WriteOp::Replace);
                Ok(CreateGraphResult::Replaced)
            }
            (_, false) => {
                let node = self.graph_map.put(name.clone(), Arc::new(graph), txn)?;
                txn.record_write(&self.graph_map, name, node, WriteOp::Create);
                Ok(CreateGraphResult::Created)
            }
        }
    }

    /// Unified graph creation interface supporting all CreateKind cases.
    pub fn create_graph(
        &self,
        name: String,
        graph: GraphRef,
        kind: CreateKind,
    ) -> CreateGraphResult {
        let txn = match txn_manager().begin_transaction(IsolationLevel::Serializable) {
            Ok(txn) => txn,
            Err(_) => return CreateGraphResult::AlreadyExists,
        };
        let res = self.create_graph_txn(name, graph, kind, txn.as_ref());
        match res {
            Ok(result) if result == CreateGraphResult::AlreadyExists => {
                txn.abort().ok();
                result
            }
            Ok(result) if txn.commit().is_ok() => result,
            Ok(_) | Err(_) => {
                txn.abort().ok();
                CreateGraphResult::AlreadyExists
            }
        }
    }

    /// Atomically create or replace a graph.
    /// Returns: (whether an existing graph was replaced, whether the operation succeeded).
    #[deprecated(note = "Use create_graph with CreateKind::CreateOrReplace instead")]
    pub fn create_or_replace_graph(&self, name: String, graph: GraphRef) -> (bool, bool) {
        let result = self.create_graph(name, graph, CreateKind::CreateOrReplace);
        match result {
            CreateGraphResult::Replaced => (true, true),
            CreateGraphResult::Created => (false, true),
            CreateGraphResult::AlreadyExists => (false, false),
        }
    }

    /// **Legacy API**: Automatically wraps the operation in a standalone transaction.
    ///
    /// This method is deprecated because it does not compose with external transactions.
    /// Use [`add_procedure_txn`](Self::add_procedure_txn) instead for transactional correctness.
    #[inline]
    #[deprecated(
        note = "Use the `_txn` variant for transactional contexts. This method uses an internal auto-commit transaction."
    )]
    pub fn add_procedure(&self, name: String, procedure: ProcedureRef) -> bool {
        let txn = match txn_manager().begin_transaction(IsolationLevel::Serializable) {
            Ok(txn) => txn,
            Err(_) => return false,
        };
        let res = self.add_procedure_txn(name, procedure, txn.as_ref());
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
    /// Use [`remove_procedure_txn`](Self::remove_procedure_txn) instead for transactional
    /// correctness.
    #[inline]
    #[deprecated(
        note = "Use the `_txn` variant for transactional contexts. This method uses an internal auto-commit transaction."
    )]
    pub fn remove_procedure(&self, name: &str) -> bool {
        let txn = match txn_manager().begin_transaction(IsolationLevel::Serializable) {
            Ok(txn) => txn,
            Err(_) => return false,
        };
        let res = self.remove_procedure_txn(name, txn.as_ref());
        match res {
            Ok(_) => txn.commit().is_ok(),
            Err(_) => {
                txn.abort().ok();
                false
            }
        }
    }
}

impl SchemaProvider for MemorySchemaCatalog {
    #[inline]
    fn parent(&self) -> Option<DirectoryRef> {
        self.parent.clone().and_then(|p| p.upgrade())
    }

    #[inline]
    fn get_graph(&self, name: &str) -> CatalogResult<Option<GraphRef>> {
        let txn = txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .map_err(|e| CatalogError::External(Box::new(e)))?;
        let res = self.get_graph_txn(name, txn.as_ref());
        txn.abort().ok();
        res
    }

    #[inline]
    fn get_graph_txn(&self, name: &str, txn: &CatalogTxn) -> CatalogResult<Option<GraphRef>> {
        Ok(self
            .graph_map
            .get(&name.to_string(), txn)
            .map(|arc| (*arc).clone()))
    }

    #[inline]
    fn graph_names(&self) -> Vec<String> {
        let txn = match txn_manager().begin_transaction(IsolationLevel::Serializable) {
            Ok(txn) => txn,
            Err(_) => return Vec::new(),
        };
        let res = self.graph_names_txn(txn.as_ref());
        txn.abort().ok();
        res
    }

    #[inline]
    fn graph_names_txn(&self, txn: &CatalogTxn) -> Vec<String> {
        self.graph_map.visible_keys(txn.start_ts(), txn.txn_id())
    }

    #[inline]
    fn get_graph_type(&self, name: &str) -> CatalogResult<Option<GraphTypeRef>> {
        let txn = txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .map_err(|e| CatalogError::External(Box::new(e)))?;
        let res = self.get_graph_type_txn(name, txn.as_ref());
        txn.abort().ok();
        res
    }

    #[inline]
    fn get_graph_type_txn(
        &self,
        name: &str,
        txn: &CatalogTxn,
    ) -> CatalogResult<Option<GraphTypeRef>> {
        Ok(self
            .graph_type_map
            .get(&name.to_string(), txn)
            .map(|arc| (*arc).clone()))
    }

    #[inline]
    fn graph_type_names(&self) -> Vec<String> {
        let txn = match txn_manager().begin_transaction(IsolationLevel::Serializable) {
            Ok(txn) => txn,
            Err(_) => return Vec::new(),
        };
        let res = self.graph_type_names_txn(txn.as_ref());
        txn.abort().ok();
        res
    }

    #[inline]
    fn graph_type_names_txn(&self, txn: &CatalogTxn) -> Vec<String> {
        self.graph_type_map
            .visible_keys(txn.start_ts(), txn.txn_id())
    }

    #[inline]
    fn get_procedure(&self, name: &str) -> CatalogResult<Option<ProcedureRef>> {
        let txn = txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .map_err(|e| CatalogError::External(Box::new(e)))?;
        let res = self.get_procedure_txn(name, txn.as_ref());
        txn.abort().ok();
        res
    }

    #[inline]
    fn get_procedure_txn(
        &self,
        name: &str,
        txn: &CatalogTxn,
    ) -> CatalogResult<Option<ProcedureRef>> {
        Ok(self
            .procedure_map
            .get(&name.to_string(), txn)
            .map(|arc| (*arc).clone()))
    }

    #[inline]
    fn procedure_names(&self) -> Vec<String> {
        let txn = match txn_manager().begin_transaction(IsolationLevel::Serializable) {
            Ok(txn) => txn,
            Err(_) => return Vec::new(),
        };
        let res = self.procedure_names_txn(txn.as_ref());
        txn.abort().ok();
        res
    }

    #[inline]
    fn procedure_names_txn(&self, txn: &CatalogTxn) -> Vec<String> {
        self.procedure_map
            .visible_keys(txn.start_ts(), txn.txn_id())
    }
}
