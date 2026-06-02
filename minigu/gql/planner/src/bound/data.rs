use minigu_catalog::label_set::LabelSet;
use minigu_common::types::{LabelId, PropertyId};
use minigu_common::value::ScalarValue;
use serde::Serialize;
use smol_str::SmolStr;

#[derive(Debug, Clone, Serialize)]
pub enum BoundDataModifyingStatement {
    Insert(BoundInsertStatement),
}

/// An `INSERT` statement, fully resolved against the current graph's type
/// system. Vertices are inserted first in the order they appear, then edges.
/// Edges reference the inserted vertices by index into `vertices`.
#[derive(Debug, Clone, Serialize)]
pub struct BoundInsertStatement {
    pub vertices: Vec<BoundInsertVertex>,
    pub edges: Vec<BoundInsertEdge>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BoundInsertVertex {
    /// User-visible variable name (e.g., `a` in `(a:Person {...})`).
    pub variable: SmolStr,
    /// Resolved label id (single label per vertex in this iteration).
    pub label_id: LabelId,
    /// Full label set (currently always a singleton; kept for symmetry with
    /// `MemoryGraph` APIs that take a `LabelSet`).
    pub labels: LabelSet,
    /// Property values aligned with `vertex_type.properties()` order. Each
    /// entry carries the resolved property id and a literal value coerced into
    /// the property's declared logical type.
    pub properties: Vec<BoundInsertProperty>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BoundInsertEdge {
    /// Optional user-visible variable name.
    pub variable: Option<SmolStr>,
    pub label_id: LabelId,
    pub labels: LabelSet,
    pub properties: Vec<BoundInsertProperty>,
    /// Index into [`BoundInsertStatement::vertices`] for the source endpoint.
    pub src_index: usize,
    /// Index into [`BoundInsertStatement::vertices`] for the destination endpoint.
    pub dst_index: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct BoundInsertProperty {
    pub name: SmolStr,
    pub property_id: PropertyId,
    /// Resolved, type-coerced literal value.
    pub value: ScalarValue,
}
