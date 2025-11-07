use winnow::combinator::{
    alt, delimited, dispatch, empty, fail, opt, peek, preceded, separated, separated_pair, seq,
    terminated,
};
use winnow::token::one_of;
use winnow::{ModalResult, Parser};

use super::common::is_or_colon;
use super::lexical::{
    edge_synonym, edge_type_name, implies, label_name, node_synonym, node_type_name, property_name,
    regular_identifier, unsigned_integer,
};
use crate::ast::*;
use crate::imports::Box;
use crate::lexer::TokenKind;
use crate::parser::token::{TokenStream, any};
use crate::parser::utils::{SpannedParserExt, ToSpanned, def_parser_alias};
use crate::span::{OptSpanned, Spanned, VecSpanned};

pub fn typed(input: &mut TokenStream) -> ModalResult<()> {
    one_of((TokenKind::Typed, TokenKind::DoubleColon))
        .void()
        .parse_next(input)
}

pub fn nested_graph_type_specification(
    input: &mut TokenStream,
) -> ModalResult<VecSpanned<GraphElementType>> {
    delimited(
        TokenKind::LeftBrace,
        element_type_list,
        TokenKind::RightBrace,
    )
    .parse_next(input)
}

pub fn element_type_list(input: &mut TokenStream) -> ModalResult<VecSpanned<GraphElementType>> {
    separated(1.., element_type_specification, TokenKind::Comma).parse_next(input)
}

pub fn element_type_specification(
    input: &mut TokenStream,
) -> ModalResult<Spanned<GraphElementType>> {
    dispatch! {peek(any);
        TokenKind::Node | TokenKind::Vertex => {
            node_type_specification
                .map_inner(Box::new)
                .map_inner(GraphElementType::Node)
        },
        TokenKind::Directed
        | TokenKind::Undirected
        | TokenKind::Edge
        | TokenKind::Relationship => {
            edge_type_specification
                .map_inner(Box::new)
                .map_inner(GraphElementType::Edge)
        },
        _ => alt((
            edge_type_specification
                .map_inner(Box::new)
                .map_inner(GraphElementType::Edge),
            node_type_specification
                .map_inner(Box::new)
                .map_inner(GraphElementType::Node),
            fail
        ))
    }
    .parse_next(input)
}

pub fn node_type_specification(input: &mut TokenStream) -> ModalResult<Spanned<NodeType>> {
    dispatch! {peek(any);
        TokenKind::LeftParen => node_type_pattern,
        TokenKind::Node | TokenKind::Vertex => alt((node_type_pattern, node_type_phrase)),
        _ => fail,
    }
    .parse_next(input)
}

pub fn node_type_pattern(input: &mut TokenStream) -> ModalResult<Spanned<NodeType>> {
    seq! {NodeType {
        name: opt(preceded((node_synonym, opt(TokenKind::Type)), node_type_name)),
        _: TokenKind::LeftParen,
        alias: opt(local_node_type_alias),
        filler: opt(node_type_filler),
        _: TokenKind::RightParen,
    }}
    .spanned()
    .parse_next(input)
}

pub fn node_type_phrase(input: &mut TokenStream) -> ModalResult<Spanned<NodeType>> {
    preceded(
        (node_synonym, opt(TokenKind::Type)),
        (
            node_type_phrase_filler,
            opt(preceded(TokenKind::As, local_node_type_alias)),
        ),
    )
    .map(|((name, filler), alias)| NodeType {
        name,
        alias,
        filler,
    })
    .spanned()
    .parse_next(input)
}

def_parser_alias!(local_node_type_alias, regular_identifier, Spanned<Ident>);
def_parser_alias!(node_type_alias, regular_identifier, Spanned<Ident>);

