use std::collections::HashMap;

use gql_parser::ast::{
    DataModifyingStatement, EdgePatternKind, ElementPattern, ElementPatternFiller,
    ElementPatternPredicate, FieldOrProperty, Ident, InsertPath, InsertStatement, LabelExpr,
    LinearDataModifyingStatement,
};
use gql_parser::span::Spanned;
use minigu_catalog::label_set::LabelSet;
use minigu_catalog::property::Property;
use minigu_catalog::provider::{EdgeTypeRef, GraphTypeProvider, VertexTypeRef};
use minigu_common::data_type::LogicalType;
use minigu_common::types::{LabelId, PropertyId};
use minigu_common::value::ScalarValue;
use smol_str::SmolStr;

use super::Binder;
use super::error::{BindError, BindResult};
use crate::bound::{
    BoundDataModifyingStatement, BoundInsertEdge, BoundInsertProperty, BoundInsertStatement,
    BoundInsertVertex,
};

impl Binder<'_> {
    pub fn bind_data_modifying_statement(
        &mut self,
        statement: &DataModifyingStatement,
    ) -> BindResult<BoundDataModifyingStatement> {
        match statement {
            DataModifyingStatement::Insert(stmt) => self
                .bind_insert_statement(stmt)
                .map(BoundDataModifyingStatement::Insert),
        }
    }

    pub fn bind_linear_data_modifying_statement(
        &mut self,
        statements: &LinearDataModifyingStatement,
    ) -> BindResult<Vec<BoundDataModifyingStatement>> {
        statements
            .iter()
            .map(|s| self.bind_data_modifying_statement(s.value()))
            .collect()
    }

    fn bind_insert_statement(
        &mut self,
        stmt: &InsertStatement,
    ) -> BindResult<BoundInsertStatement> {
        let graph = self
            .current_graph
            .as_ref()
            .ok_or(BindError::CurrentGraphNotSpecified)?
            .clone();
        let graph_type = graph.graph_type();

        // First pass: walk every path, classify each ElementPattern as either a
        // vertex (allocating a slot in `vertices`) or an edge (recorded for the
        // second pass). Build the variable -> index map so edges can resolve
        // their endpoints.
        let mut vertices: Vec<BoundInsertVertex> = Vec::new();
        let mut var_index: HashMap<SmolStr, usize> = HashMap::new();
        // Edge work items: (path_index, element_index, src_var, dst_var)
        struct PendingEdge<'a> {
            filler: &'a ElementPatternFiller,
            kind: EdgePatternKind,
            src_var: SmolStr,
            dst_var: SmolStr,
        }
        let mut pending_edges: Vec<PendingEdge<'_>> = Vec::new();

        for path in &stmt.paths {
            let path = path.value();
            // A path's elements must alternate Node, Edge, Node, Edge, ...
            // Insert paths have shape `Node (Edge Node)*` so the first element
            // is a node and after that pairs of (edge, node).
            let mut iter = path.elements.iter().peekable();
            let first = iter
                .next()
                .ok_or_else(|| BindError::InsertPathTooComplex(0))?;
            let first_var = self.bind_insert_vertex(
                first.value(),
                graph_type.as_ref(),
                &mut vertices,
                &mut var_index,
            )?;
            let mut prev_var = first_var;
            while let Some(edge_el) = iter.next() {
                let edge_pat = match edge_el.value() {
                    ElementPattern::Edge { kind, filler } => (kind.clone(), filler),
                    ElementPattern::Node(_) => {
                        return Err(BindError::InsertPathTooComplex(path.elements.len()));
                    }
                };
                let next_node = iter
                    .next()
                    .ok_or_else(|| BindError::InsertPathTooComplex(path.elements.len()))?;
                let next_var = self.bind_insert_vertex(
                    next_node.value(),
                    graph_type.as_ref(),
                    &mut vertices,
                    &mut var_index,
                )?;

                // Resolve direction: which endpoint is the GQL "src"?
                let (src_var, dst_var) = match edge_pat.0 {
                    EdgePatternKind::Right => (prev_var.clone(), next_var.clone()),
                    EdgePatternKind::Left => (next_var.clone(), prev_var.clone()),
                    EdgePatternKind::Undirected
                    | EdgePatternKind::LeftRight
                    | EdgePatternKind::LeftUndirected
                    | EdgePatternKind::RightUndirected
                    | EdgePatternKind::Any => {
                        return Err(BindError::InsertUndirectedEdgeNotSupported);
                    }
                };
                pending_edges.push(PendingEdge {
                    filler: edge_pat.1,
                    kind: edge_pat.0,
                    src_var,
                    dst_var,
                });
                prev_var = next_var;
            }
        }

        // Second pass: bind edges now that all vertex variables exist.
        let mut edges = Vec::with_capacity(pending_edges.len());
        for pe in pending_edges {
            let bound = self.bind_insert_edge(
                pe.filler,
                graph_type.as_ref(),
                &vertices,
                &var_index,
                &pe.src_var,
                &pe.dst_var,
            )?;
            edges.push(bound);
        }

        Ok(BoundInsertStatement { vertices, edges })
    }

    fn bind_insert_vertex(
        &self,
        pattern: &ElementPattern,
        graph_type: &dyn GraphTypeProvider,
        vertices: &mut Vec<BoundInsertVertex>,
        var_index: &mut HashMap<SmolStr, usize>,
    ) -> BindResult<SmolStr> {
        let filler = match pattern {
            ElementPattern::Node(f) => f,
            ElementPattern::Edge { .. } => {
                return Err(BindError::InsertPathTooComplex(0));
            }
        };

        let variable = filler
            .variable
            .as_ref()
            .map(|v: &Spanned<Ident>| SmolStr::new(v.value().as_str()))
            .ok_or(BindError::InsertAnonymousVertexNotSupported)?;

        // If this variable was already declared as a vertex, this is a
        // back-reference rather than a new vertex creation. Reuse it.
        if let Some(_existing_index) = var_index.get(&variable) {
            // The endpoint is referenced again; the spec says the same variable
            // may appear in the same INSERT only once as a definition. For the
            // standalone-INSERT scope we treat any *additional* occurrence with
            // a non-empty filler as a duplicate; otherwise accept the
            // back-reference.
            if filler.label.is_some() || filler.predicate.is_some() {
                return Err(BindError::InsertDuplicateVariable(variable));
            }
            return Ok(variable);
        }

        let label_id = bind_single_label(filler.label.as_ref(), graph_type)
            .ok_or(BindError::InsertVertexLabelMissing)??;
        let label_set: LabelSet = LabelSet::from_iter([label_id]);
        let vertex_type = graph_type.get_vertex_type(&label_set)?.ok_or_else(|| {
            BindError::InsertVertexTypeNotFound {
                label: label_id_to_smol(graph_type, label_id),
            }
        })?;

        let label_name = label_id_to_smol(graph_type, label_id);
        let properties = bind_property_list(
            filler.predicate.as_ref(),
            vertex_type.properties(),
            &label_name,
        )?;

        let index = vertices.len();
        vertices.push(BoundInsertVertex {
            variable: variable.clone(),
            label_id,
            labels: label_set,
            properties,
        });
        var_index.insert(variable.clone(), index);
        Ok(variable)
    }

    fn bind_insert_edge(
        &self,
        filler: &ElementPatternFiller,
        graph_type: &dyn GraphTypeProvider,
        vertices: &[BoundInsertVertex],
        var_index: &HashMap<SmolStr, usize>,
        src_var: &SmolStr,
        dst_var: &SmolStr,
    ) -> BindResult<BoundInsertEdge> {
        let label_id = bind_single_label(filler.label.as_ref(), graph_type)
            .ok_or(BindError::InsertEdgeLabelMissing)??;
        let label_set: LabelSet = LabelSet::from_iter([label_id]);
        let edge_type = graph_type.get_edge_type(&label_set)?.ok_or_else(|| {
            BindError::InsertEdgeTypeNotFound {
                label: label_id_to_smol(graph_type, label_id),
            }
        })?;

        let src_index = *var_index
            .get(src_var)
            .ok_or_else(|| BindError::InsertEndpointNotDefined(src_var.clone()))?;
        let dst_index = *var_index
            .get(dst_var)
            .ok_or_else(|| BindError::InsertEndpointNotDefined(dst_var.clone()))?;

        let expected_src_set = edge_type.src().label_set();
        let expected_dst_set = edge_type.dst().label_set();
        let actual_src_set: LabelSet = LabelSet::from_iter([vertices[src_index].label_id]);
        let actual_dst_set: LabelSet = LabelSet::from_iter([vertices[dst_index].label_id]);
        if expected_src_set != actual_src_set || expected_dst_set != actual_dst_set {
            return Err(BindError::InsertEdgeEndpointTypeMismatch {
                label: label_id_to_smol(graph_type, label_id),
                expected_src: label_set_to_smol(graph_type, &expected_src_set),
                expected_dst: label_set_to_smol(graph_type, &expected_dst_set),
                actual_src: label_id_to_smol(graph_type, vertices[src_index].label_id),
                actual_dst: label_id_to_smol(graph_type, vertices[dst_index].label_id),
            });
        }

        let label_name = label_id_to_smol(graph_type, label_id);
        let properties = bind_property_list(
            filler.predicate.as_ref(),
            edge_type.properties(),
            &label_name,
        )?;

        let variable = filler
            .variable
            .as_ref()
            .map(|v| SmolStr::new(v.value().as_str()));
        Ok(BoundInsertEdge {
            variable,
            label_id,
            labels: label_set,
            properties,
            src_index,
            dst_index,
        })
    }
}

