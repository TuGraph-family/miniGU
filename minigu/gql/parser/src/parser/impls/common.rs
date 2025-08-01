use winnow::combinator::{
    alt, delimited, dispatch, empty, fail, opt, peek, preceded, repeat, separated, separated_pair,
    seq, terminated,
};
use winnow::token::one_of;
use winnow::{ModalResult, Parser};

use super::lexical::{
    binding_variable, edge_synonym, edges_synonym, element_variable, field_name, label_name,
    path_variable, property_name, subpath_variable, unsigned_integer,
};
use super::object_expr::graph_expression;
use super::object_ref::schema_reference;
use super::predicate::search_condition;
use super::value_expr::{
    aggregating_value_expression, binding_variable_reference, value_expression,
};
use crate::ast::*;
use crate::imports::Box;
use crate::lexer::TokenKind;
use crate::parser::impls::value_expr::non_negative_integer_specification;
use crate::parser::precedence::{Assoc, Precedence, precedence};
use crate::parser::token::{TokenStream, any};
use crate::parser::utils::{SpannedParserExt, ToSpanned, def_parser_alias};
use crate::span::{Spanned, VecSpanned};

const PREC_INIT: Precedence = 0;
const PREC_DISJUNCTION: Precedence = 1;
const PREC_CONJUNCTION: Precedence = 2;
const PREC_NEGATION: Precedence = 3;

pub fn use_graph_clause(input: &mut TokenStream) -> ModalResult<Spanned<GraphExpr>> {
    preceded(TokenKind::Use, graph_expression).parse_next(input)
}

pub fn at_schema_clause(input: &mut TokenStream) -> ModalResult<Spanned<SchemaRef>> {
    preceded(TokenKind::At, schema_reference).parse_next(input)
}

pub fn yield_clause(input: &mut TokenStream) -> ModalResult<Spanned<Yield>> {
    preceded(
        TokenKind::Yield,
        separated(1.., yield_item, TokenKind::Comma),
    )
    .spanned()
    .parse_next(input)
}

pub fn yield_item(input: &mut TokenStream) -> ModalResult<Spanned<YieldItem>> {
    seq! {YieldItem {
        name: yield_item_name,
        alias: opt(yield_item_alias),
    }}
    .spanned()
    .parse_next(input)
}

pub fn yield_item_alias(input: &mut TokenStream) -> ModalResult<Spanned<Ident>> {
    preceded(TokenKind::As, binding_variable).parse_next(input)
}

def_parser_alias!(yield_item_name, field_name, Spanned<Ident>);

pub fn is_or_colon(input: &mut TokenStream) -> ModalResult<()> {
    one_of((TokenKind::Is, TokenKind::Colon))
        .void()
        .parse_next(input)
}

pub fn graph_pattern_binding_table(
    input: &mut TokenStream,
) -> ModalResult<Spanned<GraphPatternBindingTable>> {
    seq! {GraphPatternBindingTable {
        pattern: graph_pattern,
        yield_clause: opt(graph_pattern_yield_clause).map(Option::unwrap_or_default),
    }}
    .spanned()
    .parse_next(input)
}

pub fn graph_pattern_yield_clause(input: &mut TokenStream) -> ModalResult<VecSpanned<Ident>> {
    preceded(
        TokenKind::Yield,
        separated(1.., binding_variable_reference, TokenKind::Comma),
    )
    .parse_next(input)
}

pub fn graph_pattern(input: &mut TokenStream) -> ModalResult<Spanned<GraphPattern>> {
    seq! {GraphPattern {
        match_mode: opt(match_mode),
        patterns: separated(1.., path_pattern, TokenKind::Comma),
        keep: opt(keep_clause),
        where_clause: opt(graph_pattern_where_clause),
    }}
    .spanned()
    .parse_next(input)
}

pub fn keep_clause(input: &mut TokenStream) -> ModalResult<Spanned<PathPatternPrefix>> {
    preceded(TokenKind::Keep, path_pattern_prefix).parse_next(input)
}

pub fn match_mode(input: &mut TokenStream) -> ModalResult<Spanned<MatchMode>> {
    dispatch! {any;
        TokenKind::Repeatable => element_bindings_or_elements.value(MatchMode::Repeatable),
        TokenKind::Different => edge_bindings_or_edges.value(MatchMode::Different),
        _ => fail,
    }
    .spanned()
    .parse_next(input)
}

