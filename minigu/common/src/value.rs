use std::hash::Hash;
use std::sync::Arc;

use arrow::array::{
    Array, ArrayRef, AsArray, BooleanArray, Float32Array, Float64Array, Int8Array, Int16Array,
    Int32Array, Int64Array, NullArray, StringArray, UInt8Array, UInt16Array, UInt32Array,
    UInt64Array,
};
use arrow::datatypes::DataType;
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};

use crate::types::{EdgeId, LabelId, VertexId};

const EPSILON: f64 = 1e-10;

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

pub type Nullable<T> = Option<T>;

/// A wrapper around floats providing implementations of `Eq` and `Hash`.
pub type F32 = OrderedFloat<f32>;
pub type F64 = OrderedFloat<f64>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScalarValue {
    Null,
    Boolean(Nullable<bool>),
    Int8(Nullable<i8>),
    Int16(Nullable<i16>),
    Int32(Nullable<i32>),
    Int64(Nullable<i64>),
    UInt8(Nullable<u8>),
    UInt16(Nullable<u16>),
    UInt32(Nullable<u32>),
    UInt64(Nullable<u64>),
    Float32(Nullable<F32>),
    Float64(Nullable<F64>),
    String(Nullable<String>),
    Vertex(Nullable<VertexValue>),
    Edge(Nullable<EdgeValue>),
}

impl ScalarValue {
    #[allow(unused)]
    pub fn to_scalar_array(&self) -> ArrayRef {
        match self {
            ScalarValue::Null => Arc::new(NullArray::new(1)),
            ScalarValue::Boolean(value) => Arc::new(BooleanArray::from_iter([*value])),
            ScalarValue::Int8(value) => Arc::new(Int8Array::from_iter([*value])),
            ScalarValue::Int16(value) => Arc::new(Int16Array::from_iter([*value])),
            ScalarValue::Int32(value) => Arc::new(Int32Array::from_iter([*value])),
            ScalarValue::Int64(value) => Arc::new(Int64Array::from_iter([*value])),
            ScalarValue::UInt8(value) => Arc::new(UInt8Array::from_iter([*value])),
            ScalarValue::UInt16(value) => Arc::new(UInt16Array::from_iter([*value])),
            ScalarValue::UInt32(value) => Arc::new(UInt32Array::from_iter([*value])),
            ScalarValue::UInt64(value) => Arc::new(UInt64Array::from_iter([*value])),
            ScalarValue::Float32(value) => {
                Arc::new(Float32Array::from_iter([value.map(|f| f.into_inner())]))
            }
            ScalarValue::Float64(value) => {
                Arc::new(Float64Array::from_iter([value.map(|f| f.into_inner())]))
            }
            ScalarValue::String(value) => Arc::new(StringArray::from_iter([value])),
            ScalarValue::Vertex(value) => todo!(),
            ScalarValue::Edge(_value) => todo!(),
        }
    }

    pub fn get_bool(&self) -> Result<bool, String> {
        match self {
            ScalarValue::Boolean(Some(val)) => Ok(*val),
            ScalarValue::Boolean(None) => Err("Null value".to_string()),
            _ => Err("Not a Boolean value".to_string()),
        }
    }

    pub fn get_int8(&self) -> Result<i8, String> {
        match self {
            ScalarValue::Int8(Some(val)) => Ok(*val),
            ScalarValue::Int8(None) => Err("Null value".to_string()),
            _ => Err("Not an Int8 value".to_string()),
        }
    }

    pub fn get_int16(&self) -> Result<i16, String> {
        match self {
            ScalarValue::Int16(Some(val)) => Ok(*val),
            ScalarValue::Int16(None) => Err("Null value".to_string()),
            _ => Err("Not an Int16 value".to_string()),
        }
    }

    pub fn get_int32(&self) -> Result<i32, String> {
        match self {
            ScalarValue::Int32(Some(val)) => Ok(*val),
            ScalarValue::Int32(None) => Err("Null value".to_string()),
            _ => Err("Not an Int32 value".to_string()),
        }
    }

    pub fn get_int64(&self) -> Result<i64, String> {
        match self {
            ScalarValue::Int64(Some(val)) => Ok(*val),
            ScalarValue::Int64(None) => Err("Null value".to_string()),
            _ => Err("Not an Int64 value".to_string()),
        }
    }

