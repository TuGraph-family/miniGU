use minigu_catalog::provider::{
    EdgeTypeRef, GraphRef, GraphTypeRef, ProcedureRef, PropertyRef, SchemaRef, VertexTypeRef,
};
use serde::Serialize;

pub type Ident = smol_str::SmolStr;

/// When constructing a `BoundStatement`, all metadata access methods must be wrapped into a unified
/// struct,
///
/// Throughout the binding phases, all metadata (including directories, schemas, and graph types)
/// should be accessed exclusively via provider or reference interfaces, avoiding direct dependence
/// on concrete structs. This ensures abstraction consistency and improves modularity and
/// testability.

/// A catalog wrapper for a schema, containing its name and internal reference.
#[derive(Debug, Serialize)]
pub struct SchemaCatalogRef {
    /// The identifier (name) of the schema.
    pub name: Ident,
    /// Internal reference to the actual schema object (not serialized).
    #[serde(skip_serializing)]
    pub schema_ref: SchemaRef,
}

/// A catalog wrapper for a graph type, containing its name and internal reference.
#[derive(Debug, Serialize)]
pub struct GraphTypeCatalogRef {
    /// The identifier (name) of the graph type.
    pub name: Ident,
    /// Internal reference to the actual graph type object (not serialized).
    #[serde(skip_serializing)]
    pub graph_type_ref: GraphTypeRef,
}

/// A catalog wrapper for a graph instance, containing its name and internal reference.
#[derive(Debug, Serialize)]
pub struct GraphCatalogRef {
    /// The identifier (name) of the graph.
    pub name: Ident,
    /// Internal reference to the actual graph object (not serialized).
    #[serde(skip_serializing)]
    pub graph_ref: GraphRef,
}

/// A catalog wrapper for a vertex type, containing its name and internal reference.
#[derive(Debug, Serialize)]
pub struct VertexTypeCatalogRef {
    /// The identifier (name) of the vertex type.
    pub name: Ident,
    /// Internal reference to the actual vertex type object (not serialized).
    #[serde(skip_serializing)]
    pub vertex_type_ref: VertexTypeRef,
}

/// A catalog wrapper for an edge type, containing its name and internal reference.
#[derive(Debug, Serialize)]
pub struct EdgeTypeCatalogRef {
    /// The identifier (name) of the edge type.
    pub name: Ident,
    /// Internal reference to the actual edge type object (not serialized).
    #[serde(skip_serializing)]
    pub edge_type_ref: EdgeTypeRef,
}

/// A catalog wrapper for a property, containing its name and internal reference.
#[derive(Debug, Serialize)]
pub struct PropertyCatalogRef {
    /// The identifier (name) of the property.
    pub name: Ident,
    /// Internal reference to the actual property object (not serialized).
    #[serde(skip_serializing)]
    pub property_ref: PropertyRef,
}

#[derive(Debug, Serialize)]
pub struct CallProcedureCatalogRef {
    pub name: Ident,
    // TODO: Add CallProcedureCatalog
    #[serde(skip_serializing)]
    pub procedure_ref: ProcedureRef,
}
