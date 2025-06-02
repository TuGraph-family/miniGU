use std::collections::HashMap;

use gql_parser::ast::{
    CompositeQueryStatement, LinearQueryStatement, Procedure,
    QueryConjunction as AstQueryConjunction, SetOp as AstSetOp, SetOpKind as AstSetOpKind,
    SetQuantifier as AstSetQuantifier, Statement,
};
use itertools::Itertools;

use super::Binder;
use crate::error::{BindResult, not_implemented};
use crate::procedure::procedure_spec::{BoundProcedure, BoundStatement};
use crate::procedure::query::{
    BoundCompositeQueryStatement, BoundLinearQueryStatement, QueryConjunction, SetOp, SetOpKind,
};
use crate::procedure::value_expr::SetQuantifier;

impl Binder {
    pub fn bind_composite_query_statement(
        &mut self,
        statement: &CompositeQueryStatement,
    ) -> BindResult<BoundCompositeQueryStatement> {
        match statement {
            CompositeQueryStatement::Conjunction { .. } => {
                not_implemented("query conjunction".into(), None)
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
        todo!()
    }

    pub fn bind_query_conjunction(
        &mut self,
        conjunction: &AstQueryConjunction,
    ) -> BindResult<QueryConjunction> {
        match conjunction {
            AstQueryConjunction::SetOp(set_op) => {
                Ok(QueryConjunction::SetOp(set_op.clone().into()))
            }
            AstQueryConjunction::Otherwise => Ok(QueryConjunction::Otherwise),
        }
    }
}

impl From<AstSetQuantifier> for SetQuantifier {
    fn from(quantifier: AstSetQuantifier) -> Self {
        match quantifier {
            AstSetQuantifier::Distinct => SetQuantifier::Distinct,
            AstSetQuantifier::All => SetQuantifier::All,
        }
    }
}

impl From<AstSetOpKind> for SetOpKind {
    fn from(kind: AstSetOpKind) -> Self {
        match kind {
            AstSetOpKind::Union => SetOpKind::Union,
            AstSetOpKind::Except => SetOpKind::Except,
            AstSetOpKind::Intersect => SetOpKind::Intersect,
        }
    }
}

impl From<AstSetOp> for SetOp {
    fn from(set_op: AstSetOp) -> Self {
        let kind = set_op.kind.value().clone().into();
        let quantifier = set_op.quantifier.as_ref().map(|q| q.value().clone().into());
        SetOp { kind, quantifier }
    }
}
