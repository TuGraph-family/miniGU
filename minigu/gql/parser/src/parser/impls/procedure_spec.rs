extern crate alloc;
use alloc::boxed::Box;

use winnow::combinator::{alt, delimited, dispatch, fail, opt, peek, repeat, seq};
use winnow::{ModalResult, Parser};

use super::catalog::linear_catalog_modifying_statement;
use super::common::{at_schema_clause, yield_clause};
use super::data::linear_data_modifying_statement;
use super::query::composite_query_statement;
use super::variable::{
    binding_table_variable_definition, graph_variable_definition, value_variable_definition,
};
use crate::ast::*;
use crate::lexer::TokenKind;
use crate::parser::token::{TokenStream, any};
use crate::parser::utils::{SpannedParserExt, ToSpanned, def_parser_alias};
use crate::span::Spanned;

pub fn nested_procedure_specification(input: &mut TokenStream) -> ModalResult<Spanned<Procedure>> {
    delimited(
        TokenKind::LeftBrace,
        procedure_specification,
        TokenKind::RightBrace,
    )
    .update_span()
    .parse_next(input)
}

def_parser_alias!(procedure_specification, procedure_body, Spanned<Procedure>);
def_parser_alias!(
    nested_data_modifying_procedure_specification,
    nested_procedure_specification,
    Spanned<Procedure>
);
def_parser_alias!(
    nested_query_specification,
    nested_procedure_specification,
    Spanned<Procedure>
);

pub fn procedure_body(input: &mut TokenStream) -> ModalResult<Spanned<Procedure>> {
    seq! {Procedure {
        at: opt(at_schema_clause),
        binding_variable_defs: repeat(0.., binding_variable_definition),
        statement: statement,
        next_statements: repeat(0.., next_statement),
    }}
    .spanned()
    .parse_next(input)
}

pub fn binding_variable_definition(
    input: &mut TokenStream,
) -> ModalResult<Spanned<BindingVariableDef>> {
    dispatch! {peek(any);
        TokenKind::Property | TokenKind::Graph => {
            graph_variable_definition.map_inner(BindingVariableDef::Graph)
        },
        TokenKind::Binding | TokenKind::Table => {
            binding_table_variable_definition.map_inner(BindingVariableDef::BindingTable)
        },
        TokenKind::Value => {
            value_variable_definition.map_inner(BindingVariableDef::Value)
        },
        _ => fail
    }
    .parse_next(input)
}

pub fn statement(input: &mut TokenStream) -> ModalResult<Spanned<Statement>> {
    alt((
        composite_query_statement.map_inner(Statement::Query),
        linear_catalog_modifying_statement
            .map(Statement::Catalog)
            .spanned(),
        linear_data_modifying_statement.map_inner(Statement::Data),
        utility_statement.map_inner(Statement::Utility),
    ))
    .parse_next(input)
}

pub fn utility_statement(input: &mut TokenStream) -> ModalResult<Spanned<UtilityStatement>> {
    alt((explain_statement.map_inner(UtilityStatement::Explain), fail)).parse_next(input)
}

pub fn explain_statement(input: &mut TokenStream) -> ModalResult<Spanned<ExplainStatement>> {
    seq! {ExplainStatement{
        _: TokenKind::Explain,
        statement: statement.map(Box::new),
    }}
    .spanned()
    .parse_next(input)
}

pub fn next_statement(input: &mut TokenStream) -> ModalResult<Spanned<NextStatement>> {
    seq! {NextStatement{
        _: TokenKind::Next,
        yield_clause: opt(yield_clause),
        statement: statement,
    }}
    .spanned()
    .parse_next(input)
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use insta::assert_yaml_snapshot;

    use super::*;
    use crate::parser::utils::parse;
}
