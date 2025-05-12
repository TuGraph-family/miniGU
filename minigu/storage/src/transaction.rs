use std::sync::Weak;

use common::datatype::types::{EdgeId, LabelId, VertexId};
use common::datatype::value::PropertyValue;

use crate::model::edge::Edge;
use crate::model::vertex::Vertex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
/// Represents a commit timestamp used for multi-version concurrency control (MVCC).
/// It can either represent a transaction ID which starts from 1 << 63,
/// or a commit timestamp which starts from 0. So, we can determine a timestamp is
/// a transaction ID if the highest bit is set to 1, or a commit timestamp if the highest bit is 0.
pub struct Timestamp(pub u64);

impl Timestamp {
    // The start of the transaction ID range.
    pub(super) const TXN_ID_START: u64 = 1 << 63;

    /// Create timestamp by a given commit ts
    pub fn with_ts(timestamp: u64) -> Self {
        Self(timestamp)
    }

    /// Returns the maximum possible commit timestamp.
    pub fn max_commit_ts() -> Self {
        Self(u64::MAX & !Self::TXN_ID_START)
    }

    /// Returns true if the timestamp is a transaction ID.
    pub fn is_txn_id(&self) -> bool {
        self.0 & Self::TXN_ID_START != 0
    }

    /// Returns true if the timestamp is a commit timestamp.
    pub fn is_commit_ts(&self) -> bool {
        self.0 & Self::TXN_ID_START == 0
    }
}

pub type UndoPtr = Weak<UndoEntry>;

#[derive(Debug, Clone)]
/// Represents an undo log entry for multi-version concurrency control.
pub struct UndoEntry {
    /// The delta operation of the undo entry.
    delta: DeltaOp,
    /// The timestamp when this version is committed.
    timestamp: Timestamp,
    /// The next undo entry in the undo buffer.
    next: UndoPtr,
}

impl UndoEntry {
    /// Create a UndoEntry
    pub(super) fn new(delta: DeltaOp, timestamp: Timestamp, next: UndoPtr) -> Self {
        Self {
            delta,
            timestamp,
            next,
        }
    }

    /// Get the data of the undo entry.
    pub(super) fn delta(&self) -> &DeltaOp {
        &self.delta
    }

    /// Get the end timestamp of the undo entry.
    pub(super) fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    /// Get the next undo ptr of the undo entry.
    pub(super) fn next(&self) -> UndoPtr {
        self.next.clone()
    }
}

#[derive(Debug, Clone)]
pub struct SetPropsOp {
    pub indices: Vec<usize>,
    pub props: Vec<PropertyValue>,
}

#[derive(Debug, Clone)]
pub enum DeltaOp {
    DelVertex(VertexId),
    DelEdge(EdgeId),
    CreateVertex(Vertex),
    CreateEdge(Edge),
    SetVertexProps(VertexId, SetPropsOp),
    SetEdgeProps(EdgeId, SetPropsOp),
    AddLabel(LabelId),
    RemoveLabel(LabelId),
}

pub enum IsolationLevel {
    Snapshot,
    Serializable,
}
