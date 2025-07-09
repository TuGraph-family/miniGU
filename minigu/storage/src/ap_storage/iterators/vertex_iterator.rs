use crate::ap_storage::olap_graph::{OlapStorage, OlapVertex};
use crate::error::StorageError;

pub struct VertexIter<'a> {
    pub storage: &'a OlapStorage,
    // Vertex index
    pub idx: usize,
}

impl Iterator for VertexIter<'_> {
    type Item = Result<OlapVertex, StorageError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.storage.vertices.read().unwrap().len() {
            return None;
        }

        while self
            .storage
            .vertices
            .read()
            .unwrap()
            .get(self.idx)
            .is_none()
        {
            self.idx += 1;
        }

        let clone = self
            .storage
            .vertices
            .read()
            .unwrap()
            .get(self.idx)
            .cloned()?;
        self.idx += 1;
        Some(Ok(clone))
    }
}
