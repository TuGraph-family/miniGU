use minigu_common::data_type::DataSchemaRef;
use serde::Serialize;

use super::value_expr::SetQuantifier;

#[derive(Debug, Clone, Serialize)]
pub enum BoundCompositeQueryStatement {
    Conjunction {
        conjunction: QueryConjunction,
        left: Box<BoundCompositeQueryStatement>,
        right: Box<BoundCompositeQueryStatement>,
    },
    Primary(BoundLinearQueryStatement),
}

impl BoundCompositeQueryStatement {
    pub fn schema(&self) -> DataSchemaRef {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct BoundLinearQueryStatement {}

#[derive(Debug, Clone, Serialize)]
pub struct BoundSimpleQueryStatement {}

#[derive(Debug, Clone, Serialize)]
pub enum QueryConjunction {
    SetOp(SetOp),
    Otherwise,
}

#[derive(Debug, Clone, Serialize)]
pub enum SetOpKind {
    Union,
    Except,
    Intersect,
}

#[derive(Debug, Clone, Serialize)]
pub struct SetOp {
    pub kind: SetOpKind,
    pub quantifier: Option<SetQuantifier>,
}
