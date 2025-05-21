use gql_parser::ast::{
    GroupBy, Ident, MatchMode, PathPatternPrefix, Procedure,
    QueryConjunction, SetQuantifier,
};
use macro_rules_attribute::apply;

use crate::macros::base;
use crate::program::bound_statement::common::{BoundPathPattern, BoundProcedure};
use crate::program::bound_statement::expr::{BoundExpr, BoundGraphExpr};
use crate::program::bound_statement::procedure::BoundCallProcedureStatement;

#[apply(base)]
pub enum BoundLinearQueryStatement {
    Focused(BoundFocusedLinearQueryStatement),
    Ambient(BoundAmbientLinearQueryStatement),
}

#[apply(base)]
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

#[apply(base)]
pub struct BoundFocusedLinearQueryStatementPart {
    // TODO: replace expr to graph id.
    pub use_graph: BoundGraphExpr,
    pub statements: Vec<BoundSimpleQueryStatement>,
}

#[apply(base)]
pub enum BoundAmbientLinearQueryStatement {
    Parts {
        parts: Vec<BoundSimpleQueryStatement>,
        result: BoundResultStatement,
    },
    Nested(Box<Procedure>),
}

#[apply(base)]
pub enum BoundSimpleQueryStatement {
    Match(BoundMatchStatement),
    Call(BoundCallProcedureStatement),
}

#[apply(base)]
pub enum BoundMatchStatement {
    Simple(BoundGraphPatternBindingTable),
    Optional(Vec<BoundMatchStatement>),
}

#[apply(base)]
pub struct BoundGraphPatternBindingTable {
    pub pattern: BoundGraphPattern,
    // TODO: Handle yield_item.
    pub yield_item: Vec<Ident>,
}

#[apply(base)]
pub struct BoundGraphPattern {
    pub match_mode: Option<MatchMode>,
    pub patterns: Vec<BoundPathPattern>,
    pub keep: Option<PathPatternPrefix>,
    pub where_clause: Option<BoundExpr>,
}

#[apply(base)]
pub struct BoundReturnStatement {
    pub quantifier: Option<SetQuantifier>,
    pub items: BoundReturn,
    pub group_by: Option<GroupBy>,
}

#[apply(base)]
pub enum BoundReturn {
    Items(Vec<BoundReturnItem>),
    All,
}

#[apply(base)]
pub struct BoundReturnItem {
    pub value: BoundExpr,
    pub alias: Option<Ident>,
}

#[apply(base)]
pub enum BoundResultStatement {
    Return {
        statement: Box<BoundReturnStatement>,
    },
    Finish,
}

#[apply(base)]
pub enum BoundCompositeQueryStatement {
    Conjunction {
        conjunction: QueryConjunction,
        left: Box<BoundCompositeQueryStatement>,
        right: Box<BoundCompositeQueryStatement>,
    },
    Primary(BoundLinearQueryStatement),
}
