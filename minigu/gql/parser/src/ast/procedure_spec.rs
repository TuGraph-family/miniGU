//! AST definitions for *procedure specification*.

use super::{
    BindingTableVariableDef, CompositeQueryStatement, GraphVariableDef,
    LinearCatalogModifyingStatement, LinearDataModifyingStatement, SchemaRef, ValueVariableDef,
    Yield,
};
use crate::macros::base;
use crate::span::{OptSpanned, Spanned, VecSpanned};

#[apply(base)]
pub struct Procedure {
    pub at: OptSpanned<SchemaRef>,
    pub binding_variable_defs: BindingVariableDefBlock,
    pub statement: Spanned<Statement>,
    pub next_statements: VecSpanned<NextStatement>,
}

#[apply(base)]
pub enum Statement {
    Catalog(LinearCatalogModifyingStatement),
    Query(CompositeQueryStatement),
    Data(LinearDataModifyingStatement),
}

#[apply(base)]
pub struct NextStatement {
    pub yield_clause: OptSpanned<Yield>,
    pub statement: Spanned<Statement>,
}

pub type BindingVariableDefBlock = VecSpanned<BindingVariableDef>;

#[apply(base)]
pub enum BindingVariableDef {
    Graph(GraphVariableDef),
    BindingTable(BindingTableVariableDef),
    Value(ValueVariableDef),
}