pub fn element_bindings_or_elements(input: &mut TokenStream) -> ModalResult<()> {
    alt((
        (TokenKind::Element, opt(TokenKind::Bindings)).void(),
        TokenKind::Elements.void(),
    ))
    .parse_next(input)
}

pub fn edge_bindings_or_edges(input: &mut TokenStream) -> ModalResult<()> {
    alt((
        (edge_synonym, opt(TokenKind::Bindings)).void(),
        edges_synonym,
    ))
    .parse_next(input)
}

pub fn element_pattern(input: &mut TokenStream) -> ModalResult<Spanned<ElementPattern>> {
    dispatch! {peek(any);
        TokenKind::LeftParen => node_pattern,
        kind if kind.is_prefix_of_edge_pattern() => edge_pattern,
        _ => fail
    }
    .parse_next(input)
}

pub fn node_pattern(input: &mut TokenStream) -> ModalResult<Spanned<ElementPattern>> {
    delimited(
        TokenKind::LeftParen,
        element_pattern_filler,
        TokenKind::RightParen,
    )
    .map_inner(ElementPattern::Node)
    .update_span()
    .parse_next(input)
}

pub fn edge_pattern(input: &mut TokenStream) -> ModalResult<Spanned<ElementPattern>> {
    dispatch! {peek(any);
        kind if kind.is_prefix_of_full_edge_pattern() => full_edge_pattern,
        kind if kind.is_prefix_of_abbreviated_edge_pattern() => {
            abbreviated_edge_pattern
                .map(|kind| ElementPattern::Edge {
                    kind,
                    filler: ElementPatternFiller {
                        variable: None,
                        label: None,
                        predicate: None,
                    },
                })
                .spanned()
        },
        _ => fail
    }
    .parse_next(input)
}

pub fn full_edge_pattern(input: &mut TokenStream) -> ModalResult<Spanned<ElementPattern>> {
    // This avoids matching `element_pattern_filler` multiple times.
    (any, element_pattern_filler.unspanned(), any)
        .verify_map(|(left, filler, right)| {
            let kind = match (left, right) {
                (TokenKind::LeftArrowBracket, TokenKind::RightBracketMinus) => {
                    EdgePatternKind::Left
                }
                (TokenKind::TildeLeftBracket, TokenKind::RightBracketTilde) => {
                    EdgePatternKind::Undirected
                }
                (TokenKind::MinusLeftBracket, TokenKind::BracketRightArrow) => {
                    EdgePatternKind::Right
                }
                (TokenKind::LeftArrowTildeBracket, TokenKind::RightBracketTilde) => {
                    EdgePatternKind::LeftUndirected
                }
                (TokenKind::TildeLeftBracket, TokenKind::BracketTildeRightArrow) => {
                    EdgePatternKind::RightUndirected
                }
                (TokenKind::LeftArrowBracket, TokenKind::BracketRightArrow) => {
                    EdgePatternKind::LeftRight
                }
                (TokenKind::MinusLeftBracket, TokenKind::RightBracketMinus) => EdgePatternKind::Any,
                _ => return None,
            };
            Some(ElementPattern::Edge { kind, filler })
        })
        .spanned()
        .parse_next(input)
}

pub fn abbreviated_edge_pattern(input: &mut TokenStream) -> ModalResult<EdgePatternKind> {
    dispatch! {any;
        TokenKind::LeftArrow => empty.value(EdgePatternKind::Left),
        TokenKind::Tilde => empty.value(EdgePatternKind::Undirected),
        TokenKind::RightArrow => empty.value(EdgePatternKind::Right),
        TokenKind::LeftArrowTilde => empty.value(EdgePatternKind::LeftUndirected),
        TokenKind::TildeRightArrow => empty.value(EdgePatternKind::RightUndirected),
        TokenKind::LeftMinusRight => empty.value(EdgePatternKind::LeftRight),
        TokenKind::Minus => empty.value(EdgePatternKind::Any),
        _ => fail
    }
    .parse_next(input)
}

pub fn element_pattern_filler(
    input: &mut TokenStream,
) -> ModalResult<Spanned<ElementPatternFiller>> {
    seq! {ElementPatternFiller {
        variable: opt(element_variable_declaration),
        label: opt(is_label_expression),
        predicate: opt(element_pattern_predicate),
    }}
    .spanned()
    .parse_next(input)
}

pub fn element_pattern_predicate(
    input: &mut TokenStream,
) -> ModalResult<Spanned<ElementPatternPredicate>> {
    dispatch! {peek(any);
        TokenKind::Where => {
            preceded(TokenKind::Where, search_condition)
                .map(ElementPatternPredicate::Where)
        },
        TokenKind::LeftBrace => {
            element_property_specification.map(ElementPatternPredicate::Property)
        },
        _ => fail
    }
    .spanned()
    .parse_next(input)
}

