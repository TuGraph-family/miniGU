use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use gql_parser::ast::{AggregateFunction, BinaryOp, BooleanLiteral, DurationLiteralKind, Expr, Function, Ident, Literal, TemporalLiteralKind, UnaryOp, UnsignedIntegerKind, UnsignedNumericLiteral, Value};
use crate::error::Error;
use crate::program::Binder;
use crate::program::bound_statement::common::BoundFieldOrProperty;
use crate::program::bound_statement::expr::{BoundDuration, BoundExpr, BoundGraphExpr, BoundListConstructor, BoundLiteral, BoundTemporal, BoundValue};

impl Binder {

    pub(crate) fn resolve_value(&self, value:&Value) -> Result<BoundValue, Error> {
        match value {
            Value::SessionUser => Ok(BoundValue::SessionUser),
            Value::Parameter(param) => Ok(BoundValue::Parameter(param.clone())),
            Value::Literal(literal) => {
                match literal {
                    Literal::Numeric(numeric) => {
                        match numeric {
                            UnsignedNumericLiteral::Integer(numeric) => {
                                let radix = match numeric.value().kind {
                                UnsignedIntegerKind::Binary => 2,
                                UnsignedIntegerKind::Octal => 8,
                                UnsignedIntegerKind::Decimal => 10,
                                UnsignedIntegerKind::Hex => 16
                            };
                            let value = i64::from_str_radix(numeric.value().integer.as_str(), radix).map_err(
                                |e| Error::ErrorCur
                            )?;
                                Ok(BoundValue::Literal(BoundLiteral::Numeric(value)))
                            }
                        }
                    },
                    Literal::Boolean(boolean) => {
                        match boolean {
                            BooleanLiteral::True => Ok(BoundValue::Literal(BoundLiteral::Boolean(Some(true)))),
                            BooleanLiteral::False => Ok(BoundValue::Literal(BoundLiteral::Boolean(Some(false)))),
                            BooleanLiteral::Unknown => Ok(BoundValue::Literal(BoundLiteral::Boolean(None))),
                        }
                    }
                    Literal::String(string) => {
                        Ok(BoundValue::Literal(BoundLiteral::String(string.clone())))
                    }
                    Literal::Temporal(temporal) => {
                        match temporal.kind {
                            TemporalLiteralKind::Date => {
                                let value = NaiveDate::parse_from_str(temporal.literal.value(), "%Y-%m-%d").map_err(
                                    |e| Error::ErrorCur
                                )?;
                                Ok(BoundValue::Literal(BoundLiteral::Temporal(BoundTemporal::Date(value))))
                            },
                            TemporalLiteralKind::Time => {
                                let value = NaiveTime::parse_from_str(temporal.literal.value(), "%H:%M:%S%.f")
                                    .map_err(|e| Error::ErrorCur)?;
                                Ok(BoundValue::Literal(BoundLiteral::Temporal(BoundTemporal::Time(value))))
                            },
                            TemporalLiteralKind::Datetime => {
                                let value = NaiveDateTime::parse_from_str(temporal.literal.value(), "%Y-%m-%d %H:%M:%S%.f")
                                    .map_err(|e| Error::ErrorCur)?;
                                Ok(BoundValue::Literal(BoundLiteral::Temporal(BoundTemporal::DatTime(value))))
                            },
                            TemporalLiteralKind::Timestamp => {
                                let value = DateTime::parse_from_rfc3339(temporal.literal.value()).map_err(
                                    |e| Error::ErrorCur
                                );
                                Ok(BoundValue::Literal(BoundLiteral::Temporal(BoundTemporal::Timestamp(value?.to_utc()))))
                            },
                            TemporalLiteralKind::SqlDatetime => {
                                Ok(BoundValue::Literal(BoundLiteral::Temporal(BoundTemporal::SqlDatetime(temporal.literal.value().to_string()))))
                            }
                        }
                    },
                    Literal::Duration(duration) =>  {
                        match duration.kind {
                            DurationLiteralKind::Duration => {
                                // TODO
                                return Err(Error::NotSupported("Duration".to_string()));
                            },
                            DurationLiteralKind::SqlInterval => {
                                Ok(BoundValue::Literal(BoundLiteral::Duration(BoundDuration::SqlInterval(duration.literal.value().to_string()))))
                            }
                        }
                    },
                    Literal::List(list) => {
                        let type_name =  list.type_name.as_ref().map(|spanned| spanned.value().clone());
                        let mut expr_vec = Vec::new();
                        for expr in &list.values {
                            expr_vec.push(self.resolve_expr(expr.value())?)
                        }
                        Ok(BoundValue::Literal(BoundLiteral::List(BoundListConstructor {
                            type_name: type_name,
                            values: expr_vec,
                        })))
                    },
                    Literal::Record(record) => {
                        let mut field_vec = Vec::new();
                        for item in record {
                            field_vec.push(
                                BoundFieldOrProperty {
                                    id: self.resolve_field_id(item.value().name.value())?,
                                    value: self.resolve_expr(item.value().value.value())?
                                }
                            )
                        }
                        Ok(BoundValue::Literal(BoundLiteral::Record(field_vec)))
                    }
                    Literal::Null => Ok(BoundValue::Literal(BoundLiteral::Null)),
                }
            }
        }
    }
    
    pub(crate) fn resolve_expr(&self, expr: &Expr) -> Result<BoundExpr, Error> {
        match expr {
            Expr::Binary { op, left, right } => {
                Ok(BoundExpr::Binary {
                    op: op.value().clone(),
                    left: Box::new(self.resolve_expr(left.value())?),
                    right: Box::new(self.resolve_expr(right.value())?)
                })
            },
            Expr::Unary {op, child} => {
                Ok(BoundExpr::Unary {
                    op: op.value().clone(),
                    child: Box::new(self.resolve_expr(child.value())?),
                })
            },
            Expr::DurationBetween { left, right } => {
                Ok(BoundExpr::DurationBetween {
                    left: Box::new(self.resolve_expr(left.value())?),
                    right: Box::new(self.resolve_expr(right.value())?)
                })
            },
            Expr::Is { left, right } => {
                Ok(BoundExpr::Is {
                    left: Box::new(self.resolve_expr(left.value())?),
                    right: Box::new(right.value().clone())
                })
            }
            
            Expr::IsNot { left, right } => {
                Ok(BoundExpr::IsNot {
                    left: Box::new(self.resolve_expr(left.value())?),
                    right:Box::new(right.value().clone())
                })
            }
            
            Expr::Function(function) => {
                // TODO(Colin): Support bind Function.
                Ok(BoundExpr::Function(function.clone()))
            }
            
            Expr::Aggregate(aggregate) => {
                // TODO(Colin): Support bind Agg Function.
                Ok(BoundExpr::Aggregate(aggregate.clone()))
            }
            
            Expr::Variable(ident) => {
                Ok(BoundExpr::Variable(ident.clone()))
            }
            
            Expr::Value(value) => {
                // TODO(Colin): Support bind Value
                Ok(BoundExpr::Value(self.resolve_value(value)?))
            }
            
            Expr::Path(path) => {
                // TODO(Colin): Support bind Path
                Ok(BoundExpr::Path)
            }
            
            Expr::Property { source, trailing_names } => Ok(BoundExpr::Property {
                source: Box::new(self.resolve_expr(source.value())?),
                trailing_names: trailing_names.into_iter().map(|value| value.value().clone()).collect(),
            }),
            Expr::Graph(graph) => {
                Ok(BoundExpr::Graph(
                    Box::new(self.resolve_graph_expr(graph)?)
                ))
            }
        }
    }
}