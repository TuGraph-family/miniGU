use serde::Serialize;
use gql_parser::ast::{
    GroupBy, Ident, MatchMode, PathPatternPrefix, Procedure,
    QueryConjunction, SetQuantifier,
};
use crate::bound_statement::common::BoundPathPattern;
use crate::bound_statement::expr::{BoundExpr, BoundGraphExpr};
use crate::bound_statement::procedure::BoundCallProcedureStatement;
use crate::bound_statement::procedure_spec::BoundProcedure;

#[derive(Debug, Serialize)]
pub enum BoundLinearQueryStatement {
    Focused(BoundFocusedLinearQueryStatement),
    Ambient(BoundAmbientLinearQueryStatement),
}

#[derive(Debug, Serialize)]
pub enum BoundFocusedLinearQueryStatement {
    Parts {
        parts: Vec<BoundFocusedLinearQueryStatementPart>,
        result: BoundResultStatement,
    },
    Result {
        use_graph: BoundGraphExpr,
        result: BoundResultStatement,
    },
    Nested {
        use_graph: BoundGraphExpr,
        query: Box<BoundProcedure>,
    },
    Select {},
}

#[derive(Debug, Serialize)]
pub struct BoundFocusedLinearQueryStatementPart {
    // Resolver implemented: resolve_focused_linear_query_parts;
    pub use_graph: BoundGraphExpr,
    pub statements: Vec<BoundSimpleQueryStatement>,
}

#[derive(Debug, Serialize)]
pub enum BoundAmbientLinearQueryStatement {
    Parts {
        parts: Vec<BoundSimpleQueryStatement>,
        result: BoundResultStatement,
    },
    Nested(Box<Procedure>),
}

#[derive(Debug, Serialize)]
pub enum BoundSimpleQueryStatement {
    Match(BoundMatchStatement),
    Call(BoundCallProcedureStatement),
}

#[derive(Debug, Serialize)]
pub enum BoundMatchStatement {
    Simple(BoundGraphPatternBindingTable),
    Optional(Vec<BoundMatchStatement>),
}

#[derive(Debug, Serialize)]
pub struct BoundGraphPatternBindingTable {
    pub pattern: BoundGraphPattern,
    // TODO: Handle yield_item.
    pub yield_item: Vec<Ident>,
}

#[derive(Debug, Serialize)]
pub struct BoundGraphPattern {
    // Resolver implemented: resolve_graph_pattern.
    pub match_mode: Option<MatchMode>,
    pub patterns: Vec<BoundPathPattern>,
    pub keep: Option<PathPatternPrefix>,
    pub where_clause: Option<BoundExpr>,
}

#[derive(Debug, Serialize)]
pub struct BoundReturnStatement {
    pub quantifier: Option<SetQuantifier>,
    pub items: BoundReturn,
    pub group_by: Option<GroupBy>,
}

#[derive(Debug, Serialize)]
pub enum BoundReturn {
    Items(Vec<BoundReturnItem>),
    All,
}

#[derive(Debug, Serialize)]
pub struct BoundReturnItem {
    pub value: BoundExpr,
    pub alias: Option<Ident>,
}

#[derive(Debug, Serialize)]
pub enum BoundResultStatement {
    Return {
        statement: Box<BoundReturnStatement>,
    },
    Finish,
}

#[derive(Debug, Serialize)]
pub enum BoundCompositeQueryStatement {
    Conjunction {
        conjunction: QueryConjunction,
        left: Box<BoundCompositeQueryStatement>,
        right: Box<BoundCompositeQueryStatement>,
    },
    Primary(BoundLinearQueryStatement),
}
