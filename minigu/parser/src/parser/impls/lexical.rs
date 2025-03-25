use winnow::combinator::{cut_err, dispatch, empty, fail, peek};
use winnow::token::one_of;
use winnow::{ModalResult, Parser};

use crate::ast::{
    BooleanLiteral, Ident, Literal, StringLiteral, StringLiteralKind, UnsignedInteger,
    UnsignedIntegerKind, UnsignedNumericLiteral,
};
use crate::lexer::TokenKind;
use crate::parser::token::{Token, TokenStream, any};
use crate::parser::utils::{SpannedParserExt, ToSpanned, def_parser_alias};
use crate::span::Spanned;

pub fn regular_identifier(input: &mut TokenStream) -> ModalResult<Spanned<Ident>> {
    winnow::token::any
        .verify_map(|token: &Token| match &token.kind {
            &TokenKind::RegularIdentifier(name) => Some(name.into()),
            kind if kind.is_non_reserved_word() => Some(token.slice.into()),
            _ => None,
        })
        .spanned()
        .parse_next(input)
}

pub fn delimited_identifier(input: &mut TokenStream) -> ModalResult<Spanned<Ident>> {
    dispatch! {any;
        TokenKind::AccentQuoted(quoted) | TokenKind::DoubleQuoted(quoted) => {
            cut_err(empty.verify_map(|_| quoted.unescape()))
        },
        _ => fail,
    }
    .spanned()
    .parse_next(input)
}

pub fn identifier(input: &mut TokenStream) -> ModalResult<Spanned<Ident>> {
    dispatch! {peek(any);
        TokenKind::RegularIdentifier(_) => regular_identifier,
        kind if kind.is_non_reserved_word() => regular_identifier,
        TokenKind::AccentQuoted(_) | TokenKind::DoubleQuoted(_) => delimited_identifier,
        _ => fail
    }
    .parse_next(input)
}

def_parser_alias!(object_name, identifier, Spanned<Ident>);
def_parser_alias!(
    object_name_or_binding_variable,
    regular_identifier,
    Spanned<Ident>
);
def_parser_alias!(directory_name, identifier, Spanned<Ident>);
def_parser_alias!(schema_name, identifier, Spanned<Ident>);
def_parser_alias!(graph_name, identifier, Spanned<Ident>);
def_parser_alias!(delimited_graph_name, delimited_identifier, Spanned<Ident>);
def_parser_alias!(graph_type_name, identifier, Spanned<Ident>);
def_parser_alias!(node_type_name, identifier, Spanned<Ident>);
def_parser_alias!(edge_type_name, identifier, Spanned<Ident>);
def_parser_alias!(binding_table_name, identifier, Spanned<Ident>);
def_parser_alias!(
    delimited_binding_table_name,
    delimited_identifier,
    Spanned<Ident>
);
def_parser_alias!(procedure_name, identifier, Spanned<Ident>);
def_parser_alias!(label_name, identifier, Spanned<Ident>);
def_parser_alias!(property_name, identifier, Spanned<Ident>);
def_parser_alias!(field_name, identifier, Spanned<Ident>);
def_parser_alias!(element_variable, binding_variable, Spanned<Ident>);
def_parser_alias!(path_variable, binding_variable, Spanned<Ident>);
def_parser_alias!(subpath_variable, regular_identifier, Spanned<Ident>);
def_parser_alias!(binding_variable, regular_identifier, Spanned<Ident>);

pub fn unsigned_literal(input: &mut TokenStream) -> ModalResult<Spanned<Literal>> {
    dispatch! {peek(any);
        TokenKind::True | TokenKind::False | TokenKind::Unknown => {
            boolean_literal.map_inner(Literal::Boolean)
        },
        TokenKind::Null => TokenKind::Null.value(Literal::Null).spanned(),
        TokenKind::DoubleQuoted(_) | TokenKind::SingleQuoted(_) => {
            character_string_literal.map_inner(Literal::String)
        },
        kind if kind.is_prefix_of_numeric_literal() => {
            unsigned_numeric_literal.map_inner(Literal::Numeric)
        },
        _ => fail,
    }
    .parse_next(input)
}

pub fn unsigned_numeric_literal(
    input: &mut TokenStream,
) -> ModalResult<Spanned<UnsignedNumericLiteral>> {
    unsigned_integer
        .map(UnsignedNumericLiteral::Integer)
        .spanned()
        .parse_next(input)
}

pub fn unsigned_integer(input: &mut TokenStream) -> ModalResult<Spanned<UnsignedInteger>> {
    dispatch! {any;
        &TokenKind::UnsignedDecimalInteger(integer) => empty
            .value(UnsignedInteger {
                kind: UnsignedIntegerKind::Decimal,
                integer: integer.into(),
            }),
        &TokenKind::UnsignedOctalInteger(integer) => empty
            .value(UnsignedInteger {
                kind: UnsignedIntegerKind::Octal,
                integer: integer.into(),
            }),
        &TokenKind::UnsignedHexInteger(integer) => empty
            .value(UnsignedInteger {
                kind: UnsignedIntegerKind::Hex,
                integer: integer.into(),
            }),
        &TokenKind::UnsignedBinaryInteger(integer) => empty
            .value(UnsignedInteger {
                kind: UnsignedIntegerKind::Binary,
                integer: integer.into(),
            }),
        _ => fail,
    }
    .spanned()
    .parse_next(input)
}

