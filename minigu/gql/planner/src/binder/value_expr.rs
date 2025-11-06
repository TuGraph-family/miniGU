use std::str::FromStr;

use gql_parser::ast::{
    BinaryOp, BooleanLiteral, Expr, Function, Literal, NonNegativeInteger, StringLiteral,
    StringLiteralKind, UnaryOp, UnsignedInteger, UnsignedIntegerKind, UnsignedNumericLiteral,
    Value, VectorDistance, VectorLiteral,
};
use minigu_common::constants::SESSION_USER;
use minigu_common::data_type::LogicalType;
use minigu_common::error::not_implemented;
use minigu_common::types::VectorMetric;
use minigu_common::value::{F32, F64, ScalarValue, VectorValue};

use super::Binder;
use super::error::{BindError, BindResult};
use crate::bound::{BoundBinaryOp, BoundExpr, BoundUnsignedInteger};

impl Binder<'_> {
    pub fn bind_value_expression(&self, expr: &Expr) -> BindResult<BoundExpr> {
        match expr {
            Expr::Binary { .. } => not_implemented("binary expression", None),
            Expr::Unary { .. } => not_implemented("unary expression", None),
            Expr::DurationBetween { .. } => not_implemented("duration between expression", None),
            Expr::Is { .. } => not_implemented("is expression", None),
            Expr::IsNot { .. } => not_implemented("is not expression", None),
            Expr::Function(function) => self.bind_function_expression(function),
            Expr::Aggregate(_) => not_implemented("aggregate expression", None),
            Expr::Variable(variable) => {
                let field = self
                    .active_data_schema
                    .as_ref()
                    .ok_or_else(|| BindError::VariableNotFound(variable.clone()))?
                    .get_field_by_name(variable)
                    .ok_or_else(|| BindError::VariableNotFound(variable.clone()))?;
                Ok(BoundExpr::variable(
                    variable.to_string(),
                    field.ty().clone(),
                    field.is_nullable(),
                ))
            }
            Expr::Value(value) => bind_value(value),
            Expr::Path(_) => not_implemented("path expression", None),
            Expr::Property { .. } => not_implemented("property expression", None),
            Expr::Graph(_) => not_implemented("graph expression", None),
        }
    }

    fn bind_function_expression(&self, function: &Function) -> BindResult<BoundExpr> {
        match function {
            Function::Vector(vector) => self.bind_vector_distance(vector),
            Function::Generic(_) => not_implemented("generic function expression", None),
            Function::Numeric(_) => not_implemented("numeric function expression", None),
            Function::Case(_) => not_implemented("case function expression", None),
        }
    }

    fn bind_vector_distance(&self, function: &VectorDistance) -> BindResult<BoundExpr> {
        let lhs = self.bind_value_expression(function.lhs.as_ref().value())?;
        let rhs = self.bind_value_expression(function.rhs.as_ref().value())?;

        let lhs_dim = match &lhs.logical_type {
            LogicalType::Vector(dim) => *dim,
            ty => {
                return Err(BindError::InvalidVectorDistanceArgument {
                    position: 1,
                    ty: ty.clone(),
                });
            }
        };
        let rhs_dim = match &rhs.logical_type {
            LogicalType::Vector(dim) => *dim,
            ty => {
                return Err(BindError::InvalidVectorDistanceArgument {
                    position: 2,
                    ty: ty.clone(),
                });
            }
        };
        if lhs_dim != rhs_dim {
            return Err(BindError::VectorDistanceDimensionMismatch {
                left: lhs_dim,
                right: rhs_dim,
            });
        }
        let metric = if let Some(metric) = &function.metric {
            VectorMetric::from_str(metric.value().as_str())?
        } else {
            VectorMetric::L2
        };

        Ok(BoundExpr::vector_distance(lhs, rhs, metric, lhs_dim))
    }

    pub fn bind_non_negative_integer(
        &self,
        integer: &NonNegativeInteger,
    ) -> BindResult<BoundUnsignedInteger> {
        match integer {
            NonNegativeInteger::Integer(unsigned) => bind_unsigned_integer(unsigned),
            NonNegativeInteger::Parameter(_) => {
                not_implemented("parameterized non-negative integer", None)
            }
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
        Value::SessionUser => Ok(BoundExpr::value(
            SESSION_USER.into(),
            LogicalType::String,
            false,
        )),
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
        Literal::Vector(literal) => bind_vector_literal(literal),
        Literal::Null => Ok(BoundExpr::value(ScalarValue::Null, LogicalType::Null, true)),
    }
}

