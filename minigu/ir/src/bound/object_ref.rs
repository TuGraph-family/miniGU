use serde::Serialize;

use super::type_element::BoundGraphElementType;
use crate::named_ref::NamedGraphTypeRef;

#[derive(Debug, Clone, Serialize)]
pub enum BoundGraphType {
    Ref(NamedGraphTypeRef),
    Nested(Vec<BoundGraphElementType>),
}
