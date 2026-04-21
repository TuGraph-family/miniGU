use std::sync::{Arc, Mutex};

use minigu_common::IsolationLevel;
use minigu_common::types::{LabelId, PropertyId};

use crate::error::{CatalogError, CatalogResult};
use crate::label_set::LabelSet;
use crate::memory::txn_manager;
use crate::property::Property;
use crate::provider::{
    EdgeTypeProvider, EdgeTypeRef, GraphTypeProvider, PropertiesProvider, VertexTypeProvider,
    VertexTypeRef,
};
use crate::txn::CatalogTxnError;
use crate::txn::transaction::{CatalogTxn, TxnHook};
use crate::txn::versioned::{VersionedMap, WriteOp};

#[derive(Debug)]
pub struct MemoryGraphTypeCatalog {
    next_label_id: Mutex<LabelId>,
    label_map: Arc<VersionedMap<String, LabelId>>,
    vertex_type_map: Arc<VersionedMap<LabelSet, VertexTypeRef>>,
    edge_type_map: Arc<VersionedMap<LabelSet, EdgeTypeRef>>,
}

impl Default for MemoryGraphTypeCatalog {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryGraphTypeCatalog {
    #[inline]
    pub fn new() -> Self {
        Self {
            next_label_id: Mutex::new(LabelId::new(1).expect("label id should be non-zero")),
            label_map: Arc::new(VersionedMap::new()),
            vertex_type_map: Arc::new(VersionedMap::new()),
            edge_type_map: Arc::new(VersionedMap::new()),
        }
    }

    // ============== Transactional write APIs ==============

    /// Adds a new label with the given name within a transaction.
    ///
    /// # Label ID Allocation
    ///
    /// Label IDs are allocated immediately when this method is called, **before** the transaction
    /// commits. If the transaction aborts, the allocated ID will be lost, creating gaps in the
    /// ID sequence. This is a known design trade-off that matches the behavior of mainstream
    /// databases (e.g., PostgreSQL's SERIAL type).
    ///
    /// **Rationale**: Deferring ID allocation until commit would require complex coordination
    /// between multiple concurrent transactions and could lead to commit-time failures due to
    /// ID exhaustion. The current approach prioritizes simplicity and commit reliability over
    /// ID sequence continuity.
    #[inline]
    pub fn add_label_txn(
        &self,
        name: String,
        txn: &CatalogTxn,
    ) -> Result<LabelId, CatalogTxnError> {
        if self.label_map.get(&name, txn).is_some() {
            return Err(CatalogTxnError::AlreadyExists { key: name });
        }
        let mut guard = self.next_label_id.lock().expect("poisoned label id mutex");
        let label_id = *guard;
        *guard = guard
            .checked_add(1)
            .ok_or_else(|| CatalogTxnError::IllegalState {
                reason: "label id overflow".to_string(),
            })?;
        drop(guard);

        let node = self.label_map.put(name.clone(), Arc::new(label_id), txn)?;
        txn.record_write(&self.label_map, name, node, WriteOp::Create);
        Ok(label_id)
    }

    #[inline]
    pub fn remove_label_txn(&self, name: &str, txn: &CatalogTxn) -> Result<(), CatalogTxnError> {
        let key = name.to_string();
        let base = self
            .label_map
            .get_node_visible(&key, txn.start_ts(), txn.txn_id())
            .ok_or_else(|| CatalogTxnError::NotFound { key: key.clone() })?;
        let label_id = *base.value().ok_or_else(|| CatalogTxnError::IllegalState {
            reason: "label node has no value".to_string(),
        })?;
        txn.add_hook(Box::new(GraphTypeIntegrityHook::label_delete(
            self.vertex_type_map.clone(),
            self.edge_type_map.clone(),
            label_id,
        )));
        let node = self.label_map.delete(&key, txn)?;
        txn.record_write(&self.label_map, key, node, WriteOp::Delete);
        Ok(())
    }

    #[inline]
    pub fn add_vertex_type_txn(
        &self,
        label_set: LabelSet,
        vertex_type: Arc<MemoryVertexTypeCatalog>,
        txn: &CatalogTxn,
    ) -> Result<(), CatalogTxnError> {
        if self.vertex_type_map.get(&label_set, txn).is_some() {
            return Err(CatalogTxnError::AlreadyExists {
                key: format!("{:?}", label_set),
            });
        }
        let vt_ref: VertexTypeRef = vertex_type;
        let node = self
            .vertex_type_map
            .put(label_set.clone(), Arc::new(vt_ref), txn)?;
        txn.record_write(&self.vertex_type_map, label_set, node, WriteOp::Create);
        Ok(())
    }

    #[inline]
    pub fn remove_vertex_type_txn(
        &self,
        label_set: &LabelSet,
        txn: &CatalogTxn,
    ) -> Result<(), CatalogTxnError> {
        let _base = self
            .vertex_type_map
            .get_node_visible(label_set, txn.start_ts(), txn.txn_id())
            .ok_or_else(|| CatalogTxnError::NotFound {
                key: format!("{:?}", label_set),
            })?;
        txn.add_hook(Box::new(GraphTypeIntegrityHook::vertex_delete(
            self.edge_type_map.clone(),
            label_set.clone(),
        )));
        let node = self.vertex_type_map.delete(label_set, txn)?;
        txn.record_write(
            &self.vertex_type_map,
            label_set.clone(),
            node,
            WriteOp::Delete,
        );
        Ok(())
    }

