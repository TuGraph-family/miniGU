//! AST definitions for *common elements*.

use super::{Expr, Ident};
use crate::imports::{Box, Vec};
use crate::macros::{base, ext};
use crate::span::Span;

// #[apply(ext)]
// pub enum MatchMode {
//     Repeatable,
//     Different,
// }

// #[apply(base)]
// pub struct PathPattern<'a> {
//     #[cfg_attr(feature = "serde", serde(borrow))]
//     pub variable: Option<Ident<'a>>,
//     pub prefix: PathPatternPrefix,
// }

// #[apply(base)]
// pub enum ElementPattern<'a> {
//     #[cfg_attr(feature = "serde", serde(borrow))]
//     Node(NodePattern<'a>),
//     Edge(EdgePattern<'a>),
// }

// #[apply(base)]
// pub struct NodePattern<'a>(
//     pub ElementPatternFilter<'a>,
// );

// #[apply(base)]
// pub struct EdgePattern<'a> {
//     pub kind: EdgePatternKind,
//     pub filter: ElementPatternFilter<'a>,
// }

#[apply(ext)]
/// The direction of an edge pattern.
pub enum EdgePatternKind {
    /// Edge pointing left, i.e., '<-[]-' or '<-'.
    Left,
    /// Edge pointing left or undirected, i.e., '<~[]~' or '<~'.
    LeftUndirected,
    /// Edge pointing left or right, i.e., '<-[]->' or '<->'.
    LeftRight,
    /// Edge pointing right, i.e., '-[]->' or '->'.
    Right,
    /// Edge pointing right or undirected, i.e., '~[]~>' or '~>'.
    RightUndirected,
    /// Edge undirected, i.e., '~[]~' or '~'.
    Undirected,
    /// Edge with any direction, i.e., '-[]-' or '-'.
    Any,
}

// #[apply(base)]
// pub struct ElementPatternFilter<'a> {
//     #[cfg_attr(feature = "serde", serde(borrow))]
//     pub variable: Option<ElementVariableDeclaration<'a>>,
//     pub label: Option<LabelExpr<'a>>,
//     pub predicate: Option<ElementPatternPredicate<'a>>,
// }

#[apply(base)]
pub struct ElementVariableDeclaration {
    pub variable: Ident,
    pub temp: bool,
}

#[apply(base)]
pub enum ElementPatternPredicate {
    Where(Expr),
    Property(Vec<PropertyKeyValuePair>),
}

// #[apply(ext)]
// #[derive(Default)]
// pub struct PathPatternPrefix {
//     pub mode: PathMode,
//     pub search: PathSearch,
// }

// #[apply(ext)]
// #[derive(Default)]
// pub enum PathMode {
//     #[default]
//     Walk,
//     Trail,
//     Simple,
//     Acyclic,
// }

// #[apply(ext)]
// #[derive(Default)]
// pub enum PathSearch {
//     #[default]
//     All,
//     Any(usize),
//     AllShortest,
//     AnyShortest,
//     Shortest(usize),
//     ShortestGroup(usize),
// }

#[apply(base)]
pub enum LabelExpr {
    /// Label conjunction, i.e., 'label1 & label2'.
    Conjunction(Box<LabelExpr>, Box<LabelExpr>),
    /// Label disjunction, i.e., 'label1 | label2'.
    Disjunction(Box<LabelExpr>, Box<LabelExpr>),
    /// Label negation, i.e., '!label'.
    Negation(Box<LabelExpr>),
    /// A single label.
    Label(Ident),
    /// Wildcard label, i.e., '%'.
    Wildcard,
}

#[apply(base)]
pub struct PropertyKeyValuePair {
    pub name: Ident,
    pub value: Expr,
    pub span: Span,
}

#[apply(base)]
pub struct Yield {
    pub items: Vec<YieldItem>,
    pub span: Span,
}

#[apply(base)]
pub struct YieldItem {
    pub name: Ident,
    pub alias: Option<Ident>,
    pub span: Span,
}
