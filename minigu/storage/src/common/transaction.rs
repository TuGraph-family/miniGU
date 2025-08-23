use minigu_common::types::{EdgeId, LabelId, VertexId};
use minigu_common::value::ScalarValue;
pub use minigu_transaction::{IsolationLevel, Timestamp};
use serde::{Deserialize, Serialize};

use crate::common::model::edge::Edge;
use crate::common::model::vertex::Vertex;

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
