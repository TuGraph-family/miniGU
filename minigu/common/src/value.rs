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

impl ScalarValue {
    // Convert to i8
    pub fn to_i8(&self) -> Result<i8, ConversionError> {
        match self {
            ScalarValue::Int8(Some(v)) => Ok(*v),
            ScalarValue::Int8(None) => Err(ConversionError::NullValue),
            ScalarValue::Int16(Some(v)) => {
                if *v > i8::MAX as i16 || *v < i8::MIN as i16 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i8)
                }
            }
            ScalarValue::Int16(None) => Err(ConversionError::NullValue),
            ScalarValue::Int32(Some(v)) => {
                if *v > i8::MAX as i32 || *v < i8::MIN as i32 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i8)
                }
            }
            ScalarValue::Int32(None) => Err(ConversionError::NullValue),
            ScalarValue::Int64(Some(v)) => {
                if *v > i8::MAX as i64 || *v < i8::MIN as i64 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i8)
                }
            }
            ScalarValue::Int64(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt8(Some(v)) => {
                if *v > i8::MAX as u8 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i8)
                }
            }
            ScalarValue::UInt8(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt16(Some(v)) => {
                if *v > i8::MAX as u16 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i8)
                }
            }
            ScalarValue::UInt16(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt32(Some(v)) => {
                if *v > i8::MAX as u32 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i8)
                }
            }
            ScalarValue::UInt32(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt64(Some(v)) => {
                if *v > i8::MAX as u64 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i8)
                }
            }
            ScalarValue::UInt64(None) => Err(ConversionError::NullValue),
            ScalarValue::Float32(Some(v)) => {
                let f = v.into_inner();
                if f.is_nan() || f.is_infinite() {
                    Err(ConversionError::ParseError(f.to_string()))
                } else if f > i8::MAX as f32 || f < i8::MIN as f32 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(f as i8)
                }
            }
            ScalarValue::Float32(None) => Err(ConversionError::NullValue),
            ScalarValue::Float64(Some(v)) => {
                let f = v.into_inner();
                if f.is_nan() || f.is_infinite() {
                    Err(ConversionError::ParseError(f.to_string()))
                } else if f > i8::MAX as f64 || f < i8::MIN as f64 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(f as i8)
                }
            }
            ScalarValue::Float64(None) => Err(ConversionError::NullValue),
            ScalarValue::Boolean(Some(v)) => Ok(if *v { 1 } else { 0 }),
            ScalarValue::Boolean(None) => Err(ConversionError::NullValue),
            ScalarValue::String(Some(s)) => s
                .parse::<i8>()
                .map_err(|_| ConversionError::ParseError(s.clone())),
            ScalarValue::String(None) => Err(ConversionError::NullValue),
            ScalarValue::Null => Err(ConversionError::NullValue),
            _ => Err(ConversionError::IncompatibleType),
        }
    }

    // Convert to i16
    pub fn to_i16(&self) -> Result<i16, ConversionError> {
        match self {
            ScalarValue::Int8(Some(v)) => Ok(*v as i16),
            ScalarValue::Int8(None) => Err(ConversionError::NullValue),
            ScalarValue::Int16(Some(v)) => Ok(*v),
            ScalarValue::Int16(None) => Err(ConversionError::NullValue),
            ScalarValue::Int32(Some(v)) => {
                if *v > i16::MAX as i32 || *v < i16::MIN as i32 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i16)
                }
            }
            ScalarValue::Int32(None) => Err(ConversionError::NullValue),
            ScalarValue::Int64(Some(v)) => {
                if *v > i16::MAX as i64 || *v < i16::MIN as i64 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i16)
                }
            }
            ScalarValue::Int64(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt8(Some(v)) => Ok(*v as i16),
            ScalarValue::UInt8(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt16(Some(v)) => {
                if *v > i16::MAX as u16 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i16)
                }
            }
            ScalarValue::UInt16(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt32(Some(v)) => {
                if *v > i16::MAX as u32 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i16)
                }
            }
            ScalarValue::UInt32(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt64(Some(v)) => {
                if *v > i16::MAX as u64 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i16)
                }
            }
            ScalarValue::UInt64(None) => Err(ConversionError::NullValue),
            ScalarValue::Float32(Some(v)) => {
                let f = v.into_inner();
                if f.is_nan() || f.is_infinite() {
                    Err(ConversionError::ParseError(f.to_string()))
                } else if f > i16::MAX as f32 || f < i16::MIN as f32 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(f as i16)
                }
            }
            ScalarValue::Float32(None) => Err(ConversionError::NullValue),
            ScalarValue::Float64(Some(v)) => {
                let f = v.into_inner();
                if f.is_nan() || f.is_infinite() {
                    Err(ConversionError::ParseError(f.to_string()))
                } else if f > i16::MAX as f64 || f < i16::MIN as f64 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(f as i16)
                }
            }
            ScalarValue::Float64(None) => Err(ConversionError::NullValue),
            ScalarValue::Boolean(Some(v)) => Ok(if *v { 1 } else { 0 }),
            ScalarValue::Boolean(None) => Err(ConversionError::NullValue),
            ScalarValue::String(Some(s)) => s
                .parse::<i16>()
                .map_err(|_| ConversionError::ParseError(s.clone())),
            ScalarValue::String(None) => Err(ConversionError::NullValue),
            ScalarValue::Null => Err(ConversionError::NullValue),
            _ => Err(ConversionError::IncompatibleType),
        }
    }

    // Convert to i32
    pub fn to_i32(&self) -> Result<i32, ConversionError> {
        match self {
            ScalarValue::Int8(Some(v)) => Ok(*v as i32),
            ScalarValue::Int8(None) => Err(ConversionError::NullValue),
            ScalarValue::Int16(Some(v)) => Ok(*v as i32),
            ScalarValue::Int16(None) => Err(ConversionError::NullValue),
            ScalarValue::Int32(Some(v)) => Ok(*v),
            ScalarValue::Int32(None) => Err(ConversionError::NullValue),
            ScalarValue::Int64(Some(v)) => {
                if *v > i32::MAX as i64 || *v < i32::MIN as i64 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i32)
                }
            }
            ScalarValue::Int64(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt8(Some(v)) => Ok(*v as i32),
            ScalarValue::UInt8(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt16(Some(v)) => Ok(*v as i32),
            ScalarValue::UInt16(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt32(Some(v)) => {
                if *v > i32::MAX as u32 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i32)
                }
            }
            ScalarValue::UInt32(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt64(Some(v)) => {
                if *v > i32::MAX as u64 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i32)
                }
            }
            ScalarValue::UInt64(None) => Err(ConversionError::NullValue),
            ScalarValue::Float32(Some(v)) => {
                let f = v.into_inner();
                if f.is_nan() || f.is_infinite() {
                    Err(ConversionError::ParseError(f.to_string()))
                } else if f > i32::MAX as f32 || f < i32::MIN as f32 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(f as i32)
                }
            }
            ScalarValue::Float32(None) => Err(ConversionError::NullValue),
            ScalarValue::Float64(Some(v)) => {
                let f = v.into_inner();
                if f.is_nan() || f.is_infinite() {
                    Err(ConversionError::ParseError(f.to_string()))
                } else if f > i32::MAX as f64 || f < i32::MIN as f64 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(f as i32)
                }
            }
            ScalarValue::Float64(None) => Err(ConversionError::NullValue),
            ScalarValue::Boolean(Some(v)) => Ok(if *v { 1 } else { 0 }),
            ScalarValue::Boolean(None) => Err(ConversionError::NullValue),
            ScalarValue::String(Some(s)) => s
                .parse::<i32>()
                .map_err(|_| ConversionError::ParseError(s.clone())),
            ScalarValue::String(None) => Err(ConversionError::NullValue),
            ScalarValue::Null => Err(ConversionError::NullValue),
            _ => Err(ConversionError::IncompatibleType),
        }
    }

    // Convert to i64
    pub fn to_i64(&self) -> Result<i64, ConversionError> {
        match self {
            ScalarValue::Int8(Some(v)) => Ok(*v as i64),
            ScalarValue::Int8(None) => Err(ConversionError::NullValue),
            ScalarValue::Int16(Some(v)) => Ok(*v as i64),
            ScalarValue::Int16(None) => Err(ConversionError::NullValue),
            ScalarValue::Int32(Some(v)) => Ok(*v as i64),
            ScalarValue::Int32(None) => Err(ConversionError::NullValue),
            ScalarValue::Int64(Some(v)) => Ok(*v),
            ScalarValue::Int64(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt8(Some(v)) => Ok(*v as i64),
            ScalarValue::UInt8(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt16(Some(v)) => Ok(*v as i64),
            ScalarValue::UInt16(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt32(Some(v)) => Ok(*v as i64),
            ScalarValue::UInt32(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt64(Some(v)) => {
                if *v > i64::MAX as u64 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as i64)
                }
            }
            ScalarValue::UInt64(None) => Err(ConversionError::NullValue),
            ScalarValue::Float32(Some(v)) => {
                let f = v.into_inner();
                if f.is_nan() || f.is_infinite() {
                    Err(ConversionError::ParseError(f.to_string()))
                } else if f > i64::MAX as f32 || f < i64::MIN as f32 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(f as i64)
                }
            }
            ScalarValue::Float32(None) => Err(ConversionError::NullValue),
            ScalarValue::Float64(Some(v)) => {
                let f = v.into_inner();
                if f.is_nan() || f.is_infinite() {
                    Err(ConversionError::ParseError(f.to_string()))
                } else if f > i64::MAX as f64 || f < i64::MIN as f64 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(f as i64)
                }
            }
            ScalarValue::Float64(None) => Err(ConversionError::NullValue),
            ScalarValue::Boolean(Some(v)) => Ok(if *v { 1 } else { 0 }),
            ScalarValue::Boolean(None) => Err(ConversionError::NullValue),
            ScalarValue::String(Some(s)) => s
                .parse::<i64>()
                .map_err(|_| ConversionError::ParseError(s.clone())),
            ScalarValue::String(None) => Err(ConversionError::NullValue),
            ScalarValue::Null => Err(ConversionError::NullValue),
            _ => Err(ConversionError::IncompatibleType),
        }
    }

    // Convert to u8
    pub fn to_u8(&self) -> Result<u8, ConversionError> {
        match self {
            ScalarValue::Int8(Some(v)) => {
                if *v < 0 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u8)
                }
            }
            ScalarValue::Int8(None) => Err(ConversionError::NullValue),
            ScalarValue::Int16(Some(v)) => {
                if *v < 0 || *v > u8::MAX as i16 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u8)
                }
            }
            ScalarValue::Int16(None) => Err(ConversionError::NullValue),
            ScalarValue::Int32(Some(v)) => {
                if *v < 0 || *v > u8::MAX as i32 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u8)
                }
            }
            ScalarValue::Int32(None) => Err(ConversionError::NullValue),
            ScalarValue::Int64(Some(v)) => {
                if *v < 0 || *v > u8::MAX as i64 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u8)
                }
            }
            ScalarValue::Int64(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt8(Some(v)) => Ok(*v),
            ScalarValue::UInt8(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt16(Some(v)) => {
                if *v > u8::MAX as u16 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u8)
                }
            }
            ScalarValue::UInt16(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt32(Some(v)) => {
                if *v > u8::MAX as u32 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u8)
                }
            }
            ScalarValue::UInt32(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt64(Some(v)) => {
                if *v > u8::MAX as u64 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u8)
                }
            }
            ScalarValue::UInt64(None) => Err(ConversionError::NullValue),
            ScalarValue::Float32(Some(v)) => {
                let f = v.into_inner();
                if f.is_nan() || f.is_infinite() {
                    Err(ConversionError::ParseError(f.to_string()))
                } else if f < 0.0 || f > u8::MAX as f32 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(f as u8)
                }
            }
            ScalarValue::Float32(None) => Err(ConversionError::NullValue),
            ScalarValue::Float64(Some(v)) => {
                let f = v.into_inner();
                if f.is_nan() || f.is_infinite() {
                    Err(ConversionError::ParseError(f.to_string()))
                } else if f < 0.0 || f > u8::MAX as f64 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(f as u8)
                }
            }
            ScalarValue::Float64(None) => Err(ConversionError::NullValue),
            ScalarValue::Boolean(Some(v)) => Ok(if *v { 1 } else { 0 }),
            ScalarValue::Boolean(None) => Err(ConversionError::NullValue),
            ScalarValue::String(Some(s)) => s
                .parse::<u8>()
                .map_err(|_| ConversionError::ParseError(s.clone())),
            ScalarValue::String(None) => Err(ConversionError::NullValue),
            ScalarValue::Null => Err(ConversionError::NullValue),
            _ => Err(ConversionError::IncompatibleType),
        }
    }

    // Convert to u16
    pub fn to_u16(&self) -> Result<u16, ConversionError> {
        match self {
            ScalarValue::Int8(Some(v)) => {
                if *v < 0 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u16)
                }
            }
            ScalarValue::Int8(None) => Err(ConversionError::NullValue),
            ScalarValue::Int16(Some(v)) => {
                if *v < 0 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u16)
                }
            }
            ScalarValue::Int16(None) => Err(ConversionError::NullValue),
            ScalarValue::Int32(Some(v)) => {
                if *v < 0 || *v > u16::MAX as i32 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u16)
                }
            }
            ScalarValue::Int32(None) => Err(ConversionError::NullValue),
            ScalarValue::Int64(Some(v)) => {
                if *v < 0 || *v > u16::MAX as i64 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u16)
                }
            }
            ScalarValue::Int64(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt8(Some(v)) => Ok(*v as u16),
            ScalarValue::UInt8(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt16(Some(v)) => Ok(*v),
            ScalarValue::UInt16(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt32(Some(v)) => {
                if *v > u16::MAX as u32 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u16)
                }
            }
            ScalarValue::UInt32(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt64(Some(v)) => {
                if *v > u16::MAX as u64 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u16)
                }
            }
            ScalarValue::UInt64(None) => Err(ConversionError::NullValue),
            ScalarValue::Float32(Some(v)) => {
                let f = v.into_inner();
                if f.is_nan() || f.is_infinite() {
                    Err(ConversionError::ParseError(f.to_string()))
                } else if f < 0.0 || f > u16::MAX as f32 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(f as u16)
                }
            }
            ScalarValue::Float32(None) => Err(ConversionError::NullValue),
            ScalarValue::Float64(Some(v)) => {
                let f = v.into_inner();
                if f.is_nan() || f.is_infinite() {
                    Err(ConversionError::ParseError(f.to_string()))
                } else if f < 0.0 || f > u16::MAX as f64 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(f as u16)
                }
            }
            ScalarValue::Float64(None) => Err(ConversionError::NullValue),
            ScalarValue::Boolean(Some(v)) => Ok(if *v { 1 } else { 0 }),
            ScalarValue::Boolean(None) => Err(ConversionError::NullValue),
            ScalarValue::String(Some(s)) => s
                .parse::<u16>()
                .map_err(|_| ConversionError::ParseError(s.clone())),
            ScalarValue::String(None) => Err(ConversionError::NullValue),
            ScalarValue::Null => Err(ConversionError::NullValue),
            _ => Err(ConversionError::IncompatibleType),
        }
    }

    // Convert to u32
    pub fn to_u32(&self) -> Result<u32, ConversionError> {
        match self {
            ScalarValue::Int8(Some(v)) => {
                if *v < 0 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u32)
                }
            }
            ScalarValue::Int8(None) => Err(ConversionError::NullValue),
            ScalarValue::Int16(Some(v)) => {
                if *v < 0 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u32)
                }
            }
            ScalarValue::Int16(None) => Err(ConversionError::NullValue),
            ScalarValue::Int32(Some(v)) => {
                if *v < 0 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u32)
                }
            }
            ScalarValue::Int32(None) => Err(ConversionError::NullValue),
            ScalarValue::Int64(Some(v)) => {
                if *v < 0 || *v > u32::MAX as i64 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u32)
                }
            }
            ScalarValue::Int64(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt8(Some(v)) => Ok(*v as u32),
            ScalarValue::UInt8(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt16(Some(v)) => Ok(*v as u32),
            ScalarValue::UInt16(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt32(Some(v)) => Ok(*v),
            ScalarValue::UInt32(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt64(Some(v)) => {
                if *v > u32::MAX as u64 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u32)
                }
            }
            ScalarValue::UInt64(None) => Err(ConversionError::NullValue),
            ScalarValue::Float32(Some(v)) => {
                let f = v.into_inner();
                if f.is_nan() || f.is_infinite() {
                    Err(ConversionError::ParseError(f.to_string()))
                } else if f < 0.0 || f > u32::MAX as f32 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(f as u32)
                }
            }
            ScalarValue::Float32(None) => Err(ConversionError::NullValue),
            ScalarValue::Float64(Some(v)) => {
                let f = v.into_inner();
                if f.is_nan() || f.is_infinite() {
                    Err(ConversionError::ParseError(f.to_string()))
                } else if f < 0.0 || f > u32::MAX as f64 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(f as u32)
                }
            }
            ScalarValue::Float64(None) => Err(ConversionError::NullValue),
            ScalarValue::Boolean(Some(v)) => Ok(if *v { 1 } else { 0 }),
            ScalarValue::Boolean(None) => Err(ConversionError::NullValue),
            ScalarValue::String(Some(s)) => s
                .parse::<u32>()
                .map_err(|_| ConversionError::ParseError(s.clone())),
            ScalarValue::String(None) => Err(ConversionError::NullValue),
            ScalarValue::Null => Err(ConversionError::NullValue),
            _ => Err(ConversionError::IncompatibleType),
        }
    }

    // Convert to u64
    pub fn to_u64(&self) -> Result<u64, ConversionError> {
        match self {
            ScalarValue::Int8(Some(v)) => {
                if *v < 0 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u64)
                }
            }
            ScalarValue::Int8(None) => Err(ConversionError::NullValue),
            ScalarValue::Int16(Some(v)) => {
                if *v < 0 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u64)
                }
            }
            ScalarValue::Int16(None) => Err(ConversionError::NullValue),
            ScalarValue::Int32(Some(v)) => {
                if *v < 0 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u64)
                }
            }
            ScalarValue::Int32(None) => Err(ConversionError::NullValue),
            ScalarValue::Int64(Some(v)) => {
                if *v < 0 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(*v as u64)
                }
            }
            ScalarValue::Int64(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt8(Some(v)) => Ok(*v as u64),
            ScalarValue::UInt8(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt16(Some(v)) => Ok(*v as u64),
            ScalarValue::UInt16(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt32(Some(v)) => Ok(*v as u64),
            ScalarValue::UInt32(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt64(Some(v)) => Ok(*v),
            ScalarValue::UInt64(None) => Err(ConversionError::NullValue),
            ScalarValue::Float32(Some(v)) => {
                let f = v.into_inner();
                if f.is_nan() || f.is_infinite() {
                    Err(ConversionError::ParseError(f.to_string()))
                } else if f < 0.0 || f > u64::MAX as f32 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(f as u64)
                }
            }
            ScalarValue::Float32(None) => Err(ConversionError::NullValue),
            ScalarValue::Float64(Some(v)) => {
                let f = v.into_inner();
                if f.is_nan() || f.is_infinite() {
                    Err(ConversionError::ParseError(f.to_string()))
                } else if f < 0.0 || f > u64::MAX as f64 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(f as u64)
                }
            }
            ScalarValue::Float64(None) => Err(ConversionError::NullValue),
            ScalarValue::Boolean(Some(v)) => Ok(if *v { 1 } else { 0 }),
            ScalarValue::Boolean(None) => Err(ConversionError::NullValue),
            ScalarValue::String(Some(s)) => s
                .parse::<u64>()
                .map_err(|_| ConversionError::ParseError(s.clone())),
            ScalarValue::String(None) => Err(ConversionError::NullValue),
            ScalarValue::Null => Err(ConversionError::NullValue),
            _ => Err(ConversionError::IncompatibleType),
        }
    }

    // Convert to f32
    pub fn to_f32(&self) -> Result<f32, ConversionError> {
        match self {
            ScalarValue::Int8(Some(v)) => Ok(*v as f32),
            ScalarValue::Int8(None) => Err(ConversionError::NullValue),
            ScalarValue::Int16(Some(v)) => Ok(*v as f32),
            ScalarValue::Int16(None) => Err(ConversionError::NullValue),
            ScalarValue::Int32(Some(v)) => Ok(*v as f32),
            ScalarValue::Int32(None) => Err(ConversionError::NullValue),
            ScalarValue::Int64(Some(v)) => Ok(*v as f32),
            ScalarValue::Int64(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt8(Some(v)) => Ok(*v as f32),
            ScalarValue::UInt8(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt16(Some(v)) => Ok(*v as f32),
            ScalarValue::UInt16(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt32(Some(v)) => Ok(*v as f32),
            ScalarValue::UInt32(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt64(Some(v)) => Ok(*v as f32),
            ScalarValue::UInt64(None) => Err(ConversionError::NullValue),
            ScalarValue::Float32(Some(v)) => Ok(v.into_inner()),
            ScalarValue::Float32(None) => Err(ConversionError::NullValue),
            ScalarValue::Float64(Some(v)) => {
                let f = v.into_inner();
                if f.is_nan() || f.is_infinite() {
                    Err(ConversionError::ParseError(f.to_string()))
                } else if f > f32::MAX as f64 || f < f32::MIN as f64 {
                    Err(ConversionError::Overflow)
                } else {
                    Ok(f as f32)
                }
            }
            ScalarValue::Float64(None) => Err(ConversionError::NullValue),
            ScalarValue::Boolean(Some(v)) => Ok(if *v { 1.0 } else { 0.0 }),
            ScalarValue::Boolean(None) => Err(ConversionError::NullValue),
            ScalarValue::String(Some(s)) => s
                .parse::<f32>()
                .map_err(|_| ConversionError::ParseError(s.clone())),
            ScalarValue::String(None) => Err(ConversionError::NullValue),
            ScalarValue::Null => Err(ConversionError::NullValue),
            _ => Err(ConversionError::IncompatibleType),
        }
    }

    // Convert to f64
    pub fn to_f64(&self) -> Result<f64, ConversionError> {
        match self {
            ScalarValue::Int8(Some(v)) => Ok(*v as f64),
            ScalarValue::Int8(None) => Err(ConversionError::NullValue),
            ScalarValue::Int16(Some(v)) => Ok(*v as f64),
            ScalarValue::Int16(None) => Err(ConversionError::NullValue),
            ScalarValue::Int32(Some(v)) => Ok(*v as f64),
            ScalarValue::Int32(None) => Err(ConversionError::NullValue),
            ScalarValue::Int64(Some(v)) => Ok(*v as f64),
            ScalarValue::Int64(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt8(Some(v)) => Ok(*v as f64),
            ScalarValue::UInt8(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt16(Some(v)) => Ok(*v as f64),
            ScalarValue::UInt16(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt32(Some(v)) => Ok(*v as f64),
            ScalarValue::UInt32(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt64(Some(v)) => Ok(*v as f64),
            ScalarValue::UInt64(None) => Err(ConversionError::NullValue),
            ScalarValue::Float32(Some(v)) => Ok(v.into_inner() as f64),
            ScalarValue::Float32(None) => Err(ConversionError::NullValue),
            ScalarValue::Float64(Some(v)) => Ok(v.into_inner()),
            ScalarValue::Float64(None) => Err(ConversionError::NullValue),
            ScalarValue::Boolean(Some(v)) => Ok(if *v { 1.0 } else { 0.0 }),
            ScalarValue::Boolean(None) => Err(ConversionError::NullValue),
            ScalarValue::String(Some(s)) => s
                .parse::<f64>()
                .map_err(|_| ConversionError::ParseError(s.clone())),
            ScalarValue::String(None) => Err(ConversionError::NullValue),
            ScalarValue::Null => Err(ConversionError::NullValue),
            _ => Err(ConversionError::IncompatibleType),
        }
    }

    // Convert to bool
    pub fn to_bool(&self) -> Result<bool, ConversionError> {
        match self {
            ScalarValue::Int8(Some(v)) => Ok(*v != 0),
            ScalarValue::Int8(None) => Err(ConversionError::NullValue),
            ScalarValue::Int16(Some(v)) => Ok(*v != 0),
            ScalarValue::Int16(None) => Err(ConversionError::NullValue),
            ScalarValue::Int32(Some(v)) => Ok(*v != 0),
            ScalarValue::Int32(None) => Err(ConversionError::NullValue),
            ScalarValue::Int64(Some(v)) => Ok(*v != 0),
            ScalarValue::Int64(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt8(Some(v)) => Ok(*v != 0),
            ScalarValue::UInt8(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt16(Some(v)) => Ok(*v != 0),
            ScalarValue::UInt16(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt32(Some(v)) => Ok(*v != 0),
            ScalarValue::UInt32(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt64(Some(v)) => Ok(*v != 0),
            ScalarValue::UInt64(None) => Err(ConversionError::NullValue),
            ScalarValue::Float32(Some(v)) => {
                let f = v.into_inner();
                if f.is_nan() {
                    Err(ConversionError::ParseError(f.to_string()))
                } else {
                    Ok(f.abs() > EPSILON as f32)
                }
            }
            ScalarValue::Float32(None) => Err(ConversionError::NullValue),
            ScalarValue::Float64(Some(v)) => {
                let f = v.into_inner();
                if f.is_nan() {
                    Err(ConversionError::ParseError(f.to_string()))
                } else {
                    Ok(f.abs() > EPSILON)
                }
            }
            ScalarValue::Float64(None) => Err(ConversionError::NullValue),
            ScalarValue::Boolean(Some(v)) => Ok(*v),
            ScalarValue::Boolean(None) => Err(ConversionError::NullValue),
            ScalarValue::String(Some(s)) => {
                let lowered = s.to_lowercase();
                if lowered == "true" || lowered == "1" {
                    Ok(true)
                } else if lowered == "false" || lowered == "0" {
                    Ok(false)
                } else {
                    Err(ConversionError::ParseError(s.clone()))
                }
            }
            ScalarValue::String(None) => Err(ConversionError::NullValue),
            ScalarValue::Null => Err(ConversionError::NullValue),
            _ => Err(ConversionError::IncompatibleType),
        }
    }

    // Convert to String
    pub fn to_string(&self) -> Result<String, ConversionError> {
        match self {
            ScalarValue::Int8(Some(v)) => Ok(v.to_string()),
            ScalarValue::Int8(None) => Err(ConversionError::NullValue),
            ScalarValue::Int16(Some(v)) => Ok(v.to_string()),
            ScalarValue::Int16(None) => Err(ConversionError::NullValue),
            ScalarValue::Int32(Some(v)) => Ok(v.to_string()),
            ScalarValue::Int32(None) => Err(ConversionError::NullValue),
            ScalarValue::Int64(Some(v)) => Ok(v.to_string()),
            ScalarValue::Int64(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt8(Some(v)) => Ok(v.to_string()),
            ScalarValue::UInt8(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt16(Some(v)) => Ok(v.to_string()),
            ScalarValue::UInt16(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt32(Some(v)) => Ok(v.to_string()),
            ScalarValue::UInt32(None) => Err(ConversionError::NullValue),
            ScalarValue::UInt64(Some(v)) => Ok(v.to_string()),
            ScalarValue::UInt64(None) => Err(ConversionError::NullValue),
            ScalarValue::Float32(Some(v)) => Ok(v.into_inner().to_string()),
            ScalarValue::Float32(None) => Err(ConversionError::NullValue),
            ScalarValue::Float64(Some(v)) => Ok(v.into_inner().to_string()),
            ScalarValue::Float64(None) => Err(ConversionError::NullValue),
            ScalarValue::Boolean(Some(v)) => Ok(v.to_string()),
            ScalarValue::Boolean(None) => Err(ConversionError::NullValue),
            ScalarValue::String(Some(s)) => Ok(s.clone()),
            ScalarValue::String(None) => Err(ConversionError::NullValue),
            ScalarValue::Null => Err(ConversionError::NullValue),
            _ => Err(ConversionError::IncompatibleType),
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
    use ordered_float::OrderedFloat;

    use super::{ConversionError, ScalarValue};

    #[test]
    fn test_to_i8() {
        // Successful conversions
        assert_eq!(ScalarValue::Int8(Some(42)).to_i8(), Ok(42i8));
        assert_eq!(ScalarValue::Int16(Some(42)).to_i8(), Ok(42i8));
        assert_eq!(ScalarValue::Int32(Some(42)).to_i8(), Ok(42i8));
        assert_eq!(ScalarValue::Int64(Some(42)).to_i8(), Ok(42i8));
        assert_eq!(ScalarValue::UInt8(Some(42)).to_i8(), Ok(42i8));
        assert_eq!(ScalarValue::UInt16(Some(42)).to_i8(), Ok(42i8));
        assert_eq!(ScalarValue::UInt32(Some(42)).to_i8(), Ok(42i8));
        assert_eq!(ScalarValue::UInt64(Some(42)).to_i8(), Ok(42i8));
        assert_eq!(
            ScalarValue::Float32(Some(OrderedFloat(42.0))).to_i8(),
            Ok(42i8)
        );
        assert_eq!(
            ScalarValue::Float64(Some(OrderedFloat(42.0))).to_i8(),
            Ok(42i8)
        );
        assert_eq!(ScalarValue::Boolean(Some(true)).to_i8(), Ok(1i8));
        assert_eq!(ScalarValue::Boolean(Some(false)).to_i8(), Ok(0i8));
        assert_eq!(
            ScalarValue::String(Some("42".to_string())).to_i8(),
            Ok(42i8)
        );
        assert_eq!(
            ScalarValue::String(Some("-128".to_string())).to_i8(),
            Ok(-128i8)
        ); // Min value

        // Overflow cases
        assert!(matches!(
            ScalarValue::Int16(Some(128)).to_i8(),
            Err(ConversionError::Overflow)
        )); // > i8::MAX
        assert!(matches!(
            ScalarValue::Int16(Some(-129)).to_i8(),
            Err(ConversionError::Overflow)
        )); // < i8::MIN
        assert!(matches!(
            ScalarValue::UInt8(Some(128)).to_i8(),
            Err(ConversionError::Overflow)
        ));
        assert!(matches!(
            ScalarValue::Float32(Some(OrderedFloat(128.0))).to_i8(),
            Err(ConversionError::Overflow)
        ));
        assert!(matches!(
            ScalarValue::Float64(Some(OrderedFloat(-129.0))).to_i8(),
            Err(ConversionError::Overflow)
        ));

        // NaN and Infinity cases
        assert!(matches!(
            ScalarValue::Float32(Some(OrderedFloat(f32::NAN))).to_i8(),
            Err(ConversionError::ParseError(_))
        ));
        assert!(matches!(
            ScalarValue::Float32(Some(OrderedFloat(f32::INFINITY))).to_i8(),
            Err(ConversionError::ParseError(_))
        ));
        assert!(matches!(
            ScalarValue::Float64(Some(OrderedFloat(f64::NAN))).to_i8(),
            Err(ConversionError::ParseError(_))
        ));
        assert!(matches!(
            ScalarValue::Float64(Some(OrderedFloat(f64::INFINITY))).to_i8(),
            Err(ConversionError::ParseError(_))
        ));

        // Parse error
        assert!(matches!(
            ScalarValue::String(Some("invalid".to_string())).to_i8(),
            Err(ConversionError::ParseError(_))
        ));
        assert!(matches!(
            ScalarValue::String(Some("128".to_string())).to_i8(),
            Err(ConversionError::ParseError(_))
        )); // Overflow via parse

        // Null cases
        assert!(matches!(
            ScalarValue::Int8(None).to_i8(),
            Err(ConversionError::NullValue)
        ));
        assert!(matches!(
            ScalarValue::Null.to_i8(),
            Err(ConversionError::NullValue)
        ));
    }

    #[test]
    fn test_to_bool() {
        // Successful conversions
        assert_eq!(ScalarValue::Int8(Some(1)).to_bool(), Ok(true));
        assert_eq!(ScalarValue::Int8(Some(0)).to_bool(), Ok(false));
        assert_eq!(ScalarValue::Int16(Some(42)).to_bool(), Ok(true));
        assert_eq!(ScalarValue::Int32(Some(-1)).to_bool(), Ok(true)); // Non-zero is true
        assert_eq!(ScalarValue::Int64(Some(0)).to_bool(), Ok(false));
        assert_eq!(ScalarValue::UInt8(Some(1)).to_bool(), Ok(true));
        assert_eq!(ScalarValue::UInt16(Some(0)).to_bool(), Ok(false));
        assert_eq!(ScalarValue::UInt32(Some(42)).to_bool(), Ok(true));
        assert_eq!(ScalarValue::UInt64(Some(0)).to_bool(), Ok(false));
        assert_eq!(
            ScalarValue::Float32(Some(OrderedFloat(0.0))).to_bool(),
            Ok(false)
        );
        assert_eq!(
            ScalarValue::Float32(Some(OrderedFloat(1.5))).to_bool(),
            Ok(true)
        );
        assert_eq!(
            ScalarValue::Float64(Some(OrderedFloat(-1.0))).to_bool(),
            Ok(true)
        );
        assert_eq!(
            ScalarValue::Float32(Some(OrderedFloat(0.00000000001))).to_bool(),
            Ok(false)
        ); // Less than EPSILON
        assert_eq!(
            ScalarValue::Float64(Some(OrderedFloat(0.00000000001))).to_bool(),
            Ok(false)
        ); // Less than EPSILON
        assert_eq!(ScalarValue::Boolean(Some(true)).to_bool(), Ok(true));
        assert_eq!(ScalarValue::Boolean(Some(false)).to_bool(), Ok(false));
        assert_eq!(
            ScalarValue::String(Some("true".to_string())).to_bool(),
            Ok(true)
        );
        assert_eq!(
            ScalarValue::String(Some("TRUE".to_string())).to_bool(),
            Ok(true)
        ); // Case insensitive
        assert_eq!(
            ScalarValue::String(Some("false".to_string())).to_bool(),
            Ok(false)
        );
        assert_eq!(
            ScalarValue::String(Some("1".to_string())).to_bool(),
            Ok(true)
        );
        assert_eq!(
            ScalarValue::String(Some("0".to_string())).to_bool(),
            Ok(false)
        );

        // NaN cases
        assert!(matches!(
            ScalarValue::Float32(Some(OrderedFloat(f32::NAN))).to_bool(),
            Err(ConversionError::ParseError(_))
        ));
        assert!(matches!(
            ScalarValue::Float64(Some(OrderedFloat(f64::NAN))).to_bool(),
            Err(ConversionError::ParseError(_))
        ));

        // Parse error
        assert!(matches!(
            ScalarValue::String(Some("invalid".to_string())).to_bool(),
            Err(ConversionError::ParseError(_))
        ));
        assert!(matches!(
            ScalarValue::String(Some("yes".to_string())).to_bool(),
            Err(ConversionError::ParseError(_))
        ));

        // Null cases
        assert!(matches!(
            ScalarValue::Boolean(None).to_bool(),
            Err(ConversionError::NullValue)
        ));
        assert!(matches!(
            ScalarValue::Null.to_bool(),
            Err(ConversionError::NullValue)
        ));
    }

    #[test]
    fn test_to_string() {
        // Successful conversions
        assert_eq!(
            ScalarValue::Int8(Some(42)).to_string(),
            Ok("42".to_string())
        );
        assert_eq!(
            ScalarValue::Int16(Some(-42)).to_string(),
            Ok("-42".to_string())
        );
        assert_eq!(
            ScalarValue::Int32(Some(42)).to_string(),
            Ok("42".to_string())
        );
        assert_eq!(
            ScalarValue::Int64(Some(42)).to_string(),
            Ok("42".to_string())
        );
        assert_eq!(
            ScalarValue::UInt8(Some(42)).to_string(),
            Ok("42".to_string())
        );
        assert_eq!(
            ScalarValue::UInt16(Some(42)).to_string(),
            Ok("42".to_string())
        );
        assert_eq!(
            ScalarValue::UInt32(Some(42)).to_string(),
            Ok("42".to_string())
        );
        assert_eq!(
            ScalarValue::UInt64(Some(42)).to_string(),
            Ok("42".to_string())
        );
        assert_eq!(
            ScalarValue::Float32(Some(OrderedFloat(42.5))).to_string(),
            Ok("42.5".to_string())
        );
        assert_eq!(
            ScalarValue::Float64(Some(OrderedFloat(42.5))).to_string(),
            Ok("42.5".to_string())
        );
        assert_eq!(
            ScalarValue::Boolean(Some(true)).to_string(),
            Ok("true".to_string())
        );
        assert_eq!(
            ScalarValue::Boolean(Some(false)).to_string(),
            Ok("false".to_string())
        );
        assert_eq!(
            ScalarValue::String(Some("hello".to_string())).to_string(),
            Ok("hello".to_string())
        );

        // Null cases
        assert!(matches!(
            ScalarValue::String(None).to_string(),
            Err(ConversionError::NullValue)
        ));
        assert!(matches!(
            ScalarValue::Null.to_string(),
            Err(ConversionError::NullValue)
        ));
    }
}
