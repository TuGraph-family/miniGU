use winnow::combinator::{delimited, fail, opt, preceded, separated, seq};
use winnow::{ModalResult, Parser};

use super::lexical::{binding_variable, field_name};
use crate::ast::{
    BindingTableVariableDef, GraphVariableDef, Ident, Procedure, SchemaRef, ValueVariableDef,
    Yield, YieldItem,
};
use crate::lexer::TokenKind;
use crate::parser::token::TokenStream;
use crate::parser::utils::{SpannedParserExt, ToSpanned, def_parser_alias};
use crate::span::Spanned;

pub fn graph_variable_definition(
    input: &mut TokenStream,
) -> ModalResult<Spanned<GraphVariableDef>> {
    fail(input)
}

pub fn binding_table_variable_definition(
    input: &mut TokenStream,
) -> ModalResult<Spanned<BindingTableVariableDef>> {
    fail(input)
}

pub fn value_variable_definition(
    input: &mut TokenStream,
) -> ModalResult<Spanned<ValueVariableDef>> {
    fail(input)
}
