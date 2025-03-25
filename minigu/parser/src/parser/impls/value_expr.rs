use winnow::combinator::{
    alt, delimited, dispatch, empty, fail, opt, peek, preceded, repeat, separated, separated_pair,
    seq,
};
use winnow::error::ParserError;
use winnow::stream::Stream;
use winnow::token::one_of;
use winnow::{ModalResult, Parser};

use super::lexical::{
    boolean_literal, general_parameter_reference, property_name, regular_identifier,
    unsigned_integer, unsigned_literal,
};
use crate::ast::*;
use crate::imports::{Box, Vec};
use crate::lexer::TokenKind;
use crate::parser::precedence::{Assoc, Precedence, precedence};
use crate::parser::token::{TokenStream, any};
use crate::parser::utils::{SpannedParserExt, ToSpanned, def_parser_alias};
use crate::span::Spanned;

macro_rules! predefined_value_function {
    ($name:expr, $arg:expr, $callback:expr) => {
        move |input: &mut TokenStream| {
            preceded(
                $name,
                delimited(TokenKind::LeftParen, $arg, TokenKind::RightParen),
            )
            .map($callback)
            .spanned()
            .parse_next(input)
        }
    };
}

const PREC_INIT: Precedence = 0;
const PREC_OR_XOR: Precedence = 1;
const PREC_AND: Precedence = 2;
const PREC_IS: Precedence = 3;
const PREC_NOT: Precedence = 4;
const PREC_CMP: Precedence = 5;
const PREC_CONCAT: Precedence = 6;
const PREC_ADD_SUB: Precedence = 7;
const PREC_MUL_DIV: Precedence = 8;
const PREC_PLUS_MINUS: Precedence = 9;

fn value_expression_prefix(input: &mut TokenStream) -> ModalResult<(Precedence, Spanned<UnaryOp>)> {
    dispatch! {any;
        TokenKind::Not => empty.value((PREC_NOT, UnaryOp::Not)),
        TokenKind::Plus => empty.value((PREC_PLUS_MINUS, UnaryOp::Plus)),
        TokenKind::Minus => empty.value((PREC_PLUS_MINUS, UnaryOp::Minus)),
        _ => fail
    }
    .spanned()
    .map(|Spanned((prec, op), span)| (prec, Spanned(op, span)))
    .parse_next(input)
}

enum SuffixOp {
    Is(Spanned<BooleanLiteral>),
    IsNot(Spanned<BooleanLiteral>),
}

fn value_expression_suffix(input: &mut TokenStream) -> ModalResult<(Precedence, SuffixOp)> {
    dispatch! {peek((any, any));
        (TokenKind::Is, TokenKind::Not) => {
            preceded((TokenKind::Is, TokenKind::Not), boolean_literal).map(|truth| (PREC_IS, SuffixOp::IsNot(truth)))
        },
        (TokenKind::Is, _) => {
            preceded(TokenKind::Is, boolean_literal).map(|truth| (PREC_IS, SuffixOp::Is(truth)))
        },
        _ => fail,
    }
    .parse_next(input)
}

fn value_expression_infix(
    input: &mut TokenStream,
) -> ModalResult<(Assoc, Precedence, Spanned<BinaryOp>)> {
    dispatch! {any;
        TokenKind::Or => empty.value((Assoc::Left, PREC_OR_XOR, BinaryOp::Or)),
        TokenKind::Xor => empty.value((Assoc::Left, PREC_OR_XOR, BinaryOp::Xor)),
        TokenKind::And => empty.value((Assoc::Left, PREC_AND, BinaryOp::And)),
        TokenKind::LeftAngleBracket => empty.value((Assoc::Left, PREC_CMP, BinaryOp::Lt)),
        TokenKind::LessThanOrEquals => empty.value((Assoc::Left, PREC_CMP, BinaryOp::Le)),
        TokenKind::RightAngleBracket => empty.value((Assoc::Left, PREC_CMP, BinaryOp::Gt)),
        TokenKind::GreaterThanOrEquals => empty.value((Assoc::Left, PREC_CMP, BinaryOp::Ge)),
        TokenKind::Equals => empty.value((Assoc::Left, PREC_CMP, BinaryOp::Eq)),
        TokenKind::NotEquals => empty.value((Assoc::Left, PREC_CMP, BinaryOp::Ne)),
        TokenKind::Concatenation => empty.value((Assoc::Left, PREC_CONCAT, BinaryOp::Concat)),
        TokenKind::Plus => empty.value((Assoc::Left, PREC_ADD_SUB, BinaryOp::Add)),
        TokenKind::Minus => empty.value((Assoc::Left, PREC_ADD_SUB, BinaryOp::Sub)),
        TokenKind::Asterisk => empty.value((Assoc::Left, PREC_MUL_DIV, BinaryOp::Mul)),
        TokenKind::Solidus => empty.value((Assoc::Left, PREC_MUL_DIV, BinaryOp::Div)),
        _ => fail,
    }
    .spanned()
    .map(|Spanned((assoc, prec, op), span)| (assoc, prec, Spanned(op, span)))
    .parse_next(input)
}

