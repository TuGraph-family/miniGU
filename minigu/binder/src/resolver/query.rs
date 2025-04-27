use gql_parser::ast::{AmbientLinearQueryStatement, CompositeQueryStatement, ElementPattern, ElementPatternFiller, ElementPatternPredicate, FocusedLinearQueryStatement, FocusedLinearQueryStatementPart, GraphPattern, Ident, LabelExpr, LinearQueryStatement, MatchStatement, PathPattern, PathPatternExpr, PathPatternPrefix, ResultStatement, Return, ReturnStatement, SimpleQueryStatement};

use crate::error::{Error, NotSupportError};
use crate::program::bound_statement::common::{
    BoundElementPattern, BoundElementPatternFiller, BoundElementPatternPredicate,
    BoundFieldOrProperty, BoundLabelExpr, BoundPathPattern, BoundPathPatternPrefix, FieldId,
};
use crate::program::bound_statement::expr::BoundPathPatternExpr;
use crate::program::bound_statement::query::{BoundAmbientLinearQueryStatement, BoundCompositeQueryStatement, BoundFocusedLinearQueryStatement, BoundFocusedLinearQueryStatementPart, BoundGraphPattern, BoundGraphPatternBindingTable, BoundLinearQueryStatement, BoundMatchStatement, BoundResultStatement, BoundReturn, BoundReturnItem, BoundReturnStatement, BoundSimpleQueryStatement};
use crate::program::Binder;