pub fn node_type_phrase_filler(
    input: &mut TokenStream,
) -> ModalResult<(OptSpanned<Ident>, OptSpanned<NodeOrEdgeTypeFiller>)> {
    dispatch! {opt(node_type_name);
        Some(name) => (empty.value(Some(name)), opt(node_type_filler)),
        None => (empty.value(None), node_type_filler.map(Some))
    }
    .parse_next(input)
}

def_parser_alias!(
    edge_type_phrase_filler,
    node_type_phrase_filler,
    (OptSpanned<Ident>, OptSpanned<NodeOrEdgeTypeFiller>)
);

pub fn node_type_filler(input: &mut TokenStream) -> ModalResult<Spanned<NodeOrEdgeTypeFiller>> {
    dispatch! {opt(node_type_key_label_set);
        Some(key) => {
            (empty.value(key), opt(node_type_implied_content))
                .map(|(key, implied)| {
                    let (label_set, property_types) = implied.unzip();
                    let label_set = label_set.flatten();
                    let property_types = property_types.flatten();
                    NodeOrEdgeTypeFiller { key, label_set, property_types }
                })
        },
        None => {
            node_type_implied_content
                .map(|(label_set, property_types)| {
                    NodeOrEdgeTypeFiller { key: None, label_set, property_types }
                })
        },
    }
    .spanned()
    .parse_next(input)
}

def_parser_alias!(
    edge_type_filler,
    node_type_filler,
    Spanned<NodeOrEdgeTypeFiller>
);

def_parser_alias!(node_type_label_set, label_set_phrase, Spanned<LabelSet>);

pub fn node_type_key_label_set(input: &mut TokenStream) -> ModalResult<OptSpanned<LabelSet>> {
    terminated(opt(label_set_phrase), implies).parse_next(input)
}

pub fn node_type_implied_content(
    input: &mut TokenStream,
) -> ModalResult<(
    OptSpanned<LabelSet>,
    Option<VecSpanned<FieldOrPropertyType>>,
)> {
    dispatch! {opt(node_type_label_set);
        Some(label_set) => {
          (empty.value(Some(label_set)), opt(node_type_property_types))
        },
        None => {
          (empty.value(None), node_type_property_types.map(Some))
        }
    }
    .parse_next(input)
}

pub fn node_type_reference(input: &mut TokenStream) -> ModalResult<Spanned<NodeTypeRef>> {
    delimited(
        TokenKind::LeftParen,
        alt((
            node_type_alias.map_inner(NodeTypeRef::Alias),
            opt(node_type_filler.unspanned())
                .map(|filler| match filler {
                    Some(filler) => NodeTypeRef::Filler(filler),
                    None => NodeTypeRef::Empty,
                })
                .spanned(),
        )),
        TokenKind::RightParen,
    )
    .update_span()
    .parse_next(input)
}

pub fn edge_type_specification(input: &mut TokenStream) -> ModalResult<Spanned<EdgeType>> {
    dispatch! {peek(any);
        TokenKind::Edge
        | TokenKind::Relationship
        | TokenKind::LeftParen => {
            edge_type_pattern
                .map_inner(Box::new)
                .map_inner(EdgeType::Pattern)
        },
        TokenKind::Directed | TokenKind::Undirected => {
            alt((
                edge_type_pattern
                    .map_inner(Box::new)
                    .map_inner(EdgeType::Pattern),
                edge_type_phrase
                    .map_inner(Box::new)
                    .map_inner(EdgeType::Phrase),
            ))
        },
        _ => fail
    }
    .parse_next(input)
}

#[derive(Clone)]
enum EdgeKind {
    Directed,
    Undirected,
}

/// Indicates whether the edge is directed or not. This will not appear in the AST.
fn edge_kind(input: &mut TokenStream) -> ModalResult<EdgeKind> {
    dispatch! {any;
        TokenKind::Directed => empty.value(EdgeKind::Directed),
        TokenKind::Undirected => empty.value(EdgeKind::Undirected),
        _ => fail,
    }
    .parse_next(input)
}