pub fn unsigned_decimal_integer(input: &mut TokenStream) -> ModalResult<Spanned<UnsignedInteger>> {
    winnow::token::any
        .verify_map(|token: &Token| match token.kind {
            TokenKind::UnsignedDecimalInteger(integer) => Some(UnsignedInteger {
                kind: UnsignedIntegerKind::Decimal,
                integer: integer.into(),
            }),
            _ => None,
        })
        .spanned()
        .parse_next(input)
}

pub fn boolean_literal(input: &mut TokenStream) -> ModalResult<Spanned<BooleanLiteral>> {
    dispatch! {any;
        TokenKind::True => empty.value(BooleanLiteral::True),
        TokenKind::False => empty.value(BooleanLiteral::False),
        TokenKind::Unknown => empty.value(BooleanLiteral::Unknown),
        _ => fail
    }
    .spanned()
    .parse_next(input)
}

pub fn general_parameter_reference(input: &mut TokenStream) -> ModalResult<Spanned<Ident>> {
    dispatch! {any;
        TokenKind::GeneralParameterReference(name) => {
            cut_err(empty.verify_map(|_| name.unescape()))
        },
        _ => fail,
    }
    .spanned()
    .parse_next(input)
}

pub fn substituted_parameter_reference(input: &mut TokenStream) -> ModalResult<Spanned<Ident>> {
    dispatch! {any;
        TokenKind::SubstitutedParameterReference(name) => {
            cut_err(empty.verify_map(|_| name.unescape()))
        },
        _ => fail,
    }
    .spanned()
    .parse_next(input)
}

pub fn character_string_literal(input: &mut TokenStream) -> ModalResult<Spanned<StringLiteral>> {
    dispatch! {any;
        TokenKind::SingleQuoted(quoted) | TokenKind::DoubleQuoted(quoted) => {
            cut_err(empty.verify_map(|_| {
                Some(StringLiteral {
                    kind: StringLiteralKind::Char,
                    literal: quoted.unescape()?,
                })
            }))
        },
        _ => fail
    }
    .spanned()
    .parse_next(input)
}

pub fn node_synonym(input: &mut TokenStream) -> ModalResult<()> {
    one_of((TokenKind::Node, TokenKind::Vertex))
        .void()
        .parse_next(input)
}

pub fn edge_synonym(input: &mut TokenStream) -> ModalResult<()> {
    one_of((TokenKind::Edge, TokenKind::Relationship))
        .void()
        .parse_next(input)
}

pub fn edges_synonym(input: &mut TokenStream) -> ModalResult<()> {
    one_of((TokenKind::Edges, TokenKind::Relationships))
        .void()
        .parse_next(input)
}

pub fn implies(input: &mut TokenStream) -> ModalResult<()> {
    one_of((TokenKind::Implies, TokenKind::RightDoubleArrow))
        .void()
        .parse_next(input)
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use insta::assert_yaml_snapshot;

    use super::*;
    use crate::parser::utils::parse;

    #[test]
    fn test_identifier_1() {
        let parsed = parse!(identifier, "abcd");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_identifier_2() {
        let parsed = parse!(identifier, "`abcd`");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_identifier_3() {
        let parsed = parse!(identifier, "\"abcd\"");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_boolean_literal_1() {
        let parsed = parse!(boolean_literal, "true");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_boolean_literal_2() {
        let parsed = parse!(boolean_literal, "false");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_boolean_literal_3() {
        let parsed = parse!(boolean_literal, "unknown");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_unsigned_literal_1() {
        let parsed = parse!(unsigned_literal, "false");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_unsigned_literal_2() {
        let parsed = parse!(unsigned_literal, "null");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_character_string_literal() {
        let parsed = parse!(character_string_literal, "\"ab\\ncd\"");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_unsigned_integer_1() {
        let parsed = parse!(unsigned_integer, "123");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_unsigned_integer_2() {
        let parsed = parse!(unsigned_integer, "0o123");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_unsigned_integer_3() {
        let parsed = parse!(unsigned_integer, "0x123");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_unsigned_integer_4() {
        let parsed = parse!(unsigned_integer, "0b101");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_general_parameter_reference_1() {
        let parsed = parse!(general_parameter_reference, "$abc");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_general_parameter_reference_2() {
        let parsed = parse!(general_parameter_reference, "$\"abc\"");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_substituted_parameter_reference_1() {
        let parsed = parse!(substituted_parameter_reference, "$$abc");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_substituted_parameter_reference_2() {
        let parsed = parse!(substituted_parameter_reference, "$$\"abc\"");
        assert_yaml_snapshot!(parsed);
    }
}