impl Binder {
    pub(crate) fn resolve_simple_query_statement(
        &self,
        statement: &SimpleQueryStatement,
    ) -> Result<BoundSimpleQueryStatement, Error> {
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
                    return Err(Error::NotSupported("MatchStatement".to_string()));
                }
            },
            SimpleQueryStatement::Call(call) => Ok(BoundSimpleQueryStatement::Call(
                self.resolve_call_procedure(call)?,
            )),
        }
    }

    pub(crate) fn resolve_path_pattern(
        &self,
        pattern: &PathPattern,
    ) -> Result<BoundPathPattern, Error> {
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
    ) -> Result<BoundPathPatternPrefix, Error> {
        return Err(Error::ErrorCur);
    }

    pub(crate) fn resolve_path_pattern_expr(
        &self,
        pattern_expr: &PathPatternExpr,
    ) -> Result<BoundPathPatternExpr, Error> {
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
                let resolved = self.resolve_path_pattern_expr(path.value())?;
                Ok(BoundPathPatternExpr::Quantified {
                    path: Box::new(resolved),
                    quantifier: Box::new(quantifier.value().clone()),
                })
            }
            PathPatternExpr::Grouped(group) => {
                let resolved = self.resolve_path_pattern_expr(group.expr.value())?;
                // TODO: Handle group
                Ok(BoundPathPatternExpr::Grouped(group.clone()))
            }
            PathPatternExpr::Pattern(pattern) => match pattern {
                ElementPattern::Node(node) => Ok(BoundPathPatternExpr::Pattern(
                    BoundElementPattern::Node(self.resolve_element_pattern_filler(node)?),
                )),
                ElementPattern::Edge { kind, filler } => {
                    Ok(BoundPathPatternExpr::Pattern(BoundElementPattern::Edge {
                        kind: kind.clone(),
                        filler: self.resolve_element_pattern_filler(filler)?,
                    }))
                }
            },
            PathPatternExpr::Optional(pattern) => Ok(BoundPathPatternExpr::Optional(Box::new(
                self.resolve_path_pattern_expr(pattern.value())?,
            ))),
        }
    }

    pub(crate) fn resolve_element_pattern_filler(
        &self,
        filler: &ElementPatternFiller,
    ) -> Result<BoundElementPatternFiller, Error> {
        let variable = filler.variable.as_ref().map(|sp| sp.value().clone());
        let label = filler
            .label
            .as_ref()
            .map(|sp| self.resolve_label_expr(sp.value()))
            .transpose()?;
        let predict = filler
            .predicate
            .as_ref()
            .map(|sp| self.resolve_element_pattern_predicate(sp.value()))
            .transpose()?;
        Ok(BoundElementPatternFiller {
            variable,
            label,
            predicate: predict,
        })
    }

    // TODO: How to get Field Id from a field name? The graph Type? The Schema?
    pub(crate) fn resolve_field_id(&self, field: &Ident) -> Result<FieldId, Error> {
        Ok(1)
    }

    pub(crate) fn resolve_element_pattern_predicate(
        &self,
        predicate: &ElementPatternPredicate,
    ) -> Result<BoundElementPatternPredicate, Error> {
        match predicate {
            ElementPatternPredicate::Where(expr) => Ok(BoundElementPatternPredicate::Where(
                self.resolve_expr(expr.value())?,
            )),
            ElementPatternPredicate::Property(field_or_property_vec) => {
                let mut field_vec = vec![];
                for field in field_or_property_vec {
                    let id = self.resolve_field_id(field.value().name.value())?;
                    let expr = self.resolve_expr(field.value().value.value())?;
                    field_vec.push(BoundFieldOrProperty {
                        id: id,
                        value: expr,
                    })
                }
                Ok(BoundElementPatternPredicate::Property(field_vec))
            }
        }
    }

    pub(crate) fn resolve_label_expr(
        &self,
        label_expr: &LabelExpr,
    ) -> Result<BoundLabelExpr, Error> {
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
            LabelExpr::Label(label) => Ok(BoundLabelExpr::Label(
                // TODO: label to labelId
                1,
            )),
        }
    }

    pub(crate) fn resolve_graph_pattern(
        &self,
        graph_pattern: &GraphPattern,
    ) -> Result<BoundGraphPattern, Error> {
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
    ) -> Result<BoundReturnStatement, Error> {
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

    pub(crate) fn resolve_focused_linear_query_parts(&self, part: &FocusedLinearQueryStatementPart) -> Result<BoundFocusedLinearQueryStatementPart, Error> {
        let bound_graph_expr = self.resolve_graph_expr(part.use_graph.value())?;
        let mut vec_bound_stmt = vec![];
        for stmt in part.statements.iter() {
            vec_bound_stmt.push(self.resolve_simple_query_statement(stmt.value())?);
        }
        Ok(BoundFocusedLinearQueryStatementPart {
            use_graph: bound_graph_expr,
            statements: vec_bound_stmt,
        })
    }

    pub(crate) fn resolve_result_statement(&self, stmt: &ResultStatement) -> Result<BoundResultStatement, Error> {
        match stmt {
            ResultStatement::Return {statement, order_by} => {
                // TODO: Hande order by.
                Ok(BoundResultStatement::Return {
                    statement: Box::new(self.resolve_return_statement(statement.value())?)
                })
            }
            ResultStatement::Finish => Ok(BoundResultStatement::Finish),
        }
    }

    pub(crate) fn resolve_focused_linear_query_statement(
        &mut self,
        query:&FocusedLinearQueryStatement
    ) -> Result<BoundFocusedLinearQueryStatement, Error> {
        match query {
            FocusedLinearQueryStatement::Parts {parts, result} => {
                let mut vec_bound_stmt = vec![];
                for part in parts {
                    vec_bound_stmt.push(self.resolve_focused_linear_query_parts(part.value())?)
                }
                let result = self.resolve_result_statement(result.value())?;
                Ok(BoundFocusedLinearQueryStatement::Parts {
                    parts: vec_bound_stmt,
                    result,
                })
            },
            FocusedLinearQueryStatement::Result {use_graph, result} => {
                let use_graph_expr = self.resolve_graph_expr(use_graph.value())?;
                let result = self.resolve_result_statement(result.value())?;
                Ok(BoundFocusedLinearQueryStatement::Result {
                    use_graph: use_graph_expr,
                    result,
                })
            },
            FocusedLinearQueryStatement::Nested {use_graph, query} => {
                let use_graph_expr = self.resolve_graph_expr(use_graph.value())?;
                let procedure = self.bind_procedure(query.value())?;
                Ok(BoundFocusedLinearQueryStatement::Nested {
                    use_graph: use_graph_expr,
                    query: Box::new(procedure),
                })
            },
            FocusedLinearQueryStatement::Select{} => {
                return Err(Error::ErrorCur)
            }
        }
    }
    
    pub(crate) fn resolve_ambient_linear_query_statement(&self, stmt:&AmbientLinearQueryStatement) -> Result<BoundAmbientLinearQueryStatement, Error> {
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
                    },
                )
            }
            AmbientLinearQueryStatement::Nested(nested_query) => {
                return Err(Error::NotSupported("NestedQuery".to_string()));
            }
        }
        
    }

    pub(crate) fn resolve_linear_query_statement(
        &mut self,
        query: &LinearQueryStatement,
    ) -> Result<BoundLinearQueryStatement, Error> {
        match query {
            LinearQueryStatement::Focused(query) => {
                let stmt = self.resolve_focused_linear_query_statement(query)?;
                Ok(BoundLinearQueryStatement::Focused(stmt))
            }
            LinearQueryStatement::Ambient(query) => {
                let stmt = self.resolve_ambient_linear_query_statement(query)?;
                Ok(BoundLinearQueryStatement::Ambient(stmt))
            },
        }
    }

    pub (crate) fn resolve_composite_query_statement(
        &mut self,
        query: &CompositeQueryStatement,
    ) -> Result<BoundCompositeQueryStatement, Error> {
        match query {
            CompositeQueryStatement::Primary(primary_query) => {
                let stmt = self.resolve_linear_query_statement(primary_query)?;
                Ok(BoundCompositeQueryStatement::Primary(stmt))
            }
            CompositeQueryStatement::Conjunction {conjunction, left, right} => {
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