pub fn edge_type_pattern(input: &mut TokenStream) -> ModalResult<Spanned<EdgeTypePattern>> {
    let mut prefix = opt(separated_pair(
        opt(edge_kind),
        (edge_synonym, opt(TokenKind::Type)),
        edge_type_name,
    ));
    dispatch! {prefix;
        Some((Some(EdgeKind::Directed), name)) => {
            (empty.value(name), edge_type_pattern_directed.unspanned())
                .map(|(name, mut pat)| {
                    pat.name = Some(name);
                    pat
                })
        },
        Some((Some(EdgeKind::Undirected), name)) => {
            (empty.value(name), edge_type_pattern_undirected.unspanned())
                .map(|(name, mut pat)| {
                    pat.name = Some(name);
                    pat
                })
        },
        Some((None, name)) => {
            (
                empty.value(name),
                alt((
                    edge_type_pattern_directed,
                    edge_type_pattern_undirected
                ))
                .unspanned()
            )
            .map(|(name, mut pat)| {
                pat.name = Some(name);
                pat
            })
        },
        None => {
            alt((
                edge_type_pattern_directed,
                edge_type_pattern_undirected
            ))
            .unspanned()
        },
    }
    .spanned()
    .parse_next(input)
}

pub fn edge_type_phrase(input: &mut TokenStream) -> ModalResult<Spanned<EdgeTypePhrase>> {
    let mut prefix = separated_pair(
        edge_kind,
        (edge_synonym, opt(TokenKind::Type)),
        terminated(edge_type_phrase_filler, TokenKind::Connecting),
    );
    dispatch! {prefix;
        (EdgeKind::Directed, (name, filler)) => {
            (empty.value((name, filler)), endpoint_pair_directed)
                .map(|((name, filler), (left, direction, right))| {
                    EdgeTypePhrase {
                        name,
                        direction,
                        left,
                        filler,
                        right,
                    }
            })
        },
        (EdgeKind::Undirected, (name, filler)) => {
            (empty.value((name, filler)), endpoint_pair_undirected)
                .map(|((name, filler), (left, right))| {
                    EdgeTypePhrase {
                        name,
                        direction: EdgeDirection::Undirected,
                        left,
                        filler,
                        right,
                }
            })
        },
    }
    .spanned()
    .parse_next(input)
}

pub fn endpoint_pair_directed(
    input: &mut TokenStream,
) -> ModalResult<(Spanned<Ident>, EdgeDirection, Spanned<Ident>)> {
    let connector = dispatch! {any;
        TokenKind::To | TokenKind::RightArrow => empty.value(EdgeDirection::LeftToRight),
        TokenKind::LeftArrow => empty.value(EdgeDirection::RightToLeft),
        _ => fail
    };
    delimited(
        TokenKind::LeftParen,
        (node_type_alias, connector, node_type_alias),
        TokenKind::RightParen,
    )
    .parse_next(input)
}

pub fn endpoint_pair_undirected(
    input: &mut TokenStream,
) -> ModalResult<(Spanned<Ident>, Spanned<Ident>)> {
    delimited(
        TokenKind::LeftParen,
        separated_pair(
            node_type_alias,
            one_of((TokenKind::To, TokenKind::Tilde)),
            node_type_alias,
        ),
        TokenKind::RightParen,
    )
    .parse_next(input)
}

pub fn edge_type_pattern_directed(
    input: &mut TokenStream,
) -> ModalResult<Spanned<EdgeTypePattern>> {
    let arc = dispatch! {peek(any);
        TokenKind::MinusLeftBracket => {
            arc_type_pointing_right
                .map(|filler| (EdgeDirection::LeftToRight, filler))
        },
        TokenKind::LeftArrowBracket => {
            arc_type_pointing_left
                .map(|filler| (EdgeDirection::RightToLeft, filler))
        },
        _ => fail
    };
    (node_type_reference, arc, node_type_reference)
        .map(|(left, (direction, filler), right)| EdgeTypePattern {
            name: None,
            direction,
            left,
            filler,
            right,
        })
        .spanned()
        .parse_next(input)
}

