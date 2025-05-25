use serde::Serialize;

use crate::object_ref::GraphTypeRef;
use crate::statement::type_element::GraphElementType;
use crate::types::Ident;

#[derive(Debug, Serialize)]
pub enum BoundGraphType {
    Ref(GraphTypeRef),
    Nested(Vec<GraphElementType>),
}
