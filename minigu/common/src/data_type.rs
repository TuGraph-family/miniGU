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
