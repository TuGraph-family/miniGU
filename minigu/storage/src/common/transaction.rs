use minigu_common::types::{EdgeId, LabelId, VertexId};
use minigu_common::value::ScalarValue;
use serde::{Deserialize, Serialize};

use crate::common::model::edge::Edge;
use crate::common::model::vertex::Vertex;

/// Represents a commit timestamp used for multi-version concurrency control (MVCC).
/// It can either represent a transaction ID which starts from 1 << 63,
/// or a commit timestamp which starts from 0. So, we can determine a timestamp is
/// a transaction ID if the highest bit is set to 1, or a commit timestamp if the highest bit is 0.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
pub struct Timestamp(pub u64);

impl Timestamp {
    /// The start of the transaction ID range.
    pub const TXN_ID_START: u64 = 1 << 63;

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

/// Isolation level for transactions
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum IsolationLevel {
    Snapshot,
    Serializable,
}

/// Properties operation for setting vertex or edge properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetPropsOp {
    pub indices: Vec<usize>,
    pub props: Vec<ScalarValue>,
}

/// Delta operations that can be performed in a transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
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