/// Resolve a single-label expression `:Label` into its id. Returns `None` if no
/// label was provided (caller decides whether that's an error).
fn bind_single_label(
    label: Option<&Spanned<LabelExpr>>,
    graph_type: &dyn GraphTypeProvider,
) -> Option<BindResult<LabelId>> {
    let label = label?;
    Some(match label.value() {
        LabelExpr::Label(name) => graph_type
            .get_label_id(name.as_str())
            .map_err(BindError::Catalog)
            .and_then(|opt| {
                opt.ok_or_else(|| BindError::LabelNotFound(SmolStr::new(name.as_str())))
            }),
        _ => Err(BindError::InsertComplexLabelNotSupported),
    })
}

fn bind_property_list(
    predicate: Option<&Spanned<ElementPatternPredicate>>,
    declared: Vec<(PropertyId, Property)>,
    label: &SmolStr,
) -> BindResult<Vec<BoundInsertProperty>> {
    // Map property-name -> (id, declared-property)
    let declared_map: HashMap<&str, (PropertyId, &Property)> = declared
        .iter()
        .map(|(id, p)| (p.name(), (*id, p)))
        .collect();

    let provided: Vec<&Spanned<FieldOrProperty>> = match predicate {
        None => Vec::new(),
        Some(p) => match p.value() {
            ElementPatternPredicate::Property(fields) => fields.iter().collect(),
            ElementPatternPredicate::Where(_) => {
                return Err(BindError::InsertWherePredicateNotAllowed);
            }
        },
    };

    // Validate every provided property and coerce literal to the declared type.
    let mut by_name: HashMap<&str, BoundInsertProperty> = HashMap::new();
    for field in provided {
        let name = field.value().name.value().as_str();
        let (prop_id, declared_prop) =
            declared_map
                .get(name)
                .copied()
                .ok_or_else(|| BindError::InsertUnknownProperty {
                    label: label.clone(),
                    property: SmolStr::new(name),
                })?;
        let value = literal_scalar_from_expr(field.value().value.value()).ok_or_else(|| {
            BindError::InsertNonLiteralProperty {
                property: SmolStr::new(name),
            }
        })?;
        let value =
            coerce_scalar_to_type(value, declared_prop.logical_type()).ok_or_else(|| {
                BindError::InsertPropertyTypeMismatch {
                    label: label.clone(),
                    property: SmolStr::new(name),
                    expected: declared_prop.logical_type().clone(),
                    actual: LogicalType::Null, /* best-effort; precise actual omitted to keep msg
                                                * simple */
                }
            })?;
        by_name.insert(
            declared_prop.name(),
            BoundInsertProperty {
                name: SmolStr::new(declared_prop.name()),
                property_id: prop_id,
                value,
            },
        );
    }

    // Walk declared properties in type order: every non-nullable property must
    // be supplied, missing nullable properties become NULL.
    let mut out = Vec::with_capacity(declared.len());
    for (prop_id, decl) in &declared {
        match by_name.remove(decl.name()) {
            Some(bp) => out.push(bp),
            None => {
                if !decl.nullable() {
                    return Err(BindError::InsertRequiredPropertyMissing {
                        label: label.clone(),
                        property: SmolStr::new(decl.name()),
                    });
                }
                out.push(BoundInsertProperty {
                    name: SmolStr::new(decl.name()),
                    property_id: *prop_id,
                    value: ScalarValue::Null,
                });
            }
        }
    }
    Ok(out)
}

