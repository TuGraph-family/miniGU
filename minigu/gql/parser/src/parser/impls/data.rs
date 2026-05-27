use winnow::combinator::{dispatch, fail, opt, peek, preceded, repeat, separated};
use winnow::{ModalResult, Parser};

use super::common::{edge_pattern, node_pattern};
use crate::ast::{
    DataModifyingStatement, ElementPattern, InsertPath, InsertStatement,
    LinearDataModifyingStatement,
};
use crate::imports::Vec;
use crate::lexer::TokenKind;
use crate::parser::token::{TokenStream, any};
use crate::parser::utils::{SpannedParserExt, ToSpanned};
use crate::span::{Spanned, VecSpanned};

pub fn linear_data_modifying_statement(
    input: &mut TokenStream,
) -> ModalResult<Spanned<LinearDataModifyingStatement>> {
    repeat(1.., simple_data_modifying_statement)
        .map(|v: Vec<_>| v)
        .spanned()
        .parse_next(input)
}

fn simple_data_modifying_statement(
    input: &mut TokenStream,
) -> ModalResult<Spanned<DataModifyingStatement>> {
    dispatch! {peek(any);
        TokenKind::Insert => insert_statement.map_inner(DataModifyingStatement::Insert),
        _ => fail,
    }
    .parse_next(input)
}

fn insert_statement(input: &mut TokenStream) -> ModalResult<Spanned<InsertStatement>> {
    preceded(
        TokenKind::Insert,
        separated(1.., insert_path, TokenKind::Comma).map(|v: Vec<_>| v),
    )
    .map(|paths| InsertStatement { paths })
    .spanned()
    .parse_next(input)
}

/// A single insert path: a node followed by zero or more `edge node` pairs.
fn insert_path(input: &mut TokenStream) -> ModalResult<Spanned<InsertPath>> {
    (
        node_pattern,
        repeat(0.., (edge_pattern, node_pattern))
            .map(|pairs: Vec<(Spanned<ElementPattern>, Spanned<ElementPattern>)>| pairs),
    )
        .map(|(first, rest)| {
            let mut elements: VecSpanned<ElementPattern> = Vec::with_capacity(1 + rest.len() * 2);
            elements.push(first);
            for (e, n) in rest {
                elements.push(e);
                elements.push(n);
            }
            InsertPath { elements }
        })
        .spanned()
        .parse_next(input)
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use insta::assert_yaml_snapshot;

    use super::*;
    use crate::parser::utils::parse;

    #[test]
    fn test_insert_single_vertex() {
        let parsed = parse!(
            linear_data_modifying_statement,
            "INSERT (a:PERSON {name: 'Alice', age: 30})"
        );
        assert_yaml_snapshot!(parsed.unwrap());
    }

    #[test]
    fn test_insert_multiple_vertices() {
        let parsed = parse!(
            linear_data_modifying_statement,
            "INSERT (a:PERSON {name: 'A'}), (b:COMPANY {name: 'B'})"
        );
        assert_yaml_snapshot!(parsed.unwrap());
    }

    #[test]
    fn test_insert_vertex_edge_vertex() {
        let parsed = parse!(
            linear_data_modifying_statement,
            "INSERT (a:PERSON {name: 'A'})-[:KNOWS {since: 2020}]->(b:PERSON {name: 'B'})"
        );
        assert_yaml_snapshot!(parsed.unwrap());
    }

    #[test]
    fn test_insert_reverse_edge() {
        let parsed = parse!(
            linear_data_modifying_statement,
            "INSERT (a:PERSON {name: 'A'})<-[:KNOWS]-(b:PERSON {name: 'B'})"
        );
        assert_yaml_snapshot!(parsed.unwrap());
    }
}
