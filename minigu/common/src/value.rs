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