pub fn element_property_specification(
    input: &mut TokenStream,
) -> ModalResult<VecSpanned<FieldOrProperty>> {
    delimited(
        TokenKind::LeftBrace,
        separated(1.., property_key_value_pair, TokenKind::Comma),
        TokenKind::RightBrace,
    )
    .parse_next(input)
}

pub fn property_key_value_pair(input: &mut TokenStream) -> ModalResult<Spanned<FieldOrProperty>> {
    seq! {FieldOrProperty {
        name: property_name,
        _: TokenKind::Colon,
        value: value_expression,
    }}
    .spanned()
    .parse_next(input)
}

pub fn offset_synonym(input: &mut TokenStream) -> ModalResult<()> {
    one_of((TokenKind::Offset, TokenKind::Skip))
        .void()
        .parse_next(input)
}

pub fn is_label_expression(input: &mut TokenStream) -> ModalResult<Spanned<LabelExpr>> {
    preceded(is_or_colon, label_expression).parse_next(input)
}

def_parser_alias!(
    element_variable_declaration,
    element_variable,
    Spanned<Ident>
);

#[derive(Clone)]
enum InfixLabelOp {
    Conjunction,
    Disjunction,
}

fn label_expression_primary(input: &mut TokenStream) -> ModalResult<Spanned<LabelExpr>> {
    dispatch! {peek(any);
        TokenKind::LeftParen => {
            delimited(TokenKind::LeftParen, label_expression, TokenKind::RightParen).update_span()
        },
        TokenKind::Percent => TokenKind::Percent.value(LabelExpr::Wildcard).spanned(),
        _ => label_name.map_inner(LabelExpr::Label),
    }
    .parse_next(input)
}

fn label_expression_prefix(input: &mut TokenStream) -> ModalResult<(Precedence, Spanned<()>)> {
    TokenKind::Exclamation
        .void()
        .spanned()
        .map(|span| (PREC_NEGATION, span))
        .parse_next(input)
}

fn label_expression_infix(
    input: &mut TokenStream,
) -> ModalResult<(Assoc, Precedence, InfixLabelOp)> {
    dispatch! {any;
        TokenKind::Ampersand => empty.value((Assoc::Left, PREC_CONJUNCTION, InfixLabelOp::Conjunction)),
        TokenKind::VerticalBar => empty.value((Assoc::Left, PREC_DISJUNCTION, InfixLabelOp::Disjunction)),
        _ => fail
    }
    .parse_next(input)
}

pub fn label_expression(input: &mut TokenStream) -> ModalResult<Spanned<LabelExpr>> {
    precedence(
        PREC_INIT,
        label_expression_primary,
        label_expression_prefix,
        fail,
        label_expression_infix,
        |op, a| {
            let span = op.1.start..a.1.end;
            let inner = LabelExpr::Negation(Box::new(a));
            Ok(Spanned(inner, span))
        },
        |_, ()| unreachable!(),
        |a, op, b| {
            let span = a.1.start..b.1.end;
            let inner = match op {
                InfixLabelOp::Conjunction => LabelExpr::Conjunction(Box::new(a), Box::new(b)),
                InfixLabelOp::Disjunction => LabelExpr::Disjunction(Box::new(a), Box::new(b)),
            };
            Ok(Spanned(inner, span))
        },
    )
    .parse_next(input)
}

pub fn graph_pattern_quantifier(
    input: &mut TokenStream,
) -> ModalResult<Spanned<PatternQuantifier>> {
    let mut fixed_or_general = dispatch! {peek((any, any, any));
        (TokenKind::LeftBrace, kind, TokenKind::RightBrace)
            if kind != &TokenKind::Comma =>
        {
            fixed_quantifier
        },
        _ => general_quantifier,
    };
    dispatch! {peek(any);
        TokenKind::Asterisk => {
            TokenKind::Asterisk
                .value(PatternQuantifier::Asterisk)
                .spanned()
        },
        TokenKind::Plus => {
            TokenKind::Plus
                .value(PatternQuantifier::Plus)
                .spanned()
        },
        TokenKind::LeftBrace => fixed_or_general,
        _ => fail
    }
    .parse_next(input)
}

