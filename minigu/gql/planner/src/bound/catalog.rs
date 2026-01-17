use minigu_catalog::named_ref::NamedGraphRef;
use minigu_common::types::{VectorIndexKey, VectorMetric};
use serde::Serialize;
use smol_str::SmolStr;

use super::object_ref::BoundGraphType;
use super::procedure_call::BoundCallProcedureStatement;

#[derive(Debug, Clone, Serialize)]
pub enum BoundCatalogModifyingStatement {
    Call(BoundCallProcedureStatement),
    CreateSchema(BoundCreateSchemaStatement),
    DropSchema(BoundDropSchemaStatement),
    CreateGraph(BoundCreateGraphStatement),
    CreateVectorIndex(BoundCreateVectorIndexStatement),
    DropVectorIndex(BoundDropVectorIndexStatement),
    DropGraph(BoundDropGraphStatement),
    CreateGraphType(BoundCreateGraphTypeStatement),
    DropGraphType(BoundDropGraphTypeStatement),
}

#[derive(Debug, Clone, Serialize)]
pub struct BoundCreateSchemaStatement {
    pub schema_path: Vec<SmolStr>,
    pub if_not_exists: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct BoundDropSchemaStatement {
    pub schema_path: Vec<SmolStr>,
    pub if_exists: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct BoundCreateGraphStatement {
    // pub schema: SchemaRef,
    pub name: SmolStr,
    pub kind: CreateKind,
    pub graph_type: BoundGraphType,
    pub source: Option<NamedGraphRef>,
}

#[derive(Debug, Clone, Serialize)]
pub enum CreateKind {
    Create,
    CreateIfNotExists,
    CreateOrReplace,
}

#[derive(Debug, Clone, Serialize)]
pub struct BoundDropGraphStatement {
    // pub schema: NamedSchemaRef,
    pub name: SmolStr,
    pub if_exists: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct BoundCreateGraphTypeStatement {
    // pub schema: NamedSchemaRef,
    pub name: SmolStr,
    pub kind: CreateKind,
    pub source: BoundGraphType,
}

#[derive(Debug, Clone, Serialize)]
pub struct BoundDropGraphTypeStatement {
    //  pub schema: NamedSchemaRef,
    pub name: SmolStr,
    pub if_exists: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct BoundCreateVectorIndexStatement {
    pub name: SmolStr,
    pub if_not_exists: bool,
    pub index_key: VectorIndexKey,
    pub metric: VectorMetric,
    pub dimension: usize,
    pub label: SmolStr,
    pub property: SmolStr,
    /// If true, planner/executor should treat this statement as a no-op (because IF NOT EXISTS was
    /// specified and the index already exists).
    pub no_op: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct BoundDropVectorIndexStatement {
    pub name: SmolStr,
    pub if_exists: bool,
    pub index_key: Option<VectorIndexKey>,
    pub metadata: Option<minigu_catalog::provider::VectorIndexCatalogEntry>,
    pub no_op: bool,
}
