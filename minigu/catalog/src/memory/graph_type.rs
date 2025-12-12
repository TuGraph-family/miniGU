use std::sync::{Arc, Mutex};

use minigu_common::types::{LabelId, PropertyId};
use minigu_transaction::Transaction;

use crate::error::CatalogResult;
use crate::label_set::LabelSet;
use crate::property::Property;
use crate::provider::{
    EdgeTypeProvider, EdgeTypeRef, GraphTypeProvider, PropertiesProvider, VertexTypeProvider,
    VertexTypeRef,
};
use crate::txn::catalog_txn::{CatalogTxn, TxnHook};
use crate::txn::versioned_map::{VersionedMap, WriteOp};

#[derive(Debug)]
pub struct MemoryGraphTypeCatalog {
    // Because transactional reads/writes are introduced, label-id allocation must be thread-safe.
    // Use a Mutex to protect the counter; allow "gaps" after rollbacks.
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

    // ================= Transactional write APIs =================
    #[inline]
    pub fn add_label(
        &self,
        name: String,
        txn: &CatalogTxn,
    ) -> Result<LabelId, crate::txn::error::CatalogTxnError> {
        if self.label_map.get(&name, txn).is_some() {
            return Err(crate::txn::error::CatalogTxnError::AlreadyExists { key: name });
        }
        let mut guard = self.next_label_id.lock().expect("poisoned label id mutex");
        let label_id = *guard;
        *guard = guard.checked_add(1).ok_or_else(|| {
            crate::txn::error::CatalogTxnError::IllegalState {
                reason: "label id overflow".to_string(),
            }
        })?;
        drop(guard);

        let node = self.label_map.put(name.clone(), Arc::new(label_id), txn)?;
        txn.record_write(&self.label_map, name, node, WriteOp::Create);
        Ok(label_id)
    }

