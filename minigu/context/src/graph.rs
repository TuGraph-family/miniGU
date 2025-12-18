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

use crate::error::{IndexCatalogError, IndexCatalogResult};

pub enum GraphStorage {
    Memory(Arc<MemoryGraph>),
}

#[derive(Debug, Default)]
struct IndexCatalogState {
    entries: HashMap<VectorIndexKey, VectorIndexMetadata>,
    name_to_index: HashMap<String, VectorIndexKey>,
}

#[derive(Debug, Default)]
struct MemoryGraphIndexCatalog {
    state: RwLock<IndexCatalogState>,
}

impl GraphIndexCatalog for MemoryGraphIndexCatalog {
    fn get_vector_index(&self, key: VectorIndexKey) -> CatalogResult<Option<VectorIndexMetadata>> {
        let state = self.state.read().expect("index catalog should be readable");
        Ok(state.entries.get(&key).cloned())
    }

    fn get_vector_index_by_name(&self, name: &str) -> CatalogResult<Option<VectorIndexMetadata>> {
        let state = self.state.read().expect("index catalog should be readable");
        let key = state.name_to_index.get(name).copied();
        Ok(key.and_then(|key| state.entries.get(&key).cloned()))
    }

    fn insert_vector_index(&self, meta: VectorIndexMetadata) -> CatalogResult<bool> {
        let mut state = self
            .state
            .write()
            .expect("index catalog should be writable");
        match state.entries.entry(meta.key) {
            Entry::Occupied(_) => Ok(false),
            Entry::Vacant(v) => {
                v.insert(meta.clone());
                state.name_to_index.insert(meta.name.to_string(), meta.key);
                Ok(true)
            }
        }
    }

    fn remove_vector_index(&self, key: VectorIndexKey) -> CatalogResult<bool> {
        let mut state = self
            .state
            .write()
            .expect("index catalog should be writable");
        let removed = state.entries.remove(&key);
        if let Some(meta) = removed.as_ref() {
            state.name_to_index.remove(meta.name.as_str());
        }
        Ok(removed.is_some())
    }

