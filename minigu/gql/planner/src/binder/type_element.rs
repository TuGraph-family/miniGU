use gql_parser::ast::{
    EdgeType, EdgeTypePattern, EdgeTypePhrase, FieldOrPropertyType, GraphElementType,
    NodeOrEdgeTypeFiller, NodeType, NodeTypeRef as AstNodeTypeRef, ValueType,
};
use minigu_catalog::label_set::LabelSet;
use minigu_common::data_type::{DataField, LogicalType};
use minigu_common::types::LabelId;
use smol_str::SmolStr;

use super::error::{BindError, BindResult};
use crate::bound::{
    BoundEdgeType, BoundGraphElementType, BoundVertexType, EdgeDirection,
    NodeOrEdgeTypeFiller as BoundNodeOrEdgeFiller, NodeTypeRef,
};

/// Bind a graph element type (node or edge)
pub fn bind_graph_element_type(element: &GraphElementType) -> BindResult<BoundGraphElementType> {
    match element {
        GraphElementType::Node(node) => {
            let bound_node = bind_node_type(node)?;
            Ok(BoundGraphElementType::Vertex(Box::new(bound_node)))
        }
        GraphElementType::Edge(edge) => {
            let bound_edge = bind_edge_type(edge)?;
            Ok(BoundGraphElementType::Edge(Box::new(bound_edge)))
        }
    }
}

/// Bind a node type definition
fn bind_node_type(node: &NodeType) -> BindResult<BoundVertexType> {
    let name = node.name.as_ref().map(|n| n.value().clone());

    let (label_set, properties) = if let Some(filler) = &node.filler {
        bind_node_or_edge_filler(filler.value())?
    } else {
        (LabelSet::new(), vec![])
    };

    // For now, key and labels are the same
    let key = label_set.clone();

    Ok(BoundVertexType {
        name,
        key,
        labels: label_set,
        properties,
    })
}

/// Bind an edge type definition
fn bind_edge_type(edge: &EdgeType) -> BindResult<BoundEdgeType> {
    match edge {
        EdgeType::Pattern(pattern) => bind_edge_type_pattern(pattern),
        EdgeType::Phrase(phrase) => bind_edge_type_phrase(phrase),
    }
}

/// Bind an edge type pattern
fn bind_edge_type_pattern(pattern: &EdgeTypePattern) -> BindResult<BoundEdgeType> {
    let name = pattern.name.as_ref().map(|n| n.value().clone());
    let direction = bind_edge_direction(&pattern.direction);

    let (label_set, properties) = bind_node_or_edge_filler(pattern.filler.value())?;

    // Bind left and right node references
    let left = bind_node_type_ref(pattern.left.value())?;
    let right = bind_node_type_ref(pattern.right.value())?;

    // For now, key and labels are the same
    let key = label_set.clone();

    Ok(BoundEdgeType {
        name,
        key,
        labels: label_set,
        properties,
        direction,
        left,
        right,
    })
}

/// Bind an edge type phrase
fn bind_edge_type_phrase(phrase: &EdgeTypePhrase) -> BindResult<BoundEdgeType> {
    let name = phrase.name.as_ref().map(|n| n.value().clone());
    let direction = bind_edge_direction(&phrase.direction);

    let (label_set, properties) = if let Some(filler) = &phrase.filler {
        bind_node_or_edge_filler(filler.value())?
    } else {
        (LabelSet::new(), vec![])
    };

    // Phrase uses simple identifiers for left and right
    let left = NodeTypeRef::Alias(phrase.left.value().clone());
    let right = NodeTypeRef::Alias(phrase.right.value().clone());

    // For now, key and labels are the same
    let key = label_set.clone();

    Ok(BoundEdgeType {
        name,
        key,
        labels: label_set,
        properties,
        direction,
        left,
        right,
    })
}

/// Bind edge direction
fn bind_edge_direction(direction: &gql_parser::ast::EdgeDirection) -> EdgeDirection {
    match direction {
        gql_parser::ast::EdgeDirection::LeftToRight => EdgeDirection::LeftToRight,
        gql_parser::ast::EdgeDirection::RightToLeft => EdgeDirection::RightToLeft,
        gql_parser::ast::EdgeDirection::Undirected => EdgeDirection::Undirected,
    }
}

/// Bind a node type reference
fn bind_node_type_ref(node_ref: &AstNodeTypeRef) -> BindResult<NodeTypeRef> {
    match node_ref {
        AstNodeTypeRef::Alias(ident) => Ok(NodeTypeRef::Alias(ident.clone())),
        AstNodeTypeRef::Filler(filler) => {
            let bound_filler = bind_node_or_edge_filler_struct(filler)?;
            Ok(NodeTypeRef::Filler(bound_filler))
        }
        AstNodeTypeRef::Empty => Ok(NodeTypeRef::Empty),
    }
}

/// Bind node or edge filler structure
fn bind_node_or_edge_filler_struct(
    filler: &NodeOrEdgeTypeFiller,
) -> BindResult<BoundNodeOrEdgeFiller> {
    let (label_set, properties) = bind_node_or_edge_filler(filler)?;

    // For now, key is the same as label_set
    let key = if label_set.is_empty() {
        None
    } else {
        Some(label_set.clone())
    };

    Ok(BoundNodeOrEdgeFiller {
        key,
        label_set: if label_set.is_empty() {
            None
        } else {
            Some(label_set)
        },
        properties: if properties.is_empty() {
            None
        } else {
            Some(properties)
        },
    })
}

