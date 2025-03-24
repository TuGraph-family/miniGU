use super::value_expr::boolean_value_expression;
use crate::ast::Expr;
use crate::parser::utils::def_parser_alias;
use crate::span::Spanned;

def_parser_alias!(search_condition, boolean_value_expression, Spanned<Expr>);