    fn list_vector_indices(&self) -> CatalogResult<VectorIndexDefinitions> {
        let state = self.state.read().expect("index catalog should be readable");
        Ok(state.entries.values().cloned().collect())
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
    pub fn graph_type(&self) -> Arc<MemoryGraphTypeCatalog> {
        self.graph_type.clone()
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

        if let Some(existing) = self
            .index_catalog
            .get_vector_index_by_name(meta.name.as_str())?
        {
            if existing.key == meta.key {
                return Ok(false);
            }
            return Err(IndexCatalogError::NameAlreadyExists(meta.name.to_string()));
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
    _label_ids: &Option<Vec<Vec<LabelId>>>,
) -> StorageResult<bool> {
    let Some(label_specs) = _label_ids else {
        return Ok(true);
    };

    let vertex = _mem.get_vertex(_txn, _vid)?;
    let vertex_label = vertex.label_id;

    for and_labels in label_specs {
        if and_labels.is_empty() {
            return Ok(true);
        }
        if and_labels.contains(&vertex_label) {
            return Ok(true);
        }
    }
    Ok(false)
}

impl GraphContainer {
    pub fn vertex_source(
        &self,
        label_ids: &Option<Vec<Vec<LabelId>>>,
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
                if vertex_has_all_labels(&mem, &txn, vid, label_ids)? {
                    ids.push(vid);
                }
            }
        }

        // TODO(Colin): Sort IDs to ensure deterministic output in tests.
        // Remove once ORDER BY is supported.
        ids.sort_unstable();

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

#[cfg(test)]
mod tests {
    use minigu_catalog::provider::VectorIndexMetadata;
    use minigu_common::types::VectorMetric;
    use minigu_common::value::{F32, ScalarValue, VectorValue};
    use minigu_storage::common::{PropertyRecord, Vertex};
    use minigu_storage::error::{StorageError, VectorIndexError};
    use minigu_storage::tp::transaction::IsolationLevel;

    use super::*;

    fn build_container() -> (GraphContainer, Arc<MemoryGraph>) {
        let graph = MemoryGraph::new();
        let graph_type = Arc::new(MemoryGraphTypeCatalog::new());
        let container = GraphContainer::new(graph_type, GraphStorage::Memory(graph.clone()));
        (container, graph)
    }

    fn insert_vertices(
        graph: &Arc<MemoryGraph>,
        txn: &Arc<MemTransaction>,
        label: LabelId,
        dimension: usize,
        count: usize,
    ) {
        for i in 0..count as u64 {
            let mut data = vec![F32::from(0.0); dimension];
            data[0] = F32::from(i as f32);
            data[1] = F32::from((i % 10) as f32);
            let vector = VectorValue::new(data, dimension).unwrap();
            let properties = PropertyRecord::new(vec![
                ScalarValue::String(Some(format!("v{i}"))),
                ScalarValue::new_vector(dimension, Some(vector)),
            ]);
            graph
                .create_vertex(txn, Vertex::new(i + 1, label, properties))
                .unwrap();
        }
    }

    fn insert_vertices_with_start(
        graph: &Arc<MemoryGraph>,
        txn: &Arc<MemTransaction>,
        label: LabelId,
        dimension: usize,
        count: usize,
        start_id: u64,
    ) {
        for i in 0..count as u64 {
            let mut data = vec![F32::from(0.0); dimension];
            data[0] = F32::from((start_id + i) as f32);
            data[1] = F32::from(((start_id + i) % 10) as f32);
            let vector = VectorValue::new(data, dimension).unwrap();
            let properties = PropertyRecord::new(vec![
                ScalarValue::String(Some(format!("v{}", start_id + i))),
                ScalarValue::new_vector(dimension, Some(vector)),
            ]);
            graph
                .create_vertex(txn, Vertex::new(start_id + i, label, properties))
                .unwrap();
        }
    }

    #[test]
    fn create_and_drop_vector_index_updates_catalog_and_storage() {
        let (container, graph) = build_container();
        let txn = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();

        let label = LabelId::new(1).unwrap();
        let property_id = 1u32;
        let dimension = 104usize;
        insert_vertices(&graph, &txn, label, dimension, 50);

        let key = VectorIndexKey::new(label, property_id);
        let meta = VectorIndexMetadata {
            name: "idx_entity_embedding".into(),
            key,
            metric: VectorMetric::L2,
            dimension,
        };

        // Wrong index key should fail (no matching vectors) and leave catalog/storage untouched.
        let wrong_key = VectorIndexKey::new(LabelId::new(2).unwrap(), property_id);
        let wrong_meta = VectorIndexMetadata {
            name: "idx_wrong_label".into(),
            key: wrong_key,
            metric: VectorMetric::L2,
            dimension,
        };
        let err = container
            .create_vector_index(graph.as_ref(), &txn, wrong_meta.clone())
            .unwrap_err();
        assert!(matches!(
            err,
            crate::error::IndexCatalogError::Storage(StorageError::VectorIndex(
                VectorIndexError::EmptyDataset
            ))
        ));
        assert!(
            container
                .index_catalog()
                .get_vector_index(wrong_key)
                .unwrap()
                .is_none()
        );
        assert!(graph.get_vector_index(wrong_key).is_none());

        let created = container
            .create_vector_index(graph.as_ref(), &txn, meta.clone())
            .unwrap();
        assert!(created);
        assert!(graph.get_vector_index(key).is_some());
        assert!(
            container
                .index_catalog()
                .get_vector_index(key)
                .unwrap()
                .is_some()
        );

        let dropped = container
            .drop_vector_index(graph.as_ref(), key, Some(meta))
            .unwrap();
        assert!(dropped);
        assert!(graph.get_vector_index(key).is_none());
        assert!(
            container
                .index_catalog()
                .get_vector_index(key)
                .unwrap()
                .is_none()
        );
        let not_removed = container
            .drop_vector_index(graph.as_ref(), wrong_key, None)
            .unwrap();
        assert!(!not_removed);
    }

    #[test]
    fn duplicate_create_vector_index_is_idempotent() {
        let (container, graph) = build_container();
        let txn = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();

        let label = LabelId::new(1).unwrap();
        let property_id = 1u32;
        let dimension = 104usize;
        insert_vertices(&graph, &txn, label, dimension, 10);

        let key = VectorIndexKey::new(label, property_id);
        let meta = VectorIndexMetadata {
            name: "dup_idx".into(),
            key,
            metric: VectorMetric::L2,
            dimension,
        };

        let first = container
            .create_vector_index(graph.as_ref(), &txn, meta.clone())
            .unwrap();
        assert!(first);

        let second = container
            .create_vector_index(graph.as_ref(), &txn, meta)
            .unwrap();
        assert!(!second);
        assert_eq!(
            container
                .index_catalog()
                .list_vector_indices()
                .unwrap()
                .len(),
            1
        );
    }

    #[test]
    fn concurrent_drop_vector_index_only_drops_once() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        let (container, graph) = build_container();
        let container = Arc::new(container);
        let key = VectorIndexKey::new(LabelId::new(1).unwrap(), 1);

        // prepare data and index
        let txn = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();
        insert_vertices(&graph, &txn, LabelId::new(1).unwrap(), 104, 20);
        let meta = VectorIndexMetadata {
            name: "idx_drop_concurrent".into(),
            key,
            metric: VectorMetric::L2,
            dimension: 104,
        };
        assert!(
            container
                .create_vector_index(graph.as_ref(), &txn, meta.clone())
                .unwrap()
        );

        let success = Arc::new(AtomicUsize::new(0));
        let mut handles = Vec::new();
        for _ in 0..2 {
            let c = Arc::clone(&container);
            let g = Arc::clone(&graph);
            let m = meta.clone();
            let s = Arc::clone(&success);
            handles.push(std::thread::spawn(move || {
                let _txn = g
                    .txn_manager()
                    .begin_transaction(IsolationLevel::Serializable)
                    .unwrap();
                if c.drop_vector_index(g.as_ref(), m.key, Some(m.clone()))
                    .unwrap_or(false)
                {
                    s.fetch_add(1, Ordering::SeqCst);
                }
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(success.load(Ordering::SeqCst), 1);
        assert!(
            container
                .index_catalog()
                .get_vector_index(key)
                .unwrap()
                .is_none()
        );
        assert!(graph.get_vector_index(key).is_none());
    }

    #[test]
    fn create_vector_index_rolls_back_on_storage_error() {
        let (container, graph) = build_container();
        let txn = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();

        let label = LabelId::new(1).unwrap();
        let property_id = 1u32;
        let dimension = 104usize;
        let key = VectorIndexKey::new(label, property_id);
        let meta = VectorIndexMetadata {
            name: "idx_entity_embedding".into(),
            key,
            metric: VectorMetric::L2,
            dimension,
        };

        let err = container
            .create_vector_index(graph.as_ref(), &txn, meta.clone())
            .unwrap_err();
        assert!(matches!(err, crate::error::IndexCatalogError::Storage(_)));
        assert!(
            container
                .index_catalog()
                .get_vector_index(key)
                .unwrap()
                .is_none()
        );
        assert!(graph.get_vector_index(key).is_none());
    }

    #[test]
    fn duplicate_vector_index_name() {
        let (container, graph) = build_container();
        let txn = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();

        let label_one = LabelId::new(1).unwrap();
        let label_two = LabelId::new(2).unwrap();
        let dimension = 104usize;
        insert_vertices(&graph, &txn, label_one, dimension, 10);
        insert_vertices_with_start(&graph, &txn, label_two, dimension, 10, 100);

        let key_one = VectorIndexKey::new(label_one, 1);
        let key_two = VectorIndexKey::new(label_two, 1);
        let meta_one = VectorIndexMetadata {
            name: "dup_name".into(),
            key: key_one,
            metric: VectorMetric::L2,
            dimension,
        };
        let meta_two = VectorIndexMetadata {
            name: "dup_name".into(),
            key: key_two,
            metric: VectorMetric::L2,
            dimension,
        };

        assert!(
            container
                .create_vector_index(graph.as_ref(), &txn, meta_one)
                .unwrap()
        );
        let err = container
            .create_vector_index(graph.as_ref(), &txn, meta_two)
            .unwrap_err();
        assert!(matches!(
            err,
            crate::error::IndexCatalogError::NameAlreadyExists(_)
        ));
    }

    #[test]
    fn vector_index_name_lookup_updates_on_create_and_drop() {
        let (container, graph) = build_container();
        let txn = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();

        let label = LabelId::new(1).unwrap();
        let dimension = 104usize;
        insert_vertices(&graph, &txn, label, dimension, 10);

        let key = VectorIndexKey::new(label, 1);
        let meta = VectorIndexMetadata {
            name: "name_lookup".into(),
            key,
            metric: VectorMetric::L2,
            dimension,
        };

        assert!(
            container
                .create_vector_index(graph.as_ref(), &txn, meta.clone())
                .unwrap()
        );

        let found = container
            .index_catalog()
            .get_vector_index_by_name("name_lookup")
            .unwrap();
        assert!(found.is_some());

        assert!(
            container
                .drop_vector_index(graph.as_ref(), key, Some(meta))
                .unwrap()
        );

        let found = container
            .index_catalog()
            .get_vector_index_by_name("name_lookup")
            .unwrap();
        assert!(found.is_none());
    }
}
