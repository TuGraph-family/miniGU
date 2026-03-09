use std::any::Any;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fmt::{self, Debug};
use std::sync::{Arc, Mutex, RwLock};

use minigu_catalog::error::CatalogResult;
use minigu_catalog::memory::graph_type::MemoryGraphTypeCatalog;
use minigu_catalog::provider::{
    GraphIndexCatalog, GraphIndexCatalogRef, GraphProvider, GraphTypeRef,
    VectorIndexCatalogEntries, VectorIndexCatalogEntry,
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
    entries: HashMap<VectorIndexKey, VectorIndexCatalogEntry>,
    name_to_index: HashMap<String, VectorIndexKey>,
}

#[derive(Debug, Default)]
struct MemoryGraphIndexCatalog {
    state: RwLock<IndexCatalogState>,
}

impl GraphIndexCatalog for MemoryGraphIndexCatalog {
    fn get_vector_index(
        &self,
        key: VectorIndexKey,
    ) -> CatalogResult<Option<VectorIndexCatalogEntry>> {
        let state = self.state.read().expect("index catalog should be readable");
        Ok(state.entries.get(&key).cloned())
    }

    fn get_vector_index_by_name(
        &self,
        name: &str,
    ) -> CatalogResult<Option<VectorIndexCatalogEntry>> {
        let state = self.state.read().expect("index catalog should be readable");
        let key = state.name_to_index.get(name).copied();
        Ok(key.and_then(|key| state.entries.get(&key).cloned()))
    }

