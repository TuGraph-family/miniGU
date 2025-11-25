use std::any::Any;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fmt::{self, Debug};
use std::sync::{Arc, Mutex, RwLock};

use minigu_catalog::error::CatalogResult;
use minigu_catalog::memory::graph_type::MemoryGraphTypeCatalog;
use minigu_catalog::provider::{
    GraphIndexCatalog, GraphIndexCatalogRef, GraphProvider, GraphTypeRef, VectorIndexDefinitions,
    VectorIndexMetadata,
};
use minigu_common::types::{LabelId, VectorIndexKey, VertexIdArray};
use minigu_storage::error::StorageResult;
use minigu_storage::tp::MemoryGraph;
use minigu_storage::tp::transaction::{IsolationLevel, MemTransaction};
use minigu_transaction::manager::GraphTxnManager;

use crate::error::IndexCatalogResult;

pub enum GraphStorage {
    Memory(Arc<MemoryGraph>),
}

#[derive(Debug, Default)]
struct MemoryGraphIndexCatalog {
    entries: RwLock<HashMap<VectorIndexKey, VectorIndexMetadata>>,
}

impl GraphIndexCatalog for MemoryGraphIndexCatalog {
    fn get_vector_index(&self, key: VectorIndexKey) -> CatalogResult<Option<VectorIndexMetadata>> {
        Ok(self
            .entries
            .read()
            .expect("index map should be readable")
            .get(&key)
            .cloned())
    }

    fn insert_vector_index(&self, meta: VectorIndexMetadata) -> CatalogResult<bool> {
        let mut guard = self.entries.write().expect("index map should be writable");
        match guard.entry(meta.key) {
            Entry::Occupied(_) => Ok(false),
            Entry::Vacant(v) => {
                v.insert(meta);
                Ok(true)
            }
        }
    }

    fn remove_vector_index(&self, key: VectorIndexKey) -> CatalogResult<bool> {
        let mut guard = self.entries.write().expect("index map should be writable");
        Ok(guard.remove(&key).is_some())
    }

    fn list_vector_indices(&self) -> CatalogResult<VectorIndexDefinitions> {
        Ok(self
            .entries
            .read()
            .expect("index map should be readable")
            .values()
            .cloned()
            .collect())
    }
}

pub struct GraphContainer {
    graph_type: Arc<MemoryGraphTypeCatalog>,
    graph_storage: GraphStorage,
    index_catalog: Arc<dyn GraphIndexCatalog>,
    index_op_lock: Mutex<()>,
}

impl GraphContainer {
    pub fn new(graph_type: Arc<MemoryGraphTypeCatalog>, graph_storage: GraphStorage) -> Self {
        Self {
            graph_type,
            graph_storage,
            index_catalog: Arc::new(MemoryGraphIndexCatalog::default()),
            index_op_lock: Mutex::new(()),
        }
    }

    #[inline]
    pub fn graph_storage(&self) -> &GraphStorage {
        &self.graph_storage
    }

    #[inline]
    pub fn index_catalog(&self) -> &Arc<dyn GraphIndexCatalog> {
        &self.index_catalog
    }

    pub fn create_vector_index(
        &self,
        graph: &MemoryGraph,
        txn: &Arc<MemTransaction>,
        meta: VectorIndexMetadata,
    ) -> IndexCatalogResult<bool> {
        let _guard = self
            .index_op_lock
            .lock()
            .expect("index op lock should be acquirable");

        if self.index_catalog.get_vector_index(meta.key)?.is_some() {
            return Ok(false);
        }

        let inserted = self.index_catalog.insert_vector_index(meta.clone())?;
        if !inserted {
            return Ok(false);
        }

        if let Err(err) = graph.build_vector_index(txn, meta.key) {
            let _ = self.index_catalog.remove_vector_index(meta.key);
            return Err(err.into());
        }

        Ok(true)
    }

    pub fn drop_vector_index(
        &self,
        graph: &MemoryGraph,
        key: VectorIndexKey,
        rollback_meta: Option<VectorIndexMetadata>,
    ) -> IndexCatalogResult<bool> {
        let _guard = self
            .index_op_lock
            .lock()
            .expect("index op lock should be acquirable");

        let removed = self.index_catalog.remove_vector_index(key)?;
        if !removed {
            return Ok(false);
        }

        if let Err(err) = graph.delete_vector_index(key) {
            if let Some(meta) = rollback_meta {
                let _ = self.index_catalog.insert_vector_index(meta);
            }
            return Err(err.into());
        }

        Ok(true)
    }
}

// TODO: Remove and use a checker.
fn vertex_has_all_labels(
    _mem: &Arc<MemoryGraph>,
    _txn: &Arc<minigu_storage::tp::transaction::MemTransaction>,
    _vid: u64,
    _label_ids: &[LabelId],
) -> StorageResult<bool> {
    for label_id in _label_ids {
        if _mem.get_vertex(_txn, _vid)?.label_id != *label_id {
            return Ok(false);
        }
    }
    Ok(true)
}

impl GraphContainer {
    pub fn vertex_source(
        &self,
        label_ids: &[LabelId],
        batch_size: usize,
    ) -> StorageResult<Box<dyn Iterator<Item = Arc<VertexIdArray>> + Send + 'static>> {
        let mem = match self.graph_storage() {
            GraphStorage::Memory(m) => Arc::clone(m),
        };
        let txn = mem
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)?;
        let mut ids: Vec<u64> = Vec::new();
        {
            let it = mem.iter_vertices(&txn)?;
            for v in it {
                let v = v?;
                let vid = v.vid();
                if label_ids.is_empty() || vertex_has_all_labels(&mem, &txn, vid, label_ids)? {
                    ids.push(vid);
                }
            }
        }

        let mut pos = 0usize;
        let iter = std::iter::from_fn(move || {
            if pos >= ids.len() {
                return None;
            }
            let end = (pos + batch_size).min(ids.len());
            let slice = &ids[pos..end];
            pos = end;
            Some(Arc::new(VertexIdArray::from_iter(slice.iter().copied())))
        });

        Ok(Box::new(iter))
    }
}

impl Debug for GraphContainer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GraphContainer")
            .field("graph_type", &self.graph_type)
            .finish()
    }
}

impl GraphProvider for GraphContainer {
    #[inline]
    fn graph_type(&self) -> GraphTypeRef {
        self.graph_type.clone()
    }

    fn index_catalog(&self) -> Option<GraphIndexCatalogRef> {
        Some(self.index_catalog.clone())
    }

    #[inline]
    fn as_any(&self) -> &dyn Any {
        self
    }
}
