use std::fmt;
use std::sync::{Arc, LazyLock};

use arrow::datatypes::{
    DataType, Field as ArrowField, FieldRef as ArrowFieldRef, Fields as ArrowFields,
    Schema as ArrowSchema,
};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::constants::{
    DST_FIELD_NAME, EID_FIELD_NAME, LABEL_FIELD_NAME, SRC_FIELD_NAME, VID_FIELD_NAME,
};

// 定义一个小的 EPSILON 用于浮点数比较
const EPSILON: f64 = 1e-10;

pub(crate) struct PredefinedFields;

impl PredefinedFields {
    pub(crate) fn vid() -> ArrowFieldRef {
        static VID_FIELD: LazyLock<ArrowFieldRef> = LazyLock::new(|| {
            ArrowField::new(VID_FIELD_NAME.to_string(), DataType::UInt64, false).into()
        });
        VID_FIELD.clone()
    }

    pub(crate) fn label() -> ArrowFieldRef {
        static LABEL_FIELD: LazyLock<ArrowFieldRef> = LazyLock::new(|| {
            ArrowField::new(LABEL_FIELD_NAME.to_string(), DataType::UInt32, false).into()
        });
        LABEL_FIELD.clone()
    }

    pub(crate) fn eid() -> ArrowFieldRef {
        static EID_FIELD: LazyLock<ArrowFieldRef> = LazyLock::new(|| {
            ArrowField::new(EID_FIELD_NAME.to_string(), DataType::UInt64, false).into()
        });
        EID_FIELD.clone()
    }

    pub(crate) fn src() -> ArrowFieldRef {
        static SRC_FIELD: LazyLock<ArrowFieldRef> = LazyLock::new(|| {
            ArrowField::new(SRC_FIELD_NAME.to_string(), DataType::UInt64, false).into()
        });
        SRC_FIELD.clone()
    }

    pub(crate) fn dst() -> ArrowFieldRef {
        static DST_FIELD: LazyLock<ArrowFieldRef> = LazyLock::new(|| {
            ArrowField::new(DST_FIELD_NAME.to_string(), DataType::UInt64, false).into()
        });
        DST_FIELD.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogicalType {
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float32,
    Float64,
    Boolean,
    String,
    Vertex(Vec<DataField>),
    Edge(Vec<DataField>),
    Record(Vec<DataField>),
    Null,
}

impl LogicalType {
    #[inline]
    pub fn to_arrow_data_type(&self) -> DataType {
        match self {
            LogicalType::Int8 => DataType::Int8,
            LogicalType::Int16 => DataType::Int16,
            LogicalType::Int32 => DataType::Int32,
            LogicalType::Int64 => DataType::Int64,
            LogicalType::UInt8 => DataType::UInt8,
            LogicalType::UInt16 => DataType::UInt16,
            LogicalType::UInt32 => DataType::UInt32,
            LogicalType::UInt64 => DataType::UInt64,
            LogicalType::Float32 => DataType::Float32,
            LogicalType::Float64 => DataType::Float64,
            LogicalType::Boolean => DataType::Boolean,
            LogicalType::String => DataType::Utf8,
            LogicalType::Vertex(fields) => {
                let vid_field = PredefinedFields::vid();
                let label_id = PredefinedFields::label();
                let fields = [vid_field, label_id]
                    .into_iter()
                    .chain(fields.iter().map(DataField::to_arrow_field).map(Arc::new))
                    .collect();
                DataType::Struct(fields)
            }
            LogicalType::Edge(fields) => {
                let eid_field = PredefinedFields::eid();
                let label_field = PredefinedFields::label();
                let src_field = PredefinedFields::src();
                let dst_field = PredefinedFields::dst();
                let fields = [eid_field, label_field, src_field, dst_field]
                    .into_iter()
                    .chain(fields.iter().map(DataField::to_arrow_field).map(Arc::new))
                    .collect();
                DataType::Struct(fields)
            }
            LogicalType::Record(fields) => {
                let fields = fields
                    .iter()
                    .map(DataField::to_arrow_field)
                    .map(Arc::new)
                    .collect();
                DataType::Struct(fields)
            }
            LogicalType::Null => DataType::Null,
        }
    }
}

impl fmt::Display for LogicalType {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogicalType::Int8 => write!(f, "int8"),
            LogicalType::Int16 => write!(f, "int16"),
            LogicalType::Int32 => write!(f, "int32"),
            LogicalType::Int64 => write!(f, "int64"),
            LogicalType::UInt8 => write!(f, "uint8"),
            LogicalType::UInt16 => write!(f, "uint16"),
            LogicalType::UInt32 => write!(f, "uint32"),
            LogicalType::UInt64 => write!(f, "uint64"),
            LogicalType::Float32 => write!(f, "float32"),
            LogicalType::Float64 => write!(f, "float64"),
            LogicalType::Boolean => write!(f, "boolean"),
            LogicalType::String => write!(f, "string"),
            LogicalType::Vertex(properties) => {
                write!(f, "vertex {{ {} }}", properties.iter().join(","))
            }
            LogicalType::Edge(properties) => {
                write!(f, "edge {{ {} }}", properties.iter().join(","))
            }
            LogicalType::Record(fields) => {
                write!(f, "record {{ {} }}", fields.iter().join(","))
            }
            LogicalType::Null => write!(f, "null"),
        }
    }
}

pub type DataSchemaRef = Arc<DataSchema>;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DataSchema(Vec<DataField>);

impl DataSchema {
    #[inline]
    pub fn new(fields: Vec<DataField>) -> Self {
        Self(fields)
    }

    pub fn append(&mut self, schema: &DataSchema) {
        self.0.extend(schema.0.iter().cloned());
    }

    pub fn get_field_by_name(&self, name: &str) -> Option<&DataField> {
        self.0.iter().find(|field| field.name() == name)
    }

    pub fn get_field_index_by_name(&self, name: &str) -> Option<usize> {
        self.0.iter().position(|field| field.name() == name)
    }

    #[inline]
    pub fn fields(&self) -> &[DataField] {
        &self.0
    }

