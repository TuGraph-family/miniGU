use gql_parser::ast::{
    BindingVariableDefBlock, EdgePatternKind, Ident
    , Yield,
};
use gql_parser::span::OptSpanned;
use macro_rules_attribute::apply;
use serde::Serialize;
use minigu_catalog::provider::{PropertyRef, SchemaRef};
use minigu_common::types::LabelId;
use crate::bound_statement::expr::{BoundExpr, BoundPathPatternExpr};
use crate::catalog_ref::PropertyCatalogRef;
use crate::macros::base;

#[derive(Debug, Serialize)]
pub struct BoundElementPatternFiller {
    pub variable: Option<Ident>,
    pub label: Option<BoundLabelExpr>,
    pub predicate: Option<BoundElementPatternPredicate>,
}

#[derive(Debug, Serialize)]
pub enum BoundElementPatternPredicate {
    Where(BoundExpr),
    Property(Vec<BoundFieldOrProperty>),
}

#[derive(Debug, Serialize)]
pub struct BoundFieldOrProperty {
    pub id: PropertyCatalogRef,
    pub value: BoundExpr,
}


// Currently only support Label.
#[derive(Debug, Serialize)]
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
#[derive(Debug, Serialize)]
pub enum BoundElementPattern {
    Node(BoundElementPatternFiller),
    Edge {
        kind: EdgePatternKind,
        filler: BoundElementPatternFiller,
    },
}

#[derive(Debug, Serialize)]
pub struct BoundPathPattern {
    // Resolver implemented : resolve_path_pattern
    pub variable: Option<Ident>,
    pub prefix: Option<BoundPathPatternPrefix>,
    pub expr: BoundPathPatternExpr,
}

#[derive(Debug, Serialize)]
pub enum BoundPathPatternPrefix {}