pub fn value_expression(input: &mut TokenStream) -> ModalResult<Spanned<Expr>> {
    precedence(
        PREC_INIT,
        value_expression_operand,
        value_expression_prefix,
        value_expression_suffix,
        value_expression_infix,
        |op, a| {
            let span = op.1.start..a.1.end;
            Ok(Spanned(
                Expr::Unary {
                    op,
                    child: Box::new(a),
                },
                span,
            ))
        },
        |a, op| match op {
            SuffixOp::Is(right) => {
                let span = a.1.start..right.1.end;
                Ok(Spanned(
                    Expr::Is {
                        left: Box::new(a),
                        right,
                    },
                    span,
                ))
            }
            SuffixOp::IsNot(right) => {
                let span = a.1.start..right.1.end;
                Ok(Spanned(
                    Expr::IsNot {
                        left: Box::new(a),
                        right,
                    },
                    span,
                ))
            }
        },
        |a, op, b| {
            let span = a.1.start..b.1.end;
            Ok(Spanned(
                Expr::Binary {
                    op,
                    left: Box::new(a),
                    right: Box::new(b),
                },
                span,
            ))
        },
    )
    .parse_next(input)
}

fn value_expression_operand(input: &mut TokenStream) -> ModalResult<Spanned<Expr>> {
    dispatch! {peek((any, opt(any)));
        (kind, Some(TokenKind::LeftParen))
            if kind.is_prefix_of_value_function() =>
        {
            value_function.map_inner(Expr::Function)
        },
        _ => {
            value_expression_primary
        }
    }
    .parse_next(input)
}

pub fn value_expression_primary_inner(input: &mut TokenStream) -> ModalResult<Spanned<Expr>> {
    dispatch! {peek(any);
        TokenKind::Path => path_value_constructor.map_inner(Expr::Path),
        TokenKind::Case | TokenKind::Coalesce | TokenKind::Nullif => case_expression,
        kind if kind.is_prefix_of_aggregate_function() => aggregate_function.map_inner(Expr::Aggregate),
        _ => unsigned_value_specification.map_inner(Expr::Value),
    }
    .parse_next(input)
}

pub fn value_expression_primary(input: &mut TokenStream) -> ModalResult<Spanned<Expr>> {
    let base = dispatch! {peek(any);
        TokenKind::LeftParen => parenthesized_value_expression,
        kind if kind.is_prefix_of_regular_identifier() => binding_variable_reference.map_inner(Expr::Variable),
        _ => value_expression_primary_inner,
    };
    (
        base,
        repeat(0.., preceded(TokenKind::Period, property_name)),
    )
        .map(|(source, trailing_names): (_, Vec<_>)| {
            if trailing_names.is_empty() {
                source.0
            } else {
                Expr::Property {
                    source: Box::new(source),
                    trailing_names,
                }
            }
        })
        .spanned()
        .parse_next(input)
}

pub fn parenthesized_value_expression(input: &mut TokenStream) -> ModalResult<Spanned<Expr>> {
    delimited(
        TokenKind::LeftParen,
        value_expression,
        TokenKind::RightParen,
    )
    .update_span()
    .parse_next(input)
}