    #[inline]
    pub fn add_edge_type_txn(
        &self,
        label_set: LabelSet,
        edge_type: Arc<MemoryEdgeTypeCatalog>,
        txn: &CatalogTxn,
    ) -> Result<(), CatalogTxnError> {
        let src_ls = edge_type.src().label_set();
        let dst_ls = edge_type.dst().label_set();
        if self.vertex_type_map.get(&src_ls, txn).is_none()
            || self.vertex_type_map.get(&dst_ls, txn).is_none()
        {
            return Err(CatalogTxnError::ReferentialIntegrity {
                reason: "edge src/dst vertex type not found".to_string(),
            });
        }
        if self.edge_type_map.get(&label_set, txn).is_some() {
            return Err(CatalogTxnError::AlreadyExists {
                key: format!("{:?}", label_set),
            });
        }
        txn.add_hook(Box::new(GraphTypeIntegrityHook::edge_src_dst_exist(
            self.vertex_type_map.clone(),
            src_ls.clone(),
            dst_ls.clone(),
        )));
        let et_ref: EdgeTypeRef = edge_type;
        let node = self
            .edge_type_map
            .put(label_set.clone(), Arc::new(et_ref), txn)?;
        txn.record_write(&self.edge_type_map, label_set, node, WriteOp::Create);
        Ok(())
    }

    #[inline]
    pub fn remove_edge_type_txn(
        &self,
        label_set: &LabelSet,
        txn: &CatalogTxn,
    ) -> Result<(), CatalogTxnError> {
        let _base = self
            .edge_type_map
            .get_node_visible(label_set, txn.start_ts(), txn.txn_id())
            .ok_or_else(|| CatalogTxnError::NotFound {
                key: format!("{:?}", label_set),
            })?;
        let node = self.edge_type_map.delete(label_set, txn)?;
        txn.record_write(
            &self.edge_type_map,
            label_set.clone(),
            node,
            WriteOp::Delete,
        );
        Ok(())
    }

    #[inline]
    pub fn replace_vertex_type_txn(
        &self,
        label_set: &LabelSet,
        new_vertex_type: Arc<MemoryVertexTypeCatalog>,
        txn: &CatalogTxn,
    ) -> Result<(), CatalogTxnError> {
        let _ = self
            .vertex_type_map
            .get_node_visible(label_set, txn.start_ts(), txn.txn_id())
            .ok_or_else(|| CatalogTxnError::NotFound {
                key: format!("{:?}", label_set),
            })?;
        let vt_ref: VertexTypeRef = new_vertex_type;
        let node = self
            .vertex_type_map
            .put(label_set.clone(), Arc::new(vt_ref), txn)?;
        txn.record_write(
            &self.vertex_type_map,
            label_set.clone(),
            node,
            WriteOp::Replace,
        );
        Ok(())
    }

    #[inline]
    pub fn replace_edge_type_txn(
        &self,
        label_set: &LabelSet,
        new_edge_type: Arc<MemoryEdgeTypeCatalog>,
        txn: &CatalogTxn,
    ) -> Result<(), CatalogTxnError> {
        let src_ls = new_edge_type.src().label_set();
        let dst_ls = new_edge_type.dst().label_set();
        if self.vertex_type_map.get(&src_ls, txn).is_none()
            || self.vertex_type_map.get(&dst_ls, txn).is_none()
        {
            return Err(CatalogTxnError::ReferentialIntegrity {
                reason: "edge src/dst vertex type not found".to_string(),
            });
        }
        let _ = self
            .edge_type_map
            .get_node_visible(label_set, txn.start_ts(), txn.txn_id())
            .ok_or_else(|| CatalogTxnError::NotFound {
                key: format!("{:?}", label_set),
            })?;
        txn.add_hook(Box::new(GraphTypeIntegrityHook::edge_src_dst_exist(
            self.vertex_type_map.clone(),
            src_ls.clone(),
            dst_ls.clone(),
        )));
        let et_ref: EdgeTypeRef = new_edge_type;
        let node = self
            .edge_type_map
            .put(label_set.clone(), Arc::new(et_ref), txn)?;
        txn.record_write(
            &self.edge_type_map,
            label_set.clone(),
            node,
            WriteOp::Replace,
        );
        Ok(())
    }

