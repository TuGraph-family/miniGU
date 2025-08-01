use std::sync::Arc;

use gql_parser::ast::{
    AmbientLinearQueryStatement, CompositeQueryStatement, FocusedLinearQueryStatement,
    FocusedLinearQueryStatementPart, LinearQueryStatement, MatchStatement,
    NullOrdering as AstNullOrdering, OrderByAndPageStatement, Ordering, QueryConjunction,
    ResultStatement, Return, ReturnStatement, SetOp, SetOpKind, SetQuantifier,
    SimpleQueryStatement, SortSpec,
};
use itertools::Itertools;
use minigu_common::data_type::{DataField, DataSchema, DataSchemaRef};
use minigu_common::error::not_implemented;
use minigu_common::ordering::{NullOrdering, SortOrdering};

use super::Binder;
use super::error::{BindError, BindResult};
use crate::bound::{
    BoundCompositeQueryStatement, BoundExpr, BoundLinearQueryStatement,
    BoundOrderByAndPageStatement, BoundQueryConjunction, BoundResultStatement,
    BoundReturnStatement, BoundSetOp, BoundSetOpKind, BoundSetQuantifier,
    BoundSimpleQueryStatement, BoundSortSpec,
};

impl Binder<'_> {
    pub fn bind_composite_query_statement(
        &mut self,
        statement: &CompositeQueryStatement,
    ) -> BindResult<BoundCompositeQueryStatement> {
        match statement {
            CompositeQueryStatement::Conjunction { .. } => {
                not_implemented("query conjunction", None)
            }
            CompositeQueryStatement::Primary(statement) => {
                let statement = self.bind_linear_query_statement(statement)?;
                Ok(BoundCompositeQueryStatement::Primary(statement))
            }
        }
    }

    pub fn bind_linear_query_statement(
        &mut self,
        statement: &LinearQueryStatement,
    ) -> BindResult<BoundLinearQueryStatement> {
        match statement {
            LinearQueryStatement::Focused(statement) => {
                self.bind_focused_linear_query_statement(statement)
            }
            LinearQueryStatement::Ambient(statement) => {
                self.bind_ambient_linear_query_statement(statement)
            }
        }
    }

    pub fn bind_focused_linear_query_statement(
        &mut self,
        statement: &FocusedLinearQueryStatement,
    ) -> BindResult<BoundLinearQueryStatement> {
        match statement {
            FocusedLinearQueryStatement::Parts { parts, result } => {
                let statements = parts
                    .iter()
                    .map(|p| self.bind_focused_linear_query_statement_part(p.value()))
                    .reduce(|a, b| {
                        let mut a = a?;
                        a.extend(b?);
                        Ok(a)
                    })
                    .transpose()?
                    .unwrap_or_default();
                let result = self.bind_result_statement(result.value())?;
                Ok(BoundLinearQueryStatement::Query { statements, result })
            }
            FocusedLinearQueryStatement::Result { use_graph, result } => {
                let _graph = self.bind_graph_expr(use_graph.value())?;
                let result = self.bind_result_statement(result.value())?;
                Ok(BoundLinearQueryStatement::Query {
                    statements: vec![],
                    result,
                })
            }
            FocusedLinearQueryStatement::Nested { use_graph, query } => {
                let _graph = self.bind_graph_expr(use_graph.value())?;
                let query = self.bind_procedure(query.value())?;
                Ok(BoundLinearQueryStatement::Nested(Box::new(query)))
            }
            FocusedLinearQueryStatement::Select { .. } => not_implemented("select statement", None),
        }
    }

    pub fn bind_focused_linear_query_statement_part(
        &mut self,
        part: &FocusedLinearQueryStatementPart,
    ) -> BindResult<Vec<BoundSimpleQueryStatement>> {
        let graph = self.bind_graph_expr(part.use_graph.value())?;
        self.current_graph = Some(graph);
        part.statements
            .iter()
            .map(|s| self.bind_simple_query_statement(s.value()))
            .try_collect()
    }

    pub fn bind_ambient_linear_query_statement(
        &mut self,
        statement: &AmbientLinearQueryStatement,
    ) -> BindResult<BoundLinearQueryStatement> {
        match statement {
            AmbientLinearQueryStatement::Parts { parts, result } => {
                let statements = parts
                    .iter()
                    .map(|p| self.bind_simple_query_statement(p.value()))
                    .try_collect()?;
                let result = self.bind_result_statement(result.value())?;
                Ok(BoundLinearQueryStatement::Query { statements, result })
            }
            AmbientLinearQueryStatement::Nested(query) => self
                .bind_procedure(query.value())
                .map(Box::new)
                .map(BoundLinearQueryStatement::Nested),
        }
    }

    pub fn bind_simple_query_statement(
        &mut self,
        statement: &SimpleQueryStatement,
    ) -> BindResult<BoundSimpleQueryStatement> {
        match statement {
            SimpleQueryStatement::Match(statement) => todo!(),
            SimpleQueryStatement::Call(statement) => {
                let statement = self.bind_call_procedure_statement(statement)?;
                let schema = statement
                    .schema()
                    .ok_or_else(|| BindError::DataSchemaNotProvided(statement.name()))?;
                if let Some(active_schema) = &mut self.active_data_schema {
                    todo!()
                } else {
                    self.active_data_schema = Some(schema.as_ref().clone());
                }
                Ok(BoundSimpleQueryStatement::Call(statement))
            }
            SimpleQueryStatement::OrderByAndPage(_) => {
                not_implemented("standalone order by and page statement", None)
            }
        }
    }

    pub fn bind_match_statement(&mut self, statement: &MatchStatement) -> BindResult<()> {
        match statement {
            MatchStatement::Simple(table) => todo!(),
            MatchStatement::Optional(_) => not_implemented("optional match statement", None),
        }
    }

    pub fn bind_result_statement(
        &mut self,
        statement: &ResultStatement,
    ) -> BindResult<BoundResultStatement> {
        match statement {
            ResultStatement::Finish => Ok(BoundResultStatement::Finish),
            ResultStatement::Return {
                statement,
                order_by,
            } => {
                let statement = self.bind_return_statement(statement.value())?;
                self.active_data_schema = Some(statement.schema.as_ref().clone());
                let order_by_and_page = order_by
                    .as_ref()
                    .map(|o| self.bind_order_by_and_page_statement(o.value()))
                    .transpose()?;
                Ok(BoundResultStatement::Return {
                    statement,
                    order_by_and_page,
                })
            }
        }
    }

    pub fn bind_return_statement(
        &self,
        statement: &ReturnStatement,
    ) -> BindResult<BoundReturnStatement> {
        let quantifier = statement
            .quantifier
            .as_ref()
            .map(|q| bind_set_quantifier(q.value()));
        let (items, schema) = self.bind_return(statement.items.value())?;
        Ok(BoundReturnStatement {
            quantifier,
            items,
            schema,
        })
    }

    pub fn bind_return(&self, ret: &Return) -> BindResult<(Option<Vec<BoundExpr>>, DataSchemaRef)> {
        match ret {
            Return::Items(items) => {
                let mut fields = Vec::new();
                let mut exprs = Vec::new();
                for item in items {
                    let item = item.value();
                    let expr = self.bind_value_expression(item.value.value())?;
                    let name = if let Some(alias) = &item.alias {
                        alias.value().to_string()
                    } else {
                        expr.to_string()
                    };
                    fields.push(DataField::new(
                        name,
                        expr.logical_type.clone(),
                        expr.nullable,
                    ));
                    exprs.push(expr);
                }
                let schema = Arc::new(DataSchema::new(fields));
                Ok((Some(exprs), schema))
            }
            Return::All => {
                let schema = self
                    .active_data_schema
                    .as_ref()
                    .ok_or_else(|| BindError::NoColumnInReturnStatement)?
                    .clone();
                Ok((None, Arc::new(schema)))
            }
        }
    }

    pub fn bind_order_by_and_page_statement(
        &self,
        order_by_and_page: &OrderByAndPageStatement,
    ) -> BindResult<BoundOrderByAndPageStatement> {
        let order_by = order_by_and_page
            .order_by
            .iter()
            .map(|s| self.bind_sort_spec(s.value()))
            .try_collect()?;
        let offset = order_by_and_page
            .offset
            .as_ref()
            .map(|o| self.bind_non_negative_integer(o.value()))
            .transpose()?
            .map(|o| o.to_usize());
        let limit = order_by_and_page
            .limit
            .as_ref()
            .map(|l| self.bind_non_negative_integer(l.value()))
            .transpose()?
            .map(|l| l.to_usize());
        Ok(BoundOrderByAndPageStatement {
            order_by,
            offset,
            limit,
        })
    }

    pub fn bind_sort_spec(&self, sort_spec: &SortSpec) -> BindResult<BoundSortSpec> {
        let key = self.bind_value_expression(sort_spec.key.value())?;
        let ordering = sort_spec
            .ordering
            .as_ref()
            .map(|o| bind_ordering(o.value()))
            .unwrap_or_default();
        let null_ordering = sort_spec
            .null_ordering
            .as_ref()
            .map(|o| bind_null_ordering(o.value()))
            .unwrap_or_default();
        Ok(BoundSortSpec {
            key,
            ordering,
            null_ordering,
        })
    }
}

