use gql_parser::ast::*;
use minigu_catalog::label_set::LabelSet;
use minigu_catalog::provider::{EdgeTypeRef, VertexTypeRef};
use minigu_common::types::LabelId;

use crate::binder::binder::Binder;
use crate::bound_statement::common::*;
use crate::bound_statement::expr::{BoundGraphExpr, BoundPathPatternExpr};
use crate::bound_statement::query::*;
use crate::catalog_ref::PropertyCatalogRef;
use crate::error::{BindError, BindResult};

impl Binder {
    pub(crate) fn resolve_simple_query_statement(
        &mut self,
        statement: &SimpleQueryStatement,
    ) -> BindResult<BoundSimpleQueryStatement> {
        match statement {
            SimpleQueryStatement::Match(m) => match m {
                MatchStatement::Simple(binding_table) => {
                    let pattern =
                        self.resolve_graph_pattern(binding_table.value().pattern.value())?;
                    // TODO: Handle yield items.
                    let yield_items = binding_table
                        .value()
                        .yield_clause
                        .iter()
                        .map(|v| v.value().clone())
                        .collect();
                    Ok(BoundSimpleQueryStatement::Match(
                        BoundMatchStatement::Simple(BoundGraphPatternBindingTable {
                            pattern,
                            yield_item: yield_items,
                        }),
                    ))
                }
                _ => {
                    return Err(BindError::NotSupported("MatchStatement".to_string()));
                }
            },
            SimpleQueryStatement::Call(call) => Ok(BoundSimpleQueryStatement::Call(
                self.resolve_call_procedure(call)?,
            )),
        }
    }

    pub(crate) fn resolve_path_pattern(
        &mut self,
        pattern: &PathPattern,
    ) -> Result<BoundPathPattern, BindError> {
        let variable = pattern.variable.as_ref().map(|v| v.value().clone());
        let prefix = pattern
            .prefix
            .as_ref()
            .map(|v| self.resolve_path_pattern_prefix(v.value()))
            .transpose()?;
        let expr = self.resolve_path_pattern_expr(pattern.expr.value())?;
        Ok(BoundPathPattern {
            variable,
            prefix,
            expr,
        })
    }

    pub(crate) fn resolve_path_pattern_prefix(
        &self,
        prefix: &PathPatternPrefix,
    ) -> BindResult<BoundPathPatternPrefix> {
        Err(BindError::NotSupported("Prefix".to_string()))
    }

    pub(crate) fn resolve_path_pattern_expr(
        &mut self,
        pattern_expr: &PathPatternExpr,
    ) -> Result<BoundPathPatternExpr, BindError> {
        match pattern_expr {
            PathPatternExpr::Union(list) => {
                let resolved = list
                    .iter()
                    .map(|e| self.resolve_path_pattern_expr(e.value()))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(BoundPathPatternExpr::Union(resolved))
            }
            PathPatternExpr::Alternation(list) => {
                let resolved = list
                    .iter()
                    .map(|e| self.resolve_path_pattern_expr(e.value()))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(BoundPathPatternExpr::Alternation(resolved))
            }
            PathPatternExpr::Concat(list) => {
                let resolved = list
                    .iter()
                    .map(|e| self.resolve_path_pattern_expr(e.value()))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(BoundPathPatternExpr::Concat(resolved))
            }
            PathPatternExpr::Quantified { path, quantifier } => {
                return Err(BindError::NotSupported("Quantified".to_string()));
            }
            PathPatternExpr::Grouped(group) => {
                return Err(BindError::NotSupported("Grouped".to_string()));
            }
            PathPatternExpr::Pattern(pattern) => match pattern {
                ElementPattern::Node(node) => Ok(BoundPathPatternExpr::Pattern(
                    BoundElementPattern::Node(self.resolve_element_pattern_filler(true, node)?),
                )),
                ElementPattern::Edge { kind, filler } => {
                    Ok(BoundPathPatternExpr::Pattern(BoundElementPattern::Edge {
                        kind: kind.clone(),
                        filler: self.resolve_element_pattern_filler(false, filler)?,
                    }))
                }
            },
            PathPatternExpr::Optional(pattern) => Ok(BoundPathPatternExpr::Optional(Box::new(
                self.resolve_path_pattern_expr(pattern.value())?,
            ))),
        }
    }