    // ============== Compatibility wrappers (auto-commit) ==============
    /// **Legacy API**: Automatically wraps the operation in a standalone transaction.
    ///
    /// This method is deprecated because it does not compose with external transactions.
    /// Use [`add_label_txn`](Self::add_label_txn) instead for transactional correctness.
    #[inline]
    #[deprecated(
        note = "Use the `_txn` variant for transactional contexts. This method uses an internal auto-commit transaction."
    )]
    pub fn add_label(&mut self, name: String) -> Option<LabelId> {
        let txn = txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .ok()?;
        let res = self.add_label_txn(name, txn.as_ref());
        match res {
            Ok(id) => {
                if txn.commit().is_ok() {
                    Some(id)
                } else {
                    None
                }
            }
            Err(_) => {
                txn.abort().ok();
                None
            }
        }
    }

    /// **Legacy API**: Automatically wraps the operation in a standalone transaction.
    ///
    /// This method is deprecated because it does not compose with external transactions.
    /// Use [`remove_label_txn`](Self::remove_label_txn) instead for transactional correctness.
    #[inline]
    #[deprecated(
        note = "Use the `_txn` variant for transactional contexts. This method uses an internal auto-commit transaction."
    )]
    pub fn remove_label(&mut self, name: &str) -> bool {
        let txn = match txn_manager().begin_transaction(IsolationLevel::Serializable) {
            Ok(txn) => txn,
            Err(_) => return false,
        };
        let res = self.remove_label_txn(name, txn.as_ref());
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
    /// Use [`add_vertex_type_txn`](Self::add_vertex_type_txn) instead for transactional
    /// correctness.
    #[inline]
    #[deprecated(
        note = "Use the `_txn` variant for transactional contexts. This method uses an internal auto-commit transaction."
    )]
    pub fn add_vertex_type(
        &mut self,
        label_set: LabelSet,
        vertex_type: Arc<MemoryVertexTypeCatalog>,
    ) -> bool {
        let txn = match txn_manager().begin_transaction(IsolationLevel::Serializable) {
            Ok(txn) => txn,
            Err(_) => return false,
        };
        let res = self.add_vertex_type_txn(label_set, vertex_type, txn.as_ref());
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
    /// Use [`remove_vertex_type_txn`](Self::remove_vertex_type_txn) instead for transactional
    /// correctness.
    #[inline]
    #[deprecated(
        note = "Use the `_txn` variant for transactional contexts. This method uses an internal auto-commit transaction."
    )]
    pub fn remove_vertex_type(&mut self, label_set: &LabelSet) -> bool {
        let txn = match txn_manager().begin_transaction(IsolationLevel::Serializable) {
            Ok(txn) => txn,
            Err(_) => return false,
        };
        let res = self.remove_vertex_type_txn(label_set, txn.as_ref());
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
    /// Use [`add_edge_type_txn`](Self::add_edge_type_txn) instead for transactional correctness.
    #[inline]
    #[deprecated(
        note = "Use the `_txn` variant for transactional contexts. This method uses an internal auto-commit transaction."
    )]
    pub fn add_edge_type(
        &mut self,
        label_set: LabelSet,
        edge_type: Arc<MemoryEdgeTypeCatalog>,
    ) -> bool {
        let txn = match txn_manager().begin_transaction(IsolationLevel::Serializable) {
            Ok(txn) => txn,
            Err(_) => return false,
        };
        let res = self.add_edge_type_txn(label_set, edge_type, txn.as_ref());
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
    /// Use [`remove_edge_type_txn`](Self::remove_edge_type_txn) instead for transactional
    /// correctness.
    #[inline]
    #[deprecated(
        note = "Use the `_txn` variant for transactional contexts. This method uses an internal auto-commit transaction."
    )]
    pub fn remove_edge_type(&mut self, label_set: &LabelSet) -> bool {
        let txn = match txn_manager().begin_transaction(IsolationLevel::Serializable) {
            Ok(txn) => txn,
            Err(_) => return false,
        };
        let res = self.remove_edge_type_txn(label_set, txn.as_ref());
        match res {
            Ok(_) => txn.commit().is_ok(),
            Err(_) => {
                txn.abort().ok();
                false
            }
        }
    }
}

// ================ Pre-commit validation hook implementation ================
#[derive(Debug)]
struct GraphTypeIntegrityHook {
    vertex_type_map: Arc<VersionedMap<LabelSet, VertexTypeRef>>,
    edge_type_map: Arc<VersionedMap<LabelSet, EdgeTypeRef>>,
    kind: IntegrityKind,
}

#[derive(Debug)]
enum IntegrityKind {
    LabelDelete { label_id: LabelId },
    VertexDelete { vertex_label_set: LabelSet },
    EdgeSrcDstExist { src: LabelSet, dst: LabelSet },
}

impl GraphTypeIntegrityHook {
    fn label_delete(
        vertex_type_map: Arc<VersionedMap<LabelSet, VertexTypeRef>>,
        edge_type_map: Arc<VersionedMap<LabelSet, EdgeTypeRef>>,
        label_id: LabelId,
    ) -> Self {
        Self {
            vertex_type_map,
            edge_type_map,
            kind: IntegrityKind::LabelDelete { label_id },
        }
    }

    fn vertex_delete(
        edge_type_map: Arc<VersionedMap<LabelSet, EdgeTypeRef>>,
        vertex_label_set: LabelSet,
    ) -> Self {
        Self {
            vertex_type_map: Arc::new(VersionedMap::new()),
            edge_type_map,
            kind: IntegrityKind::VertexDelete { vertex_label_set },
        }
    }

    fn edge_src_dst_exist(
        vertex_type_map: Arc<VersionedMap<LabelSet, VertexTypeRef>>,
        src: LabelSet,
        dst: LabelSet,
    ) -> Self {
        Self {
            vertex_type_map,
            edge_type_map: Arc::new(VersionedMap::new()),
            kind: IntegrityKind::EdgeSrcDstExist { src, dst },
        }
    }
}