pub fn bind_ordering(ordering: &Ordering) -> SortOrdering {
    match ordering {
        Ordering::Asc => SortOrdering::Ascending,
        Ordering::Desc => SortOrdering::Descending,
    }
}

pub fn bind_null_ordering(null_ordering: &AstNullOrdering) -> NullOrdering {
    match null_ordering {
        AstNullOrdering::First => NullOrdering::First,
        AstNullOrdering::Last => NullOrdering::Last,
    }
}

pub fn bind_query_conjunction(conjunction: &QueryConjunction) -> BindResult<BoundQueryConjunction> {
    match conjunction {
        QueryConjunction::SetOp(set_op) => Ok(BoundQueryConjunction::SetOp(bind_set_op(set_op))),
        QueryConjunction::Otherwise => Ok(BoundQueryConjunction::Otherwise),
    }
}

pub fn bind_set_op(set_op: &SetOp) -> BoundSetOp {
    let kind = bind_set_op_kind(set_op.kind.value());
    let quantifier = set_op
        .quantifier
        .as_ref()
        .map(|q| bind_set_quantifier(q.value()));
    BoundSetOp { kind, quantifier }
}

pub fn bind_set_quantifier(quantifier: &SetQuantifier) -> BoundSetQuantifier {
    match quantifier {
        SetQuantifier::Distinct => BoundSetQuantifier::Distinct,
        SetQuantifier::All => BoundSetQuantifier::All,
    }
}

pub fn bind_set_op_kind(kind: &SetOpKind) -> BoundSetOpKind {
    match kind {
        SetOpKind::Union => BoundSetOpKind::Union,
        SetOpKind::Except => BoundSetOpKind::Except,
        SetOpKind::Intersect => BoundSetOpKind::Intersect,
    }
}
