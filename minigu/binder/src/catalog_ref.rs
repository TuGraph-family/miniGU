use minigu_catalog::provider::{EdgeTypeRef, GraphRef, GraphTypeRef, PropertyRef, SchemaRef, VertexTypeRef};
use serde::Serialize;

pub type Ident=smol_str::SmolStr;

/// A catalog wrapper for a schema, containing its name and internal reference.
#[derive(Debug, Serialize)]
pub struct SchemaCatalog {
    /// The identifier (name) of the schema.
    pub name: Ident,
    /// Internal reference to the actual schema object (not serialized).
    #[serde(skip_serializing)]
    pub obj_ref: SchemaRef,
}

/// A catalog wrapper for a graph type, containing its name and internal reference.
#[derive(Debug, Serialize)]
pub struct GraphTypeCatalog {
    /// The identifier (name) of the graph type.
    pub name: Ident,
    /// Internal reference to the actual graph type object (not serialized).
    #[serde(skip_serializing)]
    pub obj_ref: GraphTypeRef,
}

/// A catalog wrapper for a graph instance, containing its name and internal reference.
#[derive(Debug, Serialize)]
pub struct GraphCatalog {
    /// The identifier (name) of the graph.
    pub name: Ident,
    /// Internal reference to the actual graph object (not serialized).
    #[serde(skip_serializing)]
    pub obj_ref: GraphRef,
}

/// A catalog wrapper for a vertex type, containing its name and internal reference.
#[derive(Debug, Serialize)]
pub struct VertexTypeCatalog {
    /// The identifier (name) of the vertex type.
    name: Ident,
    /// Internal reference to the actual vertex type object (not serialized).
    #[serde(skip_serializing)]
    obj_ref: VertexTypeRef,
}

/// A catalog wrapper for an edge type, containing its name and internal reference.
#[derive(Debug, Serialize)]
pub struct EdgeTypeCatalog {
    /// The identifier (name) of the edge type.
    name: Ident,
    /// Internal reference to the actual edge type object (not serialized).
    #[serde(skip_serializing)]
    obj_ref: EdgeTypeRef,
}

/// A catalog wrapper for a property, containing its name and internal reference.
#[derive(Debug, Serialize)]
pub struct PropertyCatalog {
    /// The identifier (name) of the property.
    name: Ident,
    /// Internal reference to the actual property object (not serialized).
    #[serde(skip_serializing)]
    obj_ref: PropertyRef,
}

