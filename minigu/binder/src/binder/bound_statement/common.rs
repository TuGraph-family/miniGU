use gql_parser::ast::{
    BindingVariableDefBlock, EdgePatternKind, Ident
    , Yield,
};
use gql_parser::span::OptSpanned;
use macro_rules_attribute::apply;
use minigu_catalog::provider::SchemaRef;
use crate::macros::base;
use crate::program::bound_statement::catalog::LinearBoundCatalogModifyingStatement;
use crate::program::bound_statement::data::BoundLinearDataModifyingStatement;
use crate::program::bound_statement::expr::{BoundExpr, BoundPathPatternExpr};
use crate::program::bound_statement::object_ref::BoundSchemaRef;
use crate::program::bound_statement::query::BoundCompositeQueryStatement;


pub type LabelId = u32;
pub type FieldId = u32;
pub type ProcedureId = u32;
pub type GraphTypeId = u32;
pub type GraphId = u32;

#[apply(base)]
pub struct BoundElementPatternFiller {
    pub variable: Option<Ident>,
    pub label: Option<BoundLabelExpr>,
    pub predicate: Option<BoundElementPatternPredicate>,
}

#[apply(base)]
pub enum BoundElementPatternPredicate {
    Where(BoundExpr),
    Property(Vec<BoundFieldOrProperty>),
}

#[apply(base)]
pub struct BoundFieldOrProperty {
    pub id: FieldId,
    pub value: BoundExpr,
}


// Currently only support Label.
#[apply(base)]
pub enum BoundLabelExpr {
    /// Label conjunction, i.e., 'label1 & label2'.
    Conjunction(Box<BoundLabelExpr>, Box<BoundLabelExpr>),
    /// Label disjunction, i.e., 'label1 | label2'.
    Disjunction(Box<BoundLabelExpr>, Box<BoundLabelExpr>),
    /// Label negation, i.e., '!label'.
    Negation(Box<BoundLabelExpr>),
    /// A single label.
    Label(LabelId),
    /// Wildcard label, i.e., '%'.
    Wildcard,
}

#[apply(base)]
pub enum ElementPatternPredicate {
    Where(BoundExpr),
    Property(Vec<BoundFieldOrProperty>),
}

#[apply(base)]
pub enum BoundElementPattern {
    Node(BoundElementPatternFiller),
    Edge {
        kind: EdgePatternKind,
        filler: BoundElementPatternFiller,
    },
}

#[apply(base)]
pub struct BoundPathPattern {
    pub variable: Option<Ident>,
    pub prefix: Option<BoundPathPatternPrefix>,
    pub expr: BoundPathPatternExpr,
}

#[apply(base)]
pub enum BoundPathPatternPrefix {}
