use gql_parser::ast::{GraphElementType, Ident, SchemaPath, SchemaPathSegment};
use macro_rules_attribute::apply;
use serde::Serialize;
use minigu_catalog::types::SchemaId;
use crate::bound_statement::catalog::BoundGraphTypeRef;
use crate::bound_statement::expr::BoundGraphExpr;
use crate::catalog_ref::{CallProcedureCatalogRef, GraphTypeCatalogRef};
use crate::macros::base;


// Standard schema path, without relative paths and variables.
#[apply(base)]
pub struct CanonicalSchemaPath {
    pub segments: Vec<Ident>,
}


#[derive(Debug, Serialize)]
pub enum BoundProcedureRef {
    // Catalog ref or just parameter.
    Ref(CallProcedureCatalogRef),
    Parameter(Ident),
}
#[derive(Debug, Serialize)]
pub enum BoundOfGraphType {
    Like(BoundGraphExpr),
    Ref(BoundGraphTypeRef),
    Nested(Vec<GraphElementType>),
    Any,
}

#[derive(Debug, Serialize)]
pub enum BoundGraphTypeSource {
    Copy(BoundGraphTypeRef),
    Like(BoundGraphExpr),
    Nested(Vec<GraphElementType>),
}

#[apply(base)]
pub enum CatalogObjRefType {
    // To identify the category to which the ID in the catalog object belongs
    GraphRef,
    ProcedureRef,
    GraphTypeRef,
    BindingTableRef,
}

impl CanonicalSchemaPath {
    pub fn to_string(&self) -> String {
        format!("/{}", self.segments.join("/"))
    }

    pub fn default() -> Self {
        CanonicalSchemaPath { segments: vec![] }
    }
}

pub fn normalize_path(path: &SchemaPath) -> CanonicalSchemaPath {
    let mut stack = Vec::new();
    for segment in path.iter() {
        match &segment.value() {
            SchemaPathSegment::Name(ident) => {
                stack.push(ident.clone());
            }
            // If empty, ignore it. This indicates that the root directory was reached by ascending
            // via "..".
            SchemaPathSegment::Parent => {
                stack.pop();
            }
        }
    }
    CanonicalSchemaPath { segments: stack }
}