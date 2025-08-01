use winnow::combinator::{alt, dispatch, empty, fail, opt, peek, preceded, repeat, terminated};
use winnow::{ModalResult, Parser};

use super::lexical::{
    binding_table_name, delimited_binding_table_name, delimited_graph_name, directory_name,
    graph_name, object_name, schema_name, substituted_parameter_reference,
};
use crate::ast::*;
use crate::lexer::TokenKind;
use crate::parser::token::{TokenStream, any};
use crate::parser::utils::{SpannedParserExt, ToSpanned, def_parser_alias};
use crate::span::Spanned;

pub fn simple_directory_path<const GREEDY: bool>(
    input: &mut TokenStream,
) -> ModalResult<SchemaPath> {
    if GREEDY {
        repeat(
            1..,
            terminated(
                directory_name.map_inner(SchemaPathSegment::Name),
                TokenKind::Solidus,
            ),
        )
        .parse_next(input)
    } else {
        repeat(
            1..,
            terminated(
                directory_name.map_inner(SchemaPathSegment::Name),
                (
                    TokenKind::Solidus,
                    peek((directory_name, TokenKind::Solidus)),
                ),
            ),
        )
        .parse_next(input)
    }
}

pub fn relative_directory_path<const GREEDY: bool>(
    input: &mut TokenStream,
) -> ModalResult<SchemaPath> {
    (
        repeat(
            1..,
            terminated(
                TokenKind::DoublePeriod
                    .value(SchemaPathSegment::Parent)
                    .spanned(),
                TokenKind::Solidus,
            ),
        ),
        opt(simple_directory_path::<GREEDY>),
    )
        .map(|(mut prefix, suffix): (SchemaPath, _)| {
            prefix.extend(suffix.into_iter().flatten());
            prefix
        })
        .parse_next(input)
}

pub fn absolute_directory_path<const GREEDY: bool>(
    input: &mut TokenStream,
) -> ModalResult<SchemaPath> {
    preceded(TokenKind::Solidus, opt(simple_directory_path::<GREEDY>))
        .map(Option::unwrap_or_default)
        .parse_next(input)
}

pub fn predefined_schema_reference(
    input: &mut TokenStream,
) -> ModalResult<Spanned<PredefinedSchemaRef>> {
    dispatch! {any;
        TokenKind::HomeSchema => empty.value(PredefinedSchemaRef::Home),
        TokenKind::CurrentSchema | TokenKind::Period => empty.value(PredefinedSchemaRef::Current),
        _ => fail
    }
    .spanned()
    .parse_next(input)
}

fn merge_directory_path_and_schema_name(
    mut directory: SchemaPath,
    schema: Spanned<Ident>,
) -> SchemaPath {
    directory.push(Spanned(SchemaPathSegment::Name(schema.0), schema.1));
    directory
}

pub fn relative_catalog_schema_reference<const GREEDY: bool>(
    input: &mut TokenStream,
) -> ModalResult<Spanned<SchemaRef>> {
    alt((
        predefined_schema_reference.map_inner(SchemaRef::Predefined),
        (relative_directory_path::<GREEDY>, schema_name)
            .map(|(directory, schema)| {
                SchemaRef::Relative(merge_directory_path_and_schema_name(directory, schema))
            })
            .spanned(),
    ))
    .parse_next(input)
}

pub fn absolute_catalog_schema_reference<const GREEDY: bool>(
    input: &mut TokenStream,
) -> ModalResult<Spanned<SchemaRef>> {
    (absolute_directory_path::<GREEDY>, schema_name)
        .map(|(directory, schema)| {
            SchemaRef::Absolute(merge_directory_path_and_schema_name(directory, schema))
        })
        .spanned()
        .parse_next(input)
}

pub fn catalog_schema_parent_and_name(input: &mut TokenStream) -> ModalResult<Spanned<SchemaPath>> {
    (absolute_directory_path::<true>, schema_name)
        .map(|(directory, schema)| merge_directory_path_and_schema_name(directory, schema))
        .spanned()
        .parse_next(input)
}