pub fn arc_type_pointing_right(
    input: &mut TokenStream,
) -> ModalResult<Spanned<NodeOrEdgeTypeFiller>> {
    delimited(
        TokenKind::MinusLeftBracket,
        edge_type_filler,
        TokenKind::BracketRightArrow,
    )
    .update_span()
    .parse_next(input)
}

pub fn arc_type_pointing_left(
    input: &mut TokenStream,
) -> ModalResult<Spanned<NodeOrEdgeTypeFiller>> {
    delimited(
        TokenKind::LeftArrowBracket,
        edge_type_filler,
        TokenKind::RightBracketMinus,
    )
    .update_span()
    .parse_next(input)
}

pub fn arc_type_undirected(input: &mut TokenStream) -> ModalResult<Spanned<NodeOrEdgeTypeFiller>> {
    delimited(
        TokenKind::TildeLeftBracket,
        edge_type_filler,
        TokenKind::RightBracketTilde,
    )
    .update_span()
    .parse_next(input)
}

pub fn edge_type_pattern_undirected(
    input: &mut TokenStream,
) -> ModalResult<Spanned<EdgeTypePattern>> {
    (
        node_type_reference,
        arc_type_undirected,
        node_type_reference,
    )
        .map(|(left, filler, right)| EdgeTypePattern {
            name: None,
            direction: EdgeDirection::Undirected,
            left,
            filler,
            right,
        })
        .spanned()
        .parse_next(input)
}

pub fn label_set_phrase(input: &mut TokenStream) -> ModalResult<Spanned<LabelSet>> {
    dispatch! {peek(any);
        TokenKind::Label => {
            preceded(TokenKind::Label, label_name)
                .map(|label| [label].into())
                .spanned()
        },
        TokenKind::Labels => preceded(TokenKind::Labels, label_set_specification),
        TokenKind::Is | TokenKind::Colon => preceded(is_or_colon, label_set_specification),
        _ => fail
    }
    .parse_next(input)
}

pub fn label_set_specification(input: &mut TokenStream) -> ModalResult<Spanned<LabelSet>> {
    separated(1.., label_name, TokenKind::Ampersand)
        .spanned()
        .parse_next(input)
}

def_parser_alias!(
    node_type_property_types,
    property_types_specification,
    VecSpanned<FieldOrPropertyType>
);

pub fn property_types_specification(
    input: &mut TokenStream,
) -> ModalResult<VecSpanned<FieldOrPropertyType>> {
    delimited(
        TokenKind::LeftBrace,
        separated(0.., property_type, TokenKind::Comma),
        TokenKind::RightBrace,
    )
    .parse_next(input)
}

pub fn property_type(input: &mut TokenStream) -> ModalResult<Spanned<FieldOrPropertyType>> {
    seq! {FieldOrPropertyType {
        name: property_name,
        _: opt(typed),
        value_type: value_type,
    }}
    .spanned()
    .parse_next(input)
}

pub fn value_type(input: &mut TokenStream) -> ModalResult<Spanned<ValueType>> {
    dispatch! {peek(any);
        TokenKind::Path => path_value_type,
        kind if kind.is_prefix_of_predefined_type() => predefined_type,
        _ => fail
    }
    .parse_next(input)
}

pub fn path_value_type(input: &mut TokenStream) -> ModalResult<Spanned<ValueType>> {
    preceded(TokenKind::Path, opt(not_null))
        .map(|not_null| ValueType::Path {
            not_null: not_null.is_some(),
        })
        .spanned()
        .parse_next(input)
}

