use minigu_catalog::named_ref::NamedGraphTypeRef;
use serde::Serialize;

use super::type_element::BoundGraphElementType;

#[derive(Debug, Clone, Serialize)]
pub enum BoundGraphType {
    Ref(NamedGraphTypeRef),
    Nested(Vec<BoundGraphElementType>),
}