pub fn fixed_quantifier(input: &mut TokenStream) -> ModalResult<Spanned<PatternQuantifier>> {
    delimited(
        TokenKind::LeftBrace,
        unsigned_integer,
        TokenKind::RightBrace,
    )
    .map(PatternQuantifier::Fixed)
    .spanned()
    .parse_next(input)
}

pub fn general_quantifier(input: &mut TokenStream) -> ModalResult<Spanned<PatternQuantifier>> {
    delimited(
        TokenKind::LeftBrace,
        separated_pair(
            opt(unsigned_integer),
            TokenKind::Comma,
            opt(unsigned_integer),
        ),
        TokenKind::RightBrace,
    )
    .map(|(lower_bound, upper_bound)| PatternQuantifier::General {
        lower_bound,
        upper_bound,
    })
    .spanned()
    .parse_next(input)
}

pub fn path_pattern(input: &mut TokenStream) -> ModalResult<Spanned<PathPattern>> {
    seq! {PathPattern {
        variable: opt(path_variable_declaration),
        prefix: opt(path_pattern_prefix),
        expr: path_pattern_expression,
    }}
    .spanned()
    .parse_next(input)
}

pub fn path_pattern_expression(input: &mut TokenStream) -> ModalResult<Spanned<PathPatternExpr>> {
    path_pattern_expression_inner.spanned().parse_next(input)
}

fn path_pattern_expression_inner(input: &mut TokenStream) -> ModalResult<PathPatternExpr> {
    let paths = path_term.parse_next(input)?;
    match opt(peek(any)).parse_next(input)? {
        Some(TokenKind::Alternation) => {
            let paths = repeat(1.., preceded(TokenKind::Alternation, path_term))
                .fold(
                    // FIXME: Remove `.clone()` here once `fold`'s init is changed to FnOnce.
                    // https://github.com/winnow-rs/winnow/issues/513
                    || [paths.clone()].into(),
                    |mut current: VecSpanned<_>, next| {
                        current.push(next);
                        current
                    },
                )
                .parse_next(input)?;
            Ok(PathPatternExpr::Alternation(paths))
        }
        Some(TokenKind::VerticalBar) => {
            let paths = repeat(1.., preceded(TokenKind::VerticalBar, path_term))
                .fold(
                    // FIXME: Same as above.
                    || [paths.clone()].into(),
                    |mut current: VecSpanned<_>, next| {
                        current.push(next);
                        current
                    },
                )
                .parse_next(input)?;
            Ok(PathPatternExpr::Union(paths))
        }
        _ => Ok(paths.0),
    }
}

pub fn path_term(input: &mut TokenStream) -> ModalResult<Spanned<PathPatternExpr>> {
    repeat(1.., path_factor)
        .map(PathPatternExpr::Concat)
        .spanned()
        .parse_next(input)
}

pub fn path_factor(input: &mut TokenStream) -> ModalResult<Spanned<PathPatternExpr>> {
    path_factor_inner.spanned().parse_next(input)
}

fn path_factor_inner(input: &mut TokenStream) -> ModalResult<PathPatternExpr> {
    let path = path_primary.parse_next(input)?;
    match opt(peek(any)).parse_next(input)? {
        Some(TokenKind::QuestionMark) => TokenKind::QuestionMark
            .value(PathPatternExpr::Optional(Box::new(path)))
            .parse_next(input),
        Some(TokenKind::Asterisk | TokenKind::Plus | TokenKind::LeftBrace) => {
            let quantifier = graph_pattern_quantifier(input)?;
            Ok(PathPatternExpr::Quantified {
                path: Box::new(path),
                quantifier,
            })
        }
        _ => Ok(path.0),
    }
}

// TODO: Add simplified path pattern expression.
pub fn path_primary(input: &mut TokenStream) -> ModalResult<Spanned<PathPatternExpr>> {
    dispatch! {peek(any);
        TokenKind::LeftParen => alt((
            parenthesized_path_pattern_expression.map_inner(PathPatternExpr::Grouped),
            element_pattern.map_inner(PathPatternExpr::Pattern),
        )),
        kind if kind.is_prefix_of_edge_pattern() => {
            element_pattern.map_inner(PathPatternExpr::Pattern)
        },
        _ => fail,
    }
    .parse_next(input)
}

pub fn parenthesized_path_pattern_expression(
    input: &mut TokenStream,
) -> ModalResult<Spanned<GroupedPathPattern>> {
    seq! {GroupedPathPattern {
        _: TokenKind::LeftParen,
        variable: opt(subpath_variable_declaration),
        mode: opt(path_mode_prefix),
        expr: path_pattern_expression.map(Box::new),
        where_clause: opt(parenthesized_path_pattern_where_clause),
        _: TokenKind::RightParen,
    }}
    .spanned()
    .parse_next(input)
}

