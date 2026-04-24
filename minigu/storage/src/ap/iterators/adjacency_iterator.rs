use std::num::NonZeroU32;

use minigu_common::types::VertexId;
use minigu_transaction::Timestamp;

use crate::ap::olap_graph::{OlapEdge, OlapPropertyStore, OlapStorage, OlapStorageEdge};
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
            let block = match temporary.get(self.block_idx) {
                Some(block) => block,
                None => {
                    self.block_idx = usize::MAX;
                    return None;
                }
            };

            // Return if tombstone
            if block.is_tombstone {
                if block.pre_block_index.is_none() {
                    self.block_idx = usize::MAX;
                    return None;
                }
                self.block_idx = block.pre_block_index.unwrap();
                continue;
            }

            // Move to next block
            if self.offset == BLOCK_CAPACITY {
                self.offset = 0;
                self.block_idx = block.pre_block_index.unwrap_or(usize::MAX);
                continue;
            }

            if self.offset < BLOCK_CAPACITY {
                let raw: &OlapStorageEdge = &block.edges[self.offset];
                if raw.label_id == NonZeroU32::new(1) && raw.dst_id == 1 {
                    self.offset = 0;
                    self.block_idx = block.pre_block_index.unwrap_or(usize::MAX);
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
                                .and_then(|versions| {
                                    crate::ap::olap_graph::latest_committed_prop_value(versions)
                                })
                            {
                                props.set_prop(col_idx, Some(val));
                            }
                        }
                        props
                    },
                };
                self.offset += 1;
                return Some(Ok(edge));
            }
            self.block_idx = block.pre_block_index.unwrap_or(usize::MAX);
        }
        None
    }
}

#[allow(dead_code)]
pub struct AdjacencyIteratorAtTs<'a> {
    pub storage: &'a OlapStorage,
    // Vertex ID
    pub vertex_id: VertexId,
    // Index of the current block
    pub block_idx: usize,
    // Offset within block
    pub offset: usize,
    pub txn_id: Option<Timestamp>,
    pub start_ts: Timestamp,
}
impl Iterator for AdjacencyIteratorAtTs<'_> {
    type Item = Result<OlapEdge, StorageError>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.block_idx != usize::MAX {
            let temporary = self.storage.edges.read().unwrap();
            let block = match temporary.get(self.block_idx) {
                Some(block) => block,
                None => {
                    self.block_idx = usize::MAX;
                    return None;
                }
            };

            // Return if tombstone
            if block.is_tombstone {
                if block.pre_block_index.is_none() {
                    self.block_idx = usize::MAX;
                    return None;
                }
                self.block_idx = block.pre_block_index.unwrap();
                continue;
            }

            if block.min_ts.is_commit_ts() && self.start_ts.raw() < block.min_ts.raw() {
                self.block_idx = block.pre_block_index.unwrap_or(usize::MAX);
                self.offset = 0;
                continue;
            }

            // Move to next block
            if self.offset == BLOCK_CAPACITY {
                self.offset = 0;
                self.block_idx = block.pre_block_index.unwrap_or(usize::MAX);
                continue;
            }

            if self.offset < BLOCK_CAPACITY {
                let raw: &OlapStorageEdge = &block.edges[self.offset];
                // Scan next block once scanned empty edge
                if raw.label_id == NonZeroU32::new(1) && raw.dst_id == 1 {
                    self.offset = 0;
                    self.block_idx = block.pre_block_index.unwrap_or(usize::MAX);
                    continue;
                }

                // Visibility filtering by edge commit_ts using snapshot start_ts
                if raw.commit_ts.is_txn_id() {
                    if let Some(txn_id) = self.txn_id {
                        if raw.commit_ts != txn_id {
                            self.offset += 1;
                            continue;
                        }
                    } else {
                        self.offset += 1;
                        continue;
                    }
                } else if raw.commit_ts.raw() > self.start_ts.raw() {
                    self.offset += 1;
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
                                .and_then(|versions| {
                                    crate::ap::olap_graph::prop_value_visible_at(
                                        versions,
                                        self.txn_id,
                                        self.start_ts,
                                    )
                                })
                            {
                                props.set_prop(col_idx, Some(val));
                            }
                        }
                        props
                    },
                };
                self.offset += 1;
                return Some(Ok(edge));
            }

            self.block_idx = block.pre_block_index.unwrap_or(usize::MAX);
        }
        None
    }
}
