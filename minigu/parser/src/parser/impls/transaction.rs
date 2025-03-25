use winnow::combinator::{dispatch, empty, fail, preceded, separated};
use winnow::{ModalResult, Parser};

use crate::ast::{StartTransaction, TransactionMode};
use crate::lexer::TokenKind;
use crate::parser::token::{TokenStream, any};
use crate::parser::utils::ToSpanned;
use crate::span::Spanned;

pub fn start_transaction_command(
    input: &mut TokenStream,
) -> ModalResult<Spanned<StartTransaction>> {
    preceded(
        (TokenKind::Start, TokenKind::Transaction),
        separated(0.., transaction_access_mode, TokenKind::Comma),
    )
    .map(StartTransaction)
    .spanned()
    .parse_next(input)
}

pub fn transaction_access_mode(input: &mut TokenStream) -> ModalResult<Spanned<TransactionMode>> {
    dispatch! {(any, any);
        (TokenKind::Read, TokenKind::Only) => empty.value(TransactionMode::ReadOnly),
        (TokenKind::Read, TokenKind::Write) => empty.value(TransactionMode::ReadWrite),
        _ => fail
    }
    .spanned()
    .parse_next(input)
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use insta::assert_yaml_snapshot;

    use super::*;
    use crate::parser::utils::parse;

    #[test]
    fn test_start_transaction_command_1() {
        let parsed = parse!(start_transaction_command, "start transaction");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_start_transaction_command_2() {
        let parsed = parse!(
            start_transaction_command,
            "start transaction read only, read write"
        );
        assert_yaml_snapshot!(parsed);
    }
}
