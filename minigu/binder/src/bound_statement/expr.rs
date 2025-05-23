use std::time::Duration;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use macro_rules_attribute::apply;
use serde::Serialize;
use gql_parser::ast::{AggregateFunction, BinaryOp, BooleanLiteral, Function, GroupedPathPattern, ListTypeName, PatternQuantifier, StringLiteral, UnaryOp};
use crate::bound_statement::common::{BoundElementPattern, BoundFieldOrProperty};
use crate::catalog_ref::{GraphCatalogRef, Ident};


#[derive(Debug, Serialize)]
pub enum BoundGraphExpr {
    // Bound Of GraphExpr.
    // From the Expr, the corresponding Graph
    // object can be determined in advance.
    // Resolver implemented: resolve_graph_expr
    Ref(GraphCatalogRef),
    Object(BoundObjectExpr),
}

#[derive(Debug, Serialize)]
pub enum BoundObjectExpr {
    Variable(BoundExpr),
    Expr(BoundExpr),
}


#[derive(Debug, Serialize)]
pub enum BoundPathPatternExpr {
    Union(Vec<BoundPathPatternExpr>),
    Alternation(Vec<BoundPathPatternExpr>),
    Concat(Vec<BoundPathPatternExpr>),
    Quantified{
        path: Box<BoundPathPatternExpr>,
        quantifier: Box<PatternQuantifier>,
    },
    Optional(Box<BoundPathPatternExpr>),
    Grouped(GroupedPathPattern),
    Pattern(BoundElementPattern),
}

#[derive(Debug, Serialize)]
pub enum BoundExpr{
    Binary {
        op: BinaryOp,
        left: Box<BoundExpr>,
        right: Box<BoundExpr>,
    },
    Unary {
        op: UnaryOp,
        child: Box<BoundExpr>,
    },
    DurationBetween {
        left: Box<BoundExpr>,
        right: Box<BoundExpr>,
    },
    Is{
        left: Box<BoundExpr>,
        right: Box<BooleanLiteral>,
    },
    IsNot {
        left: Box<BoundExpr>,
        right: Box<BooleanLiteral>,
    },
    Function(Function),
    Aggregate(AggregateFunction),
    Variable(Ident),
    Value(BoundValue),
    Path,
    Property {
        source: Box<BoundExpr>,
        trailing_names: Vec<Ident>,
    },
    Graph(Box<BoundGraphExpr>)
}

#[derive(Debug, Serialize)]
pub struct BoundVariable {
    pub name: Ident,
}

#[derive(Debug, Serialize)]
pub enum BoundValue {
    SessionUser,
    Parameter(Ident),
    Literal(BoundLiteral),
}

#[derive(Debug, Serialize)]
pub enum BoundLiteral {
    // Currently use i64 to store numeric number.
    Numeric(i64),
    Boolean(Option<bool>),
    String(StringLiteral),
    Temporal(BoundTemporal),
    Duration(BoundDuration),
    List(BoundListConstructor),
    Record(Vec<BoundFieldOrProperty>),
    Null
}

#[derive(Debug, Serialize)]
pub enum BoundTemporal {
    Date(NaiveDate),
    Time(NaiveTime),
    DatTime(NaiveDateTime),
    Timestamp(DateTime<Utc>),
    SqlDatetime(String)
}

#[derive(Debug, Serialize)]
pub enum BoundDuration {
    Duration(Duration),
    SqlInterval(String),
}

#[derive(Debug, Serialize)]
pub struct BoundListConstructor {
    pub type_name: Option<ListTypeName>,
    pub values: Vec<BoundExpr>,
}
