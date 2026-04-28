use std::collections::HashMap;
use std::sync::Arc;

use minigu_catalog::label_set::LabelSet;
use minigu_catalog::memory::graph_type::{
    MemoryEdgeTypeCatalog, MemoryGraphTypeCatalog, MemoryVertexTypeCatalog,
};
use minigu_catalog::memory::txn_manager;
use minigu_catalog::property::Property;
use minigu_catalog::provider::{GraphRef, GraphTypeProvider, VertexTypeRef};
use minigu_catalog::txn::{CatalogTxn, CatalogTxnError};
use minigu_catalog::{CreateGraphResult, CreateKind as CatalogCreateKind, DropGraphResult};
use minigu_common::IsolationLevel;
use minigu_common::data_type::DataField;
use minigu_common::types::LabelId;
use minigu_context::graph::{GraphContainer, GraphStorage};
use minigu_context::session::SessionContext;
use minigu_planner::bound::{
    BoundEdgeType, BoundGraphElementType, BoundVertexType, CreateKind as PlannerCreateKind,
    NodeTypeRef,
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
            if let Err(e) = create_graph_impl(&plan, &session) {
                yield Err(e);
            }
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

            match drop_graph_impl(&plan, &session) {
                Ok(_) => {}
                Err(e) => {
                    yield Err(e);
                }
            }
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

    let new_graph_container = build_graph_container(plan)?;

    let catalog_kind = match plan.kind {
        PlannerCreateKind::Create => CatalogCreateKind::Create,
        PlannerCreateKind::CreateIfNotExists => CatalogCreateKind::CreateIfNotExists,
        PlannerCreateKind::CreateOrReplace => CatalogCreateKind::CreateOrReplace,
    };

    let result =
        schema_catalog.create_graph(plan.name.to_string(), new_graph_container, catalog_kind);

    match (plan.kind, result) {
        (PlannerCreateKind::Create, CreateGraphResult::AlreadyExists) => Err(execution_error(
            format!("Graph '{}' already exists", plan.name),
        )),
        _ => Ok(()),
    }
}

/// Factory: Builds the graph container logic without side effects
fn build_graph_container(plan: &CreateGraph) -> ExecutionResult<GraphRef> {
    let graph_type = match &plan.graph_type {
        minigu_planner::bound::BoundGraphType::Nested(elements) => {
            let catalog = MemoryGraphTypeCatalog::new();
            let txn = txn_manager()
                .begin_transaction(IsolationLevel::Serializable)
                .map_err(|e| {
                    execution_error(format!("Failed to begin catalog transaction: {e}"))
                })?;
            populate_graph_type(&catalog, elements, txn.as_ref())?;
            txn.commit().map_err(|e| {
                execution_error(format!("Failed to commit graph type catalog: {e}"))
            })?;
            Arc::new(catalog)
        }
        _ => {
            return Err(execution_error(
                "Only nested graph type definitions are supported",
            ));
        }
    };

    let memory_graph = MemoryGraph::in_memory();
    let graph_storage = GraphStorage::Memory(memory_graph);

    Ok(Arc::new(GraphContainer::new(graph_type, graph_storage)))
}

/// Populate graph type catalog with vertex and edge types
fn populate_graph_type(
    graph_type: &MemoryGraphTypeCatalog,
    elements: &[BoundGraphElementType],
    txn: &CatalogTxn,
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
                    txn,
                )?;
            }
            BoundGraphElementType::Edge(edge) => {
                let name_string = edge.name.as_ref().map(|s| s.to_string());
                register_labels_from_label_set(
                    graph_type,
                    &edge.labels,
                    &name_string,
                    &mut label_registry,
                    txn,
                )?;

                // Also register labels from node type references
                register_labels_from_node_ref(graph_type, &edge.left, &mut label_registry, txn)?;
                register_labels_from_node_ref(graph_type, &edge.right, &mut label_registry, txn)?;
            }
        }
    }

    // Second pass: Create vertex and edge types
    for element in elements {
        match element {
            BoundGraphElementType::Vertex(vertex) => {
                add_vertex_type_to_catalog(graph_type, vertex, &label_registry, txn)?;
            }
            BoundGraphElementType::Edge(edge) => {
                add_edge_type_to_catalog(graph_type, edge, &label_registry, txn)?;
            }
        }
    }

    Ok(())
}

