use winnow::combinator::{alt, dispatch, fail, opt, peek, preceded, repeat, seq};
use winnow::{ModalResult, Parser};

use super::object_expr::graph_expression;
use super::object_ref::*;
use super::procedure_call::call_procedure_statement;
use super::type_element::{nested_graph_type_specification, typed};
use crate::ast::*;
use crate::lexer::TokenKind;
use crate::parser::token::{TokenStream, any};
use crate::parser::utils::{SpannedParserExt, ToSpanned, def_parser_alias};
use crate::span::Spanned;

pub fn linear_catalog_modifying_statement(
    input: &mut TokenStream,
) -> ModalResult<LinearCatalogModifyingStatement> {
    repeat(1.., simple_catalog_modifying_statement).parse_next(input)
}

pub fn simple_catalog_modifying_statement(
    input: &mut TokenStream,
) -> ModalResult<Spanned<CatalogModifyingStatement>> {
    dispatch! {peek(any);
        TokenKind::Create | TokenKind::Drop => primitive_catalog_modifying_statement,
        TokenKind::Optional | TokenKind::Call => {
            call_catalog_modifying_procedure_statement
                .map_inner(CatalogModifyingStatement::Call)
        },
        _ => fail,
    }
    .parse_next(input)
}

pub fn primitive_catalog_modifying_statement(
    input: &mut TokenStream,
) -> ModalResult<Spanned<CatalogModifyingStatement>> {
    dispatch! {peek((any, any));
        (TokenKind::Create, TokenKind::Schema) => {
            create_schema_statement.map_inner(CatalogModifyingStatement::CreateSchema)
        },
        (TokenKind::Drop, TokenKind::Schema) => {
            drop_schema_statement.map_inner(CatalogModifyingStatement::DropSchema)
        },
        (TokenKind::Create, TokenKind::Property | TokenKind::Graph | TokenKind::Or) => {
            alt((
                create_graph_type_statement.map_inner(CatalogModifyingStatement::CreateGraphType),
                create_graph_statement.map_inner(CatalogModifyingStatement::CreateGraph),
            ))
        },
        (TokenKind::Drop, TokenKind::Property | TokenKind::Graph) => {
            alt((
                drop_graph_type_statement.map_inner(CatalogModifyingStatement::DropGraphType),
                drop_graph_statement.map_inner(CatalogModifyingStatement::DropGraph),
            ))
        },
        _ => fail,
    }
    .parse_next(input)
}

def_parser_alias!(
    call_catalog_modifying_procedure_statement,
    call_procedure_statement,
    Spanned<CallProcedureStatement>
);

fn if_not_exists(input: &mut TokenStream) -> ModalResult<()> {
    (TokenKind::If, TokenKind::Not, TokenKind::Exists)
        .void()
        .parse_next(input)
}

fn if_exists(input: &mut TokenStream) -> ModalResult<()> {
    (TokenKind::If, TokenKind::Exists).void().parse_next(input)
}

pub fn create_schema_statement(
    input: &mut TokenStream,
) -> ModalResult<Spanned<CreateSchemaStatement>> {
    seq! {CreateSchemaStatement {
        _: (TokenKind::Create, TokenKind::Schema),
        if_not_exists: opt(if_not_exists).map(|o| o.is_some()),
        path: catalog_schema_parent_and_name,
    }}
    .spanned()
    .parse_next(input)
}

pub fn drop_schema_statement(input: &mut TokenStream) -> ModalResult<Spanned<DropSchemaStatement>> {
    seq! {DropSchemaStatement {
        _: (TokenKind::Drop, TokenKind::Schema),
        if_exists: opt(if_exists).map(|o| o.is_some()),
        path: catalog_schema_parent_and_name,
    }}
    .spanned()
    .parse_next(input)
}

pub fn open_graph_type(input: &mut TokenStream) -> ModalResult<Spanned<OfGraphType>> {
    (
        opt(typed),
        TokenKind::Any,
        opt((opt(TokenKind::Property), TokenKind::Graph)),
    )
        .value(OfGraphType::Any)
        .spanned()
        .parse_next(input)
}

pub fn of_graph_type(input: &mut TokenStream) -> ModalResult<Spanned<OfGraphType>> {
    dispatch! {peek(any);
        TokenKind::Like => graph_type_like_graph.map(OfGraphType::Like),
        _ => preceded(opt(typed), alt((
            graph_type_reference.map(OfGraphType::Ref),
            preceded(
                opt((opt(TokenKind::Property), TokenKind::Graph)),
                nested_graph_type_specification
            ).map(OfGraphType::Nested),
            fail
        )))
    }
    .spanned()
    .parse_next(input)
}

pub fn graph_type_like_graph(input: &mut TokenStream) -> ModalResult<Spanned<GraphExpr>> {
    preceded(TokenKind::Like, graph_expression).parse_next(input)
}

pub fn graph_source(input: &mut TokenStream) -> ModalResult<Spanned<GraphExpr>> {
    preceded(
        (TokenKind::As, TokenKind::Copy, TokenKind::Of),
        graph_expression,
    )
    .parse_next(input)
}

fn create_graph_statement_kind(
    input: &mut TokenStream,
) -> ModalResult<Spanned<CreateGraphOrGraphTypeStatementKind>> {
    preceded(
        TokenKind::Create,
        alt((
            preceded(
                (opt(TokenKind::Property), TokenKind::Graph),
                opt(if_not_exists),
            )
            .map(|if_not_exists| match if_not_exists {
                Some(()) => CreateGraphOrGraphTypeStatementKind::CreateIfNotExists,
                None => CreateGraphOrGraphTypeStatementKind::Create,
            }),
            (
                TokenKind::Or,
                TokenKind::Replace,
                opt(TokenKind::Property),
                TokenKind::Graph,
            )
                .value(CreateGraphOrGraphTypeStatementKind::CreateOrReplace),
            fail,
        )),
    )
    .spanned()
    .parse_next(input)
}

