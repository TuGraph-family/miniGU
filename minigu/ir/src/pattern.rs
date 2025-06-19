use serde::{Deserialize, Serialize};

use crate::bound::BoundLabelExpr;

pub type PatternId = u32;

// pub enum PathPatternExpr {
//     Union(VecSpanned<PathPatternExpr>),
//     Alternation(VecSpanned<PathPatternExpr>),
//     Concat(VecSpanned<PathPatternExpr>),
//     Quantified {
//         path: BoxSpanned<PathPatternExpr>,
//         quantifier: Spanned<PatternQuantifier>,
//     },
//     Optional(BoxSpanned<PathPatternExpr>),
//     Grouped(GroupedPathPattern),
//     Pattern(ElementPattern),
// }

pub enum PathPattern {
    Union(Vec<PathPattern>),
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct PatternGraph {
    vertices: Vec<PatternVertex>,
    edges: Vec<PatternEdge>,
}

#[derive(Debug, Default, Clone)]
pub struct PatternGraphBuilder {
    vertices: Vec<PatternVertex>,
    edges: Vec<PatternEdge>,
}

impl PatternGraphBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_vertex(&mut self, vertex: PatternVertex) -> &mut Self {
        self.vertices.push(vertex);
        self
    }

    pub fn add_edge(&mut self, edge: PatternEdge) -> &mut Self {
        self.edges.push(edge);
        self
    }

    pub fn build(self) -> PatternGraph {
        PatternGraph {
            vertices: self.vertices,
            edges: self.edges,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PatternVertex {
    id: PatternId,
    label: Option<BoundLabelExpr>,
}

impl PatternVertex {
    pub fn new(id: PatternId, label: Option<BoundLabelExpr>) -> Self {
        Self { id, label }
    }

    pub fn id(&self) -> PatternId {
        self.id
    }

    pub fn label(&self) -> Option<&BoundLabelExpr> {
        self.label.as_ref()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PatternEdge {
    id: PatternId,
    ty: PatternEdgeType,
    label: Option<BoundLabelExpr>,
    src: usize,
    dst: usize,
}

impl PatternEdge {
    pub fn new(
        id: PatternId,
        ty: PatternEdgeType,
        label: Option<BoundLabelExpr>,
        src: usize,
        dst: usize,
    ) -> Self {
        Self {
            id,
            ty,
            label,
            src,
            dst,
        }
    }

    pub fn id(&self) -> PatternId {
        self.id
    }

    pub fn ty(&self) -> PatternEdgeType {
        self.ty
    }

    pub fn label(&self) -> Option<&BoundLabelExpr> {
        self.label.as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatternEdgeType {
    Directed,
    Undirected,
    Any,
}
