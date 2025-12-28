pub mod iterators;
pub mod model;
pub mod wal;

// Re-export commonly used types
use minigu_common::types::{EdgeId, LabelId, VertexId};
use minigu_common::value::ScalarValue;
pub use model::edge::*;
pub use model::properties::*;
pub use model::schema::*;
use serde::{Deserialize, Serialize};
pub use wal::*;

pub use crate::common::model::vertex::Vertex;

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
