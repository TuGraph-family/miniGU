use std::sync::Arc;

use arrow::array::{
    Array, ArrayRef, AsArray, BooleanArray, Float32Array, Float64Array, Int8Array, Int16Array,
    Int32Array, Int64Array, NullArray, StringArray, UInt8Array, UInt16Array, UInt32Array,
    UInt64Array,
};
use arrow::datatypes::DataType;
use serde::{Deserialize, Serialize};

use crate::types::{EdgeId, LabelId, VertexId};

pub type Nullable<T> = Option<T>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    Float32(Nullable<f32>),
    Float64(Nullable<f64>),
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
            ScalarValue::Float32(value) => Arc::new(Float32Array::from_iter([*value])),
            ScalarValue::Float64(value) => Arc::new(Float64Array::from_iter([*value])),
            ScalarValue::String(value) => Arc::new(StringArray::from_iter([value])),
            ScalarValue::Vertex(_value) => todo!(),
            ScalarValue::Edge(_value) => todo!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PropertyValue {
    name: String,
    value: ScalarValue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VertexValue {
    id: VertexId,
    label: LabelId,
    properties: Vec<PropertyValue>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EdgeValue {
    eid: EdgeId,
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
        $m!(float32, f32, Float32);
        $m!(float64, f64, Float64);
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
            paste::paste! {
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
            paste::paste! {
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

pub trait IndexScalarValue {
    fn index(&self, index: usize) -> ScalarValue;
}

impl IndexScalarValue for dyn Array + '_ {
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
                array.is_valid(index).then(|| array.value(index)).into()
            }
            DataType::Float64 => {
                let array: &Float64Array = self.as_primitive();
                array.is_valid(index).then(|| array.value(index)).into()
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