pub fn non_parenthesized_value_expression_primary(
    input: &mut TokenStream,
) -> ModalResult<Spanned<Expr>> {
    alt((
        non_parenthesized_value_expression_primary_special_case,
        binding_variable_reference.map_inner(Expr::Variable),
        fail,
    ))
    .parse_next(input)
}

// TODO: Implement this using `value_expression_primary_inner`.
pub fn non_parenthesized_value_expression_primary_special_case(
    input: &mut TokenStream,
) -> ModalResult<Spanned<Expr>> {
    fail(input)
}

pub fn case_expression(input: &mut TokenStream) -> ModalResult<Spanned<Expr>> {
    dispatch! {peek(any);
        TokenKind::Nullif | TokenKind::Coalesce => {
            case_abbreviation.map_inner(Expr::Function)
        },
        _ => fail
    }
    .parse_next(input)
}

pub fn case_abbreviation(input: &mut TokenStream) -> ModalResult<Spanned<Function>> {
    dispatch! {peek(any);
        TokenKind::Nullif => predefined_value_function!(
            TokenKind::Nullif,
            (value_expression, value_expression),
            |(a, b)| CaseFunction::NullIf(Box::new(a), Box::new(b))
        ),
        TokenKind::Coalesce => {
            preceded(
                TokenKind::Coalesce,
                delimited(
                    TokenKind::LeftParen,
                    separated(1.., value_expression, TokenKind::Comma),
                    TokenKind::RightParen
                )
            )
            .map(CaseFunction::Coalesce)
            .spanned()
        },
        _ => fail
    }
    .map_inner(Function::Case)
    .parse_next(input)
}

pub fn set_quantifier(input: &mut TokenStream) -> ModalResult<Spanned<SetQuantifier>> {
    dispatch! {any;
        TokenKind::Distinct => empty.value(SetQuantifier::Distinct),
        TokenKind::All => empty.value(SetQuantifier::All),
        _ => fail
    }
    .spanned()
    .parse_next(input)
}

pub fn unsigned_value_specification(input: &mut TokenStream) -> ModalResult<Spanned<Value>> {
    dispatch! {peek(any);
        TokenKind::SessionUser => TokenKind::SessionUser.value(Value::SessionUser).spanned(),
        TokenKind::GeneralParameterReference(_) => {
            dynamic_parameter_specification.map_inner(Value::Parameter)
        },
        _ => unsigned_literal.map_inner(Value::Literal)
    }
    .parse_next(input)
}

pub fn non_negative_integer_specification(
    input: &mut TokenStream,
) -> ModalResult<Spanned<NonNegativeInteger>> {
    dispatch! {peek(any);
        kind if kind.is_prefix_of_unsigned_integer() => {
            unsigned_integer.map_inner(NonNegativeInteger::Integer)
        },
        TokenKind::GeneralParameterReference(_) => {
            dynamic_parameter_specification.map_inner(NonNegativeInteger::Parameter)
        },
        _ => fail
    }
    .parse_next(input)
}

def_parser_alias!(
    dynamic_parameter_specification,
    general_parameter_reference,
    Spanned<Ident>
);
def_parser_alias!(
    binding_variable_reference,
    regular_identifier,
    Spanned<Ident>
);

pub fn list_value_constructor(input: &mut TokenStream) -> ModalResult<Spanned<ListConstructor>> {
    seq! {ListConstructor {
        type_name: opt(list_value_type_name),
        _: TokenKind::LeftBracket,
        values: separated(0.., value_expression, TokenKind::Comma),
        _: TokenKind::RightBracket,
    }}
    .spanned()
    .parse_next(input)
}

pub fn list_value_type_name(input: &mut TokenStream) -> ModalResult<Spanned<ListTypeName>> {
    dispatch! {any;
        TokenKind::List => empty.value(ListTypeName::List),
        TokenKind::Array => empty.value(ListTypeName::Array),
        _ => fail
    }
    .spanned()
    .parse_next(input)
}

pub fn path_value_constructor(input: &mut TokenStream) -> ModalResult<Spanned<PathConstructor>> {
    preceded(
        TokenKind::Path,
        delimited(
            TokenKind::LeftBracket,
            path_element_list,
            TokenKind::RightBracket,
        ),
    )
    .update_span()
    .parse_next(input)
}

