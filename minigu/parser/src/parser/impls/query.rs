use winnow::combinator::{
    alt, delimited, dispatch, empty, fail, opt, peek, preceded, repeat, separated,
    separated_foldl1, seq,
};
use winnow::{ModalResult, Parser};

use super::common::{
    graph_pattern_binding_table, limit_clause, offset_clause, order_by_clause, use_graph_clause,
};
use super::lexical::identifier;
use super::object_expr::graph_expression;
use super::procedure_call::call_procedure_statement;
use super::procedure_spec::nested_query_specification;
use super::session::{session_close_command, session_reset_command, session_set_command};
use super::value_expr::{aggregating_value_expression, binding_variable_reference, set_quantifier};
use crate::ast::*;
use crate::imports::{Box, Vec};
use crate::lexer::TokenKind;
use crate::parser::token::{Token, TokenStream, any};
use crate::parser::utils::{SpannedParserExt, ToSpanned, def_parser_alias};
use crate::span::{Spanned, VecSpanned};

pub fn composite_query_statement(
    input: &mut TokenStream,
) -> ModalResult<Spanned<CompositeQueryStatement>> {
    separated_foldl1(
        composite_query_primary,
        query_conjunction,
        |left, conjunction, right| {
            let span = left.1.start..right.1.end;
            let inner = CompositeQueryStatement::Conjunction {
                conjunction,
                left: Box::new(left),
                right: Box::new(right),
            };
            Spanned(inner, span)
        },
    )
    .parse_next(input)
}

pub fn query_conjunction(input: &mut TokenStream) -> ModalResult<Spanned<QueryConjunction>> {
    dispatch! {peek(any);
        TokenKind::Union
        | TokenKind::Except
        | TokenKind::Intersect => set_operator.map_inner(QueryConjunction::SetOp),
        TokenKind::Otherwise => TokenKind::Otherwise.value(QueryConjunction::Otherwise).spanned(),
        _ => fail
    }
    .parse_next(input)
}

pub fn set_operator(input: &mut TokenStream) -> ModalResult<Spanned<SetOp>> {
    seq! {SetOp {
        kind: set_operator_kind,
        quantifier: opt(set_quantifier)
    }}
    .spanned()
    .parse_next(input)
}

pub fn set_operator_kind(input: &mut TokenStream) -> ModalResult<Spanned<SetOpKind>> {
    dispatch! {any;
        TokenKind::Union => empty.value(SetOpKind::Union),
        TokenKind::Except => empty.value(SetOpKind::Except),
        TokenKind::Intersect => empty.value(SetOpKind::Intersect),
        _ => fail
    }
    .spanned()
    .parse_next(input)
}

pub fn composite_query_primary(
    input: &mut TokenStream,
) -> ModalResult<Spanned<CompositeQueryStatement>> {
    linear_query_statement
        .map_inner(CompositeQueryStatement::Primary)
        .parse_next(input)
}

pub fn linear_query_statement(
    input: &mut TokenStream,
) -> ModalResult<Spanned<LinearQueryStatement>> {
    dispatch! {peek(any);
        TokenKind::Use | TokenKind::Select => {
            focused_linear_query_statement.map_inner(LinearQueryStatement::Focused)
        },
        kind if kind.is_prefix_of_ambient_linear_query_statement() => {
            ambient_linear_query_statement.map_inner(LinearQueryStatement::Ambient)
        },
        _ => fail
    }
    .parse_next(input)
}

pub fn ambient_linear_query_statement(
    input: &mut TokenStream,
) -> ModalResult<Spanned<AmbientLinearQueryStatement>> {
    dispatch! {peek(any);
        TokenKind::LeftBrace => {
            nested_query_specification
                .map(Box::new)
                .map(AmbientLinearQueryStatement::Nested)
        },
        kind if kind.is_prefix_of_simple_query_statement() || kind.is_prefix_of_result_statement() => {
            (repeat(0.., simple_query_statement), primitive_result_statement)
                .map(|(parts, result)| AmbientLinearQueryStatement::Parts {
                    parts, result
                })
        },
        _ => fail
    }
    .spanned()
    .parse_next(input)
}

pub fn focused_linear_query_statement(
    input: &mut TokenStream,
) -> ModalResult<Spanned<FocusedLinearQueryStatement>> {
    let parts = seq! {FocusedLinearQueryStatement::Parts {
        parts: repeat(1.., focused_linear_query_statement_part),
        result: primitive_result_statement
    }}
    .spanned();
    let result = seq! {FocusedLinearQueryStatement::Result {
        use_graph: use_graph_clause,
        result: primitive_result_statement
    }}
    .spanned();
    let nested = seq! {FocusedLinearQueryStatement::Nested {
        use_graph: use_graph_clause,
        query: nested_query_specification.map(Box::new),
    }}
    .spanned();
    alt((parts, result, nested, select_statement)).parse_next(input)
}

pub fn focused_linear_query_statement_part(
    input: &mut TokenStream,
) -> ModalResult<Spanned<FocusedLinearQueryStatementPart>> {
    seq! {FocusedLinearQueryStatementPart {
        use_graph: use_graph_clause,
        statements: repeat(1.., simple_query_statement)
    }}
    .spanned()
    .parse_next(input)
}

pub fn select_statement(
    input: &mut TokenStream,
) -> ModalResult<Spanned<FocusedLinearQueryStatement>> {
    fail(input)
}