pub fn path_or_paths(input: &mut TokenStream) -> ModalResult<()> {
    one_of((TokenKind::Path, TokenKind::Paths))
        .void()
        .parse_next(input)
}

pub fn path_pattern_prefix(input: &mut TokenStream) -> ModalResult<Spanned<PathPatternPrefix>> {
    dispatch! {peek(any);
        TokenKind::Walk
        | TokenKind::Trail
        | TokenKind::Simple
        | TokenKind::Acyclic => path_mode_prefix.map_inner(PathPatternPrefix::PathMode),
        TokenKind::All
        | TokenKind::Any
        | TokenKind::Shortest => path_search_prefix.map_inner(PathPatternPrefix::PathSearch),
        _ => fail,
    }
    .parse_next(input)
}

pub fn path_mode_prefix(input: &mut TokenStream) -> ModalResult<Spanned<PathMode>> {
    terminated(path_mode, opt(path_or_paths)).parse_next(input)
}

pub fn path_search_prefix(input: &mut TokenStream) -> ModalResult<Spanned<PathSearchMode>> {
    dispatch! {peek((any, opt(any)));
        (TokenKind::All, Some(TokenKind::Shortest))
        | (TokenKind::Any, Some(TokenKind::Shortest))
        | (TokenKind::Shortest, _) => shortest_path_search,
        (TokenKind::All, _) => all_path_search,
        (TokenKind::Any, _) => any_path_search,
        _ => fail,
    }
    .parse_next(input)
}

pub fn all_path_search(input: &mut TokenStream) -> ModalResult<Spanned<PathSearchMode>> {
    delimited(TokenKind::All, opt(path_mode), opt(path_or_paths))
        .map(PathSearchMode::All)
        .spanned()
        .parse_next(input)
}

pub fn any_path_search(input: &mut TokenStream) -> ModalResult<Spanned<PathSearchMode>> {
    seq! {PathSearchMode::Any {
        number: opt(number_of_paths),
        mode: opt(path_mode),
        _: opt(path_or_paths),
    }}
    .spanned()
    .parse_next(input)
}

def_parser_alias!(
    number_of_paths,
    non_negative_integer_specification,
    Spanned<NonNegativeInteger>
);
def_parser_alias!(
    number_of_groups,
    non_negative_integer_specification,
    Spanned<NonNegativeInteger>
);

pub fn shortest_path_search(input: &mut TokenStream) -> ModalResult<Spanned<PathSearchMode>> {
    let mut prefix = preceded(
        TokenKind::Shortest,
        terminated((opt(number_of_groups), opt(path_mode)), opt(path_or_paths)),
    );
    let mut suffix = opt(one_of((TokenKind::Group, TokenKind::Groups)).void());
    let mut path_or_group = dispatch! {(prefix.by_ref(), suffix.by_ref());
        ((number, mode), Some(())) => {
            empty.value(PathSearchMode::CountedShortestGroup { number, mode })
        },
        ((Some(number), mode), None) => {
            empty.value(PathSearchMode::CountedShortest { number, mode })
        },
        _ => fail
    }
    .spanned();
    dispatch! {peek((any, any));
        (TokenKind::All, TokenKind::Shortest) => all_shortest_path_search,
        (TokenKind::Any, TokenKind::Shortest) => any_shortest_path_search,
        (TokenKind::Shortest, _) => path_or_group,
        _ => fail
    }
    .parse_next(input)
}

pub fn all_shortest_path_search(input: &mut TokenStream) -> ModalResult<Spanned<PathSearchMode>> {
    delimited(
        (TokenKind::All, TokenKind::Shortest),
        opt(path_mode),
        opt(path_or_paths),
    )
    .map(PathSearchMode::AllShortest)
    .spanned()
    .parse_next(input)
}

pub fn any_shortest_path_search(input: &mut TokenStream) -> ModalResult<Spanned<PathSearchMode>> {
    delimited(
        (TokenKind::Any, TokenKind::Shortest),
        opt(path_mode),
        opt(path_or_paths),
    )
    .map(PathSearchMode::AnyShortest)
    .spanned()
    .parse_next(input)
}