pub fn create_graph_statement(
    input: &mut TokenStream,
) -> ModalResult<Spanned<CreateGraphStatement>> {
    seq! {CreateGraphStatement {
        kind: create_graph_statement_kind,
        path: catalog_graph_parent_and_name,
        graph_type: alt((open_graph_type, of_graph_type)),
        source: opt(graph_source)
    }}
    .spanned()
    .parse_next(input)
}

pub fn drop_graph_statement(input: &mut TokenStream) -> ModalResult<Spanned<DropGraphStatement>> {
    seq! {DropGraphStatement {
        _: (TokenKind::Drop, opt(TokenKind::Property), TokenKind::Graph),
        if_exists: opt(if_exists).map(|if_exists| if_exists.is_some()),
        path: catalog_graph_parent_and_name,
    }}
    .spanned()
    .parse_next(input)
}

fn create_graph_type_statement_kind(
    input: &mut TokenStream,
) -> ModalResult<Spanned<CreateGraphOrGraphTypeStatementKind>> {
    preceded(
        TokenKind::Create,
        alt((
            preceded(
                (opt(TokenKind::Property), TokenKind::Graph, TokenKind::Type),
                opt(if_not_exists),
            )
            .map(|if_not_exists| match if_not_exists {
                Some(()) => CreateGraphOrGraphTypeStatementKind::CreateIfNotExists,
                None => CreateGraphOrGraphTypeStatementKind::Create,
            }),
            (
                TokenKind::Or,
                TokenKind::Replace,
                opt(TokenKind::Property),
                TokenKind::Graph,
                TokenKind::Type,
            )
                .value(CreateGraphOrGraphTypeStatementKind::CreateOrReplace),
            fail,
        )),
    )
    .spanned()
    .parse_next(input)
}

pub fn graph_type_source(input: &mut TokenStream) -> ModalResult<Spanned<GraphTypeSource>> {
    dispatch! {peek((any, any));
        (TokenKind::As, TokenKind::Copy) => {
            preceded(TokenKind::As, copy_of_graph_type)
                .map(GraphTypeSource::Copy)
                .spanned()
        },
        (TokenKind::Copy, TokenKind::Of) => {
            copy_of_graph_type.map(GraphTypeSource::Copy).spanned()
        },
        (TokenKind::Like, _) => {
            graph_type_like_graph.map(GraphTypeSource::Like).spanned()
        },
        (TokenKind::As, TokenKind::LeftBrace) => {
            preceded(TokenKind::As, nested_graph_type_specification)
                .map(GraphTypeSource::Nested)
                .spanned()
        },
        (TokenKind::LeftBrace, _) => {
            nested_graph_type_specification
                .map(GraphTypeSource::Nested)
                .spanned()
        },
        _ => fail,
    }
    .parse_next(input)
}

pub fn copy_of_graph_type(input: &mut TokenStream) -> ModalResult<Spanned<GraphTypeRef>> {
    preceded((TokenKind::Copy, TokenKind::Of), graph_type_reference).parse_next(input)
}

pub fn create_graph_type_statement(
    input: &mut TokenStream,
) -> ModalResult<Spanned<CreateGraphTypeStatement>> {
    seq! {CreateGraphTypeStatement {
        kind: create_graph_type_statement_kind,
        path: catalog_graph_type_parent_and_name,
        source: graph_type_source,
    }}
    .spanned()
    .parse_next(input)
}

pub fn drop_graph_type_statement(
    input: &mut TokenStream,
) -> ModalResult<Spanned<DropGraphTypeStatement>> {
    seq! {DropGraphTypeStatement {
        _: (TokenKind::Drop, opt(TokenKind::Property), TokenKind::Graph, TokenKind::Type),
        if_exists: opt(if_exists).map(|if_exists| if_exists.is_some()),
        path: catalog_graph_type_parent_and_name,
    }}
    .spanned()
    .parse_next(input)
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use insta::assert_yaml_snapshot;

    use super::*;
    use crate::parser::utils::parse;

    #[test]
    fn test_create_schema_statement() {
        let parsed = parse!(create_schema_statement, "create schema if not exists /a/b");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_drop_schema_statement() {
        let parsed = parse!(drop_schema_statement, "drop schema if exists /a/b");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_create_graph_statement() {
        let parsed = parse!(
            create_graph_statement,
            "create graph if not exists myGraph ::any as copy of /a/b"
        );
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_create_graph_statement_with_type() {
        let parsed = parse!(
            create_graph_statement,
            r"create graph if not exists myGraph { 
                (a:Person {id int, name string}),
                (a) ~[:knows {since int}]~ (a)
            }"
        );
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_drop_graph_statement() {
        let parsed = parse!(drop_graph_statement, "drop graph if exists /a/b/c");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_create_graph_type_statement() {
        let parsed = parse!(
            create_graph_type_statement,
            "create or replace property graph type  /a/b like HOME_GRAPH"
        );
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_create_graph_type_statement_with_type() {
        let parsed = parse!(
            create_graph_type_statement,
            r"create graph type /a/b {
                (a:Person {id int, name string}),
                (a) ~[:knows {since int}]~ (a)
            }"
        );
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_drop_graph_type_statement() {
        let parsed = parse!(
            drop_graph_type_statement,
            "drop property graph type if exists /a/b/c"
        );
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_simple_catalog_modifying_statement() {
        let parsed = parse!(
            simple_catalog_modifying_statement,
            "call create_graph(\"abc\")"
        );
        assert_yaml_snapshot!(parsed);
    }
}
