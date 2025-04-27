use gql_parser::ast::{GraphElementType, Ident, SchemaPath, SchemaPathSegment};
use macro_rules_attribute::apply;
use minigu_catalog::schema::SchemaId;
use crate::macros::base;
use crate::program::bound_statement::expr::BoundGraphExpr;

// Standard schema path, without relative paths and variables.
#[apply(base)]
pub struct CanonicalSchemaPath {
    pub segments: Vec<Ident>,
}

#[apply(base)]
pub enum BoundSchemaRef {
    // Schema Path or Ref will be bounded to schema id.
    SchemaId(SchemaId),
    UnResolvedParameter(Ident),
}

#[apply(base)]
pub struct BoundCatalogObjectRef {
    pub schema: SchemaId,
    // Parsed ID
    pub object: Vec<u32>,
}

#[apply(base)]
pub struct BoundCatalogObjectStrRef {
    pub schema: CanonicalSchemaPath,
    // For some creation operations, where the corresponding ID cannot be
    // obtained immediately, the corresponding character name is retained here.
    pub object: Vec<Ident>,
}

#[apply(base)]
pub enum BoundProcedureRef {
    Ref(BoundCatalogObjectRef),
    Parameter(Ident),
}

#[apply(base)]
pub struct BoundGraphRef {
    pub path: BoundCatalogObjectRef,
}

#[apply(base)]
pub enum BoundGraphTypeRef {
    Ref(BoundCatalogObjectRef),
    Parameter(Ident),
}

#[apply(base)]
pub enum BoundOfGraphType {
    Like(BoundGraphExpr),
    Ref(BoundGraphTypeRef),
    Nested(Vec<GraphElementType>),
    Any,
}

#[apply(base)]
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

impl BoundSchemaRef {
    pub fn as_canonical(&self) -> Option<&CanonicalSchemaPath> {
        match self {
            BoundSchemaRef::Canonical(path) => Some(path),
            // TODO: Handle Parameter.
            _ => None,
        }
    }
}