pub fn path_mode(input: &mut TokenStream) -> ModalResult<Spanned<PathMode>> {
    dispatch! {any;
        TokenKind::Walk => empty.value(PathMode::Walk),
        TokenKind::Trail => empty.value(PathMode::Trail),
        TokenKind::Simple => empty.value(PathMode::Simple),
        TokenKind::Acyclic => empty.value(PathMode::Acyclic),
        _ => fail,
    }
    .spanned()
    .parse_next(input)
}

pub fn graph_pattern_where_clause(input: &mut TokenStream) -> ModalResult<Spanned<Expr>> {
    preceded(TokenKind::Where, search_condition).parse_next(input)
}

def_parser_alias!(
    parenthesized_path_pattern_where_clause,
    graph_pattern_where_clause,
    Spanned<Expr>
);

pub fn path_variable_declaration(input: &mut TokenStream) -> ModalResult<Spanned<Ident>> {
    terminated(path_variable, TokenKind::Equals).parse_next(input)
}

pub fn subpath_variable_declaration(input: &mut TokenStream) -> ModalResult<Spanned<Ident>> {
    terminated(subpath_variable, TokenKind::Equals).parse_next(input)
}

pub fn order_by_clause(input: &mut TokenStream) -> ModalResult<VecSpanned<SortSpec>> {
    preceded(
        (TokenKind::Order, TokenKind::By),
        separated(1.., sort_specification, TokenKind::Comma),
    )
    .parse_next(input)
}

pub fn sort_specification(input: &mut TokenStream) -> ModalResult<Spanned<SortSpec>> {
    seq! {SortSpec {
        key: sort_key,
        ordering: opt(ordering_specification),
        null_ordering: opt(null_ordering),
    }}
    .spanned()
    .parse_next(input)
}

pub fn ordering_specification(input: &mut TokenStream) -> ModalResult<Spanned<Ordering>> {
    dispatch! {any;
        TokenKind::Asc | TokenKind::Ascending => empty.value(Ordering::Asc),
        TokenKind::Desc | TokenKind::Descending => empty.value(Ordering::Desc),
        _ => fail,
    }
    .spanned()
    .parse_next(input)
}

pub fn null_ordering(input: &mut TokenStream) -> ModalResult<Spanned<NullOrdering>> {
    dispatch! {(any, any);
        (TokenKind::Nulls, TokenKind::First) => empty.value(NullOrdering::First),
        (TokenKind::Nulls, TokenKind::Last) => empty.value(NullOrdering::Last),
        _ => fail,
    }
    .spanned()
    .parse_next(input)
}

def_parser_alias!(sort_key, aggregating_value_expression, Spanned<Expr>);

pub fn limit_clause(input: &mut TokenStream) -> ModalResult<Spanned<NonNegativeInteger>> {
    preceded(TokenKind::Limit, non_negative_integer_specification).parse_next(input)
}

pub fn offset_clause(input: &mut TokenStream) -> ModalResult<Spanned<NonNegativeInteger>> {
    preceded(offset_synonym, non_negative_integer_specification).parse_next(input)
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use insta::assert_yaml_snapshot;

    use super::*;
    use crate::parser::utils::parse;

    #[test]
    fn test_graph_pattern() {
        let parsed = parse!(graph_pattern, "(a: Person) -[e:Knows]->+ (b: Person)");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_label_expression() {
        let parsed = parse!(label_expression, "!a | b & (c | %)");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_node_pattern_1() {
        let parsed = parse!(
            node_pattern,
            "(a: Person1 & Person2 {id: 123, name: \"Alice\"})"
        );
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_node_pattern_2() {
        let parsed = parse!(node_pattern, "()");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_edge_pattern_1() {
        let parsed = parse!(edge_pattern, "-[e: Knows {since: 2025}]->");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_edge_pattern_2() {
        let parsed = parse!(edge_pattern, "~[]~");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_edge_pattern_3() {
        let parsed = parse!(edge_pattern, "<->");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_graph_pattern_quantifier_1() {
        let parsed = parse!(graph_pattern_quantifier, "*");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_graph_pattern_quantifier_2() {
        let parsed = parse!(graph_pattern_quantifier, "+");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_graph_pattern_quantifier_3() {
        let parsed = parse!(graph_pattern_quantifier, "{123}");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_graph_pattern_quantifier_4() {
        let parsed = parse!(graph_pattern_quantifier, "{,456}");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_path_search_prefix_1() {
        let parsed = parse!(path_search_prefix, "all shortest");
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn test_path_search_prefix_2() {
        let parsed = parse!(path_search_prefix, "shortest 123 simple paths group");
        assert_yaml_snapshot!(parsed);
    }
}