    #[inline]
    pub fn to_arrow_schema(&self) -> ArrowSchema {
        let fields: ArrowFields = self.0.iter().map(DataField::to_arrow_field).collect();
        ArrowSchema::new(fields)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DataField {
    name: String,
    ty: LogicalType,
    nullable: bool,
}

impl DataField {
    #[inline]
    pub fn new(name: String, ty: LogicalType, nullable: bool) -> Self {
        Self { name, ty, nullable }
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn ty(&self) -> &LogicalType {
        &self.ty
    }

    #[inline]
    pub fn is_nullable(&self) -> bool {
        self.nullable
    }

    #[inline]
    pub fn to_arrow_field(&self) -> ArrowField {
        ArrowField::new(
            self.name.clone(),
            self.ty.to_arrow_data_type(),
            self.nullable,
        )
    }
}

impl fmt::Display for DataField {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}::{}", self.name, self.ty)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConversionError {
    NullValue,
    IncompatibleType,
    Overflow,
    ParseError(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Float32(f32),
    Float64(f64),
    Boolean(bool),
    String(String),
    Null,
}

impl Value {
    // Convert to i8
    pub fn to_i8(&self) -> Result<i8, ConversionError> {
        match self {
            Value::Int8(v) => Ok(*v),
            Value::Int16(v) => {
                if *v > i8::MAX as i16 || *v < i8::MIN as i16 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i8)
                }
            }
            Value::Int32(v) => {
                if *v > i8::MAX as i32 || *v < i8::MIN as i32 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i8)
                }
            }
            Value::Int64(v) => {
                if *v > i8::MAX as i64 || *v < i8::MIN as i64 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i8)
                }
            }
            Value::UInt8(v) => {
                if *v > i8::MAX as u8 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i8)
                }
            }
            Value::UInt16(v) => {
                if *v > i8::MAX as u16 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i8)
                }
            }
            Value::UInt32(v) => {
                if *v > i8::MAX as u32 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i8)
                }
            }
            Value::UInt64(v) => {
                if *v > i8::MAX as u64 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i8)
                }
            }
            Value::Float32(v) => {
                if v.is_nan() || v.is_infinite() {
                    Err(ConversionError::ParseError(v.to_string()))
                } else if *v > i8::MAX as f32 || *v < i8::MIN as f32 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i8)
                }
            }
            Value::Float64(v) => {
                if v.is_nan() || v.is_infinite() {
                    Err(ConversionError::ParseError(v.to_string()))
                } else if *v > i8::MAX as f64 || *v < i8::MIN as f64 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i8)
                }
            }
            Value::Boolean(v) => Ok(if *v { 1 } else { 0 }),
            Value::String(s) => s
                .parse::<i8>()
                .map_err(|_| ConversionError::ParseError(s.clone())),
            Value::Null => Err(ConversionError::NullValue),
        }
    }

    // Convert to i16
    pub fn to_i16(&self) -> Result<i16, ConversionError> {
        match self {
            Value::Int8(v) => Ok(*v as i16),
            Value::Int16(v) => Ok(*v),
            Value::Int32(v) => {
                if *v > i16::MAX as i32 || *v < i16::MIN as i32 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i16)
                }
            }
            Value::Int64(v) => {
                if *v > i16::MAX as i64 || *v < i16::MIN as i64 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i16)
                }
            }
            Value::UInt8(v) => Ok(*v as i16),
            Value::UInt16(v) => {
                if *v > i16::MAX as u16 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i16)
                }
            }
            Value::UInt32(v) => {
                if *v > i16::MAX as u32 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i16)
                }
            }
            Value::UInt64(v) => {
                if *v > i16::MAX as u64 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i16)
                }
            }
            Value::Float32(v) => {
                if v.is_nan() || v.is_infinite() {
                    Err(ConversionError::ParseError(v.to_string()))
                } else if *v > i16::MAX as f32 || *v < i16::MIN as f32 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i16)
                }
            }
            Value::Float64(v) => {
                if v.is_nan() || v.is_infinite() {
                    Err(ConversionError::ParseError(v.to_string()))
                } else if *v > i16::MAX as f64 || *v < i16::MIN as f64 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i16)
                }
            }
            Value::Boolean(v) => Ok(if *v { 1 } else { 0 }),
            Value::String(s) => s
                .parse::<i16>()
                .map_err(|_| ConversionError::ParseError(s.clone())),
            Value::Null => Err(ConversionError::NullValue),
        }
    }

    // Convert to i32
    pub fn to_i32(&self) -> Result<i32, ConversionError> {
        match self {
            Value::Int8(v) => Ok(*v as i32),
            Value::Int16(v) => Ok(*v as i32),
            Value::Int32(v) => Ok(*v),
            Value::Int64(v) => {
                if *v > i32::MAX as i64 || *v < i32::MIN as i64 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i32)
                }
            }
            Value::UInt8(v) => Ok(*v as i32),
            Value::UInt16(v) => Ok(*v as i32),
            Value::UInt32(v) => {
                if *v > i32::MAX as u32 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i32)
                }
            }
            Value::UInt64(v) => {
                if *v > i32::MAX as u64 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i32)
                }
            }
            Value::Float32(v) => {
                if v.is_nan() || v.is_infinite() {
                    Err(ConversionError::ParseError(v.to_string()))
                } else if *v > i32::MAX as f32 || *v < i32::MIN as f32 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i32)
                }
            }
            Value::Float64(v) => {
                if v.is_nan() || v.is_infinite() {
                    Err(ConversionError::ParseError(v.to_string()))
                } else if *v > i32::MAX as f64 || *v < i32::MIN as f64 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i32)
                }
            }
            Value::Boolean(v) => Ok(if *v { 1 } else { 0 }),
            Value::String(s) => s
                .parse::<i32>()
                .map_err(|_| ConversionError::ParseError(s.clone())),
            Value::Null => Err(ConversionError::NullValue),
        }
    }

    // Convert to i64
    pub fn to_i64(&self) -> Result<i64, ConversionError> {
        match self {
            Value::Int8(v) => Ok(*v as i64),
            Value::Int16(v) => Ok(*v as i64),
            Value::Int32(v) => Ok(*v as i64),
            Value::Int64(v) => Ok(*v),
            Value::UInt8(v) => Ok(*v as i64),
            Value::UInt16(v) => Ok(*v as i64),
            Value::UInt32(v) => Ok(*v as i64),
            Value::UInt64(v) => {
                if *v > i64::MAX as u64 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i64)
                }
            }
            Value::Float32(v) => {
                if v.is_nan() || v.is_infinite() {
                    Err(ConversionError::ParseError(v.to_string()))
                } else if *v > i64::MAX as f32 || *v < i64::MIN as f32 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i64)
                }
            }
            Value::Float64(v) => {
                if v.is_nan() || v.is_infinite() {
                    Err(ConversionError::ParseError(v.to_string()))
                } else if *v > i64::MAX as f64 || *v < i64::MIN as f64 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i64)
                }
            }
            Value::Boolean(v) => Ok(if *v { 1 } else { 0 }),
            Value::String(s) => s
                .parse::<i64>()
                .map_err(|_| ConversionError::ParseError(s.clone())),
            Value::Null => Err(ConversionError::NullValue),
        }
    }

    // Convert to u8
    pub fn to_u8(&self) -> Result<u8, ConversionError> {
        match self {
            Value::Int8(v) => {
                if *v < 0 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u8)
                }
            }
            Value::Int16(v) => {
                if *v < 0 || *v > u8::MAX as i16 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u8)
                }
            }
            Value::Int32(v) => {
                if *v < 0 || *v > u8::MAX as i32 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u8)
                }
            }
            Value::Int64(v) => {
                if *v < 0 || *v > u8::MAX as i64 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u8)
                }
            }
            Value::UInt8(v) => Ok(*v),
            Value::UInt16(v) => {
                if *v > u8::MAX as u16 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u8)
                }
            }
            Value::UInt32(v) => {
                if *v > u8::MAX as u32 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u8)
                }
            }
            Value::UInt64(v) => {
                if *v > u8::MAX as u64 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u8)
                }
            }
            Value::Float32(v) => {
                if v.is_nan() || v.is_infinite() || *v < 0.0 || *v > u8::MAX as f32 {
                    Err(ConversionError::ParseError(v.to_string()))
                } else {
                    Ok(*v as u8)
                }
            }
            Value::Float64(v) => {
                if v.is_nan() || v.is_infinite() || *v < 0.0 || *v > u8::MAX as f64 {
                    Err(ConversionError::ParseError(v.to_string()))
                } else {
                    Ok(*v as u8)
                }
            }
            Value::Boolean(v) => Ok(if *v { 1 } else { 0 }),
            Value::String(s) => s
                .parse::<u8>()
                .map_err(|_| ConversionError::ParseError(s.clone())),
            Value::Null => Err(ConversionError::NullValue),
        }
    }

    // Convert to u16
    pub fn to_u16(&self) -> Result<u16, ConversionError> {
        match self {
            Value::Int8(v) => {
                if *v < 0 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u16)
                }
            }
            Value::Int16(v) => {
                if *v < 0 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u16)
                }
            }
            Value::Int32(v) => {
                if *v < 0 || *v > u16::MAX as i32 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u16)
                }
            }
            Value::Int64(v) => {
                if *v < 0 || *v > u16::MAX as i64 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u16)
                }
            }
            Value::UInt8(v) => Ok(*v as u16),
            Value::UInt16(v) => Ok(*v),
            Value::UInt32(v) => {
                if *v > u16::MAX as u32 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u16)
                }
            }
            Value::UInt64(v) => {
                if *v > u16::MAX as u64 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u16)
                }
            }
            Value::Float32(v) => {
                if v.is_nan() || v.is_infinite() || *v < 0.0 || *v > u16::MAX as f32 {
                    Err(ConversionError::ParseError(v.to_string()))
                } else {
                    Ok(*v as u16)
                }
            }
            Value::Float64(v) => {
                if v.is_nan() || v.is_infinite() || *v < 0.0 || *v > u16::MAX as f64 {
                    Err(ConversionError::ParseError(v.to_string()))
                } else {
                    Ok(*v as u16)
                }
            }
            Value::Boolean(v) => Ok(if *v { 1 } else { 0 }),
            Value::String(s) => s
                .parse::<u16>()
                .map_err(|_| ConversionError::ParseError(s.clone())),
            Value::Null => Err(ConversionError::NullValue),
        }
    }

    // Convert to u32
    pub fn to_u32(&self) -> Result<u32, ConversionError> {
        match self {
            Value::Int8(v) => {
                if *v < 0 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u32)
                }
            }
            Value::Int16(v) => {
                if *v < 0 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u32)
                }
            }
            Value::Int32(v) => {
                if *v < 0 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u32)
                }
            }
            Value::Int64(v) => {
                if *v < 0 || *v > u32::MAX as i64 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u32)
                }
            }
            Value::UInt8(v) => Ok(*v as u32),
            Value::UInt16(v) => Ok(*v as u32),
            Value::UInt32(v) => Ok(*v),
            Value::UInt64(v) => {
                if *v > u32::MAX as u64 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u32)
                }
            }
            Value::Float32(v) => {
                if v.is_nan() || v.is_infinite() || *v < 0.0 || *v > u32::MAX as f32 {
                    Err(ConversionError::ParseError(v.to_string()))
                } else {
                    Ok(*v as u32)
                }
            }
            Value::Float64(v) => {
                if v.is_nan() || v.is_infinite() || *v < 0.0 || *v > u32::MAX as f64 {
                    Err(ConversionError::ParseError(v.to_string()))
                } else {
                    Ok(*v as u32)
                }
            }
            Value::Boolean(v) => Ok(if *v { 1 } else { 0 }),
            Value::String(s) => s
                .parse::<u32>()
                .map_err(|_| ConversionError::ParseError(s.clone())),
            Value::Null => Err(ConversionError::NullValue),
        }
    }

    // Convert to u64
    pub fn to_u64(&self) -> Result<u64, ConversionError> {
        match self {
            Value::Int8(v) => {
                if *v < 0 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u64)
                }
            }
            Value::Int16(v) => {
                if *v < 0 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u64)
                }
            }
            Value::Int32(v) => {
                if *v < 0 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u64)
                }
            }
            Value::Int64(v) => {
                if *v < 0 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u64)
                }
            }
            Value::UInt8(v) => Ok(*v as u64),
            Value::UInt16(v) => Ok(*v as u64),
            Value::UInt32(v) => Ok(*v as u64),
            Value::UInt64(v) => Ok(*v),
            Value::Float32(v) => {
                if v.is_nan() || v.is_infinite() || *v < 0.0 || *v > u64::MAX as f32 {
                    Err(ConversionError::ParseError(v.to_string()))
                } else {
                    Ok(*v as u64)
                }
            }
            Value::Float64(v) => {
                if v.is_nan() || v.is_infinite() || *v < 0.0 || *v > u64::MAX as f64 {
                    Err(ConversionError::ParseError(v.to_string()))
                } else {
                    Ok(*v as u64)
                }
            }
            Value::Boolean(v) => Ok(if *v { 1 } else { 0 }),
            Value::String(s) => s
                .parse::<u64>()
                .map_err(|_| ConversionError::ParseError(s.clone())),
            Value::Null => Err(ConversionError::NullValue),
        }
    }

    // Convert to f32
    pub fn to_f32(&self) -> Result<f32, ConversionError> {
        match self {
            Value::Int8(v) => Ok(*v as f32),
            Value::Int16(v) => Ok(*v as f32),
            Value::Int32(v) => Ok(*v as f32),
            Value::Int64(v) => Ok(*v as f32),
            Value::UInt8(v) => Ok(*v as f32),
            Value::UInt16(v) => Ok(*v as f32),
            Value::UInt32(v) => Ok(*v as f32),
            Value::UInt64(v) => Ok(*v as f32),
            Value::Float32(v) => Ok(*v),
            Value::Float64(v) => {
                if v.is_nan() || v.is_infinite() || *v > f32::MAX as f64 || *v < f32::MIN as f64 {
                    Err(ConversionError::ParseError(v.to_string()))
                } else {
                    Ok(*v as f32)
                }
            }
            Value::Boolean(v) => Ok(if *v { 1.0 } else { 0.0 }),
            Value::String(s) => s
                .parse::<f32>()
                .map_err(|_| ConversionError::ParseError(s.clone())),
            Value::Null => Err(ConversionError::NullValue),
        }
    }

    // Convert to f64
    pub fn to_f64(&self) -> Result<f64, ConversionError> {
        match self {
            Value::Int8(v) => Ok(*v as f64),
            Value::Int16(v) => Ok(*v as f64),
            Value::Int32(v) => Ok(*v as f64),
            Value::Int64(v) => Ok(*v as f64),
            Value::UInt8(v) => Ok(*v as f64),
            Value::UInt16(v) => Ok(*v as f64),
            Value::UInt32(v) => Ok(*v as f64),
            Value::UInt64(v) => Ok(*v as f64),
            Value::Float32(v) => Ok(*v as f64),
            Value::Float64(v) => Ok(*v),
            Value::Boolean(v) => Ok(if *v { 1.0 } else { 0.0 }),
            Value::String(s) => s
                .parse::<f64>()
                .map_err(|_| ConversionError::ParseError(s.clone())),
            Value::Null => Err(ConversionError::NullValue),
        }
    }

    // Convert to bool
    pub fn to_bool(&self) -> Result<bool, ConversionError> {
        match self {
            Value::Int8(v) => Ok(*v != 0),
            Value::Int16(v) => Ok(*v != 0),
            Value::Int32(v) => Ok(*v != 0),
            Value::Int64(v) => Ok(*v != 0),
            Value::UInt8(v) => Ok(*v != 0),
            Value::UInt16(v) => Ok(*v != 0),
            Value::UInt32(v) => Ok(*v != 0),
            Value::UInt64(v) => Ok(*v != 0),
            Value::Float32(v) => {
                if v.is_nan() {
                    Err(ConversionError::ParseError(v.to_string()))
                } else {
                    Ok(v.abs() > EPSILON as f32)
                }
            }
            Value::Float64(v) => {
                if v.is_nan() {
                    Err(ConversionError::ParseError(v.to_string()))
                } else {
                    Ok(v.abs() > EPSILON)
                }
            }
            Value::Boolean(v) => Ok(*v),
            Value::String(s) => {
                let lowered = s.to_lowercase();
                if lowered == "true" || lowered == "1" {
                    Ok(true)
                } else if lowered == "false" || lowered == "0" {
                    Ok(false)
                } else {
                    Err(ConversionError::ParseError(s.clone()))
                }
            }
            Value::Null => Err(ConversionError::NullValue),
        }
    }

    // Convert to String
    pub fn to_string(&self) -> Result<String, ConversionError> {
        match self {
            Value::Int8(v) => Ok(v.to_string()),
            Value::Int16(v) => Ok(v.to_string()),
            Value::Int32(v) => Ok(v.to_string()),
            Value::Int64(v) => Ok(v.to_string()),
            Value::UInt8(v) => Ok(v.to_string()),
            Value::UInt16(v) => Ok(v.to_string()),
            Value::UInt32(v) => Ok(v.to_string()),
            Value::UInt64(v) => Ok(v.to_string()),
            Value::Float32(v) => Ok(v.to_string()),
            Value::Float64(v) => Ok(v.to_string()),
            Value::Boolean(v) => Ok(v.to_string()),
            Value::String(s) => Ok(s.clone()),
            Value::Null => Err(ConversionError::NullValue),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ConversionError, Value};

    #[test]
    fn test_to_i8() {
        // Successful conversions
        assert_eq!(Value::Int8(42).to_i8(), Ok(42i8));
        assert_eq!(Value::Int16(42).to_i8(), Ok(42i8));
        assert_eq!(Value::Int32(42).to_i8(), Ok(42i8));
        assert_eq!(Value::Int64(42).to_i8(), Ok(42i8));
        assert_eq!(Value::UInt8(42).to_i8(), Ok(42i8));
        assert_eq!(Value::UInt16(42).to_i8(), Ok(42i8));
        assert_eq!(Value::UInt32(42).to_i8(), Ok(42i8));
        assert_eq!(Value::UInt64(42).to_i8(), Ok(42i8));
        assert_eq!(Value::Float32(42.0).to_i8(), Ok(42i8));
        assert_eq!(Value::Float64(42.0).to_i8(), Ok(42i8));
        assert_eq!(Value::Boolean(true).to_i8(), Ok(1i8));
        assert_eq!(Value::Boolean(false).to_i8(), Ok(0i8));
        assert_eq!(Value::String("42".to_string()).to_i8(), Ok(42i8));
        assert_eq!(Value::String("-128".to_string()).to_i8(), Ok(-128i8)); // Min value

        // Overflow cases
        assert!(matches!(
            Value::Int16(128).to_i8(),
            Err(ConversionError::Overflow)
        )); // > i8::MAX
        assert!(matches!(
            Value::Int16(-129).to_i8(),
            Err(ConversionError::Overflow)
        )); // < i8::MIN
        assert!(matches!(
            Value::UInt8(128).to_i8(),
            Err(ConversionError::Overflow)
        ));
        assert!(matches!(
            Value::Float32(128.0).to_i8(),
            Err(ConversionError::Overflow)
        ));
        assert!(matches!(
            Value::Float64(-129.0).to_i8(),
            Err(ConversionError::Overflow)
        ));

        // NaN and Infinity cases
        assert!(matches!(
            Value::Float32(f32::NAN).to_i8(),
            Err(ConversionError::ParseError(_))
        ));
        assert!(matches!(
            Value::Float32(f32::INFINITY).to_i8(),
            Err(ConversionError::ParseError(_))
        ));
        assert!(matches!(
            Value::Float64(f64::NAN).to_i8(),
            Err(ConversionError::ParseError(_))
        ));
        assert!(matches!(
            Value::Float64(f64::INFINITY).to_i8(),
            Err(ConversionError::ParseError(_))
        ));

        // Parse error
        assert!(matches!(
            Value::String("invalid".to_string()).to_i8(),
            Err(ConversionError::ParseError(_))
        ));
        assert!(matches!(
            Value::String("128".to_string()).to_i8(),
            Err(ConversionError::ParseError(_))
        )); // Overflow via parse

        // Null
        assert!(matches!(
            Value::Null.to_i8(),
            Err(ConversionError::NullValue)
        ));
    }

    #[test]
    fn test_to_i16() {
        // Successful conversions
        assert_eq!(Value::Int8(42).to_i16(), Ok(42i16));
        assert_eq!(Value::Int16(42).to_i16(), Ok(42i16));
        assert_eq!(Value::Int32(42).to_i16(), Ok(42i16));
        assert_eq!(Value::Int64(42).to_i16(), Ok(42i16));
        assert_eq!(Value::UInt8(42).to_i16(), Ok(42i16));
        assert_eq!(Value::UInt16(42).to_i16(), Ok(42i16));
        assert_eq!(Value::UInt32(42).to_i16(), Ok(42i16));
        assert_eq!(Value::UInt64(42).to_i16(), Ok(42i16));
        assert_eq!(Value::Float32(42.0).to_i16(), Ok(42i16));
        assert_eq!(Value::Float64(42.0).to_i16(), Ok(42i16));
        assert_eq!(Value::Boolean(true).to_i16(), Ok(1i16));
        assert_eq!(Value::Boolean(false).to_i16(), Ok(0i16));
        assert_eq!(Value::String("42".to_string()).to_i16(), Ok(42i16));
        assert_eq!(Value::String("-32768".to_string()).to_i16(), Ok(-32768i16)); // Min value

        // Overflow cases
        assert!(matches!(
            Value::Int32(32768).to_i16(),
            Err(ConversionError::Overflow)
        )); // > i16::MAX
        assert!(matches!(
            Value::Int32(-32769).to_i16(),
            Err(ConversionError::Overflow)
        )); // < i16::MIN
        assert!(matches!(
            Value::UInt16(32768).to_i16(),
            Err(ConversionError::Overflow)
        ));
        assert!(matches!(
            Value::Float64(32768.0).to_i16(),
            Err(ConversionError::Overflow)
        ));

        // NaN and Infinity cases
        assert!(matches!(
            Value::Float32(f32::NAN).to_i16(),
            Err(ConversionError::ParseError(_))
        ));
        assert!(matches!(
            Value::Float32(f32::INFINITY).to_i16(),
            Err(ConversionError::ParseError(_))
        ));
        assert!(matches!(
            Value::Float64(f64::NAN).to_i16(),
            Err(ConversionError::ParseError(_))
        ));
        assert!(matches!(
            Value::Float64(f64::INFINITY).to_i16(),
            Err(ConversionError::ParseError(_))
        ));

        // Parse error
        assert!(matches!(
            Value::String("invalid".to_string()).to_i16(),
            Err(ConversionError::ParseError(_))
        ));

        // Null
        assert!(matches!(
            Value::Null.to_i16(),
            Err(ConversionError::NullValue)
        ));
    }

    #[test]
    fn test_to_i32() {
        // Successful conversions
        assert_eq!(Value::Int8(42).to_i32(), Ok(42i32));
        assert_eq!(Value::Int16(42).to_i32(), Ok(42i32));
        assert_eq!(Value::Int32(42).to_i32(), Ok(42i32));
        assert_eq!(Value::Int64(42).to_i32(), Ok(42i32));
        assert_eq!(Value::UInt8(42).to_i32(), Ok(42i32));
        assert_eq!(Value::UInt16(42).to_i32(), Ok(42i32));
        assert_eq!(Value::UInt32(42).to_i32(), Ok(42i32));
        assert_eq!(Value::UInt64(42).to_i32(), Ok(42i32));
        assert_eq!(Value::Float32(42.0).to_i32(), Ok(42i32));
        assert_eq!(Value::Float64(42.0).to_i32(), Ok(42i32));
        assert_eq!(Value::Boolean(true).to_i32(), Ok(1i32));
        assert_eq!(Value::Boolean(false).to_i32(), Ok(0i32));
        assert_eq!(Value::String("42".to_string()).to_i32(), Ok(42i32));
        assert_eq!(
            Value::String("-2147483648".to_string()).to_i32(),
            Ok(-2147483648i32)
        ); // Min value

        // Overflow cases
        assert!(matches!(
            Value::Int64(2147483648i64).to_i32(),
            Err(ConversionError::Overflow)
        )); // > i32::MAX
        assert!(matches!(
            Value::Int64(-2147483649i64).to_i32(),
            Err(ConversionError::Overflow)
        )); // < i32::MIN
        assert!(matches!(
            Value::UInt32(2147483648u32).to_i32(),
            Err(ConversionError::Overflow)
        ));
        assert!(matches!(
            Value::Float64(2147483648.0).to_i32(),
            Err(ConversionError::Overflow)
        ));

        // NaN and Infinity cases
        assert!(matches!(
            Value::Float32(f32::NAN).to_i32(),
            Err(ConversionError::ParseError(_))
        ));
        assert!(matches!(
            Value::Float32(f32::INFINITY).to_i32(),
            Err(ConversionError::ParseError(_))
        ));
        assert!(matches!(
            Value::Float64(f64::NAN).to_i32(),
            Err(ConversionError::ParseError(_))
        ));
        assert!(matches!(
            Value::Float64(f64::INFINITY).to_i32(),
            Err(ConversionError::ParseError(_))
        ));

        // Parse error
        assert!(matches!(
            Value::String("invalid".to_string()).to_i32(),
            Err(ConversionError::ParseError(_))
        ));

        // Null
        assert!(matches!(
            Value::Null.to_i32(),
            Err(ConversionError::NullValue)
        ));
    }

    #[test]
    fn test_to_i64() {
        // Successful conversions
        assert_eq!(Value::Int8(42).to_i64(), Ok(42i64));
        assert_eq!(Value::Int16(42).to_i64(), Ok(42i64));
        assert_eq!(Value::Int32(42).to_i64(), Ok(42i64));
        assert_eq!(Value::Int64(42).to_i64(), Ok(42i64));
        assert_eq!(Value::UInt8(42).to_i64(), Ok(42i64));
        assert_eq!(Value::UInt16(42).to_i64(), Ok(42i64));
        assert_eq!(Value::UInt32(42).to_i64(), Ok(42i64));
        assert_eq!(Value::UInt64(42).to_i64(), Ok(42i64));
        assert_eq!(Value::Float32(42.0).to_i64(), Ok(42i64));
        assert_eq!(Value::Float64(42.0).to_i64(), Ok(42i64));
        assert_eq!(Value::Boolean(true).to_i64(), Ok(1i64));
        assert_eq!(Value::Boolean(false).to_i64(), Ok(0i64));
        assert_eq!(Value::String("42".to_string()).to_i64(), Ok(42i64));
        assert_eq!(
            Value::String("-9223372036854775808".to_string()).to_i64(),
            Ok(-9223372036854775808i64)
        ); // Min value

        // Overflow cases
        assert!(matches!(
            Value::UInt64(9223372036854775808u64).to_i64(),
            Err(ConversionError::Overflow)
        )); // > i64::MAX
        assert!(matches!(
            Value::Float64(f64::MAX).to_i64(),
            Err(ConversionError::Overflow)
        )); // i64::MAX
        assert!(matches!(
            Value::Float64(f64::MIN).to_i64(),
            Err(ConversionError::Overflow)
        )); //  i64::MIN
        assert!(matches!(
            Value::Float32(f32::MAX).to_i64(),
            Err(ConversionError::Overflow)
        )); //  i64::MAX as f32

        // NaN and Infinity cases
        assert!(matches!(
            Value::Float32(f32::NAN).to_i64(),
            Err(ConversionError::ParseError(_))
        ));
        assert!(matches!(
            Value::Float32(f32::INFINITY).to_i64(),
            Err(ConversionError::ParseError(_))
        ));
        assert!(matches!(
            Value::Float64(f64::NAN).to_i64(),
            Err(ConversionError::ParseError(_))
        ));
        assert!(matches!(
            Value::Float64(f64::INFINITY).to_i64(),
            Err(ConversionError::ParseError(_))
        ));

        // Parse error
        assert!(matches!(
            Value::String("invalid".to_string()).to_i64(),
            Err(ConversionError::ParseError(_))
        ));

        // Null
        assert!(matches!(
            Value::Null.to_i64(),
            Err(ConversionError::NullValue)
        ));
    }

    #[test]
    fn test_to_u8() {
        // Successful conversions
        assert_eq!(Value::Int8(42).to_u8(), Ok(42u8));
        assert_eq!(Value::Int16(42).to_u8(), Ok(42u8));
        assert_eq!(Value::Int32(42).to_u8(), Ok(42u8));
        assert_eq!(Value::Int64(42).to_u8(), Ok(42u8));
        assert_eq!(Value::UInt8(42).to_u8(), Ok(42u8));
        assert_eq!(Value::UInt16(42).to_u8(), Ok(42u8));
        assert_eq!(Value::UInt32(42).to_u8(), Ok(42u8));
        assert_eq!(Value::UInt64(42).to_u8(), Ok(42u8));
        assert_eq!(Value::Float32(42.0).to_u8(), Ok(42u8));
        assert_eq!(Value::Float64(42.0).to_u8(), Ok(42u8));
        assert_eq!(Value::Boolean(true).to_u8(), Ok(1u8));
        assert_eq!(Value::Boolean(false).to_u8(), Ok(0u8));
        assert_eq!(Value::String("42".to_string()).to_u8(), Ok(42u8));
        assert_eq!(Value::String("255".to_string()).to_u8(), Ok(255u8)); // Max value

        // Overflow cases
        assert!(matches!(
            Value::Int8(-1).to_u8(),
            Err(ConversionError::Overflow)
        )); // Negative
        assert!(matches!(
            Value::Int16(256).to_u8(),
            Err(ConversionError::Overflow)
        )); // > u8::MAX
        assert!(matches!(
            Value::Float32(-0.1).to_u8(),
            Err(ConversionError::ParseError(_))
        )); // Negative float
        assert!(matches!(
            Value::Float64(256.0).to_u8(),
            Err(ConversionError::ParseError(_))
        ));

        // NaN and Infinity cases
        assert!(matches!(
            Value::Float32(f32::NAN).to_u8(),
            Err(ConversionError::ParseError(_))
        ));
        assert!(matches!(
            Value::Float32(f32::INFINITY).to_u8(),
            Err(ConversionError::ParseError(_))
        ));
        assert!(matches!(
            Value::Float64(f64::NAN).to_u8(),
            Err(ConversionError::ParseError(_))
        ));
        assert!(matches!(
            Value::Float64(f64::INFINITY).to_u8(),
            Err(ConversionError::ParseError(_))
        ));

        // Parse error
        assert!(matches!(
            Value::String("invalid".to_string()).to_u8(),
            Err(ConversionError::ParseError(_))
        ));

        // Null
        assert!(matches!(
            Value::Null.to_u8(),
            Err(ConversionError::NullValue)
        ));
    }

    #[test]
    fn test_to_u16() {
        // Successful conversions
        assert_eq!(Value::Int8(42).to_u16(), Ok(42u16));
        assert_eq!(Value::Int16(42).to_u16(), Ok(42u16));
        assert_eq!(Value::Int32(42).to_u16(), Ok(42u16));
        assert_eq!(Value::Int64(42).to_u16(), Ok(42u16));
        assert_eq!(Value::UInt8(42).to_u16(), Ok(42u16));
        assert_eq!(Value::UInt16(42).to_u16(), Ok(42u16));
        assert_eq!(Value::UInt32(42).to_u16(), Ok(42u16));
        assert_eq!(Value::UInt64(42).to_u16(), Ok(42u16));
        assert_eq!(Value::Float32(42.0).to_u16(), Ok(42u16));
        assert_eq!(Value::Float64(42.0).to_u16(), Ok(42u16));
        assert_eq!(Value::Boolean(true).to_u16(), Ok(1u16));
        assert_eq!(Value::Boolean(false).to_u16(), Ok(0u16));
        assert_eq!(Value::String("42".to_string()).to_u16(), Ok(42u16));
        assert_eq!(Value::String("65535".to_string()).to_u16(), Ok(65535u16)); // Max value

        // Overflow cases
        assert!(matches!(
            Value::Int16(-1).to_u16(),
            Err(ConversionError::Overflow)
        )); // Negative
        assert!(matches!(
            Value::UInt32(65536).to_u16(),
            Err(ConversionError::Overflow)
        )); // > u16::MAX
        assert!(matches!(
            Value::Float64(-0.1).to_u16(),
            Err(ConversionError::ParseError(_))
        ));

        // NaN and Infinity cases
        assert!(matches!(
            Value::Float32(f32::NAN).to_u16(),
            Err(ConversionError::ParseError(_))
        ));
        assert!(matches!(
            Value::Float32(f32::INFINITY).to_u16(),
            Err(ConversionError::ParseError(_))
        ));
        assert!(matches!(
            Value::Float64(f64::NAN).to_u16(),
            Err(ConversionError::ParseError(_))
        ));
        assert!(matches!(
            Value::Float64(f64::INFINITY).to_u16(),
            Err(ConversionError::ParseError(_))
        ));

        // Parse error
        assert!(matches!(
            Value::String("invalid".to_string()).to_u16(),
            Err(ConversionError::ParseError(_))
        ));

        // Null
        assert!(matches!(
            Value::Null.to_u16(),
            Err(ConversionError::NullValue)
        ));
    }

    #[test]
    fn test_to_u32() {
        // Successful conversions
        assert_eq!(Value::Int8(42).to_u32(), Ok(42u32));
        assert_eq!(Value::Int16(42).to_u32(), Ok(42u32));
        assert_eq!(Value::Int32(42).to_u32(), Ok(42u32));
        assert_eq!(Value::Int64(42).to_u32(), Ok(42u32));
        assert_eq!(Value::UInt8(42).to_u32(), Ok(42u32));
        assert_eq!(Value::UInt16(42).to_u32(), Ok(42u32));
        assert_eq!(Value::UInt32(42).to_u32(), Ok(42u32));
        assert_eq!(Value::UInt64(42).to_u32(), Ok(42u32));
        assert_eq!(Value::Float32(42.0).to_u32(), Ok(42u32));
        assert_eq!(Value::Float64(42.0).to_u32(), Ok(42u32));
        assert_eq!(Value::Boolean(true).to_u32(), Ok(1u32));
        assert_eq!(Value::Boolean(false).to_u32(), Ok(0u32));
        assert_eq!(Value::String("42".to_string()).to_u32(), Ok(42u32));
        assert_eq!(
            Value::String("4294967295".to_string()).to_u32(),
            Ok(4294967295u32)
        ); // Max value

        // Overflow cases
        assert!(matches!(
            Value::Int32(-1).to_u32(),
            Err(ConversionError::Overflow)
        )); // Negative
        assert!(matches!(
            Value::UInt64(4294967296u64).to_u32(),
            Err(ConversionError::Overflow)
        )); // > u32::MAX
        assert!(matches!(
            Value::Float32(-0.1).to_u32(),
            Err(ConversionError::ParseError(_))
        ));

        // NaN and Infinity cases
        assert!(matches!(
            Value::Float32(f32::NAN).to_u32(),
            Err(ConversionError::ParseError(_))
        ));
        assert!(matches!(
            Value::Float32(f32::INFINITY).to_u32(),
            Err(ConversionError::ParseError(_))
        ));
        assert!(matches!(
            Value::Float64(f64::NAN).to_u32(),
            Err(ConversionError::ParseError(_))
        ));
        assert!(matches!(
            Value::Float64(f64::INFINITY).to_u32(),
            Err(ConversionError::ParseError(_))
        ));

        // Parse error
        assert!(matches!(
            Value::String("invalid".to_string()).to_u32(),
            Err(ConversionError::ParseError(_))
        ));

        // Null
        assert!(matches!(
            Value::Null.to_u32(),
            Err(ConversionError::NullValue)
        ));
    }

    #[test]
    fn test_to_u64() {
        // Successful conversions
        assert_eq!(Value::Int8(42).to_u64(), Ok(42u64));
        assert_eq!(Value::Int16(42).to_u64(), Ok(42u64));
        assert_eq!(Value::Int32(42).to_u64(), Ok(42u64));
        assert_eq!(Value::Int64(42).to_u64(), Ok(42u64));
        assert_eq!(Value::UInt8(42).to_u64(), Ok(42u64));
        assert_eq!(Value::UInt16(42).to_u64(), Ok(42u64));
        assert_eq!(Value::UInt32(42).to_u64(), Ok(42u64));
        assert_eq!(Value::UInt64(42).to_u64(), Ok(42u64));
        assert_eq!(Value::Float32(42.0).to_u64(), Ok(42u64));
        assert_eq!(Value::Float64(42.0).to_u64(), Ok(42u64));
        assert_eq!(Value::Boolean(true).to_u64(), Ok(1u64));
        assert_eq!(Value::Boolean(false).to_u64(), Ok(0u64));
        assert_eq!(Value::String("42".to_string()).to_u64(), Ok(42u64));
        assert_eq!(
            Value::String("18446744073709551615".to_string()).to_u64(),
            Ok(18446744073709551615u64)
        ); // Max value

        // Overflow cases
        assert!(matches!(
            Value::Int64(-1).to_u64(),
            Err(ConversionError::Overflow)
        )); // Negative
        assert!(matches!(
            Value::Float64(-0.1).to_u64(),
            Err(ConversionError::ParseError(_))
        )); // Negative float
        assert!(matches!(
            Value::Float64(f64::MAX).to_u64(),
            Err(ConversionError::ParseError(_))
        )); //  u64::MAX
        assert!(matches!(
            Value::Float32(f32::MAX).to_u64(),
            Err(ConversionError::ParseError(_))
        )); //  u64::MAX as f32

        // NaN and Infinity cases
        assert!(matches!(
            Value::Float32(f32::NAN).to_u64(),
            Err(ConversionError::ParseError(_))
        ));
        assert!(matches!(
            Value::Float32(f32::INFINITY).to_u64(),
            Err(ConversionError::ParseError(_))
        ));
        assert!(matches!(
            Value::Float64(f64::NAN).to_u64(),
            Err(ConversionError::ParseError(_))
        ));
        assert!(matches!(
            Value::Float64(f64::INFINITY).to_u64(),
            Err(ConversionError::ParseError(_))
        ));

        // Parse error
        assert!(matches!(
            Value::String("invalid".to_string()).to_u64(),
            Err(ConversionError::ParseError(_))
        ));

        // Null
        assert!(matches!(
            Value::Null.to_u64(),
            Err(ConversionError::NullValue)
        ));
    }

    #[test]
    fn test_to_f32() {
        // Successful conversions
        assert_eq!(Value::Int8(42).to_f32(), Ok(42.0f32));
        assert_eq!(Value::Int16(42).to_f32(), Ok(42.0f32));
        assert_eq!(Value::Int32(42).to_f32(), Ok(42.0f32));
        assert_eq!(Value::Int64(42).to_f32(), Ok(42.0f32));
        assert_eq!(Value::UInt8(42).to_f32(), Ok(42.0f32));
        assert_eq!(Value::UInt16(42).to_f32(), Ok(42.0f32));
        assert_eq!(Value::UInt32(42).to_f32(), Ok(42.0f32));
        assert_eq!(Value::UInt64(42).to_f32(), Ok(42.0f32));
        assert_eq!(Value::Float32(42.5).to_f32(), Ok(42.5f32));
        assert_eq!(Value::Float64(42.5).to_f32(), Ok(42.5f32));
        assert_eq!(Value::Boolean(true).to_f32(), Ok(1.0f32));
        assert_eq!(Value::Boolean(false).to_f32(), Ok(0.0f32));
        assert_eq!(Value::String("42.5".to_string()).to_f32(), Ok(42.5f32));

        // Overflow cases
        assert!(matches!(
            Value::Float64(f64::MAX).to_f32(),
            Err(ConversionError::ParseError(_))
        )); // f64 max > f32 max
        assert!(matches!(
            Value::Float64(f64::MIN).to_f32(),
            Err(ConversionError::ParseError(_))
        )); // f64 min < f32 min
        assert!(matches!(
            Value::Float64(f64::NAN).to_f32(),
            Err(ConversionError::ParseError(_))
        ));
        assert!(matches!(
            Value::Float64(f64::INFINITY).to_f32(),
            Err(ConversionError::ParseError(_))
        ));

        // Parse error
        assert!(matches!(
            Value::String("invalid".to_string()).to_f32(),
            Err(ConversionError::ParseError(_))
        ));

        // Null
        assert!(matches!(
            Value::Null.to_f32(),
            Err(ConversionError::NullValue)
        ));
    }

    #[test]
    fn test_to_f64() {
        // Successful conversions
        assert_eq!(Value::Int8(42).to_f64(), Ok(42.0f64));
        assert_eq!(Value::Int16(42).to_f64(), Ok(42.0f64));
        assert_eq!(Value::Int32(42).to_f64(), Ok(42.0f64));
        assert_eq!(Value::Int64(42).to_f64(), Ok(42.0f64));
        assert_eq!(Value::UInt8(42).to_f64(), Ok(42.0f64));
        assert_eq!(Value::UInt16(42).to_f64(), Ok(42.0f64));
        assert_eq!(Value::UInt32(42).to_f64(), Ok(42.0f64));
        assert_eq!(Value::UInt64(42).to_f64(), Ok(42.0f64));
        assert_eq!(Value::Float32(42.5).to_f64(), Ok(42.5f64));
        assert_eq!(Value::Float64(42.5).to_f64(), Ok(42.5f64));
        assert_eq!(Value::Boolean(true).to_f64(), Ok(1.0f64));
        assert_eq!(Value::Boolean(false).to_f64(), Ok(0.0f64));
        assert_eq!(Value::String("42.5".to_string()).to_f64(), Ok(42.5f64));

        // No overflow cases from smaller types to f64, as f64 can represent all

        // Parse error
        assert!(matches!(
            Value::String("invalid".to_string()).to_f64(),
            Err(ConversionError::ParseError(_))
        ));

        // Null
        assert!(matches!(
            Value::Null.to_f64(),
            Err(ConversionError::NullValue)
        ));
    }

    #[test]
    fn test_to_bool() {
        // Successful conversions
        assert_eq!(Value::Int8(1).to_bool(), Ok(true));
        assert_eq!(Value::Int8(0).to_bool(), Ok(false));
        assert_eq!(Value::Int16(42).to_bool(), Ok(true));
        assert_eq!(Value::Int32(-1).to_bool(), Ok(true)); // Non-zero is true
        assert_eq!(Value::Int64(0).to_bool(), Ok(false));
        assert_eq!(Value::UInt8(1).to_bool(), Ok(true));
        assert_eq!(Value::UInt16(0).to_bool(), Ok(false));
        assert_eq!(Value::UInt32(42).to_bool(), Ok(true));
        assert_eq!(Value::UInt64(0).to_bool(), Ok(false));
        assert_eq!(Value::Float32(0.0).to_bool(), Ok(false));
        assert_eq!(Value::Float32(1.5).to_bool(), Ok(true));
        assert_eq!(Value::Float64(-1.0).to_bool(), Ok(true));
        assert_eq!(Value::Float32(0.00000000001).to_bool(), Ok(false)); // Less than EPSILON
        assert_eq!(Value::Float64(0.00000000001).to_bool(), Ok(false)); // Less than EPSILON
        assert_eq!(Value::Boolean(true).to_bool(), Ok(true));
        assert_eq!(Value::Boolean(false).to_bool(), Ok(false));
        assert_eq!(Value::String("true".to_string()).to_bool(), Ok(true));
        assert_eq!(Value::String("TRUE".to_string()).to_bool(), Ok(true)); // Case insensitive
        assert_eq!(Value::String("false".to_string()).to_bool(), Ok(false));
        assert_eq!(Value::String("1".to_string()).to_bool(), Ok(true));
        assert_eq!(Value::String("0".to_string()).to_bool(), Ok(false));

        // NaN cases
        assert!(matches!(
            Value::Float32(f32::NAN).to_bool(),
            Err(ConversionError::ParseError(_))
        ));
        assert!(matches!(
            Value::Float64(f64::NAN).to_bool(),
            Err(ConversionError::ParseError(_))
        ));

        // Parse error
        assert!(matches!(
            Value::String("invalid".to_string()).to_bool(),
            Err(ConversionError::ParseError(_))
        ));
        assert!(matches!(
            Value::String("yes".to_string()).to_bool(),
            Err(ConversionError::ParseError(_))
        ));

        // Null
        assert!(matches!(
            Value::Null.to_bool(),
            Err(ConversionError::NullValue)
        ));
    }

    #[test]
    fn test_to_string() {
        // Successful conversions
        assert_eq!(Value::Int8(42).to_string(), Ok("42".to_string()));
        assert_eq!(Value::Int16(-42).to_string(), Ok("-42".to_string()));
        assert_eq!(Value::Int32(42).to_string(), Ok("42".to_string()));
        assert_eq!(Value::Int64(42).to_string(), Ok("42".to_string()));
        assert_eq!(Value::UInt8(42).to_string(), Ok("42".to_string()));
        assert_eq!(Value::UInt16(42).to_string(), Ok("42".to_string()));
        assert_eq!(Value::UInt32(42).to_string(), Ok("42".to_string()));
        assert_eq!(Value::UInt64(42).to_string(), Ok("42".to_string()));
        assert_eq!(Value::Float32(42.5).to_string(), Ok("42.5".to_string()));
        assert_eq!(Value::Float64(42.5).to_string(), Ok("42.5".to_string()));
        assert_eq!(Value::Boolean(true).to_string(), Ok("true".to_string()));
        assert_eq!(Value::Boolean(false).to_string(), Ok("false".to_string()));
        assert_eq!(
            Value::String("hello".to_string()).to_string(),
            Ok("hello".to_string())
        );

        // No overflow or parse errors for to_string, as it handles all primitives

        // Null
        assert!(matches!(
            Value::Null.to_string(),
            Err(ConversionError::NullValue)
        ));
    }
}
