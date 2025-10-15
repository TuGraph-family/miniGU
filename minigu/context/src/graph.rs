use std::any::Any;
use std::fmt::{self, Debug};
use std::sync::Arc;

use minigu_catalog::memory::graph_type::MemoryGraphTypeCatalog;
use minigu_catalog::provider::{GraphProvider, GraphTypeRef};
use minigu_common::types::{LabelId, VertexIdArray};
use minigu_storage::common::IsolationLevel;
use minigu_storage::error::StorageResult;
use minigu_storage::tp::MemoryGraph;

pub enum GraphStorage {
    Memory(Arc<MemoryGraph>),
}

pub struct GraphContainer {
    graph_type: Arc<MemoryGraphTypeCatalog>,
    graph_storage: GraphStorage,
}

impl GraphContainer {
    pub fn new(graph_type: Arc<MemoryGraphTypeCatalog>, graph_storage: GraphStorage) -> Self {
        Self {
            graph_type,
            graph_storage,
        }
    }

    #[inline]
    pub fn graph_storage(&self) -> &GraphStorage {
        &self.graph_storage
    }
}

impl GraphContainer {

    fn vertex_has_all_labels(
        _mem: &Arc<MemoryGraph>,
        _txn: &minigu_storage::tp::transaction::TransactionHandle,
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
    pub fn vertex_source(
        self: &Arc<Self>,
        label_ids: &[LabelId],
        batch_size: usize,
    )-> StorageResult<impl Iterator<Item = Arc<VertexIdArray>>> {
        let mem = match self.graph_storage() {
            GraphStorage::Memory(m) => Arc::clone(m),
        };
        let txn = mem.begin_transaction(IsolationLevel::Serializable);
        let mut ids: Vec<u64> = Vec::new();
        {
            let it = mem.iter_vertices(&txn)?;
            for v in it {
                let v = v?;
                let vid = v.vid();
                if label_ids.is_empty() || self.vertex_has_all_labels(&mem, &txn, vid, label_ids)? {
                    ids.push(vid);
                }
            }
        }

        let mut pos = 0usize;
        Ok(std::iter::from_fn(move || {
            if pos >= ids.len() { return None; }
            let end = (pos + batch_size).min(ids.len());
            let slice = &ids[pos..end];
            pos = end;
            let arr = Arc::new(VertexIdArray::from_iter(slice.iter().copied()));
            Some(arr)
        }))
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

    #[inline]
    fn as_any(&self) -> &dyn Any {
        self
    }
}
