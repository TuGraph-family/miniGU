use winnow::combinator::{alt, delimited, fail, opt, preceded, repeat, separated, seq};
use winnow::{ModalResult, Parser};

use super::lexical::{binding_variable, field_name};
use crate::ast::{
    CallProcedureStatement, CatalogModifyingStatement, Ident, InlineProcedureCall,
    LinearCatalogModifyingStatement, NamedProcedureCall, Procedure, ProcedureCall, SchemaRef,
    Yield, YieldItem,
};
use crate::lexer::TokenKind;
use crate::parser::token::TokenStream;
use crate::parser::utils::{SpannedParserExt, ToSpanned};
use crate::span::Spanned;

pub fn call_procedure_statement(
    input: &mut TokenStream,
) -> ModalResult<Spanned<CallProcedureStatement>> {
    (
        opt(TokenKind::Optional),
        preceded(TokenKind::Call, procedure_call),
    )
        .map(|(optional, procedure)| CallProcedureStatement {
            optional: optional.is_some(),
            procedure,
        })
        .spanned()
        .parse_next(input)
}

pub fn procedure_call(input: &mut TokenStream) -> ModalResult<Spanned<ProcedureCall>> {
    alt((
        inline_procedure_call.map_inner(ProcedureCall::Inline),
        named_procedure_call.map_inner(ProcedureCall::Named),
    ))
    .parse_next(input)
}

pub fn inline_procedure_call(input: &mut TokenStream) -> ModalResult<Spanned<InlineProcedureCall>> {
    fail(input)
}

pub fn named_procedure_call(input: &mut TokenStream) -> ModalResult<Spanned<NamedProcedureCall>> {
    // procedureReference LEFT_PAREN procedureArgumentList? RIGHT_PAREN yieldClause?
    fail(input)
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use insta::assert_yaml_snapshot;

    use super::*;
    use crate::parser::utils::parse;
}