/// Bind node or edge filler (labels and properties) to LabelSet
fn bind_node_or_edge_filler(
    filler: &NodeOrEdgeTypeFiller,
) -> BindResult<(LabelSet, Vec<DataField>)> {
    // Extract labels and convert to LabelSet
    let label_ids: Vec<LabelId> = if let Some(label_set) = &filler.label_set {
        // For now, we use a simple hash of label string as LabelId
        // In a real implementation, this should look up or create label IDs from a label catalog
        label_set
            .value()
            .iter()
            .map(|l| string_to_label_id(l.value()))
            .collect()
    } else {
        vec![]
    };

    let label_set: LabelSet = label_ids.into_iter().collect();

    // Extract properties
    let properties = if let Some(property_types) = &filler.property_types {
        property_types
            .iter()
            .map(|p| bind_property_type(p.value()))
            .collect::<BindResult<Vec<_>>>()?
    } else {
        vec![]
    };

    Ok((label_set, properties))
}

/// Convert string to LabelId (temporary implementation)
/// TODO: This should be replaced with proper label catalog lookup
fn string_to_label_id(label: &str) -> LabelId {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::num::NonZeroU32;

    let mut hasher = DefaultHasher::new();
    label.hash(&mut hasher);
    let hash_value = (hasher.finish() as u32) % 9999 + 1; // Range: 1-10000, avoiding 0
    NonZeroU32::new(hash_value).unwrap()
}

/// Bind a property type (field name and data type)
fn bind_property_type(property: &FieldOrPropertyType) -> BindResult<DataField> {
    let name = property.name.value().to_string();
    let logical_type = bind_value_type(property.value_type.value())?;
    let nullable = !is_not_null(property.value_type.value());

    Ok(DataField::new(name, logical_type, nullable))
}

/// Check if a value type is marked as NOT NULL
fn is_not_null(value_type: &ValueType) -> bool {
    match value_type {
        ValueType::String { not_null, .. } => *not_null,
        ValueType::SignedNumeric { not_null, .. } => *not_null,
        ValueType::UnsignedNumeric { not_null, .. } => *not_null,
        ValueType::Float { not_null, .. } => *not_null,
        ValueType::Bool { not_null, .. } => *not_null,
        ValueType::Temporal { not_null, .. } => *not_null,
        ValueType::Vector { not_null, .. } => *not_null,
        _ => false,
    }
}

/// Bind a value type to a logical type
fn bind_value_type(value_type: &ValueType) -> BindResult<LogicalType> {
    match value_type {
        ValueType::String { .. } => Ok(LogicalType::String),
        ValueType::SignedNumeric { kind, .. } => bind_signed_numeric_type(kind.value()),
        ValueType::UnsignedNumeric { kind, .. } => bind_unsigned_numeric_type(kind.value()),
        ValueType::Float { kind, .. } => bind_float_type(kind.value()),
        ValueType::Bool { .. } => Ok(LogicalType::Boolean),
        ValueType::Temporal { kind, .. } => bind_temporal_type(kind.value()),
        ValueType::Vector { dimension, .. } => {
            let dim_value = dimension.value();
            let dim = dim_value
                .integer
                .parse::<usize>()
                .map_err(|_| BindError::InvalidInteger(dim_value.integer.clone()))?;
            Ok(LogicalType::Vector(dim))
        }
        _ => Err(BindError::UnsupportedValueType),
    }
}

/// Bind signed numeric type
fn bind_signed_numeric_type(kind: &gql_parser::ast::NumericTypeKind) -> BindResult<LogicalType> {
    use gql_parser::ast::NumericTypeKind;
    match kind {
        NumericTypeKind::Int8 => Ok(LogicalType::Int8),
        NumericTypeKind::Int16 | NumericTypeKind::Small => Ok(LogicalType::Int16),
        NumericTypeKind::Int32 | NumericTypeKind::Int(_) => Ok(LogicalType::Int32),
        NumericTypeKind::Int64 | NumericTypeKind::Big => Ok(LogicalType::Int64),
        _ => Err(BindError::UnsupportedValueType),
    }
}

/// Bind unsigned numeric type
fn bind_unsigned_numeric_type(kind: &gql_parser::ast::NumericTypeKind) -> BindResult<LogicalType> {
    use gql_parser::ast::NumericTypeKind;
    match kind {
        NumericTypeKind::Int8 => Ok(LogicalType::UInt8),
        NumericTypeKind::Int16 | NumericTypeKind::Small => Ok(LogicalType::UInt16),
        NumericTypeKind::Int32 | NumericTypeKind::Int(_) => Ok(LogicalType::UInt32),
        NumericTypeKind::Int64 | NumericTypeKind::Big => Ok(LogicalType::UInt64),
        _ => Err(BindError::UnsupportedValueType),
    }
}

/// Bind float type
fn bind_float_type(kind: &gql_parser::ast::FloatTypeKind) -> BindResult<LogicalType> {
    use gql_parser::ast::FloatTypeKind;
    match kind {
        FloatTypeKind::Float16 | FloatTypeKind::Real | FloatTypeKind::Float32 => {
            Ok(LogicalType::Float32)
        }
        FloatTypeKind::Float64 | FloatTypeKind::Double | FloatTypeKind::Float { .. } => {
            Ok(LogicalType::Float64)
        }
        _ => Err(BindError::UnsupportedValueType),
    }
}

/// Bind temporal type (currently only DATE is supported, mapped to String)
fn bind_temporal_type(kind: &gql_parser::ast::TemporalTypeKind) -> BindResult<LogicalType> {
    use gql_parser::ast::TemporalTypeKind;
    match kind {
        TemporalTypeKind::Date => Ok(LogicalType::String),
        _ => Err(BindError::UnsupportedValueType),
    }
}
