use gql_parser::ast::ValueType;
use minigu_catalog::label_set::LabelSet;
use serde::Serialize;

use crate::types::Ident;

#[derive(Debug, Serialize)]
pub enum GraphElementType {
    Node(Box<NodeType>),
    Edge(Box<EdgeType>),
}

#[derive(Debug, Serialize)]
pub struct NodeType {
    pub name: Option<Ident>,
    pub alias: Option<Ident>,
    pub filler: Option<NodeOrEdgeTypeFiller>,
}

#[derive(Debug, Serialize)]
pub enum EdgeType {
    Pattern(Box<EdgeTypePattern>),
    Phrase(Box<EdgeTypePhrase>),
}
#[derive(Debug, Serialize)]
pub enum EdgeDirection {
    LeftToRight,
    RightToLeft,
    Undirected,
}
#[derive(Debug, Serialize)]
pub enum NodeTypeRef {
    Alias(Ident),
    Filler(NodeOrEdgeTypeFiller),
    Empty,
}

#[derive(Debug, Serialize)]
pub struct EdgeTypePattern {
    pub name: Option<Ident>,
    pub direction: EdgeDirection,
    pub left: NodeTypeRef,
    pub filler: NodeOrEdgeTypeFiller,
    pub right: NodeTypeRef,
}

#[derive(Debug, Serialize)]
pub struct NodeOrEdgeTypeFiller {
    pub key: Option<LabelSet>,
    pub label_set: Option<LabelSet>,
    pub property_types: Option<Vec<FieldOrPropertyType>>,
}

#[derive(Debug, Serialize)]
pub struct FieldOrPropertyType {
    pub name: Ident,
    pub value_type: ValueType,
}

#[derive(Debug, Serialize)]
pub struct EdgeTypePhrase {
    pub name: Option<Ident>,
    pub direction: EdgeDirection,
    pub left: Ident,
    pub filler: Option<NodeOrEdgeTypeFiller>,
    pub right: Ident,
}
