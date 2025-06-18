use std::sync::Arc;

use minigu_common::types::LabelId;
use serde::Serialize;

use crate::bound::BoundExpr;

#[derive(Debug, Clone, Serialize)]
pub enum BoundLabelExpr {
    Conjunction(Box<BoundLabelExpr>, Box<BoundLabelExpr>),
    Disjunction(Box<BoundLabelExpr>, Box<BoundLabelExpr>),
    Negation(Box<BoundLabelExpr>),
    Label(LabelId),
    Any,
}

// TODO: Add support for path search prefix
#[derive(Debug, Clone, Serialize)]
pub struct BoundPathPattern {
    pub mode: Option<BoundPathMode>,
    pub expr: BoundPathPatternExpr,
}

#[derive(Debug, Clone, Serialize)]
pub enum BoundPathPatternExpr {
    Union(Vec<BoundPathPatternExpr>),
    Alternation(Vec<BoundPathPatternExpr>),
    Concat(Vec<BoundPathPatternExpr>),
    Quantified {
        path: Box<BoundPathPatternExpr>,
        quantifier: BoundPatternQuantifier,
    },
    Optional(Box<BoundPathPatternExpr>),
    Subpath(Arc<BoundSubpathPattern>),
    Pattern(BoundElementPattern),
}

#[derive(Debug, Clone, Serialize)]
pub struct BoundPatternQuantifier {
    pub lower_bound: Option<usize>,
    pub upper_bound: Option<usize>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BoundSubpathPattern {
    pub mode: Option<BoundPathMode>,
    pub expr: BoundPathPatternExpr,
    pub predicate: Option<BoundExpr>,
}

#[derive(Debug, Clone, Serialize)]
pub enum BoundMatchMode {
    Repeatable,
    Different,
}

#[derive(Debug, Clone, Serialize)]
pub enum BoundPathMode {
    Walk,
    Trail,
    Simple,
    Acyclic,
}

#[derive(Debug, Clone, Serialize)]
pub enum BoundElementPattern {
    Vertex(Arc<BoundVertexPattern>),
    Edge(Arc<BoundEdgePattern>),
}

#[derive(Debug, Clone, Serialize)]
pub struct BoundVertexPattern {
    pub label: Option<BoundLabelExpr>,
    pub predicate: Option<BoundExpr>,
}

#[derive(Debug, Clone, Serialize)]
pub enum BoundEdgePatternKind {
    Left,
    LeftUndirected,
    LeftRight,
    Right,
    RightUndirected,
    Undirected,
    Any,
}

#[derive(Debug, Clone, Serialize)]
pub struct BoundEdgePattern {
    pub kind: BoundEdgePatternKind,
    pub label: Option<BoundLabelExpr>,
    pub predicate: Option<BoundExpr>,
}

// TODO: Add support for keep clause
#[derive(Debug, Clone, Serialize)]
pub struct BoundGraphPattern {
    pub match_mode: Option<BoundMatchMode>,
    pub paths: Vec<Arc<BoundPathPattern>>,
    pub predicate: Option<BoundExpr>,
}

// match p1 = (a)-->()-->(b), p2 = (c)-->(d) return *;
// a, b, p1, c, d, p2
// (a, b, p1), (c, d, p2)
// a: VertexRef(a)
// b: VertexRef(a)
// a: ColumnRef(0)
// b: ColumnRef(1)
// (a) --> (b)
// (a, b, p1)
// VertexScan  --> Projection
