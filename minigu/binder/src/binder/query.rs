use gql_parser::ast::{
    AmbientLinearQueryStatement, CompositeQueryStatement, FocusedLinearQueryStatement,
    LinearQueryStatement, MatchStatement, Procedure, QueryConjunction as AstQueryConjunction,
    ResultStatement, SetOp as AstSetOp, SetOpKind as AstSetOpKind,
    SetQuantifier as AstSetQuantifier, SimpleQueryStatement, Statement,
};
use itertools::Itertools;
use minigu_ir::bound::{
    BoundCompositeQueryStatement, BoundLinearQueryStatement, BoundSimpleQueryStatement,
    QueryConjunction, SetOp, SetOpKind, SetQuantifier,
};

use super::Binder;
use crate::error::{BindResult, not_implemented};

impl Binder {
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
                not_implemented("focused linear query statement parts", None)
            }
            FocusedLinearQueryStatement::Result { use_graph, result } => {
                not_implemented("focused linear query statement result", None)
            }
            FocusedLinearQueryStatement::Nested { .. } => {
                not_implemented("nested focused linear query statement", None)
            }
            FocusedLinearQueryStatement::Select { .. } => not_implemented("select statement", None),
        }
    }

    pub fn bind_ambient_linear_query_statement(
        &mut self,
        statement: &AmbientLinearQueryStatement,
    ) -> BindResult<BoundLinearQueryStatement> {
        match statement {
            AmbientLinearQueryStatement::Parts { parts, result } => {
                let statements: Vec<_> = parts
                    .iter()
                    .map(|p| self.bind_simple_query_statement(p.value()))
                    .try_collect()?;
                let result = self.bind_result_statement(result.value())?;
                todo!()
            }
            AmbientLinearQueryStatement::Nested(query) => {
                not_implemented("nested ambient linear query statement", None)
            }
        }
    }

    pub fn bind_simple_query_statement(
        &mut self,
        statement: &SimpleQueryStatement,
    ) -> BindResult<BoundSimpleQueryStatement> {
        match statement {
            SimpleQueryStatement::Match(statement) => todo!(),
            SimpleQueryStatement::Call(statement) => todo!(),
            SimpleQueryStatement::OrderByAndPage(statement) => todo!(),
        }
    }

    pub fn bind_match_statement(&mut self, statement: &MatchStatement) -> BindResult<()> {
        todo!()
    }

    pub fn bind_result_statement(&mut self, statement: &ResultStatement) -> BindResult<()> {
        todo!()
    }
}

pub fn bind_query_conjunction(conjunction: &AstQueryConjunction) -> BindResult<QueryConjunction> {
    match conjunction {
        AstQueryConjunction::SetOp(set_op) => Ok(QueryConjunction::SetOp(bind_set_op(set_op))),
        AstQueryConjunction::Otherwise => Ok(QueryConjunction::Otherwise),
    }
}

pub fn bind_set_op(set_op: &AstSetOp) -> SetOp {
    let kind = bind_set_op_kind(set_op.kind.value());
    let quantifier = set_op
        .quantifier
        .as_ref()
        .map(|q| bind_set_quantifier(q.value()));
    SetOp { kind, quantifier }
}

pub fn bind_set_quantifier(quantifier: &AstSetQuantifier) -> SetQuantifier {
    match quantifier {
        AstSetQuantifier::Distinct => SetQuantifier::Distinct,
        AstSetQuantifier::All => SetQuantifier::All,
    }
}

pub fn bind_set_op_kind(kind: &AstSetOpKind) -> SetOpKind {
    match kind {
        AstSetOpKind::Union => SetOpKind::Union,
        AstSetOpKind::Except => SetOpKind::Except,
        AstSetOpKind::Intersect => SetOpKind::Intersect,
    }
}