/// Extract a literal scalar value from a parser `Expr`, or `None` if the
/// expression is not a literal.
fn literal_scalar_from_expr(expr: &gql_parser::ast::Expr) -> Option<ScalarValue> {
    use gql_parser::ast::{Expr, Literal, UnaryOp, Value};
    use minigu_common::value::{F32, F64};

    fn from_literal(literal: &Literal) -> Option<ScalarValue> {
        match literal {
            Literal::Boolean(b) => Some(match b {
                gql_parser::ast::BooleanLiteral::True => ScalarValue::Boolean(Some(true)),
                gql_parser::ast::BooleanLiteral::False => ScalarValue::Boolean(Some(false)),
                gql_parser::ast::BooleanLiteral::Unknown => ScalarValue::Boolean(None),
            }),
            Literal::String(s) => match s.kind {
                gql_parser::ast::StringLiteralKind::Char => {
                    Some(ScalarValue::String(Some(s.literal.to_string())))
                }
                _ => None,
            },
            Literal::Numeric(n) => match n {
                gql_parser::ast::UnsignedNumericLiteral::Integer(i) => {
                    let s = i.value().integer.as_str();
                    if let Ok(v) = s.parse::<i64>() {
                        // Widest fitting int representation; coercion narrows later.
                        Some(ScalarValue::Int64(Some(v)))
                    } else {
                        None
                    }
                }
                gql_parser::ast::UnsignedNumericLiteral::Float(f) => f
                    .value()
                    .float
                    .parse::<f64>()
                    .ok()
                    .map(|v| ScalarValue::Float64(Some(F64::from(v)))),
            },
            Literal::Null => Some(ScalarValue::Null),
            _ => None,
        }
    }

    match expr {
        Expr::Value(Value::Literal(l)) => from_literal(l),
        Expr::Unary { op, child } => {
            let inner = literal_scalar_from_expr(child.value())?;
            match op.value() {
                UnaryOp::Plus => Some(inner),
                UnaryOp::Minus => match inner {
                    ScalarValue::Int64(Some(v)) => Some(ScalarValue::Int64(Some(-v))),
                    ScalarValue::Float64(Some(v)) => {
                        Some(ScalarValue::Float64(Some(F64::from(-v.into_inner()))))
                    }
                    _ => None,
                },
                UnaryOp::Not => None,
            }
        }
        _ => None,
    }
}