    #[inline]
    pub fn remove_label(
        &self,
        name: &str,
        txn: &CatalogTxn,
    ) -> Result<(), crate::txn::error::CatalogTxnError> {
        let key = name.to_string();
        let base = self
            .label_map
            .get_node_visible(&key, txn.start_ts(), txn.txn_id())
            .ok_or_else(|| crate::txn::error::CatalogTxnError::NotFound { key: key.clone() })?;
        // Compute the label id for referential integrity checks.
        let label_id = *base
            .value()
            .ok_or_else(|| crate::txn::error::CatalogTxnError::IllegalState {
                reason: "label node has no value".to_string(),
            })?
            .as_ref();
        // Pre-commit validation hook: ensure no type uses this label.
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
    pub fn add_vertex_type(
        &self,
        label_set: LabelSet,
        vertex_type: VertexTypeRef,
        txn: &CatalogTxn,
    ) -> Result<(), crate::txn::error::CatalogTxnError> {
        // Referential integrity: ensure labels exist (weak check: at least one label exists)
        // and the key is not occupied. The label_set cannot be iterated here, so only enforce
        // uniqueness on insert; upstream is responsible for ensuring label consistency.
        if self.vertex_type_map.get(&label_set, txn).is_some() {
            return Err(crate::txn::error::CatalogTxnError::AlreadyExists {
                key: format!("{:?}", label_set),
            });
        }
        let node = self
            .vertex_type_map
            .put(label_set.clone(), Arc::new(vertex_type), txn)?;
        txn.record_write(&self.vertex_type_map, label_set, node, WriteOp::Create);
        Ok(())
    }

    #[inline]
    pub fn remove_vertex_type(
        &self,
        label_set: &LabelSet,
        txn: &CatalogTxn,
    ) -> Result<(), crate::txn::error::CatalogTxnError> {
        let _base = self
            .vertex_type_map
            .get_node_visible(label_set, txn.start_ts(), txn.txn_id())
            .ok_or_else(|| crate::txn::error::CatalogTxnError::NotFound {
                key: format!("{:?}", label_set),
            })?;
        // Pre-commit validation hook: ensure no edge type references this vertex type.
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
    pub fn add_edge_type(
        &self,
        label_set: LabelSet,
        edge_type: EdgeTypeRef,
        txn: &CatalogTxn,
    ) -> Result<(), crate::txn::error::CatalogTxnError> {
        // Referential integrity: source/destination vertex types must exist.
        let src_ls = edge_type.src().label_set();
        let dst_ls = edge_type.dst().label_set();
        if self.vertex_type_map.get(&src_ls, txn).is_none()
            || self.vertex_type_map.get(&dst_ls, txn).is_none()
        {
            return Err(crate::txn::error::CatalogTxnError::ReferentialIntegrity {
                reason: "edge src/dst vertex type not found".to_string(),
            });
        }
        if self.edge_type_map.get(&label_set, txn).is_some() {
            return Err(crate::txn::error::CatalogTxnError::AlreadyExists {
                key: format!("{:?}", label_set),
            });
        }
        // Pre-commit validation hook: re-check in the latest committed view that src/dst still
        // exist.
        txn.add_hook(Box::new(GraphTypeIntegrityHook::edge_src_dst_exist(
            self.vertex_type_map.clone(),
            src_ls.clone(),
            dst_ls.clone(),
        )));
        let node = self
            .edge_type_map
            .put(label_set.clone(), Arc::new(edge_type), txn)?;
        txn.record_write(&self.edge_type_map, label_set, node, WriteOp::Create);
        Ok(())
    }

    #[inline]
    pub fn remove_edge_type(
        &self,
        label_set: &LabelSet,
        txn: &CatalogTxn,
    ) -> Result<(), crate::txn::error::CatalogTxnError> {
        let _base = self
            .edge_type_map
            .get_node_visible(label_set, txn.start_ts(), txn.txn_id())
            .ok_or_else(|| crate::txn::error::CatalogTxnError::NotFound {
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

    // ================= REPLACE (property changes) =================
    #[inline]
    pub fn replace_vertex_type(
        &self,
        label_set: &LabelSet,
        new_vertex_type: VertexTypeRef,
        txn: &CatalogTxn,
    ) -> Result<(), crate::txn::error::CatalogTxnError> {
        let _ = self
            .vertex_type_map
            .get_node_visible(label_set, txn.start_ts(), txn.txn_id())
            .ok_or_else(|| crate::txn::error::CatalogTxnError::NotFound {
                key: format!("{:?}", label_set),
            })?;
        let node = self
            .vertex_type_map
            .put(label_set.clone(), Arc::new(new_vertex_type), txn)?;
        txn.record_write(
            &self.vertex_type_map,
            label_set.clone(),
            node,
            WriteOp::Replace,
        );
        Ok(())
    }

    #[inline]
    pub fn replace_edge_type(
        &self,
        label_set: &LabelSet,
        new_edge_type: EdgeTypeRef,
        txn: &CatalogTxn,
    ) -> Result<(), crate::txn::error::CatalogTxnError> {
        // Referential integrity: the newly defined src/dst must exist.
        let src_ls = new_edge_type.src().label_set();
        let dst_ls = new_edge_type.dst().label_set();
        if self.vertex_type_map.get(&src_ls, txn).is_none()
            || self.vertex_type_map.get(&dst_ls, txn).is_none()
        {
            return Err(crate::txn::error::CatalogTxnError::ReferentialIntegrity {
                reason: "edge src/dst vertex type not found".to_string(),
            });
        }
        let _ = self
            .edge_type_map
            .get_node_visible(label_set, txn.start_ts(), txn.txn_id())
            .ok_or_else(|| crate::txn::error::CatalogTxnError::NotFound {
                key: format!("{:?}", label_set),
            })?;
        // Pre-commit validation hook: re-check in the latest committed view that src/dst still
        // exist.
        txn.add_hook(Box::new(GraphTypeIntegrityHook::edge_src_dst_exist(
            self.vertex_type_map.clone(),
            src_ls.clone(),
            dst_ls.clone(),
        )));
        let node = self
            .edge_type_map
            .put(label_set.clone(), Arc::new(new_edge_type), txn)?;
        txn.record_write(
            &self.edge_type_map,
            label_set.clone(),
            node,
            WriteOp::Replace,
        );
        Ok(())
    }
}

// ================ Pre-commit validation hook implementation ================
#[derive(Debug)]
struct GraphTypeIntegrityHook {
    // Hold the containers directly to avoid requiring weak references to Self.
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
            vertex_type_map: Arc::new(VersionedMap::new()), // Unused.
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
    fn precommit(&self, txn: &CatalogTxn) -> Result<(), crate::txn::error::CatalogTxnError> {
        use crate::txn::error::CatalogTxnError;
        match &self.kind {
            IntegrityKind::LabelDelete { label_id } => {
                // (1) Vertex-type keys must not contain this label.
                let v_keys = self
                    .vertex_type_map
                    .visible_keys(txn.start_ts(), txn.txn_id());
                if v_keys.iter().any(|k| k.contains(*label_id)) {
                    return Err(CatalogTxnError::ReferentialIntegrity {
                        reason: format!("label {:?} is still used by vertex types", label_id),
                    });
                }
                // (2) Edge-type keys must not contain this label.
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
                // No edge type should reference this vertex type.
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
                // In the latest committed/snapshot view, both src and dst vertex types must exist.
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
    fn get_label_id(&self, name: &str, txn: &CatalogTxn) -> CatalogResult<Option<LabelId>> {
        Ok(self
            .label_map
            .get(&name.to_string(), txn)
            .map(|arc| *arc.as_ref()))
    }

    #[inline]
    fn label_names(&self, txn: &CatalogTxn) -> Vec<String> {
        self.label_map.visible_keys(txn.start_ts(), txn.txn_id())
    }

    #[inline]
    fn get_vertex_type(
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
    fn vertex_type_keys(&self, txn: &CatalogTxn) -> Vec<LabelSet> {
        self.vertex_type_map
            .visible_keys(txn.start_ts(), txn.txn_id())
    }

    #[inline]
    fn get_edge_type(
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
    fn edge_type_keys(&self, txn: &CatalogTxn) -> Vec<LabelSet> {
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
    use minigu_transaction::{GraphTxnManager, IsolationLevel, Transaction};

    use super::*;
    use crate::label_set::LabelSet;
    use crate::txn::manager::CatalogTxnManager;

    #[test]
    fn add_label_and_visibility_commit() {
        let mgr = CatalogTxnManager::new();
        let gt = MemoryGraphTypeCatalog::new();

        let t1 = mgr.begin_transaction(IsolationLevel::Serializable).unwrap();
        let id = gt.add_label("Person".to_string(), &t1).unwrap();
        // visible to self
        assert_eq!(gt.get_label_id("Person", &t1).unwrap(), Some(id));
        // not visible to other txn before commit
        let t2 = mgr.begin_transaction(IsolationLevel::Serializable).unwrap();
        assert_eq!(gt.get_label_id("Person", &t2).unwrap(), None);
        // commit then new txn sees it
        t1.commit().unwrap();
        let t3 = mgr.begin_transaction(IsolationLevel::Serializable).unwrap();
        assert_eq!(gt.get_label_id("Person", &t3).unwrap(), Some(id));
    }

    #[test]
    fn add_vertex_and_edge_types_with_refs() {
        let mgr = CatalogTxnManager::new();
        let gt = MemoryGraphTypeCatalog::new();

        // Seed labels
        let seed = mgr.begin_transaction(IsolationLevel::Serializable).unwrap();
        let l_a = gt.add_label("A".to_string(), &seed).unwrap();
        let l_b = gt.add_label("B".to_string(), &seed).unwrap();
        seed.commit().unwrap();

        // Add vertex types and edge type in a txn
        let t1 = mgr.begin_transaction(IsolationLevel::Serializable).unwrap();
        let ls_a: LabelSet = [l_a].into_iter().collect();
        let ls_b: LabelSet = [l_b].into_iter().collect();
        let vt_a = Arc::new(MemoryVertexTypeCatalog::new(ls_a.clone(), vec![]));
        let vt_b = Arc::new(MemoryVertexTypeCatalog::new(ls_b.clone(), vec![]));
        gt.add_vertex_type(ls_a.clone(), vt_a.clone() as _, &t1)
            .unwrap();
        gt.add_vertex_type(ls_b.clone(), vt_b.clone() as _, &t1)
            .unwrap();

        // Edge type requires src/dst exist
        let et = Arc::new(MemoryEdgeTypeCatalog::new(
            [l_a, l_b].into_iter().collect(),
            vt_a.clone() as _,
            vt_b.clone() as _,
            vec![],
        ));
        gt.add_edge_type(et.label_set(), et.clone() as _, &t1)
            .unwrap();

        // visibility
        assert!(gt.get_vertex_type(&ls_a, &t1).unwrap().is_some());
        let t2 = mgr.begin_transaction(IsolationLevel::Serializable).unwrap();
        assert!(gt.get_vertex_type(&ls_a, &t2).unwrap().is_none());

        t1.commit().unwrap();
        let t3 = mgr.begin_transaction(IsolationLevel::Serializable).unwrap();
        assert!(
            gt.get_edge_type(&([l_a, l_b].into_iter().collect()), &t3)
                .unwrap()
                .is_some()
        );
    }

    #[test]
    fn remove_types_visibility() {
        let mgr = CatalogTxnManager::new();
        let gt = MemoryGraphTypeCatalog::new();

        // Seed labels and vertex type
        let seed = mgr.begin_transaction(IsolationLevel::Serializable).unwrap();
        let l = gt.add_label("X".to_string(), &seed).unwrap();
        let ls: LabelSet = [l].into_iter().collect();
        let vt = Arc::new(MemoryVertexTypeCatalog::new(ls.clone(), vec![]));
        gt.add_vertex_type(ls.clone(), vt.clone() as _, &seed)
            .unwrap();
        seed.commit().unwrap();

        // Remove vertex type in txn
        let t1 = mgr.begin_transaction(IsolationLevel::Serializable).unwrap();
        gt.remove_vertex_type(&ls, &t1).unwrap();
        assert!(gt.get_vertex_type(&ls, &t1).unwrap().is_none());
        let t2 = mgr.begin_transaction(IsolationLevel::Serializable).unwrap();
        assert!(gt.get_vertex_type(&ls, &t2).unwrap().is_some());
        t1.commit().unwrap();
        let t3 = mgr.begin_transaction(IsolationLevel::Serializable).unwrap();
        assert!(gt.get_vertex_type(&ls, &t3).unwrap().is_none());
    }
}
