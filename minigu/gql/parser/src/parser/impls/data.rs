use winnow::combinator::{delimited, fail, opt, preceded, separated, seq};
use winnow::{ModalResult, Parser};

use super::lexical::{binding_variable, field_name};
use crate::ast::{Ident, LinearDataModifyingStatement, Procedure, SchemaRef, Yield, YieldItem};
use crate::lexer::TokenKind;
use crate::parser::token::TokenStream;
use crate::parser::utils::{SpannedParserExt, ToSpanned, def_parser_alias};
use crate::span::Spanned;

pub fn linear_data_modifying_statement(
    input: &mut TokenStream,
) -> ModalResult<Spanned<LinearDataModifyingStatement>> {
    fail(input)
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use insta::assert_yaml_snapshot;

    use super::*;
    use crate::parser::utils::parse;
}