pub fn path_element_list(input: &mut TokenStream) -> ModalResult<Spanned<PathConstructor>> {
    seq! {PathConstructor {
        start: value_expression_primary.map(Box::new),
        steps: repeat(0.., path_element_list_step),
    }}
    .spanned()
    .parse_next(input)
}

pub fn path_element_list_step(input: &mut TokenStream) -> ModalResult<Spanned<PathStep>> {
    seq! {PathStep {
        _: TokenKind::Comma,
        edge: value_expression_primary,
        _: TokenKind::Comma,
        node: value_expression_primary,
    }}
    .spanned()
    .parse_next(input)
}

pub fn value_function(input: &mut TokenStream) -> ModalResult<Spanned<Function>> {
    dispatch! {peek(any);
        kind if kind.is_prefix_of_regular_identifier() => {
            generic_value_function.map_inner(Function::Generic)
        },
        kind if kind.is_prefix_of_numeric_value_function() => {
            numeric_value_function.map_inner(Function::Numeric)
        },
        _ => fail
    }
    .parse_next(input)
}

/// This allows UDFs to be used as value functions.
pub fn generic_value_function(input: &mut TokenStream) -> ModalResult<Spanned<GenericFunction>> {
    seq! {GenericFunction {
        name: regular_identifier,
        _: TokenKind::LeftParen,
        args: separated(0.., value_expression, TokenKind::Comma),
        _: TokenKind::RightParen,
    }}
    .spanned()
    .parse_next(input)
}

pub fn numeric_value_function(input: &mut TokenStream) -> ModalResult<Spanned<NumericFunction>> {
    dispatch! {peek(any);
        TokenKind::CharLength | TokenKind::CharacterLength => char_length_function,
        TokenKind::ByteLength | TokenKind::OctetLength => byte_length_function,
        TokenKind::PathLength => path_length_function,
        TokenKind::Abs => absolute_value_function,
        _ => fail
    }
    .parse_next(input)
}

pub fn char_length_function(input: &mut TokenStream) -> ModalResult<Spanned<NumericFunction>> {
    predefined_value_function!(
        one_of((TokenKind::CharLength, TokenKind::CharacterLength)),
        value_expression,
        |a| NumericFunction::CharLength(Box::new(a))
    )
    .parse_next(input)
}

pub fn byte_length_function(input: &mut TokenStream) -> ModalResult<Spanned<NumericFunction>> {
    predefined_value_function!(
        one_of((TokenKind::ByteLength, TokenKind::OctetLength)),
        value_expression,
        |a| NumericFunction::ByteLength(Box::new(a))
    )
    .parse_next(input)
}

pub fn path_length_function(input: &mut TokenStream) -> ModalResult<Spanned<NumericFunction>> {
    predefined_value_function!(TokenKind::PathLength, value_expression, |a| {
        NumericFunction::PathLength(Box::new(a))
    })
    .parse_next(input)
}

pub fn absolute_value_function(input: &mut TokenStream) -> ModalResult<Spanned<NumericFunction>> {
    predefined_value_function!(TokenKind::Abs, value_expression, |a| {
        NumericFunction::Absolute(Box::new(a))
    })
    .parse_next(input)
}

pub fn aggregate_function(input: &mut TokenStream) -> ModalResult<Spanned<AggregateFunction>> {
    dispatch! {peek(any);
        kind if kind.is_prefix_of_general_set_function() => {
            alt((
                (
                    TokenKind::Count,
                    TokenKind::LeftParen,
                    TokenKind::Asterisk,
                    TokenKind::RightParen,
                )
                    .value(AggregateFunction::Count)
                    .spanned(),
                general_set_function
                    .map_inner(AggregateFunction::General),
            ))
        },
        TokenKind::PercentileCont | TokenKind::PercentileDisc => {
            binary_set_function.map_inner(AggregateFunction::Binary)
        },
        _ => fail,
    }
    .parse_next(input)
}

pub fn general_set_function(input: &mut TokenStream) -> ModalResult<Spanned<GeneralSetFunction>> {
    seq! {GeneralSetFunction {
        kind: general_set_function_type,
        _: TokenKind::LeftParen,
        quantifier: opt(set_quantifier),
        expr: value_expression.map(Box::new),
        _: TokenKind::RightParen,
    }}
    .spanned()
    .parse_next(input)
}