    pub(crate) fn resolve_element_pattern_filler(
        &mut self,
        is_node: bool,
        filler: &ElementPatternFiller,
    ) -> Result<BoundElementPatternFiller, BindError> {
        let variable = filler.variable.as_ref().map(|sp| sp.value().clone());

        let label = filler
            .label
            .as_ref()
            .map(|sp| self.resolve_label_expr(sp.value()))
            .transpose()?;
        // currently, we only consider simple label.
        let label_id = match label {
            None => return Err(BindError::NotSupported("Empty Label".to_string())),
            Some(ref label_expr) => {
                match label_expr {
                    BoundLabelExpr::Label(label_id) => label_id,
                    _ => return Err(BindError::NotSupported("Label expr".to_string())),
                }
            }
        };
        let predict = filler
            .predicate
            .as_ref()
            .map(|sp| self.resolve_element_pattern_predicate(is_node, &label_id, sp.value()))
            .transpose()?;
        
        if variable.as_ref().is_some() {
            self.variable_context.insert(variable.as_ref().unwrap().clone(), label_id.clone());
        }
        Ok( BoundElementPatternFiller {
            variable:variable.clone(), label, predicate: predict,
        })
    }

    pub(crate) fn resolve_element_pattern_predicate(
        &self,
        is_node: bool,
        label_id: &LabelId,
        predicate: &ElementPatternPredicate,
    ) -> Result<BoundElementPatternPredicate, BindError> {
        match predicate {
            ElementPatternPredicate::Where(expr) => Ok(BoundElementPatternPredicate::Where(
                self.resolve_expr(expr.value())?,
            )),
            ElementPatternPredicate::Property(field_or_property_vec) => {
                enum GraphElementType  {
                    Vertex(VertexTypeRef),
                    Edge(EdgeTypeRef),
                }
                let graph = self.graph.as_ref().ok_or(BindError::GraphNotSpecify)?;
                let graph_type = graph.graph_type();

                let label_set = LabelSet::new([label_id.clone()]);
                let element_type = if is_node {
                    graph_type.get_vertex_type(&label_set).map(|opt| opt.map(GraphElementType::Vertex))
                } else {
                    graph_type.get_edge_type(&label_set).map(|opt| opt.map(GraphElementType::Edge))
                }.map_err(|e| BindError::External(Box::new(e)))?
                    .ok_or_else(||BindError::LabelNotFound(label_id.to_string()))?;
                let mut field_vec = vec![];
                for field in field_or_property_vec {
                    let field_name = &field.value().name;
                    let property = match &element_type {
                        GraphElementType::Vertex(graph_type) => {graph_type.get_property(field_name.value())},
                        GraphElementType::Edge(graph_type) => {graph_type.get_property(field_name.value())},
                    }.map_err(|e| BindError::External(Box::new(e)))?
                        .ok_or_else(||BindError::InvalidField(field_name.value().to_string()))?;
                    let value = self.resolve_expr(field.value().value.value())?;
                    field_vec.push(BoundFieldOrProperty {
                        id: PropertyCatalogRef {
                            name: field_name.value().clone(),
                            property_ref: property,
                        },
                        value,
                    })
                }
                Ok(BoundElementPatternPredicate::Property(field_vec))
            }
        }
    }

