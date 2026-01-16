use std::collections::HashMap;
use std::sync::Arc;

use minigu_catalog::label_set::LabelSet;
use minigu_catalog::memory::graph_type::{
    MemoryEdgeTypeCatalog, MemoryGraphTypeCatalog, MemoryVertexTypeCatalog,
};
use minigu_catalog::memory::schema::MemorySchemaCatalog;
use minigu_catalog::provider::{GraphTypeProvider, SchemaProvider, VertexTypeRef};
use minigu_catalog::property::Property;
use minigu_common::data_type::DataField;
use minigu_common::types::LabelId;
use minigu_context::graph::{GraphContainer, GraphStorage};
use minigu_context::session::SessionContext;
use minigu_planner::bound::{
    BoundEdgeType, BoundGraphElementType, BoundVertexType, CreateKind, NodeTypeRef,
};
use minigu_planner::plan::catalog_modify::{CreateGraph, DropGraph};
use minigu_storage::tp::MemoryGraph;

use super::{Executor, IntoExecutor};
use crate::error::ExecutionResult;

/// Helper function to create execution error
fn execution_error(msg: impl Into<String>) -> crate::error::ExecutionError {
    crate::error::ExecutionError::Custom(msg.into().into())
}

/// Builder for CREATE GRAPH executor
pub struct CreateGraphBuilder {
    plan: CreateGraph,
    session: SessionContext,
}

impl CreateGraphBuilder {
    pub fn new(plan: CreateGraph, session: SessionContext) -> Self {
        Self { plan, session }
    }
}

impl IntoExecutor for CreateGraphBuilder {
    type IntoExecutor = impl Executor;

    fn into_executor(self) -> Self::IntoExecutor {
        gen move {
            let CreateGraphBuilder { plan, session } = self;

            let _result = create_graph_impl(&plan, &session);

            // Return empty result for DDL operations
            // yield Ok(DataChunk::empty())
        }
        .into_executor()
    }
}

/// Builder for DROP GRAPH executor
pub struct DropGraphBuilder {
    plan: DropGraph,
    session: SessionContext,
}

impl DropGraphBuilder {
    pub fn new(plan: DropGraph, session: SessionContext) -> Self {
        Self { plan, session }
    }
}

impl IntoExecutor for DropGraphBuilder {
    type IntoExecutor = impl Executor;

    fn into_executor(self) -> Self::IntoExecutor {
        gen move {
            let DropGraphBuilder { plan, session } = self;

            let _result = drop_graph_impl(&plan, &session);

            // Return empty result for DDL operations
            // yield Ok(DataChunk::empty())
        }
        .into_executor()
    }
}

/// Implementation for CREATE GRAPH
fn create_graph_impl(plan: &CreateGraph, session: &SessionContext) -> ExecutionResult<()> {
    let schema_catalog = session
        .current_schema
        .as_ref()
        .ok_or_else(|| execution_error("No current schema set"))?;

    let graph_exists = schema_catalog
        .get_graph(&plan.name)
        .map_err(|e| execution_error(format!("Failed to check graph existence: {}", e)))?
        .is_some();

    match plan.kind {
        CreateKind::Create => {
            if graph_exists {
                return Err(execution_error(format!(
                    "Graph '{}' already exists",
                    plan.name
                )));
            }
        }
        CreateKind::CreateIfNotExists => {
            if graph_exists {
                return Ok(());
            }
        }
        CreateKind::CreateOrReplace => {
            if graph_exists {
                schema_catalog.remove_graph(&plan.name);
            }
        }
    }

    create_graph_with_type(plan, schema_catalog)?;

    Ok(())
}

/// Helper function to create graph with type definition
fn create_graph_with_type(
    plan: &CreateGraph,
    schema_catalog: &MemorySchemaCatalog,
) -> ExecutionResult<()> {
    let mut graph_type = MemoryGraphTypeCatalog::new();

    match &plan.graph_type {
        minigu_planner::bound::BoundGraphType::Nested(elements) => {
            populate_graph_type(&mut graph_type, elements)?;
        }
        _ => {
            return Err(execution_error(
                "Only nested graph type definitions are currently supported",
            ));
        }
    }

    let graph_type = Arc::new(graph_type);
    let memory_graph = MemoryGraph::new();
    let graph_storage = GraphStorage::Memory(memory_graph);
    let graph_container = Arc::new(GraphContainer::new(graph_type, graph_storage));

    let success = schema_catalog.add_graph(plan.name.to_string(), graph_container);

    if !success {
        return Err(execution_error(format!(
            "Failed to add graph '{}' to catalog",
            plan.name
        )));
    }

    Ok(())
}