pub fn general_set_function_type(
    input: &mut TokenStream,
) -> ModalResult<Spanned<GeneralSetFunctionKind>> {
    dispatch! {any;
        TokenKind::Avg => empty.value(GeneralSetFunctionKind::Avg),
        TokenKind::Count => empty.value(GeneralSetFunctionKind::Count),
        TokenKind::Max => empty.value(GeneralSetFunctionKind::Max),
        TokenKind::Min => empty.value(GeneralSetFunctionKind::Min),
        TokenKind::Sum => empty.value(GeneralSetFunctionKind::Sum),
        TokenKind::CollectList => empty.value(GeneralSetFunctionKind::CollectList),
        TokenKind::StddevSamp => empty.value(GeneralSetFunctionKind::StddevSamp),
        TokenKind::StddevPop => empty.value(GeneralSetFunctionKind::StddevPop),
        _ => fail,
    }
    .spanned()
    .parse_next(input)
}

pub fn binary_set_function(input: &mut TokenStream) -> ModalResult<Spanned<BinarySetFunction>> {
    seq! {BinarySetFunction {
        kind: binary_set_function_kind,
        quantifier: opt(set_quantifier),
        dependent: value_expression.map(Box::new),
        independent: preceded(TokenKind::Comma, value_expression).map(Box::new),
    }}
    .spanned()
    .parse_next(input)
}

pub fn binary_set_function_kind(
    input: &mut TokenStream,
) -> ModalResult<Spanned<BinarySetFunctionKind>> {
    dispatch! {any;
        TokenKind::PercentileCont => empty.value(BinarySetFunctionKind::PercentileCont),
        TokenKind::PercentileDisc => empty.value(BinarySetFunctionKind::PercentileDisc),
        _ => fail,
    }
    .spanned()
    .parse_next(input)
}

def_parser_alias!(boolean_value_expression, value_expression, Spanned<Expr>);
def_parser_alias!(
    aggregating_value_expression,
    value_expression,
    Spanned<Expr>
);

#[cfg(all(test, feature = "serde"))]
mod tests {
    use insta::assert_yaml_snapshot;

    use super::*;
    use crate::parser::utils::parse;

    #[test]
    fn test_set_quantifier_1() {
        let parsed = parse!(set_quantifier, "distinct");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_set_quantifier_2() {
        let parsed = parse!(set_quantifier, "all");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_list_value_type_name_1() {
        let parsed = parse!(list_value_type_name, "list");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_list_value_type_name_2() {
        let parsed = parse!(list_value_type_name, "array");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_list_value_constructor_1() {
        let parsed = parse!(list_value_constructor, "[1, 2, 3]");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_list_value_constructor_2() {
        let parsed = parse!(list_value_constructor, "list []");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_value_expression_1() {
        let parsed = parse!(value_expression, "(a and false) is not unknown");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_value_expression_2() {
        let parsed = parse!(value_expression, "a + 3 * (4 - 5) / -2");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_value_expression_3() {
        let parsed = parse!(value_expression, "a + (my_udf (a, b) * abs(-c))");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_parenthesized_value_expression() {
        let parsed = parse!(parenthesized_value_expression, "(1 + 1)");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_unsigned_value_specification_1() {
        let parsed = parse!(unsigned_value_specification, "session_user");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_unsigned_value_specification_2() {
        let parsed = parse!(unsigned_value_specification, "$abc");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_unsigned_value_specification_3() {
        let parsed = parse!(unsigned_value_specification, "123");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_aggregate_function_1() {
        let parsed = parse!(aggregate_function, "count(*)");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_aggregate_function_2() {
        let parsed = parse!(aggregate_function, "sum(distinct a)");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_numeric_value_function_1() {
        let parsed = parse!(value_function, "char_length (a)");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_generic_value_function_1() {
        let parsed = parse!(value_function, "this_is_a_udf ()");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_generic_value_function_2() {
        let parsed = parse!(value_function, "this_is_a_udf (a, b, c, d)");
        assert_yaml_snapshot!(parsed);
    }
}
