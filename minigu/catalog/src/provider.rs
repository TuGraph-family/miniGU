use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;

use minigu_common::data_type::{DataSchemaRef, LogicalType};
use minigu_common::types::{LabelId, PropertyId};

use crate::error::CatalogResult;
use crate::label_set::LabelSet;
use crate::property::Property;

pub type CatalogRef = Arc<dyn CatalogProvider>;
pub type DirectoryRef = Arc<dyn DirectoryProvider>;
pub type SchemaRef = Arc<dyn SchemaProvider>;
pub type GraphRef = Arc<dyn GraphProvider>;
pub type GraphTypeRef = Arc<dyn GraphTypeProvider>;
pub type VertexTypeRef = Arc<dyn VertexTypeProvider>;
pub type EdgeTypeRef = Arc<dyn EdgeTypeProvider>;
pub type ProcedureRef = Arc<dyn ProcedureProvider>;

/// The top-level catalog provider, responsible for managing multiple directories and schemas,
/// resembling a UNIX filesystem.
pub trait CatalogProvider: Debug + Send + Sync {
    /// Retrieves the root directory or schema of the catalog.
    fn get_root(&self) -> CatalogResult<DirectoryOrSchema>;
}

pub trait DirectoryProvider: Debug + Send + Sync {
    /// Returns the parent directory ID of the directory.
    fn parent(&self) -> Option<DirectoryRef>;

    /// Retrieves a child directory or schema by its name.
    fn get_child(&self, name: &str) -> CatalogResult<Option<DirectoryOrSchema>>;

    /// Returns the names of the children of the directory.
    fn children_names(&self) -> Vec<String>;
}

/// Represents a logical schema, which contains graphs and graph type definitions.
pub trait SchemaProvider: Debug + Send + Sync {
    /// Returns the parent directory ID of the schema.
    fn parent(&self) -> Option<DirectoryRef>;

    /// Returns the names of the graphs in the schema.
    fn graph_names(&self) -> Vec<String>;

    /// Retrieves a graph by its name.
    fn get_graph(&self, name: &str) -> CatalogResult<Option<GraphRef>>;

    /// Returns the names of the graph types in the schema.
    fn graph_type_names(&self) -> Vec<String>;

    /// Retrieves a graph type by its name.
    fn get_graph_type(&self, name: &str) -> CatalogResult<Option<GraphTypeRef>>;

    /// Returns the names of the procedures in the schema.
    fn procedure_names(&self) -> Vec<String>;

    /// Retrieves a procedure by its name.
    fn get_procedure(&self, name: &str) -> CatalogResult<Option<ProcedureRef>>;
}

/// Represents a graph, which is an instance of a graph type.
///
/// Use [`Arc::downcast`] to cast the trait object into the concrete type.
pub trait GraphProvider: Debug + Send + Sync + Any {
    /// Returns the graph type of the graph.
    fn graph_type(&self) -> GraphTypeRef;
}

/// Represents a graph type, which defines the structure of a graph.
/// It contains vertex types and edge types.
pub trait GraphTypeProvider: Debug + Send + Sync {
    /// Retrieves the ID of a label by its name.
    fn get_label_id(&self, name: &str) -> CatalogResult<Option<LabelId>>;

    /// Returns the names of the labels in the graph type.
    fn label_names(&self) -> Vec<String>;

    /// Retrieves a vertex type by its key label set.
    fn get_vertex_type(&self, key: &LabelSet) -> CatalogResult<Option<VertexTypeRef>>;

    /// Returns the keys of the vertex types in the graph type.
    fn vertex_type_keys(&self) -> Vec<LabelSet>;

    /// Retrieves an edge type by its key label set.
    fn get_edge_type(&self, key: &LabelSet) -> CatalogResult<Option<EdgeTypeRef>>;

    /// Returns the keys of the edge types in the graph type.
    fn edge_type_keys(&self) -> Vec<LabelSet>;
}

/// Represents a vertex type, which defines the structure of a vertex.
pub trait VertexTypeProvider: Debug + Send + Sync + PropertiesProvider {
    /// Returns the label set of the vertex type.
    fn label_set(&self) -> LabelSet;
}

/// Represents an edge type, which defines the structure of an edge.
pub trait EdgeTypeProvider: Debug + Send + Sync + PropertiesProvider {
    /// Returns the label set of the edge type.
    fn label_set(&self) -> LabelSet;

    /// Returns the source vertex type of the edge type.
    fn src(&self) -> VertexTypeRef;

    /// Returns the destination vertex type of the edge type.
    fn dst(&self) -> VertexTypeRef;
}

/// Represents a property set, which contains properties of a vertex or edge type.
pub trait PropertiesProvider: Debug + Send + Sync {
    /// Retrieves a property by its name.
    fn get_property(&self, name: &str) -> CatalogResult<Option<(PropertyId, &Property)>>;

    /// Returns the properties of the property set.
    fn properties(&self) -> Vec<(PropertyId, Property)>;
}

pub trait ProcedureProvider: Debug + Send + Sync + Any {
    /// Returns the parameters of the procedure.
    fn parameters(&self) -> &[LogicalType];

    /// Returns the data schema of the procedure.
    fn schema(&self) -> Option<DataSchemaRef>;
}

#[derive(Debug, Clone)]
pub enum DirectoryOrSchema {
    Directory(DirectoryRef),
    Schema(SchemaRef),
}

impl DirectoryOrSchema {
    #[inline]
    pub fn is_directory(&self) -> bool {
        matches!(self, Self::Directory(_))
    }

    #[inline]
    pub fn is_schema(&self) -> bool {
        matches!(self, Self::Schema(_))
    }

    #[inline]
    pub fn as_directory(&self) -> Option<&DirectoryRef> {
        match self {
            Self::Directory(dir) => Some(dir),
            Self::Schema(_) => None,
        }
    }

    #[inline]
    pub fn as_schema(&self) -> Option<&SchemaRef> {
        match self {
            Self::Directory(_) => None,
            Self::Schema(schema) => Some(schema),
        }
    }
}

impl From<DirectoryRef> for DirectoryOrSchema {
    #[inline]
    fn from(value: DirectoryRef) -> Self {
        Self::Directory(value)
    }
}

impl From<SchemaRef> for DirectoryOrSchema {
    #[inline]
    fn from(value: SchemaRef) -> Self {
        Self::Schema(value)
    }
}
