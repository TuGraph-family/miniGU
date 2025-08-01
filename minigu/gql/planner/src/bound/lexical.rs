use serde::Serialize;

/// This represents an unsigned integer with undetermined bit width.
///
/// We use signed types to represent unsigned integers because it's easier to implement
/// arithmetic operations like `-123` on signed types.
#[derive(Debug, Clone, Copy, Serialize)]
pub enum BoundUnsignedInteger {
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
}

impl BoundUnsignedInteger {
    pub fn to_usize(self) -> usize {
        match self {
            BoundUnsignedInteger::Int8(value) => value as usize,
            BoundUnsignedInteger::Int16(value) => value as usize,
            BoundUnsignedInteger::Int32(value) => value as usize,
            BoundUnsignedInteger::Int64(value) => value as usize,
        }
    }
}