pub fn root_schema_reference(input: &mut TokenStream) -> ModalResult<Spanned<SchemaRef>> {
    TokenKind::Solidus
        .value(SchemaPath::new())
        .map(SchemaRef::Absolute)
        .spanned()
        .parse_next(input)
}

pub fn non_root_schema_reference<const GREEDY: bool>(
    input: &mut TokenStream,
) -> ModalResult<Spanned<SchemaRef>> {
    dispatch! {peek(any);
        TokenKind::Solidus => absolute_catalog_schema_reference::<GREEDY>,
        TokenKind::DoublePeriod
        | TokenKind::HomeSchema
        | TokenKind::CurrentSchema
        | TokenKind::Period => relative_catalog_schema_reference::<GREEDY>,
        TokenKind::SubstitutedParameterReference(_) => {
            reference_parameter_specification.map_inner(SchemaRef::Parameter)
        },
        _ => fail,
    }
    .parse_next(input)
}

pub fn schema_reference(input: &mut TokenStream) -> ModalResult<Spanned<SchemaRef>> {
    alt((non_root_schema_reference::<true>, root_schema_reference)).parse_next(input)
}

def_parser_alias!(
    reference_parameter_specification,
    substituted_parameter_reference,
    Spanned<Ident>
);

pub fn catalog_object_parent_reference(input: &mut TokenStream) -> ModalResult<CatalogObjectRef> {
    dispatch! {peek(any);
        TokenKind::Solidus
        | TokenKind::DoublePeriod
        | TokenKind::HomeSchema
        | TokenKind::CurrentSchema
        | TokenKind::Period
        | TokenKind::SubstitutedParameterReference(_) => {
            alt((
                (
                    non_root_schema_reference::<false>,
                    preceded(TokenKind::Solidus, repeat(0.., terminated(object_name, TokenKind::Period)))
                )
                .map(|(schema, objects)| {
                    CatalogObjectRef {
                        schema: Some(schema),
                        objects
                    }
                }),
                (
                    root_schema_reference,
                    repeat(0.., terminated(object_name, TokenKind::Period)),
                )
                .map(|(schema, objects)| {
                    CatalogObjectRef {
                        schema: Some(schema),
                        objects,
                    }
                }),
            ))
        },
        _ => repeat(1.., terminated(object_name, TokenKind::Period))
            .map(|objects| CatalogObjectRef {
                schema: None,
                objects,
            })
    }
    .parse_next(input)
}

pub fn graph_reference(input: &mut TokenStream) -> ModalResult<Spanned<GraphRef>> {
    dispatch! {peek(any);
        TokenKind::HomeGraph => home_graph,
        TokenKind::DoubleQuoted(_) | TokenKind::AccentQuoted(_) => delimited_graph_name.map_inner(GraphRef::Name),
        _ => {
            alt((
                (catalog_object_parent_reference, graph_name).map(|(mut parent, graph)| {
                    parent.objects.push(graph);
                    GraphRef::Ref(parent)
                }).spanned(),
                reference_parameter_specification.map_inner(GraphRef::Parameter)
            ))
        }
    }
    .parse_next(input)
}

pub fn home_graph(input: &mut TokenStream) -> ModalResult<Spanned<GraphRef>> {
    dispatch! {any;
        TokenKind::HomePropertyGraph | TokenKind::HomeGraph => empty.value(GraphRef::Home),
        _ => fail
    }
    .spanned()
    .parse_next(input)
}

pub fn catalog_object_parent_and_name(
    input: &mut TokenStream,
) -> ModalResult<Spanned<CatalogObjectRef>> {
    (opt(catalog_object_parent_reference), object_name)
        .map(|(parent, graph)| {
            if let Some(mut inner) = parent {
                inner.objects.push(graph);
                inner
            } else {
                CatalogObjectRef {
                    schema: None,
                    objects: [graph].into(),
                }
            }
        })
        .spanned()
        .parse_next(input)
}

def_parser_alias!(
    catalog_graph_parent_and_name,
    catalog_object_parent_and_name,
    Spanned<CatalogObjectRef>
);

pub fn graph_type_reference(input: &mut TokenStream) -> ModalResult<Spanned<GraphTypeRef>> {
    alt((
        catalog_graph_type_parent_and_name.map_inner(GraphTypeRef::Ref),
        reference_parameter_specification.map_inner(GraphTypeRef::Parameter),
    ))
    .parse_next(input)
}