pub fn bind_numeric_literal(literal: &UnsignedNumericLiteral) -> BindResult<BoundExpr> {
    match literal {
        UnsignedNumericLiteral::Integer(integer) => {
            let unsigned = bind_unsigned_integer(integer.value())?;
            let expr = match unsigned {
                BoundUnsignedInteger::Int8(value) => {
                    BoundExpr::value(value.into(), LogicalType::Int8, false)
                }
                BoundUnsignedInteger::Int16(value) => {
                    BoundExpr::value(value.into(), LogicalType::Int16, false)
                }
                BoundUnsignedInteger::Int32(value) => {
                    BoundExpr::value(value.into(), LogicalType::Int32, false)
                }
                BoundUnsignedInteger::Int64(value) => {
                    BoundExpr::value(value.into(), LogicalType::Int64, false)
                }
            };
            Ok(expr)
        }
        UnsignedNumericLiteral::Float(float) => {
            let literal = float.value().float.as_str();
            let parsed = literal
                .parse::<f64>()
                .map_err(|_| BindError::InvalidFloatLiteral(literal.to_string()))?;
            Ok(BoundExpr::value(
                ScalarValue::Float64(Some(F64::from(parsed))),
                LogicalType::Float64,
                false,
            ))
        }
    }
}

fn bind_vector_literal(literal: &VectorLiteral) -> BindResult<BoundExpr> {
    let dimension = literal.elems.len();

    // Validate that vector is not empty
    if dimension == 0 {
        return Err(BindError::InvalidVectorLiteral(
            "vector literal must contain at least one element".into(),
        ));
    }

    let mut data = Vec::with_capacity(dimension);
    for elem in &literal.elems {
        let value = bind_vector_element(elem.value())?;
        data.push(F32::from(value));
    }

    let vector = VectorValue::new(data, dimension).map_err(BindError::InvalidVectorLiteral)?;

    Ok(BoundExpr::value(
        ScalarValue::new_vector(dimension, Some(vector)),
        LogicalType::Vector(dimension),
        false,
    ))
}

fn bind_vector_element(expr: &Expr) -> BindResult<f32> {
    match expr {
        Expr::Unary { op, child } => {
            let factor = match op.value() {
                UnaryOp::Plus => 1.0f32,
                UnaryOp::Minus => -1.0f32,
                UnaryOp::Not => {
                    return Err(BindError::InvalidVectorElement(
                        "logical not is not allowed in vector literals".into(),
                    ));
                }
            };
            let inner = bind_vector_element(child.value())?;
            Ok(factor * inner)
        }
        Expr::Value(Value::Literal(Literal::Numeric(numeric))) => {
            let scalar = bind_numeric_literal(numeric)?
                .evaluate_scalar()
                .expect("numeric literal should evaluate to scalar");
            scalar
                .to_f32()
                .map_err(|err| BindError::InvalidVectorElement(format!("{err:?}")))
        }
        _ => Err(BindError::InvalidVectorElement(
            "vector elements must be numeric literals".into(),
        )),
    }
}

pub fn bind_unsigned_integer(integer: &UnsignedInteger) -> BindResult<BoundUnsignedInteger> {
    match integer.kind {
        UnsignedIntegerKind::Binary => not_implemented("binary integer literal", None),
        UnsignedIntegerKind::Octal => not_implemented("octal integer literal", None),
        UnsignedIntegerKind::Decimal => {
            if let Ok(value) = integer.integer.parse::<i8>() {
                Ok(BoundUnsignedInteger::Int8(value))
            } else if let Ok(value) = integer.integer.parse::<i16>() {
                Ok(BoundUnsignedInteger::Int16(value))
            } else if let Ok(value) = integer.integer.parse::<i32>() {
                Ok(BoundUnsignedInteger::Int32(value))
            } else if let Ok(value) = integer.integer.parse::<i64>() {
                Ok(BoundUnsignedInteger::Int64(value))
            } else {
                Err(BindError::InvalidInteger(integer.integer.clone()))
            }
        }
        UnsignedIntegerKind::Hex => not_implemented("hex integer literal", None),
    }
}

pub fn bind_boolean_literal(literal: &BooleanLiteral) -> BoundExpr {
    match literal {
        BooleanLiteral::True => BoundExpr::value(true.into(), LogicalType::Boolean, false),
        BooleanLiteral::False => BoundExpr::value(false.into(), LogicalType::Boolean, false),
        // TODO: Is it OK to treat `unknown` as `null` here?
        BooleanLiteral::Unknown => {
            BoundExpr::value(ScalarValue::Boolean(None), LogicalType::Boolean, true)
        }
    }
}

pub fn bind_string_literal(literal: &StringLiteral) -> BindResult<BoundExpr> {
    match literal.kind {
        StringLiteralKind::Char => Ok(BoundExpr::value(
            literal.literal.as_str().into(),
            LogicalType::String,
            false,
        )),
        StringLiteralKind::Byte => not_implemented("byte string literal", None),
    }
}