pub fn predefined_type(input: &mut TokenStream) -> ModalResult<Spanned<ValueType>> {
    dispatch! {peek(any);
        TokenKind::Bool | TokenKind::Boolean => boolean_type,
        TokenKind::String | TokenKind::Char | TokenKind::Varchar => character_string_type,
        TokenKind::Bytes | TokenKind::Binary | TokenKind::Varbinary => byte_string_type,
        TokenKind::Decimal | TokenKind::Dec => decimal_numeric_type,
        TokenKind::Null | TokenKind::Nothing => immaterial_type,
        TokenKind::Vector => vector_type,
        kind if kind.is_prefix_of_signed_exact_numeric_type() => signed_binary_exact_numeric_type,
        kind if kind.is_prefix_of_unsigned_exact_numeric_type() => unsigned_binary_exact_numeric_type,
        kind if kind.is_prefix_of_temporal_type() => temporal_type,
        _ => fail
    }
    .parse_next(input)
}

pub fn vector_type(input: &mut TokenStream) -> ModalResult<Spanned<ValueType>> {
    seq! {ValueType::Vector {
        _: TokenKind::Vector,
        _: TokenKind::LeftParen,
        dimension: unsigned_integer.map(Box::new),
        _: TokenKind::RightParen,
        not_null: opt(not_null).map(|not_null| not_null.is_some()),
    }}
    .spanned()
    .parse_next(input)
}

pub fn temporal_type(input: &mut TokenStream) -> ModalResult<Spanned<ValueType>> {
    seq! {ValueType::Temporal {
        kind: temporal_type_kind,
        not_null: opt(not_null).map(|not_null| not_null.is_some()),
    }}
    .spanned()
    .parse_next(input)
}

// TODO: Add other temporal types.
pub fn temporal_type_kind(input: &mut TokenStream) -> ModalResult<Spanned<TemporalTypeKind>> {
    dispatch! {any;
        TokenKind::Date => empty.value(TemporalTypeKind::Date),
        _ => fail
    }
    .spanned()
    .parse_next(input)
}

pub fn immaterial_type(input: &mut TokenStream) -> ModalResult<Spanned<ValueType>> {
    alt((empty_type, null_type)).parse_next(input)
}

pub fn null_type(input: &mut TokenStream) -> ModalResult<Spanned<ValueType>> {
    TokenKind::Null
        .value(ValueType::Null)
        .spanned()
        .parse_next(input)
}

pub fn empty_type(input: &mut TokenStream) -> ModalResult<Spanned<ValueType>> {
    dispatch! {any;
        TokenKind::Null => not_null.value(ValueType::Empty),
        TokenKind::Nothing => empty.value(ValueType::Empty),
        _ => fail
    }
    .spanned()
    .parse_next(input)
}

pub fn boolean_type(input: &mut TokenStream) -> ModalResult<Spanned<ValueType>> {
    seq! {ValueType::Bool {
        _: one_of((TokenKind::Bool, TokenKind::Boolean)),
        not_null: opt(not_null).map(|not_null| not_null.is_some()),
    }}
    .spanned()
    .parse_next(input)
}

pub fn character_string_type(input: &mut TokenStream) -> ModalResult<Spanned<ValueType>> {
    let mut parse_string = (
        opt(delimited(
            TokenKind::LeftParen,
            (opt(terminated(min_length, TokenKind::Comma)), max_length),
            TokenKind::RightParen,
        )),
        opt(not_null),
    )
        .map(|(length, not_null)| {
            let (min_length, max_length) = length.unzip();
            let min_length = min_length.flatten().map(Box::new);
            let max_length = max_length.map(Box::new);
            let not_null = not_null.is_some();
            ValueType::String {
                min_length,
                max_length,
                not_null,
            }
        });
    let mut parse_char = (
        opt(delimited(
            TokenKind::LeftParen,
            fixed_length,
            TokenKind::RightParen,
        )),
        opt(not_null),
    )
        .map(|(length, not_null)| {
            let length = length.map(Box::new);
            let not_null = not_null.is_some();
            ValueType::Char { length, not_null }
        });
    let mut parse_varchar = (
        opt(delimited(
            TokenKind::LeftParen,
            max_length,
            TokenKind::RightParen,
        )),
        opt(not_null),
    )
        .map(|(max_length, not_null)| {
            let max_length = max_length.map(Box::new);
            let not_null = not_null.is_some();
            ValueType::Varchar {
                max_length,
                not_null,
            }
        });
    dispatch! {any;
        TokenKind::String => parse_string,
        TokenKind::Char => parse_char,
        TokenKind::Varchar => parse_varchar,
        _ => fail
    }
    .spanned()
    .parse_next(input)
}

