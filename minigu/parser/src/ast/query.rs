use super::{
    CallProcedureStatement, Expr, GraphExpr, GraphPatternBindingTable, Ident, NonNegativeInteger,
    Procedure, SetQuantifier,
};
use crate::macros::base;
use crate::span::{BoxSpanned, OptSpanned, Spanned, VecSpanned};

#[apply(base)]
pub enum CompositeQueryStatement {
    Conjunction {
        conjunction: Spanned<QueryConjunction>,
        left: BoxSpanned<CompositeQueryStatement>,
        right: BoxSpanned<CompositeQueryStatement>,
    },
    Primary(LinearQueryStatement),
}

#[apply(base)]
pub enum LinearQueryStatement {
    Focused(FocusedLinearQueryStatement),
    Ambient(AmbientLinearQueryStatement),
}

#[apply(base)]
pub enum FocusedLinearQueryStatement {
    Parts {
        parts: VecSpanned<FocusedLinearQueryStatementPart>,
        result: Spanned<ResultStatement>,
    },
    Result {
        use_graph: Spanned<GraphExpr>,
        result: Spanned<ResultStatement>,
    },
    Nested {
        use_graph: Spanned<GraphExpr>,
        query: BoxSpanned<Procedure>,
    },
    Select {},
}

#[apply(base)]
pub enum AmbientLinearQueryStatement {
    Parts {
        parts: VecSpanned<SimpleQueryStatement>,
        result: Spanned<ResultStatement>,
    },
    Nested(BoxSpanned<Procedure>),
}

#[apply(base)]
pub struct FocusedLinearQueryStatementPart {
    pub use_graph: Spanned<GraphExpr>,
    pub statements: VecSpanned<SimpleQueryStatement>,
}

#[apply(base)]
pub enum SimpleQueryStatement {
    Match(MatchStatement),
    // Let,
    // For,
    // Filter,
    Call(CallProcedureStatement),
}

#[apply(base)]
pub enum ResultStatement {
    Return {
        statement: BoxSpanned<ReturnStatement>,
        order_by: Option<BoxSpanned<OrderByAndPageStatement>>,
    },
    Finish,
}

#[apply(base)]
pub struct ReturnStatement {
    pub quantifier: OptSpanned<SetQuantifier>,
    pub items: Spanned<Return>,
    pub group_by: OptSpanned<GroupBy>,
}

#[apply(base)]
pub enum Return {
    Items(VecSpanned<ReturnItem>),
    All,
}

#[apply(base)]
pub struct ReturnItem {
    pub value: Spanned<Expr>,
    pub alias: OptSpanned<Ident>,
}

#[apply(base)]
pub enum QueryConjunction {
    SetOp(SetOp),
    Otherwise,
}

#[apply(base)]
pub enum SetOpKind {
    Union,
    Except,
    Intersect,
}

#[apply(base)]
pub struct SetOp {
    pub kind: Spanned<SetOpKind>,
    pub quantifier: OptSpanned<SetQuantifier>,
}

#[apply(base)]
pub enum MatchStatement {
    Simple(Spanned<GraphPatternBindingTable>),
    Optional(VecSpanned<MatchStatement>),
}

#[apply(base)]
pub struct OrderByAndPageStatement {
    pub order_by: VecSpanned<SortSpec>,
    pub offset: OptSpanned<NonNegativeInteger>,
    pub limit: OptSpanned<NonNegativeInteger>,
}

#[apply(base)]
pub struct SortSpec {
    pub key: Spanned<Expr>,
    pub ordering: OptSpanned<Ordering>,
    pub null_ordering: OptSpanned<NullOrdering>,
}

#[apply(base)]
pub enum Ordering {
    Asc,
    Desc,
}

#[apply(base)]
pub enum NullOrdering {
    First,
    Last,
}

pub type GroupBy = VecSpanned<Ident>;