/// Coerce a literal scalar into the declared logical type. Numeric narrowing
/// is allowed when it fits; type families must match. Returns `None` if no
/// safe coercion exists.
fn coerce_scalar_to_type(value: ScalarValue, target: &LogicalType) -> Option<ScalarValue> {
    use minigu_common::value::F32;

    if matches!(value, ScalarValue::Null) {
        return Some(ScalarValue::Null);
    }
    match target {
        LogicalType::Int8 => value.to_i8().ok().map(|v| ScalarValue::Int8(Some(v))),
        LogicalType::Int16 => value.to_i16().ok().map(|v| ScalarValue::Int16(Some(v))),
        LogicalType::Int32 => value.to_i32().ok().map(|v| ScalarValue::Int32(Some(v))),
        LogicalType::Int64 => value.to_i64().ok().map(|v| ScalarValue::Int64(Some(v))),
        LogicalType::UInt8 => value.to_u8().ok().map(|v| ScalarValue::UInt8(Some(v))),
        LogicalType::UInt16 => value.to_u16().ok().map(|v| ScalarValue::UInt16(Some(v))),
        LogicalType::UInt32 => value.to_u32().ok().map(|v| ScalarValue::UInt32(Some(v))),
        LogicalType::UInt64 => value.to_u64().ok().map(|v| ScalarValue::UInt64(Some(v))),
        LogicalType::Float32 => value
            .to_f32()
            .ok()
            .map(|v| ScalarValue::Float32(Some(F32::from(v)))),
        LogicalType::Float64 => value
            .to_f64()
            .ok()
            .map(|v| ScalarValue::Float64(Some(minigu_common::value::F64::from(v)))),
        LogicalType::Boolean => value.to_bool().ok().map(|v| ScalarValue::Boolean(Some(v))),
        LogicalType::String => value.to_string().ok().map(|v| ScalarValue::String(Some(v))),
        // Other types (Vector, Vertex, Edge, Record, Null) are not yet
        // supported as INSERT literals.
        _ => None,
    }
}

fn label_id_to_smol(graph_type: &dyn GraphTypeProvider, id: LabelId) -> SmolStr {
    for name in graph_type.label_names() {
        if let Ok(Some(other_id)) = graph_type.get_label_id(&name)
            && other_id == id
        {
            return SmolStr::new(name);
        }
    }
    SmolStr::new(format!("#{id:?}"))
}

fn label_set_to_smol(graph_type: &dyn GraphTypeProvider, set: &LabelSet) -> SmolStr {
    // We treat label sets as singletons in this iteration.
    if let Some(id) = set.first() {
        label_id_to_smol(graph_type, id)
    } else {
        SmolStr::new("")
    }
}

// Keep imports tidy: VertexTypeRef/EdgeTypeRef brought in for trait method
// access via `graph_type.get_vertex_type(...)` return values.
#[allow(dead_code)]
fn _ensure_traits(_v: VertexTypeRef, _e: EdgeTypeRef) {}