impl TxnHook for GraphTypeIntegrityHook {
    fn precommit(&self, txn: &CatalogTxn) -> Result<(), CatalogTxnError> {
        match &self.kind {
            IntegrityKind::LabelDelete { label_id } => {
                let v_keys = self
                    .vertex_type_map
                    .visible_keys(txn.start_ts(), txn.txn_id());
                if v_keys.iter().any(|k| k.contains(*label_id)) {
                    return Err(CatalogTxnError::ReferentialIntegrity {
                        reason: format!("label {:?} is still used by vertex types", label_id),
                    });
                }
                let e_keys = self
                    .edge_type_map
                    .visible_keys(txn.start_ts(), txn.txn_id());
                if e_keys.iter().any(|k| k.contains(*label_id)) {
                    return Err(CatalogTxnError::ReferentialIntegrity {
                        reason: format!("label {:?} is still used by edge types", label_id),
                    });
                }
                Ok(())
            }
            IntegrityKind::VertexDelete { vertex_label_set } => {
                let e_keys = self
                    .edge_type_map
                    .visible_keys(txn.start_ts(), txn.txn_id());
                for k in e_keys.into_iter() {
                    if let Some(node) =
                        self.edge_type_map
                            .get_node_visible(&k, txn.start_ts(), txn.txn_id())
                        && let Some(edge) = node.value()
                    {
                        let edge = edge.as_ref();
                        if edge.src().label_set() == *vertex_label_set
                            || edge.dst().label_set() == *vertex_label_set
                        {
                            return Err(CatalogTxnError::ReferentialIntegrity {
                                reason: "vertex type is still referenced by edge types".to_string(),
                            });
                        }
                    }
                }
                Ok(())
            }
            IntegrityKind::EdgeSrcDstExist { src, dst } => {
                let s = self
                    .vertex_type_map
                    .get_node_visible(src, txn.start_ts(), txn.txn_id())
                    .is_some();
                let d = self
                    .vertex_type_map
                    .get_node_visible(dst, txn.start_ts(), txn.txn_id())
                    .is_some();
                if s && d {
                    Ok(())
                } else {
                    Err(CatalogTxnError::ReferentialIntegrity {
                        reason: "edge src/dst vertex type not found at commit".to_string(),
                    })
                }
            }
        }
    }
}

impl GraphTypeProvider for MemoryGraphTypeCatalog {
    #[inline]
    fn get_label_id(&self, name: &str) -> CatalogResult<Option<LabelId>> {
        let txn = txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .map_err(|e| CatalogError::External(Box::new(e)))?;
        let res = self.get_label_id_txn(name, txn.as_ref());
        txn.abort().ok();
        res
    }

    #[inline]
    fn get_label_id_txn(&self, name: &str, txn: &CatalogTxn) -> CatalogResult<Option<LabelId>> {
        Ok(self
            .label_map
            .get(&name.to_string(), txn)
            .map(|arc| *arc.as_ref()))
    }

    #[inline]
    fn label_names(&self) -> Vec<String> {
        let txn = match txn_manager().begin_transaction(IsolationLevel::Serializable) {
            Ok(txn) => txn,
            Err(_) => return Vec::new(),
        };
        let res = self.label_names_txn(txn.as_ref());
        txn.abort().ok();
        res
    }

    #[inline]
    fn label_names_txn(&self, txn: &CatalogTxn) -> Vec<String> {
        self.label_map.visible_keys(txn.start_ts(), txn.txn_id())
    }

    #[inline]
    fn get_vertex_type(&self, key: &LabelSet) -> CatalogResult<Option<VertexTypeRef>> {
        let txn = txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .map_err(|e| CatalogError::External(Box::new(e)))?;
        let res = self.get_vertex_type_txn(key, txn.as_ref());
        txn.abort().ok();
        res
    }

    #[inline]
    fn get_vertex_type_txn(
        &self,
        key: &LabelSet,
        txn: &CatalogTxn,
    ) -> CatalogResult<Option<VertexTypeRef>> {
        Ok(self
            .vertex_type_map
            .get(key, txn)
            .map(|arc| arc.as_ref().clone()))
    }

    #[inline]
    fn vertex_type_keys(&self) -> Vec<LabelSet> {
        let txn = match txn_manager().begin_transaction(IsolationLevel::Serializable) {
            Ok(txn) => txn,
            Err(_) => return Vec::new(),
        };
        let res = self.vertex_type_keys_txn(txn.as_ref());
        txn.abort().ok();
        res
    }

    #[inline]
    fn vertex_type_keys_txn(&self, txn: &CatalogTxn) -> Vec<LabelSet> {
        self.vertex_type_map
            .visible_keys(txn.start_ts(), txn.txn_id())
    }

    #[inline]
    fn get_edge_type(&self, key: &LabelSet) -> CatalogResult<Option<EdgeTypeRef>> {
        let txn = txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .map_err(|e| CatalogError::External(Box::new(e)))?;
        let res = self.get_edge_type_txn(key, txn.as_ref());
        txn.abort().ok();
        res
    }

    #[inline]
    fn get_edge_type_txn(
        &self,
        key: &LabelSet,
        txn: &CatalogTxn,
    ) -> CatalogResult<Option<EdgeTypeRef>> {
        Ok(self
            .edge_type_map
            .get(key, txn)
            .map(|arc| arc.as_ref().clone()))
    }