    pub fn get_uint8(&self) -> Result<u8, String> {
        match self {
            ScalarValue::UInt8(Some(val)) => Ok(*val),
            ScalarValue::UInt8(None) => Err("Null value".to_string()),
            _ => Err("Not a UInt8 value".to_string()),
        }
    }

    pub fn get_uint16(&self) -> Result<u16, String> {
        match self {
            ScalarValue::UInt16(Some(val)) => Ok(*val),
            ScalarValue::UInt16(None) => Err("Null value".to_string()),
            _ => Err("Not a UInt16 value".to_string()),
        }
    }

    pub fn get_uint32(&self) -> Result<u32, String> {
        match self {
            ScalarValue::UInt32(Some(val)) => Ok(*val),
            ScalarValue::UInt32(None) => Err("Null value".to_string()),
            _ => Err("Not a UInt32 value".to_string()),
        }
    }

    pub fn get_uint64(&self) -> Result<u64, String> {
        match self {
            ScalarValue::UInt64(Some(val)) => Ok(*val),
            ScalarValue::UInt64(None) => Err("Null value".to_string()),
            _ => Err("Not a UInt64 value".to_string()),
        }
    }

    pub fn get_float32(&self) -> Result<f32, String> {
        match self {
            ScalarValue::Float32(Some(val)) => Ok(val.into_inner()),
            ScalarValue::Float32(None) => Err("Null value".to_string()),
            _ => Err("Not a Float32 value".to_string()),
        }
    }

    pub fn get_float64(&self) -> Result<f64, String> {
        match self {
            ScalarValue::Float64(Some(val)) => Ok(val.into_inner()),
            ScalarValue::Float64(None) => Err("Null value".to_string()),
            _ => Err("Not a Float64 value".to_string()),
        }
    }

    pub fn get_string(&self) -> Result<String, String> {
        match self {
            ScalarValue::String(Some(val)) => Ok(val.clone()),
            ScalarValue::String(None) => Err("Null value".to_string()),
            _ => Err("Not a String value".to_string()),
        }
    }

    pub fn get_vertex(&self) -> Result<VertexValue, String> {
        match self {
            ScalarValue::Vertex(Some(val)) => Ok(val.clone()),
            ScalarValue::Vertex(None) => Err("Null value".to_string()),
            _ => Err("Not a Vertex value".to_string()),
        }
    }

