//! AST definitions for *type elements*.

use super::{Ident, UnsignedInteger};
use crate::imports::Box;
use crate::macros::base;
use crate::span::{BoxSpanned, OptSpanned, Spanned, VecSpanned};

// TODO: Add union/reference types.
#[apply(base)]
pub enum ValueType {
    Char {
        length: Option<BoxSpanned<UnsignedInteger>>,
        not_null: bool,
    },
    Varchar {
        max_length: Option<BoxSpanned<UnsignedInteger>>,
        not_null: bool,
    },
    String {
        min_length: Option<BoxSpanned<UnsignedInteger>>,
        max_length: Option<BoxSpanned<UnsignedInteger>>,
        not_null: bool,
    },
    Binary {
        length: Option<BoxSpanned<UnsignedInteger>>,
        not_null: bool,
    },
    Varbinary {
        max_length: Option<BoxSpanned<UnsignedInteger>>,
        not_null: bool,
    },
    Bytes {
        min_length: Option<BoxSpanned<UnsignedInteger>>,
        max_length: Option<BoxSpanned<UnsignedInteger>>,
        not_null: bool,
    },
    SignedNumeric {
        kind: Spanned<NumericTypeKind>,
        not_null: bool,
    },
    UnsignedNumeric {
        kind: Spanned<NumericTypeKind>,
        not_null: bool,
    },
    Decimal {
        precision: Option<BoxSpanned<UnsignedInteger>>,
        scale: Option<BoxSpanned<UnsignedInteger>>,
        not_null: bool,
    },
    Float {
        kind: Spanned<FloatTypeKind>,
        not_null: bool,
    },
    Temporal {
        kind: Spanned<TemporalTypeKind>,
        not_null: bool,
    },
    List {
        type_name: Spanned<ListTypeName>,
        value_type: Option<BoxSpanned<ValueType>>,
        max_length: Option<BoxSpanned<UnsignedInteger>>,
        not_null: bool,
    },
    Vector {
        dimension: BoxSpanned<UnsignedInteger>,
        not_null: bool,
    },
    AnyRecord {
        not_null: bool,
    },
    Record {
        field_types: VecSpanned<FieldOrPropertyType>,
        not_null: bool,
    },
    Bool {
        not_null: bool,
    },
    Path {
        not_null: bool,
    },
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
    Int(Option<BoxSpanned<UnsignedInteger>>),
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
        precision: Option<BoxSpanned<UnsignedInteger>>,
        scale: Option<BoxSpanned<UnsignedInteger>>,
    },
}

#[apply(base)]
pub enum TemporalTypeKind {
    ZonedDateTime,
    TimestampWithTimeZone,
    LocalDateTime,
    Timestamp,
    Date,
    ZonedTime,
    TimeWithTimeZone,
    Duration(Spanned<DurationQualifier>),
}

#[apply(base)]
pub enum DurationQualifier {
    YearToMonth,
    DayToSecond,
}

#[apply(base)]
pub enum GraphRefType {
    Open,
    Closed(VecSpanned<GraphElementType>),
}

pub type LabelSet = VecSpanned<Ident>;

#[apply(base)]
pub enum ListTypeName {
    List,
    Array,
}

#[apply(base)]
pub struct FieldOrPropertyType {
    pub name: Spanned<Ident>,
    pub value_type: Spanned<ValueType>,
}

#[apply(base)]
pub struct NodeType {
    pub name: OptSpanned<Ident>,
    pub alias: OptSpanned<Ident>,
    pub filler: OptSpanned<NodeOrEdgeTypeFiller>,
}

#[apply(base)]
pub struct NodeOrEdgeTypeFiller {
    pub key: OptSpanned<LabelSet>,
    pub label_set: OptSpanned<LabelSet>,
    pub property_types: Option<VecSpanned<FieldOrPropertyType>>,
}

#[apply(base)]
pub enum NodeTypeRef {
    Alias(Ident),
    Filler(NodeOrEdgeTypeFiller),
    Empty,
}

#[apply(base)]
pub enum EdgeDirection {
    LeftToRight,
    RightToLeft,
    Undirected,
}

#[apply(base)]
pub struct EdgeTypePattern {
    pub name: OptSpanned<Ident>,
    pub direction: EdgeDirection,
    pub left: Spanned<NodeTypeRef>,
    pub filler: Spanned<NodeOrEdgeTypeFiller>,
    pub right: Spanned<NodeTypeRef>,
}

#[apply(base)]
pub struct EdgeTypePhrase {
    pub name: OptSpanned<Ident>,
    pub direction: EdgeDirection,
    pub left: Spanned<Ident>,
    pub filler: OptSpanned<NodeOrEdgeTypeFiller>,
    pub right: Spanned<Ident>,
}

#[apply(base)]
pub enum EdgeType {
    Pattern(Box<EdgeTypePattern>),
    Phrase(Box<EdgeTypePhrase>),
}

#[apply(base)]
pub enum GraphElementType {
    Node(Box<NodeType>),
    Edge(Box<EdgeType>),
}