pub fn byte_string_type(input: &mut TokenStream) -> ModalResult<Spanned<ValueType>> {
    let mut parse_bytes = (
        opt(delimited(
            TokenKind::LeftParen,
            (opt(terminated(min_length, TokenKind::Comma)), max_length),
            TokenKind::RightParen,
        )),
        opt(not_null),
    )
        .map(|(length, not_null)| {
            let (min_length, max_length) = length.unzip();
            let min_length = min_length.flatten().map(Box::new);
            let max_length = max_length.map(Box::new);
            let not_null = not_null.is_some();
            ValueType::Bytes {
                min_length,
                max_length,
                not_null,
            }
        });
    let mut parse_binary = (
        opt(delimited(
            TokenKind::LeftParen,
            fixed_length,
            TokenKind::RightParen,
        )),
        opt(not_null),
    )
        .map(|(length, not_null)| {
            let length = length.map(Box::new);
            let not_null = not_null.is_some();
            ValueType::Binary { length, not_null }
        });
    let mut parse_varbinary = (
        opt(delimited(
            TokenKind::LeftParen,
            max_length,
            TokenKind::RightParen,
        )),
        opt(not_null),
    )
        .map(|(max_length, not_null)| {
            let max_length = max_length.map(Box::new);
            let not_null = not_null.is_some();
            ValueType::Varbinary {
                max_length,
                not_null,
            }
        });
    dispatch! {any;
        TokenKind::Bytes => parse_bytes,
        TokenKind::Binary => parse_binary,
        TokenKind::Varbinary => parse_varbinary,
        _ => fail
    }
    .spanned()
    .parse_next(input)
}

pub fn signed_binary_exact_numeric_type(
    input: &mut TokenStream,
) -> ModalResult<Spanned<ValueType>> {
    seq! {ValueType::SignedNumeric {
        kind: signed_numeric_type_kind,
        not_null: opt(not_null).map(|not_null| not_null.is_some()),
    }}
    .spanned()
    .parse_next(input)
}

pub fn unsigned_binary_exact_numeric_type(
    input: &mut TokenStream,
) -> ModalResult<Spanned<ValueType>> {
    seq! {ValueType::UnsignedNumeric {
        kind: unsigned_numeric_type_kind,
        not_null: opt(not_null).map(|not_null| not_null.is_some()),
    }}
    .spanned()
    .parse_next(input)
}

pub fn decimal_numeric_type(input: &mut TokenStream) -> ModalResult<Spanned<ValueType>> {
    preceded(
        one_of((TokenKind::Decimal, TokenKind::Dec)),
        (
            opt(delimited(
                TokenKind::LeftParen,
                (precision, opt(preceded(TokenKind::Comma, scale))),
                TokenKind::RightParen,
            )),
            opt(not_null),
        ),
    )
    .map(|(precision_scale, not_null)| {
        let (precision, scale) = precision_scale.unzip();
        let precision = precision.map(Box::new);
        let scale = scale.flatten().map(Box::new);
        ValueType::Decimal {
            precision,
            scale,
            not_null: not_null.is_some(),
        }
    })
    .spanned()
    .parse_next(input)
}