    #[inline]
    fn edge_type_keys(&self) -> Vec<LabelSet> {
        let txn = match txn_manager().begin_transaction(IsolationLevel::Serializable) {
            Ok(txn) => txn,
            Err(_) => return Vec::new(),
        };
        let res = self.edge_type_keys_txn(txn.as_ref());
        txn.abort().ok();
        res
    }

    #[inline]
    fn edge_type_keys_txn(&self, txn: &CatalogTxn) -> Vec<LabelSet> {
        self.edge_type_map
            .visible_keys(txn.start_ts(), txn.txn_id())
    }
}

#[derive(Debug)]
pub struct MemoryVertexTypeCatalog {
    label_set: LabelSet,
    properties: Vec<Property>,
}

impl MemoryVertexTypeCatalog {
    #[inline]
    pub fn new(label_set: LabelSet, properties: Vec<Property>) -> Self {
        Self {
            label_set,
            properties,
        }
    }
}

impl VertexTypeProvider for MemoryVertexTypeCatalog {
    #[inline]
    fn label_set(&self) -> LabelSet {
        self.label_set.clone()
    }
}

impl PropertiesProvider for MemoryVertexTypeCatalog {
    fn get_property(&self, name: &str) -> CatalogResult<Option<(PropertyId, &Property)>> {
        Ok(self
            .properties
            .iter()
            .enumerate()
            .find(|(_, p)| p.name() == name)
            .map(|(i, p)| (i as PropertyId, p)))
    }

    #[inline]
    fn properties(&self) -> Vec<(PropertyId, Property)> {
        self.properties
            .iter()
            .enumerate()
            .map(|(i, p)| (i as PropertyId, p.clone()))
            .collect()
    }
}

#[derive(Debug)]
pub struct MemoryEdgeTypeCatalog {
    label_set: LabelSet,
    src: VertexTypeRef,
    dst: VertexTypeRef,
    properties: Vec<Property>,
}

impl MemoryEdgeTypeCatalog {
    #[inline]
    pub fn new(
        label_set: LabelSet,
        src: VertexTypeRef,
        dst: VertexTypeRef,
        properties: Vec<Property>,
    ) -> Self {
        Self {
            label_set,
            src,
            dst,
            properties,
        }
    }
}

impl EdgeTypeProvider for MemoryEdgeTypeCatalog {
    #[inline]
    fn label_set(&self) -> LabelSet {
        self.label_set.clone()
    }

    #[inline]
    fn src(&self) -> VertexTypeRef {
        self.src.clone()
    }

    #[inline]
    fn dst(&self) -> VertexTypeRef {
        self.dst.clone()
    }
}

impl PropertiesProvider for MemoryEdgeTypeCatalog {
    fn get_property(&self, name: &str) -> CatalogResult<Option<(PropertyId, &Property)>> {
        Ok(self
            .properties
            .iter()
            .enumerate()
            .find(|(_, p)| p.name() == name)
            .map(|(i, p)| (i as PropertyId, p)))
    }

