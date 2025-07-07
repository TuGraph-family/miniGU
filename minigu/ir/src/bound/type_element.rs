use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum BoundGraphElementType {
    // Vertex(Box<BoundVertexType>),
    // Edge(Box<BoundEdgeType>),
}

// #[derive(Debug, Serialize)]
// pub struct BoundVertexType {
//     pub key: LabelSet,
//     pub labels: LabelSet,
//     pub properties: Vec<DataField>,
// }

// pub struct BoundEdgeType {
//     pub key: LabelSet,
//     pub labels: LabelSet,
//     pub properties: Vec<DataField>,
// }

// #[derive(Debug, Serialize)]
// pub enum EdgeDirection {
//     LeftToRight,
//     RightToLeft,
//     Undirected,
// }

// #[derive(Debug, Serialize)]
// pub enum NodeTypeRef {
//     Alias(Ident),
//     Filler(NodeOrEdgeTypeFiller),
//     Empty,
// }

// #[derive(Debug, Serialize)]
// pub struct EdgeTypePattern {
//     pub name: Option<Ident>,
//     pub direction: EdgeDirection,
//     pub left: NodeTypeRef,
//     pub filler: NodeOrEdgeTypeFiller,
//     pub right: NodeTypeRef,
// }

// #[derive(Debug, Serialize)]
// pub struct NodeOrEdgeTypeFiller {
//     pub key: Option<LabelSet>,
//     pub label_set: Option<LabelSet>,
//     pub property_types: Option<Vec<FieldOrPropertyType>>,
// }

// #[derive(Debug, Serialize)]
// pub struct EdgeTypePhrase {
//     pub name: Option<Ident>,
//     pub direction: EdgeDirection,
//     pub left: Ident,
//     pub filler: Option<NodeOrEdgeTypeFiller>,
//     pub right: Ident,
// }
