use minigu_common::data_type::LogicalType;
use minigu_common::types::PropertyId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Property {
    id: PropertyId,
    logical_type: LogicalType,
    nullable: bool,
}
