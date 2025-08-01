use winnow::combinator::{alt, dispatch, fail, opt, peek, preceded};
use winnow::{ModalResult, Parser};

use super::lexical::{character_string_literal, general_parameter_reference};
use super::object_expr::graph_expression;
use super::object_ref::schema_reference;
use crate::ast::*;
use crate::lexer::TokenKind;
use crate::parser::token::{TokenStream, any};
use crate::parser::utils::ToSpanned;
use crate::span::Spanned;

pub fn session_set_command(input: &mut TokenStream) -> ModalResult<Spanned<SessionSet>> {
    preceded(
        (TokenKind::Session, TokenKind::Set),
        dispatch! {peek(any);
            TokenKind::Schema => session_set_schema_clause.map(SessionSet::Schema),
            TokenKind::Time => session_set_time_zone_clause.map(SessionSet::TimeZone),
            TokenKind::Binding | TokenKind::Table | TokenKind::Value => session_set_parameter_clause.map(SessionSet::Parameter),
            TokenKind::Property | TokenKind::Graph => alt(
                (session_set_graph_clause.map(SessionSet::Graph), session_set_parameter_clause.map(SessionSet::Parameter)),
            ),
            _ => fail
        },
    )
    .spanned()
    .parse_next(input)
}

pub fn session_set_schema_clause(input: &mut TokenStream) -> ModalResult<Spanned<SchemaRef>> {
    preceded(TokenKind::Schema, schema_reference).parse_next(input)
}

pub fn session_set_graph_clause(input: &mut TokenStream) -> ModalResult<Spanned<GraphExpr>> {
    preceded(
        (opt(TokenKind::Property), TokenKind::Graph),
        graph_expression,
    )
    .parse_next(input)
}

pub fn session_set_time_zone_clause(
    input: &mut TokenStream,
) -> ModalResult<Spanned<StringLiteral>> {
    preceded((TokenKind::Time, TokenKind::Zone), character_string_literal).parse_next(input)
}

pub fn session_set_parameter_clause(
    input: &mut TokenStream,
) -> ModalResult<Spanned<SessionSetParameter>> {
    fail(input)
}

pub fn session_reset_command(input: &mut TokenStream) -> ModalResult<Spanned<SessionReset>> {
    preceded(
        (TokenKind::Session, TokenKind::Reset),
        opt(session_reset_arguments),
    )
    .map(SessionReset)
    .spanned()
    .parse_next(input)
}

pub fn session_reset_arguments(input: &mut TokenStream) -> ModalResult<Spanned<SessionResetArgs>> {
    dispatch! {peek(any);
        TokenKind::All | TokenKind::Parameters | TokenKind::Characteristics => {
            preceded(
                opt(TokenKind::All),
                alt((
                    TokenKind::Parameters.value(SessionResetArgs::AllParameters),
                    TokenKind::Characteristics.value(SessionResetArgs::AllCharacteristics),
                )),
            )
        },
        TokenKind::Schema => TokenKind::Schema.value(SessionResetArgs::Schema),
        TokenKind::Property | TokenKind::Graph => {
            (opt(TokenKind::Property), TokenKind::Graph).value(SessionResetArgs::Graph)
        },
        TokenKind::Time => (TokenKind::Time, TokenKind::Zone).value(SessionResetArgs::TimeZone),
        TokenKind::Parameter | TokenKind::GeneralParameterReference(_) => preceded(
            opt(TokenKind::Parameter),
            session_parameter_specification.map(SessionResetArgs::Parameter),
        ),
        _ => fail
    }
    .spanned()
    .parse_next(input)
}

pub fn session_close_command(input: &mut TokenStream) -> ModalResult<()> {
    (TokenKind::Session, TokenKind::Close)
        .void()
        .parse_next(input)
}

pub fn session_parameter_specification(input: &mut TokenStream) -> ModalResult<Spanned<Ident>> {
    general_parameter_reference.parse_next(input)
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use insta::assert_yaml_snapshot;

    use super::*;
    use crate::parser::utils::parse;

    #[test]
    fn test_session_set_command_schema() {
        let parsed = parse!(session_set_command, "session set schema /");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_session_set_command_time_zone() {
        let parsed = parse!(session_set_command, "session set time zone \"UTC\"");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_session_set_command_graph() {
        let parsed = parse!(session_set_command, "session set graph /a");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_session_reset_command_1() {
        let parsed = parse!(session_reset_command, "session reset");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_session_reset_command_2() {
        let parsed = parse!(session_reset_command, "session reset all parameters");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_session_reset_command_3() {
        let parsed = parse!(session_reset_command, "session reset schema");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_session_reset_command_4() {
        let parsed = parse!(session_reset_command, "session reset property graph");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_session_reset_command_5() {
        let parsed = parse!(session_reset_command, "session reset time zone");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_session_reset_command_6() {
        let parsed = parse!(session_reset_command, "session reset parameter $abc");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_session_close_command() {
        let parsed = parse!(session_close_command, "session close");
        assert!(parsed.is_some());
    }
}
