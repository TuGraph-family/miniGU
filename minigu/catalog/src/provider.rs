use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;

use minigu_common::datatype::value::DataType;
use minigu_common::error::MiniGuError::Error;

use crate::procedure::ProcedureCatalog;
use crate::types::{
    EdgeTypeId, GraphId, Ident, LabelId, PropertyId, Result, VertexTypeId,
};

/// The top-level catalog provider, responsible for managing multiple schemas.
/// Each schema may contain graph types and actual graphs.
///
/// If failed in functions, it will return a MiniGuError::Error.
pub trait CatalogProvider: Debug + Sync + Send {
    /// Enables downcasting from trait object to concrete type.
    fn as_any(&self) -> &dyn Any;

    /// Retrieve a schema by its path.
    fn get_schema(&self, schema_path: &Vec<Ident>) -> Result<Arc<dyn SchemaProvider>> {
        Err(Error("not implemented".to_string()))
    }

    /// Create a new schema and register it in the catalog.
    fn create_schema(&self, schema: Arc<dyn SchemaProvider>) -> Result<Arc<dyn SchemaProvider>> {
        Err(Error("not implemented".to_string()))
    }

    /// Drop and remove an existing schema by its path.
    fn drop_schema(&self, schema_path: &Vec<Ident>) -> Result<Arc<dyn SchemaProvider>> {
        Err(Error("not implemented".to_string()))
    }
}

/// Represents a logical schema, which contains graphs and graph type definitions.
pub trait SchemaProvider: Debug + Sync + Send {
    fn as_any(&self) -> &dyn Any;

    fn create_graph_type(
        &self,
        graph_type: Arc<dyn GraphTypeProvider>,
    ) -> Result<Arc<dyn GraphTypeProvider>> {
        Err(Error("not implemented".to_string()))
    }

    fn get_graph_type(&self, name: &Ident) -> Result<Arc<dyn GraphTypeProvider>> {
        Err(Error("not implemented".to_string()))
    }

    fn get_graph_type_by_id(&self, id: &GraphId) -> Result<Arc<dyn GraphTypeProvider>> {
        Err(Error("not implemented".to_string()))
    }

    fn get_graph_type_id(&self, name: &Ident) -> Result<GraphId> {
        Err(Error("not implemented".to_string()))
    }

    fn drop_graph_type(&self, name: &Ident) -> Result<Arc<dyn GraphTypeProvider>> {
        Err(Error("not implemented".to_string()))
    }

    fn create_graph(&self, graph: Arc<dyn GraphProvider>) -> Result<Arc<dyn GraphProvider>> {
        Err(Error("not implemented".to_string()))
    }

    fn get_graph(&self, name: &Ident) -> Result<Arc<dyn GraphProvider>> {
        Err(Error("not implemented".to_string()))
    }

    fn get_graph_by_id(&self, id: GraphId) -> Result<Arc<dyn GraphProvider>> {
        Err(Error("not implemented".to_string()))
    }
    fn get_graph_id(&self, name: &Ident) -> Result<GraphId> {
        Err(Error("not implemented".to_string()))
    }

    fn procedure(&self, name: &Ident) -> Result<ProcedureCatalog> {
        Err(Error("not implemented".to_string()))
    }
}

/// Represents a graph, which is an instance of a graph type.
pub trait GraphProvider: Debug + Sync + Send {
    fn graph_type_ref(&self) -> Arc<dyn GraphTypeProvider>;
}

/// Represents a graph type, which defines the structure of a graph.
/// It contains vertex types and edge types.
pub trait GraphTypeProvider: Debug + Sync + Send {
    fn get_vertex_type(&self, name: &Ident) -> Result<Arc<dyn VertexTypeProvider>> {
        Err(Error("not implemented".to_string()))
    }

    fn get_vertex_type_by_id(&self, id: &VertexTypeId) -> Result<Arc<dyn VertexTypeProvider>> {
        Err(Error("not implemented".to_string()))
    }

    fn get_vertex_type_id(&self, name: &Ident) -> Result<LabelId> {
        Err(Error("not implemented".to_string()))
    }

    fn get_edge_type(&self, name: &Ident) -> Result<Arc<dyn EdgeTypeProvider>> {
        Err(Error("not implemented".to_string()))
    }
    fn get_edge_type_by_id(&self, id: &EdgeTypeId) -> Result<Arc<dyn EdgeTypeProvider>> {
        Err(Error("not implemented".to_string()))
    }

    fn get_edge_type_id(&self, name: &Ident) -> Result<LabelId> {
        Err(Error("not implemented".to_string()))
    }
}

/// Represents a property catalog, which contains properties of a vertex or edge type.
/// It provides methods to retrieve properties by name or ID.

pub trait PropertyLookupProvider: Debug + Sync + Send {
    fn get_property_by_name(&self, name: &Ident) -> Result<Arc<dyn PropertyProvider>> {
        Err(Error("not implemented".to_string()))
    }
    fn get_property_id_by_name(&self, id: &PropertyId) -> Result<Arc<dyn PropertyProvider>> {
        Err(Error("not implemented".to_string()))
    }
    fn get_property_by_id(&self, id: &PropertyId) -> Result<Arc<dyn PropertyProvider>> {
        Err(Error("not implemented".to_string()))
    }
}

/// Represents a vertex type, which defines the structure of a vertex.

pub trait VertexTypeProvider: PropertyLookupProvider {
    fn name(&self) -> Ident;
}

/// Represents an edge type, inheriting property lookup capabilities.
pub trait EdgeTypeProvider: PropertyLookupProvider {
    fn name(&self) -> Ident;
    fn src(&self) -> Arc<dyn VertexTypeProvider>;
    fn dst(&self) -> Arc<dyn VertexTypeProvider>;
}

/// Describes metadata about a property, including type and constraints.
pub trait PropertyProvider: Debug + Sync + Send {
    fn data_type(&self) -> Arc<DataType>;
    fn is_optional(&self) -> bool;
    fn is_unique(&self) -> bool;
}
