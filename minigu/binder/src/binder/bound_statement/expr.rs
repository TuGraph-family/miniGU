use macro_rules_attribute::apply;
use gql_parser::ast::{AggregateFunction, BinaryOp, BooleanLiteral, ElementPattern, Function, GroupedPathPattern, Ident, ListTypeName, PathPatternExpr, PatternQuantifier, StringLiteral, UnaryOp, Value};
use crate::macros::base;
use crate::program::bound_statement::common::{BoundElementPattern, BoundFieldOrProperty};
use crate::program::bound_statement::object_ref::{BoundCatalogObjectRef, BoundGraphRef};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime, DateTime, Utc, Duration};
use gql_parser::span::OptSpanned;


#[apply(base)]
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

#[apply(base)]
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

#[apply(base)]
pub struct BoundVariable {
    pub name: Ident,
}

#[apply(base)]
pub enum BoundValue {
    SessionUser,
    Parameter(Ident),
    Literal(BoundLiteral),
}

#[apply(base)]
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

#[apply(base)]
pub enum BoundTemporal {
    Date(NaiveDate),
    Time(NaiveTime),
    DatTime(NaiveDateTime),
    Timestamp(DateTime<Utc>),
    SqlDatetime(String)
}

#[apply(base)]
pub enum BoundDuration {
    Duration(Duration),
    SqlInterval(String),
}

#[apply(base)]
pub struct BoundListConstructor {
    pub type_name: Option<ListTypeName>,
    pub values: Vec<BoundExpr>,
}