    pub(crate) fn resolve_label_expr(&self, label_expr: &LabelExpr) -> BindResult<BoundLabelExpr> {
        match label_expr {
            LabelExpr::Wildcard => Ok(BoundLabelExpr::Wildcard),
            LabelExpr::Conjunction(lhs, rhs) => Ok(BoundLabelExpr::Conjunction(
                Box::new(self.resolve_label_expr(lhs.value())?),
                Box::new(self.resolve_label_expr(rhs.value())?),
            )),
            LabelExpr::Disjunction(lhs, rhs) => Ok(BoundLabelExpr::Disjunction(
                Box::new(self.resolve_label_expr(lhs.value())?),
                Box::new(self.resolve_label_expr(rhs.value())?),
            )),
            LabelExpr::Negation(expr) => Ok(BoundLabelExpr::Negation(Box::new(
                self.resolve_label_expr(expr.value())?,
            ))),
            LabelExpr::Label(label) => Ok(BoundLabelExpr::Label(if self.graph.is_none() {
                return Err(BindError::GraphNotSpecify);
            } else {
                let graph_type = self.graph.as_ref().unwrap().graph_type();
                let label_id = graph_type
                    .get_label_id(label)
                    .map_err(|e| BindError::External(Box::new(e)))?;
                if label_id.is_none() {
                    return Err(BindError::LabelNotFound(label.to_string()));
                }
                label_id.unwrap()
            })),
        }
    }

    pub(crate) fn resolve_graph_pattern(
        &mut self,
        graph_pattern: &GraphPattern,
    ) -> BindResult<BoundGraphPattern> {
        let match_mode = graph_pattern.match_mode.as_ref().map(|v| v.value().clone());
        let patterns = graph_pattern
            .patterns
            .iter()
            .map(|v| self.resolve_path_pattern(v.value()))
            .collect::<Result<Vec<_>, _>>()?;

        let keep = graph_pattern.keep.as_ref().map(|v| v.value().clone());

        let where_clause = match &graph_pattern.where_clause {
            Some(expr) => Some(self.resolve_expr(expr.value())?),
            None => None,
        };

        Ok(BoundGraphPattern {
            match_mode,
            patterns,
            keep,
            where_clause,
        })
    }

    pub(crate) fn resolve_return_statement(
        &self,
        ret: &ReturnStatement,
    ) -> Result<BoundReturnStatement, BindError> {
        let bound_return = match ret.items.value() {
            Return::Items(items) => {
                let mut return_items = vec![];
                for item in items {
                    let return_item = item.value();
                    let bound_expr = self.resolve_expr(&return_item.value.value())?;
                    let alias = return_item.alias.clone();
                    return_items.push(BoundReturnItem {
                        value: bound_expr,
                        alias: alias.map(|sp| sp.value().clone()),
                    });
                }
                BoundReturn::Items(return_items)
            }

            Return::All => BoundReturn::All,
        };
        Ok(BoundReturnStatement {
            quantifier: ret.clone().quantifier.map(|q| q.value().clone()),
            items: bound_return,
            group_by: ret.clone().group_by.map(|g| g.value().clone()),
        })
    }

    pub(crate) fn resolve_focused_linear_query_parts(
        &mut self,
        part: &FocusedLinearQueryStatementPart,
    ) -> Result<BoundFocusedLinearQueryStatementPart, BindError> {
        let bound_graph_expr = self.resolve_graph_expr(part.use_graph.value())?;
        match &bound_graph_expr {
            // Set current used graph ref
            BoundGraphExpr::Ref(graph_ref) => {
                self.graph = Option::from(graph_ref.graph_ref.clone());
            }
            _ => {}
        }
        let mut vec_bound_stmt = vec![];
        for stmt in part.statements.iter() {
            vec_bound_stmt.push(self.resolve_simple_query_statement(stmt.value())?);
        }
        Ok(BoundFocusedLinearQueryStatementPart {
            use_graph: bound_graph_expr,
            statements: vec_bound_stmt,
        })
    }

    pub(crate) fn resolve_result_statement(
        &self,
        stmt: &ResultStatement,
    ) -> Result<BoundResultStatement, BindError> {
        match stmt {
            ResultStatement::Return {
                statement,
                order_by,
            } => {
                // TODO: Hande order by.
                Ok(BoundResultStatement::Return {
                    statement: Box::new(self.resolve_return_statement(statement.value())?),
                })
            }
            ResultStatement::Finish => Ok(BoundResultStatement::Finish),
        }
    }