def_parser_alias!(
    catalog_graph_type_parent_and_name,
    catalog_object_parent_and_name,
    Spanned<CatalogObjectRef>
);

pub fn binding_table_reference(input: &mut TokenStream) -> ModalResult<Spanned<BindingTableRef>> {
    dispatch! {peek(any);
        TokenKind::DoubleQuoted(_) | TokenKind::AccentQuoted(_) => delimited_binding_table_name.map_inner(BindingTableRef::Name),
        _ => alt((
            (catalog_object_parent_reference, binding_table_name).map(|(mut parent, name)| {
                parent.objects.push(name);
                BindingTableRef::Ref(parent)
            }).spanned(),
            reference_parameter_specification.map_inner(BindingTableRef::Parameter),
        ))
    }
    .parse_next(input)
}

pub fn procedure_reference(input: &mut TokenStream) -> ModalResult<Spanned<ProcedureRef>> {
    alt((
        catalog_procedure_parent_and_name.map_inner(ProcedureRef::Ref),
        reference_parameter_specification.map_inner(ProcedureRef::Parameter),
    ))
    .parse_next(input)
}

def_parser_alias!(
    catalog_procedure_parent_and_name,
    catalog_object_parent_and_name,
    Spanned<CatalogObjectRef>
);

#[cfg(all(test, feature = "serde"))]
mod tests {
    use insta::assert_yaml_snapshot;

    use super::*;
    use crate::parser::utils::parse;

    #[test]
    fn test_relative_directory_path_1() {
        let parsed = parse!(relative_directory_path::<true>, "../dir/");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_relative_directory_path_2() {
        let parsed = parse!(relative_directory_path::<true>, "../");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_absolute_directory_path_1() {
        let parsed = parse!(absolute_directory_path::<true>, "/dir/");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_absolute_directory_path_2() {
        let parsed = parse!(absolute_directory_path::<true>, "/");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_predefined_schema_reference_1() {
        let parsed = parse!(predefined_schema_reference, ".");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_predefined_schema_reference_2() {
        let parsed = parse!(predefined_schema_reference, "home_schema");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_predefined_schema_reference_3() {
        let parsed = parse!(predefined_schema_reference, "current_schema");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_absolute_catalog_schema_reference_1() {
        let parsed = parse!(absolute_catalog_schema_reference::<true>, "/s");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_absolute_catalog_schema_reference_2() {
        let parsed = parse!(absolute_catalog_schema_reference::<true>, "/d/s");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_relative_catalog_schema_reference_1() {
        let parsed = parse!(relative_catalog_schema_reference::<true>, "../s");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_relative_catalog_schema_reference_2() {
        let parsed = parse!(relative_catalog_schema_reference::<true>, "../../d/s");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_catalog_object_parent_reference_1() {
        let parsed = parse!(catalog_object_parent_reference, "/");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_catalog_object_parent_reference_2() {
        let parsed = parse!(catalog_object_parent_reference, "/a/b.c.");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_catalog_object_parent_reference_3() {
        let parsed = parse!(catalog_object_parent_reference, "a.b.");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_catalog_object_parent_reference_4() {
        let parsed = parse!(catalog_object_parent_reference, "/a/b/");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_catalog_object_parent_reference_5() {
        let parsed = parse!(catalog_object_parent_reference, "/a.b.");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_catalog_object_parent_reference_6() {
        // Invalid input
        let parsed = parse!(catalog_object_parent_reference, "/a b");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_catalog_object_parent_reference_7() {
        // Invalid input
        let parsed = parse!(catalog_object_parent_reference, "//a.");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_graph_reference_1() {
        let parsed = parse!(graph_reference, "home_graph");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_graph_reference_2() {
        let parsed = parse!(graph_reference, "`graph`");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_graph_reference_3() {
        let parsed = parse!(graph_reference, "$$abc");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_graph_reference_4() {
        let parsed = parse!(graph_reference, "/a/b");
        assert_yaml_snapshot!(parsed);
    }
}