pub fn signed_numeric_type_kind(input: &mut TokenStream) -> ModalResult<Spanned<NumericTypeKind>> {
    dispatch! {peek(any);
        TokenKind::Int8 => TokenKind::Int8.value(NumericTypeKind::Int8),
        TokenKind::Int16 => TokenKind::Int16.value(NumericTypeKind::Int16),
        TokenKind::Int32 => TokenKind::Int32.value(NumericTypeKind::Int32),
        TokenKind::Int64 => TokenKind::Int64.value(NumericTypeKind::Int64),
        TokenKind::Int128 => TokenKind::Int128.value(NumericTypeKind::Int128),
        TokenKind::Int256 => TokenKind::Int256.value(NumericTypeKind::Int256),
        TokenKind::Smallint => TokenKind::Smallint.value(NumericTypeKind::Small),
        TokenKind::Int => {
            preceded(
                TokenKind::Int,
                opt(delimited(
                    TokenKind::LeftParen,
                    precision,
                    TokenKind::RightParen,
                )),
            )
            .map(|precision| NumericTypeKind::Int(precision.map(Box::new)))
        },
        TokenKind::Bigint => TokenKind::Bigint.value(NumericTypeKind::Big),
        TokenKind::Signed => {
            preceded(TokenKind::Signed, verbose_numeric_type_kind.unspanned())
        },
        kind if kind.is_prefix_of_verbose_exact_numeric_type() => verbose_numeric_type_kind.unspanned(),
        _ => fail,
    }
    .spanned()
    .parse_next(input)
}

pub fn unsigned_numeric_type_kind(
    input: &mut TokenStream,
) -> ModalResult<Spanned<NumericTypeKind>> {
    dispatch! {any;
        TokenKind::Uint8 => empty.value(NumericTypeKind::Int8),
        TokenKind::Uint16 => empty.value(NumericTypeKind::Int16),
        TokenKind::Uint32 => empty.value(NumericTypeKind::Int32),
        TokenKind::Uint64 => empty.value(NumericTypeKind::Int64),
        TokenKind::Uint128 => empty.value(NumericTypeKind::Int128),
        TokenKind::Uint256 => empty.value(NumericTypeKind::Int256),
        TokenKind::Usmallint => empty.value(NumericTypeKind::Small),
        TokenKind::Uint => {
            opt(delimited(
                TokenKind::LeftParen,
                precision,
                TokenKind::RightParen,
            ))
            .map(|precision| NumericTypeKind::Int(precision.map(Box::new)))
        },
        TokenKind::Ubigint => empty.value(NumericTypeKind::Big),
        TokenKind::Unsigned => verbose_numeric_type_kind.unspanned(),
        _ => fail,
    }
    .spanned()
    .parse_next(input)
}

pub fn verbose_numeric_type_kind(input: &mut TokenStream) -> ModalResult<Spanned<NumericTypeKind>> {
    dispatch! {any;
        TokenKind::Integer8 => empty.value(NumericTypeKind::Int8),
        TokenKind::Integer16 => empty.value(NumericTypeKind::Int16),
        TokenKind::Integer32 => empty.value(NumericTypeKind::Int32),
        TokenKind::Integer64 => empty.value(NumericTypeKind::Int64),
        TokenKind::Integer128 => empty.value(NumericTypeKind::Int128),
        TokenKind::Integer256 => empty.value(NumericTypeKind::Int256),
        TokenKind::Small => TokenKind::Integer.value(NumericTypeKind::Small),
        TokenKind::Integer => {
            opt(delimited(
                TokenKind::LeftParen,
                precision,
                TokenKind::RightParen,
            ))
            .map(|precision| NumericTypeKind::Int(precision.map(Box::new)))
        },
        TokenKind::Big => TokenKind::Integer.value(NumericTypeKind::Big),
        _ => fail,
    }
    .spanned()
    .parse_next(input)
}

def_parser_alias!(min_length, unsigned_integer, Spanned<UnsignedInteger>);
def_parser_alias!(max_length, unsigned_integer, Spanned<UnsignedInteger>);
def_parser_alias!(fixed_length, unsigned_integer, Spanned<UnsignedInteger>);
def_parser_alias!(precision, unsigned_integer, Spanned<UnsignedInteger>);
def_parser_alias!(scale, unsigned_integer, Spanned<UnsignedInteger>);