    pub fn get_edge(&self) -> Result<EdgeValue, String> {
        match self {
            ScalarValue::Edge(Some(val)) => Ok(val.clone()),
            ScalarValue::Edge(None) => Err("Null value".to_string()),
            _ => Err("Not an Edge value".to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PropertyValue {
    name: String,
    value: ScalarValue,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VertexValue {
    id: VertexId,
    label: LabelId,
    properties: Vec<PropertyValue>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EdgeValue {
    id: EdgeId,
    src: VertexId,
    dst: VertexId,
    label: LabelId,
    properties: Vec<PropertyValue>,
}

macro_rules! for_each_non_null_variant {
    ($m:ident) => {
        $m!(boolean, bool, Boolean);
        $m!(int8, i8, Int8);
        $m!(int16, i16, Int16);
        $m!(int32, i32, Int32);
        $m!(int64, i64, Int64);
        $m!(uint8, u8, UInt8);
        $m!(uint16, u16, UInt16);
        $m!(uint32, u32, UInt32);
        $m!(uint64, u64, UInt64);
        $m!(float32, F32, Float32);
        $m!(float64, F64, Float64);
        $m!(string, String, String);
        $m!(vertex_value, VertexValue, Vertex);
        $m!(edge_value, EdgeValue, Edge);
    };
}

macro_rules! impl_from_for_variant {
    ($_:ident, $ty:ty, $variant:ident) => {
        impl From<$ty> for ScalarValue {
            #[inline]
            fn from(value: $ty) -> Self {
                ScalarValue::$variant(Some(value))
            }
        }
    };
}

for_each_non_null_variant!(impl_from_for_variant);

macro_rules! impl_from_nullable_for_variant {
    ($_:ident, $ty:ty, $variant:ident) => {
        impl From<Nullable<$ty>> for ScalarValue {
            #[inline]
            fn from(value: Nullable<$ty>) -> Self {
                ScalarValue::$variant(value)
            }
        }
    };
}

for_each_non_null_variant!(impl_from_nullable_for_variant);

impl From<&str> for ScalarValue {
    #[inline]
    fn from(value: &str) -> Self {
        ScalarValue::String(Some(value.to_string()))
    }
}

impl From<Nullable<&str>> for ScalarValue {
    #[inline]
    fn from(value: Nullable<&str>) -> Self {
        ScalarValue::String(value.map(String::from))
    }
}

macro_rules! impl_as_for_variant {
    ($name:ident, $ty:ty, $variant:ident) => {
        impl ScalarValue {
            pastey::paste! {
                #[doc = concat!(" Attempts to downcast `self` to borrowed `Nullable<", stringify!($ty), ">`, returning `None` if not possible.")]
                #[inline]
                pub fn [<try_as_$name>](&self) -> Option<&Nullable<$ty>> {
                    match self {
                        ScalarValue::$variant(value) => Some(value),
                        _ => None
                    }
                }
            }
        }
    };
}

for_each_non_null_variant!(impl_as_for_variant);

macro_rules! impl_into_for_variant {
    ($name:ident, $ty:ty, $variant:ident) => {
        impl ScalarValue {
            pastey::paste! {
                #[doc = concat!(" Attempts to downcast `self` to owned `Nullable<", stringify!($ty), ">`, returning `None` if not possible.")]
                #[inline]
                pub fn [<into_$name>](self) -> Option<Nullable<$ty>> {
                    match self {
                        ScalarValue::$variant(value) => Some(value),
                        _ => None
                    }
                }
            }
        }
    };
}

for_each_non_null_variant!(impl_into_for_variant);

pub trait ScalarValueAccessor {
    fn index(&self, index: usize) -> ScalarValue;
}

impl ScalarValueAccessor for dyn Array + '_ {
    fn index(&self, index: usize) -> ScalarValue {
        match self.data_type() {
            DataType::Null => {
                assert!(index < self.len());
                ScalarValue::Null
            }
            DataType::Boolean => {
                let array = self.as_boolean();
                array.is_valid(index).then(|| array.value(index)).into()
            }
            DataType::Int8 => {
                let array: &Int8Array = self.as_primitive();
                array.is_valid(index).then(|| array.value(index)).into()
            }
            DataType::Int16 => {
                let array: &Int16Array = self.as_primitive();
                array.is_valid(index).then(|| array.value(index)).into()
            }
            DataType::Int32 => {
                let array: &Int32Array = self.as_primitive();
                array.is_valid(index).then(|| array.value(index)).into()
            }
            DataType::Int64 => {
                let array: &Int64Array = self.as_primitive();
                array.is_valid(index).then(|| array.value(index)).into()
            }
            DataType::UInt8 => {
                let array: &UInt8Array = self.as_primitive();
                array.is_valid(index).then(|| array.value(index)).into()
            }
            DataType::UInt16 => {
                let array: &UInt16Array = self.as_primitive();
                array.is_valid(index).then(|| array.value(index)).into()
            }
            DataType::UInt32 => {
                let array: &UInt32Array = self.as_primitive();
                array.is_valid(index).then(|| array.value(index)).into()
            }
            DataType::UInt64 => {
                let array: &UInt64Array = self.as_primitive();
                array.is_valid(index).then(|| array.value(index)).into()
            }
            DataType::Float32 => {
                let array: &Float32Array = self.as_primitive();
                array
                    .is_valid(index)
                    .then(|| OrderedFloat(array.value(index)))
                    .into()
            }
            DataType::Float64 => {
                let array: &Float64Array = self.as_primitive();
                array
                    .is_valid(index)
                    .then(|| OrderedFloat(array.value(index)))
                    .into()
            }
            DataType::Utf8 => {
                let array: &StringArray = self.as_string();
                array
                    .is_valid(index)
                    .then(|| array.value(index).to_string())
                    .into()
            }
            _ => todo!(),
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

        // Null
        assert!(matches!(
            Value::Null.to_string(),
            Err(ConversionError::NullValue)
        ));
    }
}
