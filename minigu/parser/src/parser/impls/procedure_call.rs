use winnow::combinator::{dispatch, fail, opt, peek, preceded, separated, seq};
use winnow::{ModalResult, Parser};

use super::object_ref::procedure_reference;
use crate::ast::{CallProcedureStatement, InlineProcedureCall, NamedProcedureCall, ProcedureCall};
use crate::lexer::TokenKind;
use crate::parser::impls::common::yield_clause;
use crate::parser::impls::value_expr::value_expression;
use crate::parser::token::{TokenStream, any};
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
    dispatch! {peek(any);
        TokenKind::LeftParen | TokenKind::LeftBrace => inline_procedure_call.map_inner(ProcedureCall::Inline),
        _ => named_procedure_call.map_inner(ProcedureCall::Named),
    }
    .parse_next(input)
}

pub fn inline_procedure_call(input: &mut TokenStream) -> ModalResult<Spanned<InlineProcedureCall>> {
    fail(input)
}

pub fn named_procedure_call(input: &mut TokenStream) -> ModalResult<Spanned<NamedProcedureCall>> {
    seq! {NamedProcedureCall {
        name: procedure_reference,
        _: TokenKind::LeftParen,
        args: separated(0.., value_expression, TokenKind::Comma),
        _: TokenKind::RightParen,
        yield_clause: opt(yield_clause),
    }}
    .spanned()
    .parse_next(input)
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use insta::assert_yaml_snapshot;

    use super::*;
    use crate::parser::utils::parse;

    #[test]
    fn test_call_procedure_statement() {
        let parsed = parse!(
            call_procedure_statement,
            "optional call /a/b/proc(1, 2, 3) yield a as a1, b as b1"
        );
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_named_procedure_call_1() {
        let parsed = parse!(named_procedure_call, "proc(1, \"abc\")");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_named_procedure_call_2() {
        let parsed = parse!(named_procedure_call, "/a/b.proc() yield a as a1, b as b1");
        assert_yaml_snapshot!(parsed);
    }
}
