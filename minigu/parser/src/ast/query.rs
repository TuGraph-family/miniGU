use crate::macros::{base, ext};

pub struct CompositeQueryExpression {}

#[apply(ext)]
pub enum SetOperator {
    Union,
    Except,
    Intersect,
}

#[apply(base)]
pub enum SimpleQueryStatement {
    Match,
    Let,
    For,
    Filter,
    OrderBy,
}
