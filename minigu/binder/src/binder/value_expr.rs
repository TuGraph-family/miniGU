use gql_parser::ast::{
    BinaryOp, BooleanLiteral, Expr, Literal, StringLiteral, StringLiteralKind, UnsignedInteger,
    UnsignedIntegerKind, UnsignedNumericLiteral, Value,
};
use minigu_common::constants::SESSION_USER;
use minigu_common::data_type::LogicalType;
use minigu_common::error::not_implemented;
use minigu_common::value::ScalarValue;
use minigu_ir::bound::{BoundBinaryOp, BoundExpr};

use super::Binder;
use crate::error::{BindError, BindResult};

impl Binder<'_> {
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
                let mut expr = self
                    .context
                    .get(variable)
                    .ok_or_else(|| BindError::VariableNotFound(variable.clone()))?
                    .clone();
                expr.name = variable.to_string();
                Ok(expr)
            }
            Expr::Value(value) => bind_value(value),
            Expr::Path(_) => not_implemented("path expression", None),
            Expr::Property { .. } => not_implemented("property expression", None),
            Expr::Graph(_) => not_implemented("graph expression", None),
        }
    }
}

pub fn bind_binary_op(op: &BinaryOp) -> BoundBinaryOp {
    match op {
        BinaryOp::Add => BoundBinaryOp::Add,
        BinaryOp::Sub => BoundBinaryOp::Sub,
        BinaryOp::Mul => BoundBinaryOp::Mul,
        BinaryOp::Div => BoundBinaryOp::Div,
        BinaryOp::Concat => BoundBinaryOp::Concat,
        BinaryOp::Or => BoundBinaryOp::Or,
        BinaryOp::Xor => BoundBinaryOp::Xor,
        BinaryOp::And => BoundBinaryOp::And,
        BinaryOp::Lt => BoundBinaryOp::Lt,
        BinaryOp::Le => BoundBinaryOp::Le,
        BinaryOp::Gt => BoundBinaryOp::Gt,
        BinaryOp::Ge => BoundBinaryOp::Ge,
        BinaryOp::Eq => BoundBinaryOp::Eq,
        BinaryOp::Ne => BoundBinaryOp::Ne,
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