pub fn order_by_and_page_statement(
    input: &mut TokenStream,
) -> ModalResult<Spanned<OrderByAndPageStatement>> {
    dispatch! {peek(any);
        TokenKind::Order => {
            (order_by_clause, opt(offset_clause), opt(limit_clause))
                .map(|(order_by, offset, limit)| OrderByAndPageStatement {
                    order_by,
                    offset,
                    limit,
                })
        },
        TokenKind::Offset => {
            (offset_clause, opt(limit_clause))
                .map(|(offset, limit)| OrderByAndPageStatement {
                    order_by: VecSpanned::new(),
                    offset: Some(offset),
                    limit,
                })
        },
        TokenKind::Limit => {
            limit_clause
                .map(|limit| OrderByAndPageStatement {
                    order_by: VecSpanned::new(),
                    offset: None,
                    limit: Some(limit),
                })
        },
        _ => fail
    }
    .spanned()
    .parse_next(input)
}

pub fn primitive_result_statement(
    input: &mut TokenStream,
) -> ModalResult<Spanned<ResultStatement>> {
    dispatch! {peek(any);
        TokenKind::Return => {
            (return_statement, opt(order_by_and_page_statement))
                .map(|(statement, order_by)| ResultStatement::Return {
                    statement: Box::new(statement),
                    order_by: order_by.map(Box::new),
                })
        },
        TokenKind::Finish => TokenKind::Finish.value(ResultStatement::Finish),
        _ => fail
    }
    .spanned()
    .parse_next(input)
}

pub fn return_statement(input: &mut TokenStream) -> ModalResult<Spanned<ReturnStatement>> {
    preceded(TokenKind::Return, return_statement_body)
        .update_span()
        .parse_next(input)
}

pub fn return_statement_body(input: &mut TokenStream) -> ModalResult<Spanned<ReturnStatement>> {
    let mut items = dispatch! {peek(any);
        TokenKind::Asterisk => TokenKind::Asterisk.value(Return::All),
        _ => return_item_list.map(Return::Items)
    }
    .spanned();
    seq! {ReturnStatement {
        quantifier: opt(set_quantifier),
        items: items,
        group_by: opt(group_by_clause)
    }}
    .spanned()
    .parse_next(input)
}

pub fn return_item_list(input: &mut TokenStream) -> ModalResult<VecSpanned<ReturnItem>> {
    separated(1.., return_item, TokenKind::Comma).parse_next(input)
}

pub fn return_item(input: &mut TokenStream) -> ModalResult<Spanned<ReturnItem>> {
    seq! {ReturnItem {
        value: aggregating_value_expression,
        alias: opt(return_item_alias)
    }}
    .spanned()
    .parse_next(input)
}

pub fn return_item_alias(input: &mut TokenStream) -> ModalResult<Spanned<Ident>> {
    preceded(TokenKind::As, identifier).parse_next(input)
}

pub fn group_by_clause(input: &mut TokenStream) -> ModalResult<Spanned<GroupBy>> {
    preceded((TokenKind::Group, TokenKind::By), grouping_element_list)
        .spanned()
        .parse_next(input)
}

pub fn grouping_element_list(input: &mut TokenStream) -> ModalResult<VecSpanned<Ident>> {
    dispatch! {peek(any);
        TokenKind::LeftParen => (TokenKind::LeftParen, TokenKind::RightParen).value(VecSpanned::new()),
        _ => separated(1.., grouping_element, TokenKind::Comma)
    }
    .parse_next(input)
}

def_parser_alias!(grouping_element, binding_variable_reference, Spanned<Ident>);

pub fn simple_query_statement(
    input: &mut TokenStream,
) -> ModalResult<Spanned<SimpleQueryStatement>> {
    dispatch! {peek(any);
        TokenKind::Match => match_statement.map_inner(SimpleQueryStatement::Match),
        // TODO: Add optional call.
        TokenKind::Call => call_query_statement.map_inner(SimpleQueryStatement::Call),
        _ => fail
    }
    .parse_next(input)
}

def_parser_alias!(
    call_query_statement,
    call_procedure_statement,
    Spanned<CallProcedureStatement>
);

pub fn match_statement(input: &mut TokenStream) -> ModalResult<Spanned<MatchStatement>> {
    dispatch! {peek(any);
        TokenKind::Match => simple_match_statement.map(MatchStatement::Simple),
        TokenKind::Optional => {
            optional_match_statement.map(MatchStatement::Optional)
        },
        _ => fail
    }
    .spanned()
    .parse_next(input)
}

pub fn simple_match_statement(
    input: &mut TokenStream,
) -> ModalResult<Spanned<GraphPatternBindingTable>> {
    preceded(TokenKind::Match, graph_pattern_binding_table).parse_next(input)
}

pub fn optional_match_statement(
    input: &mut TokenStream,
) -> ModalResult<VecSpanned<MatchStatement>> {
    let operand = dispatch! {peek(any);
        TokenKind::Match => {
            simple_match_statement
                .map(MatchStatement::Simple)
                .spanned()
                .map(|simple| [simple].into())
        },
        TokenKind::LeftBrace => {
            delimited(TokenKind::LeftBrace, match_statement_block, TokenKind::RightBrace)
        },
        TokenKind::LeftParen => {
            delimited(TokenKind::LeftParen, match_statement_block, TokenKind::RightParen)
        },
        _ => fail
    };
    preceded(TokenKind::Optional, operand).parse_next(input)
}

pub fn match_statement_block(input: &mut TokenStream) -> ModalResult<VecSpanned<MatchStatement>> {
    repeat(1.., match_statement).parse_next(input)
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use insta::assert_yaml_snapshot;

    use super::*;
    use crate::parser::utils::parse;

    #[test]
    fn test_ambient_linear_query_statement() {
        let query = parse!(
            ambient_linear_query_statement,
            r"
            MATCH (a)-[:KNOWS]->(b)
            MATCH (b)-[:KNOWS]->(c)
            RETURN a.id, count(c)
            ORDER BY a.id DESC NULLS LAST
            OFFSET 10
            LIMIT 10"
        );
        assert_yaml_snapshot!(query);
    }
}
