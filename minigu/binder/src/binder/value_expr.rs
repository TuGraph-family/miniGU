use gql_parser::ast::{
    BinaryOp as AstBinaryOp, BooleanLiteral, Expr, Literal, StringLiteral, StringLiteralKind,
    UnsignedInteger, UnsignedIntegerKind, UnsignedNumericLiteral, Value,
};
use minigu_common::constants::SESSION_USER;
use minigu_common::data_type::LogicalType;
use minigu_common::value::ScalarValue;
use minigu_ir::bound::{BinaryOp, BoundExpr};

use super::Binder;
use crate::error::{BindError, BindResult, not_implemented};

impl Binder {
    pub fn bind_value_expression(&self, expr: &Expr) -> BindResult<BoundExpr> {
        match expr {
            Expr::Binary { .. } => not_implemented("binary expression", None),
            Expr::Unary { .. } => not_implemented("unary expression", None),
            Expr::DurationBetween { .. } => not_implemented("duration between expression", None),
            Expr::Is { .. } => not_implemented("is expression", None),
            Expr::IsNot { .. } => not_implemented("is not expression", None),
            Expr::Function(_) => not_implemented("function expression", None),
            Expr::Aggregate(_) => not_implemented("aggregate expression", None),
            Expr::Variable(variable) => {
                let expr = self
                    .context
                    .get(variable)
                    .ok_or_else(|| BindError::VariableNotFound(variable.clone()))?;
                Ok(expr.clone())
            }
            Expr::Value(value) => bind_value(value),
            Expr::Path(_) => not_implemented("path expression", None),
            Expr::Property { .. } => not_implemented("property expression", None),
            Expr::Graph(_) => not_implemented("graph expression", None),
        }
    }
}

pub fn bind_binary_op(op: &AstBinaryOp) -> BinaryOp {
    match op {
        AstBinaryOp::Add => BinaryOp::Add,
        AstBinaryOp::Sub => BinaryOp::Sub,
        AstBinaryOp::Mul => BinaryOp::Mul,
        AstBinaryOp::Div => BinaryOp::Div,
        AstBinaryOp::Concat => BinaryOp::Concat,
        AstBinaryOp::Or => BinaryOp::Or,
        AstBinaryOp::Xor => BinaryOp::Xor,
        AstBinaryOp::And => BinaryOp::And,
        AstBinaryOp::Lt => BinaryOp::Lt,
        AstBinaryOp::Le => BinaryOp::Le,
        AstBinaryOp::Gt => BinaryOp::Gt,
        AstBinaryOp::Ge => BinaryOp::Ge,
        AstBinaryOp::Eq => BinaryOp::Eq,
        AstBinaryOp::Ne => BinaryOp::Ne,
    }
}

pub fn bind_value(value: &Value) -> BindResult<BoundExpr> {
    match value {
        Value::SessionUser => Ok(BoundExpr::value(SESSION_USER.into(), LogicalType::String)),
        Value::Parameter(_) => not_implemented("parameter value", None),
        Value::Literal(literal) => bind_literal(literal),
    }
}

pub fn bind_literal(literal: &Literal) -> BindResult<BoundExpr> {
    match literal {
        Literal::Numeric(literal) => bind_numeric_literal(literal),
        Literal::Boolean(literal) => Ok(bind_boolean_literal(literal)),
        Literal::String(literal) => bind_string_literal(literal),
        Literal::Temporal(_) => not_implemented("temporal literal", None),
        Literal::Duration(_) => not_implemented("duration literal", None),
        Literal::List(_) => not_implemented("list literal", None),
        Literal::Record(_) => not_implemented("record literal", None),
        Literal::Null => Ok(BoundExpr::value(ScalarValue::Null, LogicalType::Null)),
    }
}

pub fn bind_numeric_literal(literal: &UnsignedNumericLiteral) -> BindResult<BoundExpr> {
    match literal {
        UnsignedNumericLiteral::Integer(integer) => bind_unsigned_integer(integer.value()),
    }
}

pub fn bind_unsigned_integer(integer: &UnsignedInteger) -> BindResult<BoundExpr> {
    match integer.kind {
        UnsignedIntegerKind::Binary => not_implemented("binary integer literal", None),
        UnsignedIntegerKind::Octal => not_implemented("octal integer literal", None),
        UnsignedIntegerKind::Decimal => {
            // TODO: Support other integer types
            let value = integer.integer.parse::<i32>()?;
            Ok(BoundExpr::value(value.into(), LogicalType::Int32))
        }
        UnsignedIntegerKind::Hex => not_implemented("hex integer literal", None),
    }
}

pub fn bind_boolean_literal(literal: &BooleanLiteral) -> BoundExpr {
    match literal {
        BooleanLiteral::True => BoundExpr::value(true.into(), LogicalType::Boolean),
        BooleanLiteral::False => BoundExpr::value(false.into(), LogicalType::Boolean),
        // TODO: Is it OK to treat `unknown` as `null` here?
        BooleanLiteral::Unknown => {
            BoundExpr::value(ScalarValue::Boolean(None), LogicalType::Boolean)
        }
    }
}

pub fn bind_string_literal(literal: &StringLiteral) -> BindResult<BoundExpr> {
    match literal.kind {
        StringLiteralKind::Char => Ok(BoundExpr::value(
            literal.literal.as_str().into(),
            LogicalType::String,
        )),
        StringLiteralKind::Byte => not_implemented("byte string literal", None),
    }
}
