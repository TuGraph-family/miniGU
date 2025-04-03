use winnow::combinator::{alt, dispatch, empty, fail, peek, preceded};
use winnow::{ModalResult, Parser};

use super::lexical::object_name_or_binding_variable;
use super::object_ref::graph_reference;
use super::value_expr::{
    non_parenthesized_value_expression_primary_special_case, parenthesized_value_expression,
    value_expression_primary,
};
use crate::ast::{GraphExpr, ObjectExpr};
use crate::lexer::TokenKind;
use crate::parser::token::{TokenStream, any};
use crate::parser::utils::{SpannedParserExt, ToSpanned};
use crate::span::Spanned;

pub fn graph_expression(input: &mut TokenStream) -> ModalResult<Spanned<GraphExpr>> {
    dispatch! {peek(any);
        TokenKind::CurrentPropertyGraph | TokenKind::CurrentGraph => current_graph,
        _ => alt((
            graph_reference.map_inner(GraphExpr::Ref),
            object_expression_primary.map_inner(GraphExpr::Object),
            object_name_or_binding_variable.map_inner(GraphExpr::Name),
            fail,
        ))
    }
    .parse_next(input)
}

pub fn current_graph(input: &mut TokenStream) -> ModalResult<Spanned<GraphExpr>> {
    dispatch! {any;
        TokenKind::CurrentPropertyGraph | TokenKind::CurrentGraph => empty.value(GraphExpr::Current),
        _ => fail
    }
    .spanned()
    .parse_next(input)
}

pub fn object_expression_primary(input: &mut TokenStream) -> ModalResult<Spanned<ObjectExpr>> {
    dispatch!{peek(any);
        TokenKind::Variable => preceded(TokenKind::Variable, value_expression_primary).map(ObjectExpr::Variable),
        TokenKind::LeftParen => parenthesized_value_expression.map(ObjectExpr::Expr),
        _ => non_parenthesized_value_expression_primary_special_case.map(ObjectExpr::Expr),
    }.spanned().parse_next(input)
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use insta::assert_yaml_snapshot;

    use super::*;
    use crate::parser::utils::parse;

    #[test]
    fn test_graph_expression_1() {
        let parsed = parse!(graph_expression, "current_property_graph");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_graph_expression_2() {
        let parsed = parse!(graph_expression, "home_graph");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_graph_expression_3() {
        let parsed = parse!(graph_expression, "/a/b");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_graph_expression_4() {
        let parsed = parse!(graph_expression, "g");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_graph_expression_5() {
        let parsed = parse!(graph_expression, "variable a");
        assert_yaml_snapshot!(parsed);
    }
}