    pub(crate) fn resolve_focused_linear_query_statement(
        &mut self,
        query: &FocusedLinearQueryStatement,
    ) -> Result<BoundFocusedLinearQueryStatement, BindError> {
        match query {
            FocusedLinearQueryStatement::Parts { parts, result } => {
                let mut vec_bound_stmt = vec![];
                for part in parts {
                    vec_bound_stmt.push(self.resolve_focused_linear_query_parts(part.value())?)
                }
                let result = self.resolve_result_statement(result.value())?;
                Ok(BoundFocusedLinearQueryStatement::Parts {
                    parts: vec_bound_stmt,
                    result,
                })
            }
            FocusedLinearQueryStatement::Result { use_graph, result } => {
                let use_graph_expr = self.resolve_graph_expr(use_graph.value())?;
                let result = self.resolve_result_statement(result.value())?;
                Ok(BoundFocusedLinearQueryStatement::Result {
                    use_graph: use_graph_expr,
                    result,
                })
            }
            FocusedLinearQueryStatement::Nested { use_graph, query } => {
                let use_graph_expr = self.resolve_graph_expr(use_graph.value())?;
                let procedure = self.bind_procedure(
                    query.value(),
                    self.catalog.clone(),
                    Some(self.schema.clone()),
                )?;
                Ok(BoundFocusedLinearQueryStatement::Nested {
                    use_graph: use_graph_expr,
                    query: Box::new(procedure),
                })
            }
            FocusedLinearQueryStatement::Select {} => return Err(BindError::ErrorCur),
        }
    }

    pub(crate) fn resolve_ambient_linear_query_statement(
        &mut self,
        stmt: &AmbientLinearQueryStatement,
    ) -> Result<BoundAmbientLinearQueryStatement, BindError> {
        match stmt {
            AmbientLinearQueryStatement::Parts { parts, result } => {
                let mut simple_query_vec = vec![];
                for stmt in parts {
                    let mut simple_query_stmt =
                        self.resolve_simple_query_statement(stmt.value())?;
                    simple_query_vec.push(simple_query_stmt);
                }
                let bound_return = match result.value() {
                    ResultStatement::Return {
                        statement,
                        order_by,
                    } => {
                        let bound_return = self.resolve_return_statement(statement.value())?;
                        BoundResultStatement::Return {
                            statement: Box::new(bound_return),
                        }
                    }
                    ResultStatement::Finish => BoundResultStatement::Finish,
                };
                Ok(BoundAmbientLinearQueryStatement::Parts {
                    parts: simple_query_vec,
                    result: bound_return,
                })
            }
            AmbientLinearQueryStatement::Nested(nested_query) => {
                return Err(BindError::NotSupported("NestedQuery".to_string()));
            }
        }
    }

    pub(crate) fn resolve_linear_query_statement(
        &mut self,
        query: &LinearQueryStatement,
    ) -> BindResult<BoundLinearQueryStatement> {
        match query {
            LinearQueryStatement::Focused(query) => {
                let stmt = self.resolve_focused_linear_query_statement(query)?;
                Ok(BoundLinearQueryStatement::Focused(stmt))
            }
            LinearQueryStatement::Ambient(query) => {
                let stmt = self.resolve_ambient_linear_query_statement(query)?;
                Ok(BoundLinearQueryStatement::Ambient(stmt))
            }
        }
    }

    pub(crate) fn resolve_composite_query_statement(
        &mut self,
        query: &CompositeQueryStatement,
    ) -> Result<BoundCompositeQueryStatement, BindError> {
        match query {
            CompositeQueryStatement::Primary(primary_query) => {
                let stmt = self.resolve_linear_query_statement(primary_query)?;
                Ok(BoundCompositeQueryStatement::Primary(stmt))
            }
            CompositeQueryStatement::Conjunction {
                conjunction,
                left,
                right,
            } => {
                let left_stmt = self.resolve_composite_query_statement(left.value())?;
                let right_stmt = self.resolve_composite_query_statement(right.value())?;
                Ok(BoundCompositeQueryStatement::Conjunction {
                    conjunction: conjunction.value().clone(),
                    left: Box::new(left_stmt.clone()),
                    right: Box::new(right_stmt.clone()),
                })
            }
        }
    }
}
