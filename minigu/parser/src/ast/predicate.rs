//! AST definitions for *predicates*.

use super::{Expr, Ident, LabelExpr, MatchStatement, Procedure, ValueType};
use crate::macros::base;
use crate::span::{BoxSpanned, Spanned, VecSpanned};

#[apply(base)]
pub enum Predicate {
    Exists(Exists),
    Null {
        expr: BoxSpanned<Expr>,
        not: bool,
    },
    Typed {
        expr: BoxSpanned<Expr>,
        value_type: BoxSpanned<ValueType>,
        not: bool,
    },
    Directed {
        edge: Spanned<Ident>,
        not: bool,
    },
    Labeled {
        element: Spanned<Ident>,
        label: Spanned<LabelExpr>,
    },
    SrcOf {
        node: Spanned<Ident>,
        edge: Spanned<Ident>,
        not: bool,
    },
    DstOf {
        node: Spanned<Ident>,
        edge: Spanned<Ident>,
        not: bool,
    },
    AllDifferent(VecSpanned<Ident>),
    Same(VecSpanned<Ident>),
    PropertyExists {
        element: Spanned<Ident>,
        property: Spanned<Ident>,
    },
}

#[apply(base)]
pub enum Exists {
    Pattern,
    Match(VecSpanned<MatchStatement>),
    Nested(BoxSpanned<Procedure>),
}
