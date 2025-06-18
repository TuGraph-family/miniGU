use arrow::datatypes::DataType;

#[derive(Debug, Clone, PartialEq, Eq)]
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
    Vertex,
    Edge,
    Null,
}

impl LogicalType {
    #[inline]
    pub fn to_arrow_type(&self) -> DataType {
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
            LogicalType::Vertex => todo!(),
            LogicalType::Edge => todo!(),
            LogicalType::Null => DataType::Null,
        }
    }
}