    #[inline]
    fn properties(&self) -> Vec<(PropertyId, Property)> {
        self.properties
            .iter()
            .enumerate()
            .map(|(i, p)| (i as PropertyId, p.clone()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::iter;
    use std::sync::Arc;

    use minigu_common::data_type::LogicalType;

    use super::{MemoryEdgeTypeCatalog, MemoryGraphTypeCatalog, MemoryVertexTypeCatalog};
    use crate::label_set::LabelSet;
    use crate::property::Property;
    use crate::provider::GraphTypeProvider;
    use crate::txn::{CatalogTxnError, CatalogTxnManager};

    fn ls1(label: minigu_common::types::LabelId) -> LabelSet {
        iter::once(label).collect()
    }

    fn vt(label_set: LabelSet, prop_name: &str) -> Arc<MemoryVertexTypeCatalog> {
        Arc::new(MemoryVertexTypeCatalog::new(
            label_set,
            vec![Property::new(
                prop_name.to_string(),
                LogicalType::Int32,
                true,
            )],
        ))
    }

    fn empty_vt(label_set: LabelSet) -> Arc<MemoryVertexTypeCatalog> {
        Arc::new(MemoryVertexTypeCatalog::new(label_set, Vec::new()))
    }

    fn empty_et(
        label_set: LabelSet,
        src: crate::provider::VertexTypeRef,
        dst: crate::provider::VertexTypeRef,
    ) -> Arc<MemoryEdgeTypeCatalog> {
        Arc::new(MemoryEdgeTypeCatalog::new(label_set, src, dst, Vec::new()))
    }

    #[test]
    fn test_catalog_txn_creation_invariants() {
        let mgr = CatalogTxnManager::new();
        let graph_type = MemoryGraphTypeCatalog::new();

        let txn = mgr
            .begin_transaction(minigu_common::IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");

        let txn_id_1 = txn.txn_id();
        let txn_id_2 = txn.txn_id();
        assert_eq!(txn_id_1, txn_id_2);
        assert!(txn_id_1.is_txn_id(), "txn_id should be in txn-id domain");

        let start_ts = txn.start_ts();
        assert!(start_ts.raw() > 0, "start_ts should be > 0");
        assert!(
            start_ts.is_commit_ts(),
            "start_ts should be in commit-ts domain"
        );

        assert!(matches!(
            txn.isolation_level(),
            minigu_common::IsolationLevel::Serializable
        ));

        let _ = graph_type
            .get_label_id_txn("nonexistent", txn.as_ref())
            .expect("get_label_id_txn should succeed");
        assert_eq!(
            txn.txn_id(),
            txn_id_1,
            "txn_id must remain stable within a txn"
        );
    }

    #[test]
    fn test_add_label_txn_commit_visible() {
        let mgr = CatalogTxnManager::new();
        let graph_type = MemoryGraphTypeCatalog::new();

        let txn1 = mgr
            .begin_transaction(minigu_common::IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");
        let id = graph_type
            .add_label_txn("A".to_string(), txn1.as_ref())
            .expect("add_label_txn should succeed");
        txn1.commit().expect("commit should succeed");
        assert!(id.get() > 0, "LabelId must be non-zero");

        let txn2 = mgr
            .begin_transaction(minigu_common::IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");
        let visible = graph_type
            .get_label_id_txn("A", txn2.as_ref())
            .expect("get_label_id_txn should succeed");
        txn2.abort().ok();

        assert_eq!(visible, Some(id));
    }

    #[test]
    fn test_add_label_txn_abort_or_drop_not_visible_and_no_leftover_head() {
        let mgr = CatalogTxnManager::new();
        let graph_type = MemoryGraphTypeCatalog::new();

        let txn = mgr
            .begin_transaction(minigu_common::IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");
        let _ = graph_type
            .add_label_txn("A".to_string(), txn.as_ref())
            .expect("add_label_txn should succeed");

        // Exercise Drop-based best-effort abort.
        drop(txn);

        let txn2 = mgr
            .begin_transaction(minigu_common::IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");
        let visible = graph_type
            .get_label_id_txn("A", txn2.as_ref())
            .expect("get_label_id_txn should succeed");
        assert!(
            visible.is_none(),
            "aborted/uncommitted label must be invisible"
        );

        // If an uncommitted head leaked, this could fail with WriteConflict or similar.
        let res = graph_type.add_label_txn("A".to_string(), txn2.as_ref());
        assert!(
            res.is_ok(),
            "after abort/drop, the same key should be creatable again (no leaked head)"
        );
        txn2.abort().ok();
    }

    #[test]
    fn test_add_label_txn_duplicate_already_exists() {
        let mgr = CatalogTxnManager::new();
        let graph_type = MemoryGraphTypeCatalog::new();

        let txn = mgr
            .begin_transaction(minigu_common::IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");
        let _ = graph_type
            .add_label_txn("A".to_string(), txn.as_ref())
            .expect("first add_label_txn should succeed");
        let res = graph_type.add_label_txn("A".to_string(), txn.as_ref());
        assert!(matches!(res, Err(CatalogTxnError::AlreadyExists { .. })));
    }

    #[test]
    fn p0_5_remove_label_used_by_vertex_type_commit_fails_and_state_kept() {
        let mgr = CatalogTxnManager::new();
        let graph_type = MemoryGraphTypeCatalog::new();

        // T0: create label + vertex type, commit them.
        let txn0 = mgr
            .begin_transaction(minigu_common::IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");
        let label_id = graph_type
            .add_label_txn("L".to_string(), txn0.as_ref())
            .expect("add_label_txn should succeed");
        let v_ls = ls1(label_id);
        graph_type
            .add_vertex_type_txn(v_ls.clone(), empty_vt(v_ls.clone()), txn0.as_ref())
            .expect("add_vertex_type_txn should succeed");
        txn0.commit().expect("commit should succeed");

        // T1: attempt to delete label, should fail at commit due to hook.
        let txn1 = mgr
            .begin_transaction(minigu_common::IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");
        graph_type
            .remove_label_txn("L", txn1.as_ref())
            .expect("remove_label_txn should succeed");
        let commit_res = txn1.commit();
        assert!(matches!(
            commit_res,
            Err(CatalogTxnError::ReferentialIntegrity { .. })
        ));
        drop(txn1);

        // Both label and vertex type should still exist (from T0).
        let txn2 = mgr
            .begin_transaction(minigu_common::IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");
        let label_visible = graph_type
            .get_label_id_txn("L", txn2.as_ref())
            .expect("get_label_id_txn should succeed");
        assert_eq!(label_visible, Some(label_id));

        let vt_visible = graph_type
            .get_vertex_type_txn(&v_ls, txn2.as_ref())
            .expect("get_vertex_type_txn should succeed");
        assert!(
            vt_visible.is_some(),
            "vertex type should remain after failed delete"
        );
        txn2.abort().ok();
    }

    #[test]
    fn test_add_vertex_type_txn_commit_visible() {
        let mgr = CatalogTxnManager::new();
        let graph_type = MemoryGraphTypeCatalog::new();

        // Create the label first for a realistic label_set.
        let txn0 = mgr
            .begin_transaction(minigu_common::IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");
        let label_id = graph_type
            .add_label_txn("V".to_string(), txn0.as_ref())
            .expect("add_label_txn should succeed");
        txn0.commit().expect("commit should succeed");

        let key = ls1(label_id);
        let txn1 = mgr
            .begin_transaction(minigu_common::IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");
        graph_type
            .add_vertex_type_txn(key.clone(), empty_vt(key.clone()), txn1.as_ref())
            .expect("add_vertex_type_txn should succeed");
        txn1.commit().expect("commit should succeed");

        let txn2 = mgr
            .begin_transaction(minigu_common::IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");
        let vt_ref = graph_type
            .get_vertex_type_txn(&key, txn2.as_ref())
            .expect("get_vertex_type_txn should succeed")
            .expect("vertex type should be visible after commit");
        assert_eq!(vt_ref.label_set(), key);
        txn2.abort().ok();
    }

    #[test]
    fn test_remove_vertex_type_referenced_by_edge_type_commit_fails() {
        let mgr = CatalogTxnManager::new();
        let graph_type = MemoryGraphTypeCatalog::new();

        // T0: create A/B vertex types + edge type, commit.
        let txn0 = mgr
            .begin_transaction(minigu_common::IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");
        let a_id = graph_type
            .add_label_txn("A".to_string(), txn0.as_ref())
            .expect("add_label_txn should succeed");
        let b_id = graph_type
            .add_label_txn("B".to_string(), txn0.as_ref())
            .expect("add_label_txn should succeed");
        let e_id = graph_type
            .add_label_txn("E".to_string(), txn0.as_ref())
            .expect("add_label_txn should succeed");

        let a_ls = ls1(a_id);
        let b_ls = ls1(b_id);
        let e_ls = ls1(e_id);

        let a_vt: Arc<MemoryVertexTypeCatalog> = empty_vt(a_ls.clone());
        let b_vt: Arc<MemoryVertexTypeCatalog> = empty_vt(b_ls.clone());
        graph_type
            .add_vertex_type_txn(a_ls.clone(), Arc::clone(&a_vt), txn0.as_ref())
            .expect("add_vertex_type_txn(A) should succeed");
        graph_type
            .add_vertex_type_txn(b_ls.clone(), Arc::clone(&b_vt), txn0.as_ref())
            .expect("add_vertex_type_txn(B) should succeed");

        let a_ref: crate::provider::VertexTypeRef = a_vt;
        let b_ref: crate::provider::VertexTypeRef = b_vt;
        let edge = empty_et(e_ls.clone(), a_ref, b_ref);
        graph_type
            .add_edge_type_txn(e_ls.clone(), edge, txn0.as_ref())
            .expect("add_edge_type_txn should succeed");
        txn0.commit().expect("commit should succeed");

        // T1: remove vertex type A should fail at commit due to vertex_delete hook.
        let txn1 = mgr
            .begin_transaction(minigu_common::IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");
        graph_type
            .remove_vertex_type_txn(&a_ls, txn1.as_ref())
            .expect("remove_vertex_type_txn should succeed");
        let commit_res = txn1.commit();
        assert!(matches!(
            commit_res,
            Err(CatalogTxnError::ReferentialIntegrity { .. })
        ));
        drop(txn1);

        // Vertex type A and edge type E should still exist.
        let txn2 = mgr
            .begin_transaction(minigu_common::IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");
        assert!(
            graph_type
                .get_vertex_type_txn(&a_ls, txn2.as_ref())
                .unwrap()
                .is_some(),
            "vertex type A should remain after failed delete"
        );
        assert!(
            graph_type
                .get_edge_type_txn(&e_ls, txn2.as_ref())
                .unwrap()
                .is_some(),
            "edge type should remain after failed delete"
        );
        txn2.abort().ok();
    }

    #[test]
    fn test_add_edge_type_src_dst_missing_fails_immediately_and_no_write() {
        let mgr = CatalogTxnManager::new();
        let graph_type = MemoryGraphTypeCatalog::new();

        // T0: create src vertex type only.
        let txn0 = mgr
            .begin_transaction(minigu_common::IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");
        let src_id = graph_type
            .add_label_txn("SRC".to_string(), txn0.as_ref())
            .expect("add_label_txn should succeed");
        let dst_id = graph_type
            .add_label_txn("DST".to_string(), txn0.as_ref())
            .expect("add_label_txn should succeed");
        let e_id = graph_type
            .add_label_txn("E".to_string(), txn0.as_ref())
            .expect("add_label_txn should succeed");

        let src_ls = ls1(src_id);
        let dst_ls = ls1(dst_id);
        let e_ls = ls1(e_id);

        let src_vt: Arc<MemoryVertexTypeCatalog> = empty_vt(src_ls.clone());
        graph_type
            .add_vertex_type_txn(src_ls.clone(), Arc::clone(&src_vt), txn0.as_ref())
            .expect("add_vertex_type_txn(SRC) should succeed");
        txn0.commit().expect("commit should succeed");

        // T1: try to add edge type with missing dst vertex type; should fail before any write.
        let txn1 = mgr
            .begin_transaction(minigu_common::IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");
        let src_ref: crate::provider::VertexTypeRef = src_vt;
        let dst_ref: crate::provider::VertexTypeRef = empty_vt(dst_ls.clone());
        let edge = empty_et(e_ls.clone(), src_ref, dst_ref);
        let res = graph_type.add_edge_type_txn(e_ls.clone(), edge, txn1.as_ref());
        assert!(matches!(
            res,
            Err(CatalogTxnError::ReferentialIntegrity { .. })
        ));

        let visible = graph_type
            .get_edge_type_txn(&e_ls, txn1.as_ref())
            .expect("get_edge_type_txn should succeed");
        assert!(
            visible.is_none(),
            "no edge type should be visible in txn after failure"
        );

        txn1.commit().expect("empty commit should succeed");
    }

    #[test]
    fn test_edge_src_dst_exist_hook_commit_fails_if_dst_deleted_in_same_txn() {
        let mgr = CatalogTxnManager::new();
        let graph_type = MemoryGraphTypeCatalog::new();

        // T0: create A/B vertex types + edge type E, commit.
        let txn0 = mgr
            .begin_transaction(minigu_common::IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");
        let a_id = graph_type
            .add_label_txn("A".to_string(), txn0.as_ref())
            .expect("add_label_txn should succeed");
        let b_id = graph_type
            .add_label_txn("B".to_string(), txn0.as_ref())
            .expect("add_label_txn should succeed");
        let e_id = graph_type
            .add_label_txn("E".to_string(), txn0.as_ref())
            .expect("add_label_txn should succeed");

        let a_ls = ls1(a_id);
        let b_ls = ls1(b_id);
        let e_ls = ls1(e_id);

        let a_vt: Arc<MemoryVertexTypeCatalog> = empty_vt(a_ls.clone());
        let b_vt: Arc<MemoryVertexTypeCatalog> = empty_vt(b_ls.clone());
        graph_type
            .add_vertex_type_txn(a_ls.clone(), Arc::clone(&a_vt), txn0.as_ref())
            .expect("add_vertex_type_txn(A) should succeed");
        graph_type
            .add_vertex_type_txn(b_ls.clone(), Arc::clone(&b_vt), txn0.as_ref())
            .expect("add_vertex_type_txn(B) should succeed");

        let a_ref: crate::provider::VertexTypeRef = a_vt.clone();
        let b_ref: crate::provider::VertexTypeRef = b_vt.clone();
        let edge = empty_et(e_ls.clone(), a_ref, b_ref);
        graph_type
            .add_edge_type_txn(e_ls.clone(), edge, txn0.as_ref())
            .expect("add_edge_type_txn should succeed");
        txn0.commit().expect("commit should succeed");

        // T1: replace edge type (registers EdgeSrcDstExist hook), then delete dst vertex type.
        let txn1 = mgr
            .begin_transaction(minigu_common::IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");

        let a_ref_new: crate::provider::VertexTypeRef = a_vt;
        let b_ref_new: crate::provider::VertexTypeRef = b_vt;
        let new_edge = empty_et(e_ls.clone(), a_ref_new, b_ref_new);
        graph_type
            .replace_edge_type_txn(&e_ls, new_edge, txn1.as_ref())
            .expect("replace_edge_type_txn should succeed");
        graph_type
            .remove_vertex_type_txn(&b_ls, txn1.as_ref())
            .expect("remove_vertex_type_txn(B) should succeed");

        let commit_res = txn1.commit();
        assert!(matches!(
            commit_res,
            Err(CatalogTxnError::ReferentialIntegrity { .. })
        ));
        drop(txn1);

        // After abort, both vertex type B and edge type E should still exist as in T0.
        let txn2 = mgr
            .begin_transaction(minigu_common::IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");
        assert!(
            graph_type
                .get_vertex_type_txn(&b_ls, txn2.as_ref())
                .unwrap()
                .is_some(),
            "dst vertex type should remain after failed commit"
        );
        assert!(
            graph_type
                .get_edge_type_txn(&e_ls, txn2.as_ref())
                .unwrap()
                .is_some(),
            "edge type should remain after failed commit"
        );
        txn2.abort().ok();
    }

    #[test]
    fn test_replace_vertex_type_txn_atomic_replace_semantics() {
        let mgr = CatalogTxnManager::new();
        let graph_type = MemoryGraphTypeCatalog::new();

        // T0: create label + vertex type V1, commit.
        let txn0 = mgr
            .begin_transaction(minigu_common::IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");
        let id = graph_type
            .add_label_txn("V".to_string(), txn0.as_ref())
            .expect("add_label_txn should succeed");
        let key = ls1(id);
        graph_type
            .add_vertex_type_txn(key.clone(), vt(key.clone(), "p1"), txn0.as_ref())
            .expect("add_vertex_type_txn should succeed");
        txn0.commit().expect("commit should succeed");

        // T1: replace with V2 and commit.
        let txn1 = mgr
            .begin_transaction(minigu_common::IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");
        graph_type
            .replace_vertex_type_txn(&key, vt(key.clone(), "p2"), txn1.as_ref())
            .expect("replace_vertex_type_txn should succeed");
        txn1.commit().expect("commit should succeed");

        // Verify: key unchanged, value replaced.
        let txn2 = mgr
            .begin_transaction(minigu_common::IsolationLevel::Serializable)
            .expect("begin_transaction should succeed");
        let vt_ref = graph_type
            .get_vertex_type_txn(&key, txn2.as_ref())
            .expect("get_vertex_type_txn should succeed")
            .expect("vertex type should exist");
        assert_eq!(vt_ref.label_set(), key);

        let props = vt_ref.properties();
        assert!(
            props.iter().any(|(_, p)| p.name() == "p2"),
            "replaced vertex type should contain new property"
        );
        assert!(
            !props.iter().any(|(_, p)| p.name() == "p1"),
            "replaced vertex type should not contain old property"
        );
        txn2.abort().ok();
    }
}
