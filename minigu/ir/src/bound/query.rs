use minigu_common::data_type::DataSchemaRef;
use minigu_common::ordering::{NullOrdering, SortOrdering};
use serde::Serialize;

use super::value_expr::SetQuantifier;
use crate::bound::{BoundExpr, BoundProcedure};

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
pub enum BoundLinearQueryStatement {
    Query {
        statements: Vec<BoundSimpleQueryStatement>,
        result: BoundResultStatement,
    },
    Nested(Box<BoundProcedure>),
    // TODO: Implement SELECT statement
    Select,
}

impl BoundLinearQueryStatement {
    pub fn schema(&self) -> Option<DataSchemaRef> {
        match self {
            BoundLinearQueryStatement::Query { result, .. } => result.schema().cloned(),
            BoundLinearQueryStatement::Nested(query) => query.schema(),
            BoundLinearQueryStatement::Select => todo!(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum BoundResultStatement {
    Return {
        statement: BoundReturnStatement,
        order_by: Option<BoundOrderByAndPageStatement>,
    },
    Finish,
}

impl BoundResultStatement {
    pub fn schema(&self) -> Option<&DataSchemaRef> {
        match self {
            BoundResultStatement::Return { statement, .. } => Some(statement.schema()),
            BoundResultStatement::Finish => None,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct BoundReturnStatement {
    pub quantifier: Option<SetQuantifier>,
    pub items: Vec<BoundExpr>,
    // TODO: Implement GROUP BY
    pub schema: DataSchemaRef,
}

impl BoundReturnStatement {
    #[inline]
    pub fn schema(&self) -> &DataSchemaRef {
        &self.schema
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct BoundOrderByAndPageStatement {
    offset: Option<usize>,
    limit: Option<usize>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BoundSortSpec {
    pub key: BoundExpr,
    pub ordering: SortOrdering,
    pub null_ordering: NullOrdering,
}

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
