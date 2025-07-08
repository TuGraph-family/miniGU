use minigu_common::data_type::DataSchemaRef;
use minigu_common::ordering::{NullOrdering, SortOrdering};
use serde::Serialize;

use super::value_expr::BoundSetQuantifier;
use crate::bound::{BoundCallProcedureStatement, BoundExpr, BoundProcedure};

#[derive(Debug, Clone, Serialize)]
pub enum BoundCompositeQueryStatement {
    Conjunction {
        conjunction: BoundQueryConjunction,
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
        order_by_and_page: Option<BoundOrderByAndPageStatement>,
    },
    Finish,
}

impl BoundResultStatement {
    pub fn schema(&self) -> Option<&DataSchemaRef> {
        match self {
            BoundResultStatement::Return { statement, .. } => Some(&statement.schema),
            BoundResultStatement::Finish => None,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct BoundReturnStatement {
    pub quantifier: Option<BoundSetQuantifier>,
    /// If this is `None`, the statement should return all columns from the current binding table.
    pub items: Option<Vec<BoundExpr>>,
    /// The output schema of the return statement.
    pub schema: DataSchemaRef,
}

#[derive(Debug, Clone, Serialize)]
pub struct BoundOrderByAndPageStatement {
    pub order_by: Vec<BoundSortSpec>,
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BoundSortSpec {
    pub key: BoundExpr,
    pub ordering: SortOrdering,
    pub null_ordering: NullOrdering,
}

#[derive(Debug, Clone, Serialize)]
pub enum BoundSimpleQueryStatement {
    Call(BoundCallProcedureStatement),
}

#[derive(Debug, Clone, Serialize)]
pub enum BoundQueryConjunction {
    SetOp(BoundSetOp),
    Otherwise,
}

#[derive(Debug, Clone, Serialize)]
pub enum BoundSetOpKind {
    Union,
    Except,
    Intersect,
}

#[derive(Debug, Clone, Serialize)]
pub struct BoundSetOp {
    pub kind: BoundSetOpKind,
    pub quantifier: Option<BoundSetQuantifier>,
}
