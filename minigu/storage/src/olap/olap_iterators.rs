use std::num::NonZeroU32;

use minigu_common::types::VertexId;

use crate::error::StorageError;
use crate::olap::olap_graph::{
    OlapEdge, OlapPropertyStore, OlapStorage, OlapStorageEdge, OlapVertex,
};

const BLOCK_CAPACITY: usize = 256;

pub struct EdgeIter<'a> {
    pub storage: &'a OlapStorage,
    // Index of the current block
    pub block_idx: usize,
    // Offset within block
    pub offset: usize,
}
impl Iterator for EdgeIter<'_> {
    type Item = Result<OlapEdge, StorageError>;

    fn next(&mut self) -> Option<Self::Item> {
        // 1. Scan Block
        let edges = self.storage.edges.read().unwrap();
        while self.block_idx < edges.len() {
            // 1.1 If none,move to next block
            let borrow = self.storage.edges.read().unwrap();
            let block = match borrow.get(self.block_idx) {
                Some(block) => block,
                None => {
                    self.block_idx += 1;
                    self.offset = 0;
                    continue;
                }
            };
            if block.is_tombstone {
                self.block_idx += 1;
                self.offset = 0;
                continue;
            }
            // 1.2 If one block has been finished,move to next
            if self.offset == BLOCK_CAPACITY {
                self.offset = 0;
                self.block_idx += 1;
                continue;
            }
            // 2. Scan within block
            if self.offset < block.edges.len() {
                let raw: &OlapStorageEdge = &block.edges[self.offset];
                // 2.1 Scan next block once scanned empty edge
                if raw.label_id == NonZeroU32::new(1) && raw.dst_id == 1 {
                    self.offset = 0;
                    self.block_idx += 1;
                    continue;
                }
                // 2.2 Build edge result
                let edge = OlapEdge {
                    label_id: raw.label_id,
                    src_id: block.src_id,
                    dst_id: raw.dst_id,
                    properties: {
                        let mut props = OlapPropertyStore::default();

                        for (col_idx, column) in self
                            .storage
                            .property_columns
                            .read()
                            .unwrap()
                            .iter()
                            .enumerate()
                        {
                            if let Some(val) = column
                                .blocks
                                .get(self.block_idx)
                                .and_then(|blk| blk.values.get(self.offset))
                                .cloned()
                            {
                                props.set_prop(col_idx, val);
                            }
                        }
                        props
                    },
                };
                // 2.3 Increase offset
                self.offset += 1;
                return Some(Ok(edge));
            }
        }
        None
    }
}

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
#[allow(dead_code)]
pub struct AdjacencyIterator<'a> {
    pub storage: &'a OlapStorage,
    // Vertex ID
    pub vertex_id: VertexId,
    // Index of the current block
    pub block_idx: usize,
    // Offset within block
    pub offset: usize,
}
impl Iterator for AdjacencyIterator<'_> {
    type Item = Result<OlapEdge, StorageError>;

    fn next(&mut self) -> Option<Self::Item> {

        while self.block_idx != usize::MAX {
            let temporary = self.storage.edges.read().unwrap();
            let option = temporary.get(self.block_idx);

            // Return if none,should not happen
            let _v = option?;

            let block = option.unwrap();
            // Return if tombstone
            if block.is_tombstone {
                if option?.pre_block_index.is_none() {
                    self.block_idx = usize::MAX;
                    return None;
                }

                self.block_idx = block.pre_block_index.unwrap();
                continue;
            }
            // Move to next block
            if self.offset == BLOCK_CAPACITY {
                self.offset = 0;
                self.block_idx = if block.pre_block_index.is_none() {
                    usize::MAX
                } else {
                    block.pre_block_index.unwrap()
                };
                continue;
            }

            if self.offset < BLOCK_CAPACITY {
                let raw: &OlapStorageEdge = &block.edges[self.offset];
                // Scan next block once scanned empty edge
                if raw.label_id == NonZeroU32::new(1) && raw.dst_id == 1 {
                    self.offset = 0;
                    self.block_idx = if block.pre_block_index.is_none() {
                        usize::MAX
                    } else {
                        block.pre_block_index.unwrap()
                    };
                    continue;
                }
                // Build edge result
                let edge = OlapEdge {
                    label_id: raw.label_id,
                    src_id: block.src_id,
                    dst_id: raw.dst_id,
                    properties: {
                        let mut props = OlapPropertyStore::default();

                        for (col_idx, column) in self
                            .storage
                            .property_columns
                            .read()
                            .unwrap()
                            .iter()
                            .enumerate()
                        {
                            if let Some(val) = column
                                .blocks
                                .get(self.block_idx)
                                .and_then(|blk| blk.values.get(self.offset))
                                .cloned()
                            {
                                props.set_prop(col_idx, val);
                            }
                        }
                        props
                    },
                };
                self.offset += 1;
                return Some(Ok(edge));
            }
            self.block_idx = if block.pre_block_index.is_none() {
                usize::MAX
            } else {
                block.pre_block_index.unwrap()
            };
        }
        None
    }
}
