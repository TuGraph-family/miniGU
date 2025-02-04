//! AST definitions for *type elements*.

use super::{Ident, UnsignedInteger};
use crate::macros::{base, ext};

#[apply(base)]
pub struct ValueType<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub kind: ValueTypeKind<'a>,
    pub not_null: bool,
}

// TODO: Add temporal types.
#[apply(base)]
pub enum ValueTypeKind<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    Char(Option<UnsignedInteger<'a>>),
    Varchar(Option<UnsignedInteger<'a>>),
    String {
        min_length: Option<UnsignedInteger<'a>>,
        max_length: Option<UnsignedInteger<'a>>,
    },
    Binary(Option<UnsignedInteger<'a>>),
    Varbinary(Option<UnsignedInteger<'a>>),
    Bytes {
        min_length: Option<UnsignedInteger<'a>>,
        max_length: Option<UnsignedInteger<'a>>,
    },
    SignedNumeric(NumericTypeKind<'a>),
    UnsignedNumeric(NumericTypeKind<'a>),
    Decimal {
        precision: Option<UnsignedInteger<'a>>,
        scale: Option<UnsignedInteger<'a>>,
    },
    Float(FloatTypeKind<'a>),
    List {
        type_name: ListTypeName,
        value_type: Option<Box<ValueType<'a>>>,
        max_length: Option<UnsignedInteger<'a>>,
    },
    Record(Option<Vec<FieldOrPropertyType<'a>>>),
    Bool,
    Path,
    Null,
    Empty,
}

#[apply(base)]
pub enum NumericTypeKind<'a> {
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    Int256,
    Small,
    #[cfg_attr(feature = "serde", serde(borrow))]
    Int(Option<UnsignedInteger<'a>>),
    Big,
}

#[apply(base)]
pub enum FloatTypeKind<'a> {
    Float16,
    Float32,
    Float64,
    Float128,
    Float256,
    Real,
    Double,
    Float {
        #[cfg_attr(feature = "serde", serde(borrow))]
        precision: Option<UnsignedInteger<'a>>,
        scale: Option<UnsignedInteger<'a>>,
    },
}

#[apply(base)]
pub enum LabelSet<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    Label(Ident<'a>),
    Labels(Vec<Ident<'a>>),
}

#[apply(ext)]
pub struct ListTypeName {
    pub group: bool,
    pub synonym: ListTypeNameSynonym,
}

#[apply(ext)]
pub enum ListTypeNameSynonym {
    List,
    Array,
}

#[apply(base)]
pub struct FieldOrPropertyType<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub name: Ident<'a>,
    pub value_type: ValueType<'a>,
}
