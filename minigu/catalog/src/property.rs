use minigu_common::data_type::LogicalType;
use minigu_common::types::PropertyId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Property {
    id: PropertyId,
    logical_type: LogicalType,
    nullable: bool,
}

impl Property {
    #[inline]
    pub fn id(&self) -> PropertyId {
        self.id
    }

    #[inline]
    pub fn logical_type(&self) -> &LogicalType {
        &self.logical_type
    }

    #[inline]
    pub fn nullable(&self) -> bool {
        self.nullable
    }
}
