use minigu_catalog::label_set::LabelSet;
use minigu_common::data_type::DataField;
use serde::Serialize;
use smol_str::SmolStr;

/// Bound graph element type (vertex or edge)
#[derive(Debug, Clone, Serialize)]
pub enum BoundGraphElementType {
    Vertex(Box<BoundVertexType>),
    Edge(Box<BoundEdgeType>),
}

/// Bound vertex type with key and labels using LabelSet
#[derive(Debug, Clone, Serialize)]
pub struct BoundVertexType {
    /// Optional type name (e.g., "Person")
    pub name: Option<SmolStr>,
    /// Primary key label set (for now, same as labels)
    pub key: LabelSet,
    /// All labels for this vertex type
    pub labels: LabelSet,
    /// Property definitions
    pub properties: Vec<DataField>,
}

/// Bound edge type with key, labels, and node references
#[derive(Debug, Clone, Serialize)]
pub struct BoundEdgeType {
    /// Optional type name (e.g., "LIVES_IN")
    pub name: Option<SmolStr>,
    /// Primary key label set
    pub key: LabelSet,
    /// All labels for this edge type
    pub labels: LabelSet,
    /// Property definitions
    pub properties: Vec<DataField>,
    /// Edge direction
    pub direction: EdgeDirection,
    /// Source node type reference
    pub left: NodeTypeRef,
    /// Destination node type reference
    pub right: NodeTypeRef,
}

/// Edge direction
#[derive(Debug, Clone, Serialize)]
pub enum EdgeDirection {
    LeftToRight,
    RightToLeft,
    Undirected,
}

/// Reference to a node type (for edge endpoints)
#[derive(Debug, Clone, Serialize)]
pub enum NodeTypeRef {
    /// Reference to a named type by identifier (e.g., "Person")
    Alias(SmolStr),
    /// Inline node type definition
    Filler(NodeOrEdgeTypeFiller),
    /// Empty reference (no constraint on node type)
    Empty,
}

/// Inline type definition filler
#[derive(Debug, Clone, Serialize)]
pub struct NodeOrEdgeTypeFiller {
    /// Primary key label set
    pub key: Option<LabelSet>,
    /// All labels
    pub label_set: Option<LabelSet>,
    /// Property definitions (using DataField for now, can extend to FieldOrPropertyType later)
    pub properties: Option<Vec<DataField>>,
}
