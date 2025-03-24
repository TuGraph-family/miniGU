//! AST definitions for *common elements*.

use super::{Expr, FieldOrProperty, Ident, NonNegativeInteger, UnsignedInteger};
use crate::macros::base;
use crate::span::{BoxSpanned, OptSpanned, Spanned, VecSpanned};

#[apply(base)]
pub enum LabelExpr {
    /// Label conjunction, i.e., 'label1 & label2'.
    Conjunction(BoxSpanned<LabelExpr>, BoxSpanned<LabelExpr>),
    /// Label disjunction, i.e., 'label1 | label2'.
    Disjunction(BoxSpanned<LabelExpr>, BoxSpanned<LabelExpr>),
    /// Label negation, i.e., '!label'.
    Negation(BoxSpanned<LabelExpr>),
    /// A single label.
    Label(Ident),
    /// Wildcard label, i.e., '%'.
    Wildcard,
}

#[apply(base)]
pub struct PropertyKeyValuePair {
    pub name: Ident,
    pub value: Expr,
}

pub type Yield = VecSpanned<YieldItem>;

#[apply(base)]
pub struct YieldItem {
    pub name: Spanned<Ident>,
    pub alias: OptSpanned<Ident>,
}

#[apply(base)]
pub enum PathMode {
    Walk,
    Trail,
    Simple,
    Acyclic,
}

#[apply(base)]
pub enum PathSearchMode {
    All(OptSpanned<PathMode>),
    Any {
        number: OptSpanned<NonNegativeInteger>,
        mode: OptSpanned<PathMode>,
    },
    AllShortest(OptSpanned<PathMode>),
    AnyShortest(OptSpanned<PathMode>),
    CountedShortest {
        number: Spanned<NonNegativeInteger>,
        mode: OptSpanned<PathMode>,
    },
    CountedShortestGroup {
        number: OptSpanned<NonNegativeInteger>,
        mode: OptSpanned<PathMode>,
    },
}

#[apply(base)]
pub enum PathPatternPrefix {
    PathMode(PathMode),
    PathSearch(PathSearchMode),
}

#[apply(base)]
pub struct PathPattern {
    pub variable: OptSpanned<Ident>,
    pub prefix: OptSpanned<PathPatternPrefix>,
    pub expr: Spanned<PathPatternExpr>,
}

#[apply(base)]
pub struct GroupedPathPattern {
    pub variable: OptSpanned<Ident>,
    pub mode: OptSpanned<PathMode>,
    pub expr: BoxSpanned<PathPatternExpr>,
    pub where_clause: OptSpanned<Expr>,
}

// TODO: Add definition for simplified path pattern expression.
#[apply(base)]
pub enum PathPatternExpr {
    Union(VecSpanned<PathPatternExpr>),
    Alternation(VecSpanned<PathPatternExpr>),
    Concat(VecSpanned<PathPatternExpr>),
    Quantified {
        path: BoxSpanned<PathPatternExpr>,
        quantifier: Spanned<PatternQuantifier>,
    },
    Optional(BoxSpanned<PathPatternExpr>),
    Grouped(GroupedPathPattern),
    Pattern(ElementPattern),
}

#[apply(base)]
pub enum PatternQuantifier {
    Asterisk,
    Plus,
    Fixed(Spanned<UnsignedInteger>),
    General {
        lower_bound: OptSpanned<UnsignedInteger>,
        upper_bound: OptSpanned<UnsignedInteger>,
    },
}

#[apply(base)]
pub enum ElementPattern {
    Node(ElementPatternFiller),
    Edge {
        kind: EdgePatternKind,
        filler: ElementPatternFiller,
    },
}

/// The direction of an edge pattern.
#[apply(base)]
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
pub struct ElementPatternFiller {
    pub variable: OptSpanned<Ident>,
    pub label: OptSpanned<LabelExpr>,
    pub predicate: OptSpanned<ElementPatternPredicate>,
}

#[apply(base)]
pub enum ElementPatternPredicate {
    Where(Spanned<Expr>),
    Property(VecSpanned<FieldOrProperty>),
}

#[apply(base)]
pub enum MatchMode {
    Repeatable,
    Different,
}

#[apply(base)]
pub struct GraphPattern {
    pub match_mode: OptSpanned<MatchMode>,
    pub patterns: VecSpanned<PathPattern>,
    pub keep: OptSpanned<PathPatternPrefix>,
    pub where_clause: OptSpanned<Expr>,
}

#[apply(base)]
pub struct GraphPatternBindingTable {
    pub pattern: Spanned<GraphPattern>,
    pub yield_clause: VecSpanned<Ident>,
}
