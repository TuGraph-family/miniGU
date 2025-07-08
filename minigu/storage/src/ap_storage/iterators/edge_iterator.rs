use std::num::NonZeroU32;

use crate::ap_storage::olap_graph::{OlapEdge, OlapPropertyStore, OlapStorage, OlapStorageEdge};
use crate::error::StorageError;

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