/// Populate graph type catalog with vertex and edge types
fn populate_graph_type(
    graph_type: &mut MemoryGraphTypeCatalog,
    elements: &[BoundGraphElementType],
) -> ExecutionResult<()> {
    // Build a registry that maps LabelId back to label string
    // This is needed because LabelSet only stores LabelId, not strings
    let mut label_registry: HashMap<LabelId, String> = HashMap::new();

    // First pass: Register all labels and build registry
    for element in elements {
        match element {
            BoundGraphElementType::Vertex(vertex) => {
                let name_string = vertex.name.as_ref().map(|s| s.to_string());
                register_labels_from_label_set(
                    graph_type,
                    &vertex.labels,
                    &name_string,
                    &mut label_registry,
                )?;
            }
            BoundGraphElementType::Edge(edge) => {
                let name_string = edge.name.as_ref().map(|s| s.to_string());
                register_labels_from_label_set(
                    graph_type,
                    &edge.labels,
                    &name_string,
                    &mut label_registry,
                )?;

                // Also register labels from node type references
                register_labels_from_node_ref(graph_type, &edge.left, &mut label_registry)?;
                register_labels_from_node_ref(graph_type, &edge.right, &mut label_registry)?;
            }
        }
    }

    // Second pass: Create vertex and edge types
    for element in elements {
        match element {
            BoundGraphElementType::Vertex(vertex) => {
                add_vertex_type_to_catalog(graph_type, vertex, &label_registry)?;
            }
            BoundGraphElementType::Edge(edge) => {
                add_edge_type_to_catalog(graph_type, edge, &label_registry)?;
            }
        }
    }

    Ok(())
}

/// Register labels from a LabelSet to the graph type catalog
fn register_labels_from_label_set(
    graph_type: &mut MemoryGraphTypeCatalog,
    _label_set: &LabelSet,
    type_name: &Option<String>,
    label_registry: &mut HashMap<LabelId, String>,
) -> ExecutionResult<()> {
    // Since LabelSet only contains LabelId, we need to derive label strings somehow
    // For now, we use the type name as the label if available
    if let Some(name) = type_name {
        // Register this label
        let label_string = name.clone();
        graph_type.add_label(label_string.clone());

        // Get the label ID back and add to registry
        if let Ok(Some(label_id)) = graph_type.get_label_id(&label_string) {
            label_registry.insert(label_id, label_string);
        }
    }

    // Note: In the current binder implementation, label names are hashed to LabelId
    // We'll need to enhance the bound types to preserve label strings if needed
    // For now, we rely on type names

    Ok(())
}

/// Register labels from a NodeTypeRef
fn register_labels_from_node_ref(
    graph_type: &mut MemoryGraphTypeCatalog,
    node_ref: &NodeTypeRef,
    label_registry: &mut HashMap<LabelId, String>,
) -> ExecutionResult<()> {
    match node_ref {
        NodeTypeRef::Alias(name) => {
            let label_string = name.to_string();
            graph_type.add_label(label_string.clone());

            if let Ok(Some(label_id)) = graph_type.get_label_id(&label_string) {
                label_registry.insert(label_id, label_string);
            }
        }
        NodeTypeRef::Filler(filler) => {
            if let Some(_label_set) = &filler.label_set {
                // For filler, we can't easily recover label strings from LabelSet
                // This is a limitation of the current design
                // TODO: Enhance BoundNodeOrEdgeFiller to preserve label strings
            }
        }
        NodeTypeRef::Empty => {
            // No labels to register
        }
    }
    Ok(())
}

/// Add vertex type to catalog
fn add_vertex_type_to_catalog(
    graph_type: &mut MemoryGraphTypeCatalog,
    vertex: &BoundVertexType,
    _label_registry: &HashMap<LabelId, String>,
) -> ExecutionResult<()> {
    // vertex.labels is already a LabelSet containing LabelId
    let label_set = vertex.labels.clone();

    if label_set.is_empty() {
        return Err(execution_error(
            "Vertex type must have at least one label",
        ));
    }

    let properties = convert_fields_to_properties(&vertex.properties);
    let vertex_type = Arc::new(MemoryVertexTypeCatalog::new(label_set.clone(), properties));

    graph_type.add_vertex_type(label_set, vertex_type);

    Ok(())
}

