//! AST definitions for *type elements*.

use super::{Ident, UnsignedInteger};
use crate::imports::{Box, Vec};
use crate::macros::{base, ext};
use crate::span::Span;

#[apply(base)]
pub struct ValueType {
    pub kind: ValueTypeKind,
    pub not_null: bool,
    pub span: Span,
}

// TODO: Add temporal types.
#[apply(base)]
pub enum ValueTypeKind {
    Char(Option<UnsignedInteger>),
    Varchar(Option<UnsignedInteger>),
    String {
        min_length: Option<UnsignedInteger>,
        max_length: Option<UnsignedInteger>,
    },
    Binary(Option<UnsignedInteger>),
    Varbinary(Option<UnsignedInteger>),
    Bytes {
        min_length: Option<UnsignedInteger>,
        max_length: Option<UnsignedInteger>,
    },
    SignedNumeric(NumericTypeKind),
    UnsignedNumeric(NumericTypeKind),
    Decimal {
        precision: Option<UnsignedInteger>,
        scale: Option<UnsignedInteger>,
    },
    Float(FloatTypeKind),
    List {
        type_name: ListTypeName,
        value_type: Option<Box<ValueType>>,
        max_length: Option<UnsignedInteger>,
    },
    AnyRecord,
    Record(Vec<FieldOrPropertyType>),
    GraphRef(GraphRefType),
    Bool,
    Path,
    Null,
    Empty,
}

#[apply(base)]
pub enum NumericTypeKind {
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    Int256,
    Small,
    Int(Option<UnsignedInteger>),
    Big,
}

#[apply(base)]
pub enum FloatTypeKind {
    Float16,
    Float32,
    Float64,
    Float128,
    Float256,
    Real,
    Double,
    Float {
        precision: Option<UnsignedInteger>,
        scale: Option<UnsignedInteger>,
    },
}

#[apply(base)]
pub enum GraphRefType {
    Open,
    Closed(Vec<GraphElementType>),
}

#[apply(base)]
pub enum LabelSet {
    Label(Ident),
    Labels(Vec<Ident>),
}

#[apply(ext)]
pub struct ListTypeName {
    pub group: bool,
    pub synonym: ListTypeNameSynonym,
    pub span: Span,
}

#[apply(ext)]
pub enum ListTypeNameSynonym {
    List,
    Array,
}

#[apply(base)]
pub struct FieldOrPropertyType {
    pub name: Ident,
    pub value_type: ValueType,
    pub span: Span,
}

#[apply(base)]
pub struct NodeType {
    pub name: Option<Ident>,
    pub alias: Option<Ident>,
    pub filler: Option<NodeTypeFiller>,
    pub span: Span,
}

#[apply(base)]
pub struct NodeTypeFiller {
    pub key: Option<LabelSet>,
    pub label_set: Option<LabelSet>,
    pub property_types: Option<Vec<FieldOrPropertyType>>,
    pub span: Span,
}

#[apply(base)]
pub struct NodeTypeImpliedContent {
    pub label_set: Option<LabelSet>,
    pub property_types: Option<Vec<FieldOrPropertyType>>,
    pub span: Span,
}

#[apply(base)]
pub enum GraphElementType {
    Node(NodeType),
}
