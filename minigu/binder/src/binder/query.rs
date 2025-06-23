use std::sync::Arc;

use gql_parser::ast::{
    AmbientLinearQueryStatement, CompositeQueryStatement, FocusedLinearQueryStatement,
    FocusedLinearQueryStatementPart, LinearQueryStatement,
    MatchStatement, OrderByAndPageStatement, QueryConjunction, ResultStatement, Return,
    ReturnStatement, SetOp, SetOpKind, SetQuantifier, SimpleQueryStatement,
};
use itertools::Itertools;
use minigu_common::data_type::{DataField, DataSchema};
use minigu_common::error::not_implemented;
use minigu_ir::bound::{
    BoundCompositeQueryStatement, BoundExpr, BoundLinearQueryStatement,
    BoundOrderByAndPageStatement, BoundQueryConjunction, BoundResultStatement,
    BoundReturnStatement, BoundSetOp, BoundSetOpKind, BoundSetQuantifier,
    BoundSimpleQueryStatement,
};

use super::Binder;
use crate::error::BindResult;

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
                Ok(BoundSimpleQueryStatement::Call(statement))
            }
            SimpleQueryStatement::OrderByAndPage(statement) => todo!(),
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
                let order_by = order_by
                    .as_ref()
                    .map(|o| self.bind_order_by_and_page_statement(o.value()))
                    .transpose()?;
                Ok(BoundResultStatement::Return {
                    statement,
                    order_by,
                })
            }
        }
    }

    pub fn bind_return_statement(
        &mut self,
        statement: &ReturnStatement,
    ) -> BindResult<BoundReturnStatement> {
        let quantifier = statement
            .quantifier
            .as_ref()
            .map(|q| bind_set_quantifier(q.value()));
        let items = self.bind_return(statement.items.value())?;
        let fields = items
            .iter()
            .map(|e| DataField::new(e.name.clone(), e.logical_type.clone(), false))
            .collect();
        let schema = Arc::new(DataSchema::new(fields));
        // TODO: Implement GROUP BY
        Ok(BoundReturnStatement {
            quantifier,
            items,
            schema,
        })
    }

    pub fn bind_return(&mut self, ret: &Return) -> BindResult<Vec<BoundExpr>> {
        match ret {
            Return::Items(items) => {
                let mut exprs = Vec::new();
                for item in items {
                    let item = item.value();
                    let expr = self.bind_value_expression(item.value.value())?;
                    // TODO: Handle item alias
                    exprs.push(expr);
                }
                Ok(exprs)
            }
            Return::All => Ok(self.context.exprs().cloned().collect()),
        }
    }

    pub fn bind_order_by_and_page_statement(
        &self,
        order_by: &OrderByAndPageStatement,
    ) -> BindResult<BoundOrderByAndPageStatement> {
        todo!()
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
