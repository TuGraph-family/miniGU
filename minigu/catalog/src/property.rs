use minigu_common::data_type::LogicalType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Property {
    logical_type: LogicalType,
    nullable: bool,
}

impl Property {
    #[inline]
    pub fn new(logical_type: LogicalType, nullable: bool) -> Self {
        Self {
            logical_type,
            nullable,
        }
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