/// Register labels from a LabelSet to the graph type catalog
fn register_labels_from_label_set(
    graph_type: &MemoryGraphTypeCatalog,
    _label_set: &LabelSet,
    type_name: &Option<String>,
    label_registry: &mut HashMap<LabelId, String>,
    txn: &CatalogTxn,
) -> ExecutionResult<()> {
    // Since LabelSet only contains LabelId, we need to derive label strings somehow
    // For now, we use the type name as the label if available
    if let Some(name) = type_name {
        let label_string = name.clone();
        let label_id = ensure_label(graph_type, &label_string, txn)?;
        label_registry.insert(label_id, label_string);
    }

    // Note: In the current binder implementation, label names are hashed to LabelId
    // We'll need to enhance the bound types to preserve label strings if needed
    // For now, we rely on type names

    Ok(())
}

/// Register labels from a NodeTypeRef
fn register_labels_from_node_ref(
    graph_type: &MemoryGraphTypeCatalog,
    node_ref: &NodeTypeRef,
    label_registry: &mut HashMap<LabelId, String>,
    txn: &CatalogTxn,
) -> ExecutionResult<()> {
    match node_ref {
        NodeTypeRef::Alias(name) => {
            let label_string = name.to_string();
            let label_id = ensure_label(graph_type, &label_string, txn)?;
            label_registry.insert(label_id, label_string);
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
    graph_type: &MemoryGraphTypeCatalog,
    vertex: &BoundVertexType,
    _label_registry: &HashMap<LabelId, String>,
    txn: &CatalogTxn,
) -> ExecutionResult<()> {
    // vertex.labels is already a LabelSet containing LabelId
    let label_set = vertex.labels.clone();

    if label_set.is_empty() {
        return Err(execution_error("Vertex type must have at least one label"));
    }

    let properties = convert_fields_to_properties(&vertex.properties);
    let vertex_type = Arc::new(MemoryVertexTypeCatalog::new(label_set.clone(), properties));

    add_vertex_type_if_absent(graph_type, label_set, vertex_type, txn)?;

    Ok(())
}

/// Add edge type to catalog
fn add_edge_type_to_catalog(
    graph_type: &MemoryGraphTypeCatalog,
    edge: &BoundEdgeType,
    label_registry: &HashMap<LabelId, String>,
    txn: &CatalogTxn,
) -> ExecutionResult<()> {
    let label_set = edge.labels.clone();

    if label_set.is_empty() {
        return Err(execution_error("Edge type must have at least one label"));
    }

    // Resolve source and destination vertex types from NodeTypeRef
    let src_vertex_type =
        resolve_vertex_type_from_node_ref(graph_type, &edge.left, label_registry, txn)?;
    let dst_vertex_type =
        resolve_vertex_type_from_node_ref(graph_type, &edge.right, label_registry, txn)?;

    let properties = convert_fields_to_properties(&edge.properties);

    let edge_type = Arc::new(MemoryEdgeTypeCatalog::new(
        label_set.clone(),
        src_vertex_type,
        dst_vertex_type,
        properties,
    ));

    add_edge_type_if_absent(graph_type, label_set, edge_type, txn)?;

    Ok(())
}

/// Resolve vertex type from NodeTypeRef
fn resolve_vertex_type_from_node_ref(
    graph_type: &MemoryGraphTypeCatalog,
    node_ref: &NodeTypeRef,
    _label_registry: &HashMap<LabelId, String>,
    txn: &CatalogTxn,
) -> ExecutionResult<VertexTypeRef> {
    match node_ref {
        NodeTypeRef::Alias(name) => {
            // Get or create vertex type by name
            let label_string = name.to_string();
            get_or_create_vertex_type_by_name(graph_type, &label_string, txn)
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
                if let Ok(Some(vertex_type)) = graph_type.get_vertex_type_txn(label_set, txn) {
                    return Ok(vertex_type);
                }

                // Create new vertex type
                let vertex_type =
                    Arc::new(MemoryVertexTypeCatalog::new(label_set.clone(), properties));
                add_vertex_type_if_absent(graph_type, label_set.clone(), vertex_type.clone(), txn)?;

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
    graph_type: &MemoryGraphTypeCatalog,
    label_name: &str,
    txn: &CatalogTxn,
) -> ExecutionResult<VertexTypeRef> {
    // Get label ID
    let label_id = graph_type
        .get_label_id_txn(label_name, txn)
        .ok()
        .flatten()
        .ok_or_else(|| execution_error(format!("Label '{}' not found", label_name)))?;

    let label_set: LabelSet = vec![label_id].into_iter().collect();

    // Check if vertex type already exists
    if let Ok(Some(vertex_type)) = graph_type.get_vertex_type_txn(&label_set, txn) {
        return Ok(vertex_type);
    }

    // Create placeholder vertex type with no properties
    let vertex_type = Arc::new(MemoryVertexTypeCatalog::new(label_set.clone(), vec![]));
    add_vertex_type_if_absent(graph_type, label_set, vertex_type.clone(), txn)?;

    Ok(vertex_type)
}

fn ensure_label(
    graph_type: &MemoryGraphTypeCatalog,
    label: &str,
    txn: &CatalogTxn,
) -> ExecutionResult<LabelId> {
    if let Some(label_id) = graph_type
        .get_label_id_txn(label, txn)
        .map_err(|e| execution_error(format!("Failed to read label '{label}': {e}")))?
    {
        return Ok(label_id);
    }

    graph_type
        .add_label_txn(label.to_string(), txn)
        .map_err(|e| execution_error(format!("Failed to add label '{label}': {e}")))
}

fn add_vertex_type_if_absent(
    graph_type: &MemoryGraphTypeCatalog,
    label_set: LabelSet,
    vertex_type: Arc<MemoryVertexTypeCatalog>,
    txn: &CatalogTxn,
) -> ExecutionResult<()> {
    match graph_type.add_vertex_type_txn(label_set, vertex_type, txn) {
        Ok(()) | Err(CatalogTxnError::AlreadyExists { .. }) => Ok(()),
        Err(e) => Err(execution_error(format!("Failed to add vertex type: {e}"))),
    }
}

fn add_edge_type_if_absent(
    graph_type: &MemoryGraphTypeCatalog,
    label_set: LabelSet,
    edge_type: Arc<MemoryEdgeTypeCatalog>,
    txn: &CatalogTxn,
) -> ExecutionResult<()> {
    match graph_type.add_edge_type_txn(label_set, edge_type, txn) {
        Ok(()) | Err(CatalogTxnError::AlreadyExists { .. }) => Ok(()),
        Err(e) => Err(execution_error(format!("Failed to add edge type: {e}"))),
    }
}

/// Convert DataField to Property
fn convert_fields_to_properties(fields: &[DataField]) -> Vec<Property> {
    fields
        .iter()
        .map(|field| {
            Property::new(
                field.name().to_string(),
                field.ty().clone(),
                field.is_nullable(),
            )
        })
        .collect()
}

/// Implementation for DROP GRAPH
fn drop_graph_impl(plan: &DropGraph, session: &SessionContext) -> ExecutionResult<()> {
    let schema_catalog = session
        .current_schema
        .as_ref()
        .ok_or_else(|| execution_error("No current schema set"))?;

    match schema_catalog.drop_graph(&plan.name) {
        DropGraphResult::Dropped => Ok(()),
        DropGraphResult::NotFound if plan.if_exists => Ok(()),
        DropGraphResult::NotFound => Err(execution_error(format!(
            "Graph '{}' does not exist",
            plan.name
        ))),
    }
}
