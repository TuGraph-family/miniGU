use std::num::NonZeroU32;

use minigu_common::types::VertexId;

use crate::ap_storage::olap_graph::{OlapEdge, OlapPropertyStore, OlapStorage, OlapStorageEdge};
use crate::error::StorageError;

const BLOCK_CAPACITY: usize = 256;

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