    fn insert_vector_index(&self, meta: VectorIndexCatalogEntry) -> CatalogResult<bool> {
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

    fn list_vector_indices(&self) -> CatalogResult<VectorIndexCatalogEntries> {
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
        meta: VectorIndexCatalogEntry,
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
            return Err(IndexCatalogError::VectorIndexNameAlreadyExists(
                meta.name.to_string(),
            ));
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
        rollback_meta: Option<VectorIndexCatalogEntry>,
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
    use minigu_catalog::label_set::LabelSet;
    use minigu_catalog::memory::graph_type::MemoryVertexTypeCatalog;
    use minigu_catalog::property::Property;
    use minigu_common::data_type::LogicalType;
    use minigu_common::types::{PropertyId, VectorMetric};
    use minigu_common::value::{F32, ScalarValue, VectorValue};
    use minigu_storage::common::{PropertyRecord, Vertex};
    use minigu_transaction::Transaction;

    use super::*;

    const TEST_DIMENSION: usize = 104;
    const UNSUPPORTED_DIMENSION: usize = 100;
    const EMBEDDING_PROP_ID: PropertyId = 1;

    fn build_container_with_vectors(
        vertex_count: usize,
        dimension: usize,
    ) -> (GraphContainer, Arc<MemoryGraph>, VectorIndexKey) {
        let graph = MemoryGraph::in_memory();
        let mut graph_type = MemoryGraphTypeCatalog::new();

        let person_label_id = graph_type
            .add_label("PERSON".to_string())
            .expect("label should be created");
        let person_label_set: LabelSet = vec![person_label_id].into_iter().collect();
        let person_type = Arc::new(MemoryVertexTypeCatalog::new(
            person_label_set.clone(),
            vec![
                Property::new("name".to_string(), LogicalType::String, false),
                Property::new(
                    "embedding".to_string(),
                    LogicalType::Vector(dimension),
                    false,
                ),
            ],
        ));
        assert!(graph_type.add_vertex_type(person_label_set, person_type));

        let container = GraphContainer::new(
            Arc::new(graph_type),
            GraphStorage::Memory(Arc::clone(&graph)),
        );

        populate_vertices(&graph, person_label_id, vertex_count, dimension);

        let key = VectorIndexKey::new(person_label_id, EMBEDDING_PROP_ID);
        (container, graph, key)
    }

    fn populate_vertices(
        graph: &Arc<MemoryGraph>,
        label_id: LabelId,
        vertex_count: usize,
        dimension: usize,
    ) {
        let txn = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .expect("transaction should begin");
        for i in 0..vertex_count {
            let vector = build_vector(i, dimension);
            let vertex = Vertex::new(
                i as u64,
                label_id,
                PropertyRecord::new(vec![
                    ScalarValue::String(Some(format!("person{i}"))),
                    ScalarValue::new_vector(dimension, Some(vector)),
                ]),
            );
            graph
                .create_vertex(&txn, vertex)
                .expect("vertex should be inserted");
        }
        txn.commit().expect("vertex load transaction should commit");
    }

    fn build_vector(seed: usize, dimension: usize) -> VectorValue {
        let mut data = Vec::with_capacity(dimension);
        let denominator = dimension as f32;
        for idx in 0..dimension {
            data.push(F32::from(((seed + idx) as f32) / denominator));
        }
        VectorValue::new(data, dimension).expect("vector should be constructable")
    }

    fn make_entry(name: &str, key: VectorIndexKey, dimension: usize) -> VectorIndexCatalogEntry {
        VectorIndexCatalogEntry {
            name: name.into(),
            key,
            metric: VectorMetric::L2,
            dimension,
        }
    }

    #[test]
    fn create_vector_index_registers_catalog_and_builds_storage() {
        let dimension = TEST_DIMENSION;
        let (container, graph, key) = build_container_with_vectors(4, dimension);
        let txn = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();
        let entry = make_entry("person_vec", key, dimension);

        assert!(
            container
                .create_vector_index(graph.as_ref(), &txn, entry.clone())
                .unwrap(),
            "first creation should insert catalog entry"
        );

        txn.commit().unwrap();

        let indices = container.index_catalog().list_vector_indices().unwrap();
        assert_eq!(indices.len(), 1, "catalog should contain exactly one entry");
        assert_eq!(
            indices[0].name.as_str(),
            "person_vec",
            "catalog entry should use provided index name"
        );
        assert!(
            graph.get_vector_index(key).is_some(),
            "storage should build matching vector index"
        );
    }

    #[test]
    fn create_vector_index_same_name_same_key_returns_false() {
        let dimension = TEST_DIMENSION;
        let (container, graph, key) = build_container_with_vectors(4, dimension);
        let entry = make_entry("person_vec", key, dimension);

        let txn1 = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();
        assert!(
            container
                .create_vector_index(graph.as_ref(), &txn1, entry.clone())
                .unwrap(),
            "initial creation should succeed"
        );
        txn1.commit().unwrap();

        let txn2 = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();
        assert!(
            !container
                .create_vector_index(graph.as_ref(), &txn2, entry)
                .unwrap(),
            "duplicate key/name should return Ok(false)"
        );
        txn2.commit().unwrap();
    }

    #[test]
    fn create_vector_index_conflicting_name_errors() {
        let dimension = TEST_DIMENSION;
        let (container, graph, key) = build_container_with_vectors(4, dimension);
        let entry = make_entry("person_vec", key, dimension);

        let txn1 = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();
        assert!(
            container
                .create_vector_index(graph.as_ref(), &txn1, entry)
                .unwrap(),
            "initial creation should succeed"
        );
        txn1.commit().unwrap();

        let conflict_key = VectorIndexKey::new(key.label_id, key.property_id + 1);
        let txn2 = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();
        let err = container
            .create_vector_index(
                graph.as_ref(),
                &txn2,
                make_entry("person_vec", conflict_key, dimension),
            )
            .unwrap_err();
        txn2.abort().unwrap();

        match err {
            IndexCatalogError::VectorIndexNameAlreadyExists(name) => assert_eq!(
                name, "person_vec",
                "conflicting name should report offending index name"
            ),
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn create_vector_index_rolls_back_on_build_failure() {
        let dimension = TEST_DIMENSION;
        let (container, graph, key) = build_container_with_vectors(4, dimension);
        let invalid_key = VectorIndexKey::new(key.label_id, 0);
        let txn = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();
        let err = container
            .create_vector_index(
                graph.as_ref(),
                &txn,
                make_entry("invalid_vec", invalid_key, dimension),
            )
            .unwrap_err();
        txn.abort().unwrap();

        assert!(
            matches!(err, IndexCatalogError::Storage(_)),
            "storage failure should bubble up as IndexCatalogError::Storage"
        );
        let entries = container.index_catalog().list_vector_indices().unwrap();
        assert!(
            entries.is_empty(),
            "catalog should rollback insertion when storage build fails"
        );
        assert!(
            graph.get_vector_index(invalid_key).is_none(),
            "storage should not retain partially built index"
        );
    }

    #[test]
    fn create_vector_index_rejects_unsupported_dimension() {
        let dimension = UNSUPPORTED_DIMENSION;
        let (container, graph, key) = build_container_with_vectors(4, dimension);
        let txn = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();
        let err = container
            .create_vector_index(
                graph.as_ref(),
                &txn,
                make_entry("bad_dim_vec", key, dimension),
            )
            .unwrap_err();
        txn.abort().unwrap();

        assert!(
            matches!(err, IndexCatalogError::Storage(_)),
            "empty dataset should surface as storage error"
        );
        assert!(
            container
                .index_catalog()
                .list_vector_indices()
                .unwrap()
                .is_empty(),
            "catalog should remain empty for unsupported dimension build"
        );
        assert!(
            graph.get_vector_index(key).is_none(),
            "storage should not create vector index for unsupported dimension"
        );
    }

    #[test]
    fn drop_vector_index_removes_index_and_catalog_entry() {
        let dimension = TEST_DIMENSION;
        let (container, graph, key) = build_container_with_vectors(4, dimension);
        let txn = graph
            .txn_manager()
            .begin_transaction(IsolationLevel::Serializable)
            .unwrap();
        let entry = make_entry("person_vec", key, dimension);
        assert!(
            container
                .create_vector_index(graph.as_ref(), &txn, entry.clone())
                .unwrap(),
            "setup should create index successfully"
        );
        txn.commit().unwrap();

        assert!(
            container
                .drop_vector_index(graph.as_ref(), key, Some(entry))
                .unwrap(),
            "drop should remove existing index"
        );

        assert!(
            container
                .index_catalog()
                .list_vector_indices()
                .unwrap()
                .is_empty(),
            "catalog should not retain entry after drop"
        );
        assert!(
            graph.get_vector_index(key).is_none(),
            "storage vector index should be removed"
        );
    }

    #[test]
    fn drop_vector_index_missing_returns_false() {
        let dimension = TEST_DIMENSION;
        let (container, graph, key) = build_container_with_vectors(2, dimension);

        assert!(
            !container
                .drop_vector_index(graph.as_ref(), key, None)
                .unwrap(),
            "dropping non-existent index should return Ok(false)"
        );
    }
}
