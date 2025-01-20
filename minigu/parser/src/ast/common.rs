//! AST definitions for *common elements*.

use super::expression::Expression;
use super::lexical::Ident;
use crate::macros::{base, ext};
use crate::{Box, Vec};

#[apply(ext)]
pub enum MatchMode {
    Repeatable,
    Different,
}

#[apply(base)]
pub struct PathPattern<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub variable: Option<Ident<'a>>,
    pub prefix: PathPatternPrefix,
}

#[apply(base)]
pub enum ElementPattern<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    Node(NodePattern<'a>),
    Edge(EdgePattern<'a>),
}

#[apply(base)]
pub struct NodePattern<'a>(
    #[cfg_attr(feature = "serde", serde(borrow))] pub ElementPatternFilter<'a>,
);

#[apply(base)]
pub struct EdgePattern<'a> {
    pub kind: EdgePatternKind,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub filter: ElementPatternFilter<'a>,
}

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

#[apply(base)]
pub struct ElementPatternFilter<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub variable: Option<ElementVariableDeclaration<'a>>,
    pub label: Option<LabelExpression<'a>>,
    pub predicate: Option<ElementPatternPredicate<'a>>,
}

#[apply(base)]
pub struct ElementVariableDeclaration<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub variable: Ident<'a>,
    pub temp: bool,
}

#[apply(base)]
pub enum ElementPatternPredicate<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    Where(Expression<'a>),
    Property(Vec<PropertyKeyValuePair<'a>>),
}

#[apply(ext)]
#[derive(Default)]
pub struct PathPatternPrefix {
    pub mode: PathMode,
    pub search: PathSearch,
}

#[apply(ext)]
#[derive(Default)]
pub enum PathMode {
    #[default]
    Walk,
    Trail,
    Simple,
    Acyclic,
}

#[apply(ext)]
#[derive(Default)]
pub enum PathSearch {
    #[default]
    All,
    Any(usize),
    AllShortest,
    AnyShortest,
    Shortest(usize),
    ShortestGroup(usize),
}

#[apply(base)]
pub enum LabelExpression<'a> {
    /// Label conjunction, i.e., 'label1 & label2'.
    Conjunction(Box<LabelExpression<'a>>, Box<LabelExpression<'a>>),
    /// Label disjunction, i.e., 'label1 | label2'.
    Disjunction(Box<LabelExpression<'a>>, Box<LabelExpression<'a>>),
    /// Label negation, i.e., '!label'.
    Negation(Box<LabelExpression<'a>>),
    /// A single label.
    #[cfg_attr(feature = "serde", serde(borrow))]
    Label(Ident<'a>),
    /// Wildcard label, i.e., '%'.
    Wildcard,
}

#[apply(base)]
pub struct PropertyKeyValuePair<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub name: Ident<'a>,
    pub value: Expression<'a>,
}