/// Add edge type to catalog
fn add_edge_type_to_catalog(
    graph_type: &mut MemoryGraphTypeCatalog,
    edge: &BoundEdgeType,
    label_registry: &HashMap<LabelId, String>,
) -> ExecutionResult<()> {
    let label_set = edge.labels.clone();

    if label_set.is_empty() {
        return Err(execution_error("Edge type must have at least one label"));
    }

    // Resolve source and destination vertex types from NodeTypeRef
    let src_vertex_type = resolve_vertex_type_from_node_ref(graph_type, &edge.left, label_registry)?;
    let dst_vertex_type =
        resolve_vertex_type_from_node_ref(graph_type, &edge.right, label_registry)?;

    let properties = convert_fields_to_properties(&edge.properties);

    let edge_type = Arc::new(MemoryEdgeTypeCatalog::new(
        label_set.clone(),
        src_vertex_type,
        dst_vertex_type,
        properties,
    ));

    graph_type.add_edge_type(label_set, edge_type);

    Ok(())
}

/// Resolve vertex type from NodeTypeRef
fn resolve_vertex_type_from_node_ref(
    graph_type: &mut MemoryGraphTypeCatalog,
    node_ref: &NodeTypeRef,
    _label_registry: &HashMap<LabelId, String>,
) -> ExecutionResult<VertexTypeRef> {
    match node_ref {
        NodeTypeRef::Alias(name) => {
            // Get or create vertex type by name
            let label_string = name.to_string();
            get_or_create_vertex_type_by_name(graph_type, &label_string)
        }
        NodeTypeRef::Filler(filler) => {
            // Create vertex type from inline definition
            if let Some(label_set) = &filler.label_set {
                let properties = if let Some(props) = &filler.properties {
                    convert_fields_to_properties(props)
                } else {
                    vec![]
                };

                // Check if vertex type already exists
                if let Ok(Some(vertex_type)) = graph_type.get_vertex_type(label_set) {
                    return Ok(vertex_type);
                }

                // Create new vertex type
                let vertex_type =
                    Arc::new(MemoryVertexTypeCatalog::new(label_set.clone(), properties));
                graph_type.add_vertex_type(label_set.clone(), vertex_type.clone());

                Ok(vertex_type)
            } else {
                Err(execution_error("Filler must have label_set"))
            }
        }
        NodeTypeRef::Empty => Err(execution_error(
            "Cannot resolve vertex type from empty node reference",
        )),
    }
}

/// Get or create vertex type by name (string label)
fn get_or_create_vertex_type_by_name(
    graph_type: &mut MemoryGraphTypeCatalog,
    label_name: &str,
) -> ExecutionResult<VertexTypeRef> {
    // Get label ID
    let label_id = graph_type
        .get_label_id(label_name)
        .ok()
        .flatten()
        .ok_or_else(|| execution_error(format!("Label '{}' not found", label_name)))?;

    let label_set: LabelSet = vec![label_id].into_iter().collect();

    // Check if vertex type already exists
    if let Ok(Some(vertex_type)) = graph_type.get_vertex_type(&label_set) {
        return Ok(vertex_type);
    }

    // Create placeholder vertex type with no properties
    let vertex_type = Arc::new(MemoryVertexTypeCatalog::new(label_set.clone(), vec![]));
    graph_type.add_vertex_type(label_set, vertex_type.clone());

    Ok(vertex_type)
}

/// Convert DataField to Property
fn convert_fields_to_properties(fields: &[DataField]) -> Vec<Property> {
    fields
        .iter()
        .map(|field| Property::new(field.name().to_string(), field.ty().clone(), field.is_nullable()))
        .collect()
}

/// Implementation for DROP GRAPH
fn drop_graph_impl(plan: &DropGraph, session: &SessionContext) -> ExecutionResult<()> {
    let schema_catalog = session
        .current_schema
        .as_ref()
        .ok_or_else(|| execution_error("No current schema set"))?;

    let graph_exists = schema_catalog
        .get_graph(&plan.name)
        .map_err(|e| execution_error(format!("Failed to check graph existence: {}", e)))?
        .is_some();

    if !graph_exists {
        if plan.if_exists {
            return Ok(());
        } else {
            return Err(execution_error(format!(
                "Graph '{}' does not exist",
                plan.name
            )));
        }
    }

    let success = schema_catalog.remove_graph(&plan.name);

    if !success {
        return Err(execution_error(format!(
            "Failed to remove graph '{}' from catalog",
            plan.name
        )));
    }

    Ok(())
}