pub fn not_null(input: &mut TokenStream) -> ModalResult<()> {
    (TokenKind::Not, TokenKind::Null).void().parse_next(input)
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use insta::assert_yaml_snapshot;

    use super::*;
    use crate::parser::utils::parse;

    #[test]
    fn test_edge_type_pattern_directed_1() {
        let parsed = parse!(edge_type_pattern, "(a) -[:knows {since int}]-> (b)");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_edge_type_pattern_directed_2() {
        let parsed = parse!(
            edge_type_pattern,
            "(:Person) <-[:know_ => :knows {since::int}]- (:Person)"
        );
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_edge_type_pattern_undirected() {
        let parsed = parse!(
            edge_type_pattern,
            "(:Person) ~[:knows {since int}]~ (:Person)"
        );
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_edge_type_phrase_1() {
        let parsed = parse!(
            edge_type_phrase,
            "directed edge type et :transfer {amount int} connecting (a->b)"
        );
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_edge_type_phrase_2() {
        let parsed = parse!(
            edge_type_phrase,
            "undirected edge type et :transfer {amount int} connecting (a~b)"
        );
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_node_type_pattern_1() {
        let parsed = parse!(node_type_pattern, "(person:Person {id int, name string})");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_node_type_pattern_2() {
        let parsed = parse!(
            node_type_pattern,
            "(:Person1 => :Person2 {id int, name string not null})"
        );
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_node_type_phrase() {
        let parsed = parse!(node_type_phrase, "vertex type nt :Person => {id::int} as p");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_property_type() {
        let parsed = parse!(property_type, "id int");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_property_types_specification_1() {
        let parsed = parse!(property_types_specification, "{ id::int, name string }");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_property_types_specification_2() {
        let parsed = parse!(property_types_specification, "{}");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_typed() {
        let parsed = parse!(typed, "typed");
        assert!(parsed.is_some());
        let parsed = parse!(typed, "::");
        assert!(parsed.is_some());
    }

    #[test]
    fn test_immaterial_type_1() {
        let parsed = parse!(value_type, "null");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_immaterial_type_2() {
        let parsed = parse!(value_type, "nothing");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_immaterial_type_3() {
        let parsed = parse!(value_type, "null not null");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_boolean_type_1() {
        let parsed = parse!(value_type, "boolean");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_boolean_type_2() {
        let parsed = parse!(value_type, "bool not null");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_character_string_type_1() {
        let parsed = parse!(value_type, "string (1, 10) not null");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_character_string_type_2() {
        let parsed = parse!(value_type, "char (10) not null");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_character_string_type_3() {
        let parsed = parse!(value_type, "varchar");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_byte_string_type_1() {
        let parsed = parse!(value_type, "bytes (1, 10) not null");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_byte_string_type_2() {
        let parsed = parse!(value_type, "binary (10) not null");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_byte_string_type_3() {
        let parsed = parse!(value_type, "varbinary");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_exact_numeric_type_1() {
        let parsed = parse!(value_type, "int16 not null");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_exact_numeric_type_2() {
        let parsed = parse!(value_type, "signed integer32");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_exact_numeric_type_3() {
        let parsed = parse!(value_type, "unsigned big integer");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_exact_numeric_type_4() {
        let parsed = parse!(value_type, "uint(64)");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_exact_numeric_type_5() {
        let parsed = parse!(value_type, "decimal(5,2)");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_vector_type_1() {
        let parsed = parse!(value_type, "vector(128)");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_vector_type_2() {
        let parsed = parse!(value_type, "vector(256) not null");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_vector_type_3() {
        let parsed = parse!(value_type, "vector(4)");
        assert_yaml_snapshot!(parsed);
    }
}
