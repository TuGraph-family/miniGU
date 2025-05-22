use std::error::Error;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use smol_str::ToSmolStr;
use gql_parser::ast::{BooleanLiteral, DurationLiteralKind, Expr, GraphExpr, GraphRef, Literal, TemporalLiteralKind, UnsignedIntegerKind, UnsignedNumericLiteral, Value};
use crate::binder::binder::Binder;
use crate::bound_statement::common::BoundFieldOrProperty;
use crate::bound_statement::expr::{BoundDuration, BoundExpr, BoundGraphExpr, BoundListConstructor, BoundLiteral, BoundTemporal, BoundValue};
use crate::catalog_ref::GraphCatalogRef;
use crate::error::{BindError, BindResult};

impl Binder {
    pub(crate) fn resolve_graph_expr(&self, expr: &GraphExpr) -> BindResult<BoundGraphExpr> {
        let resolve_graph = |name: &str| {
            self.schema
                .get_graph(name)
                .map_err(|e| BindError::External(Box::new(e)))?
                .ok_or_else(|| BindError::GraphNotExists(name.to_string()))
        };
        match expr {
            GraphExpr::Name(name) => {
                let graph = resolve_graph(name)?;
                Ok(BoundGraphExpr::Ref(GraphCatalogRef {
                    name:name.clone(),
                    graph_ref: graph,
                }))
            },
            GraphExpr::Ref(graph_expr) => {
                match graph_expr {
                    GraphRef::Name(name) => {
                        let graph = resolve_graph(name)?;
                        Ok(BoundGraphExpr::Ref(GraphCatalogRef {
                            name:name.clone(),
                            graph_ref: graph,
                        }))
                    },
                    GraphRef::Ref(catalog_ref) => {
                        let schema = catalog_ref
                            .schema
                            .as_ref()
                            .map(|s| self.resolve_schema_ref(s.value()))
                            .transpose()?
                            .unwrap_or_else(|| self.schema.clone());
                        if catalog_ref.objects.len() != 1 {
                            return Err(BindError::NotSupported("Only one graph is allowed".to_string()));
                        };
                        let graph_name = catalog_ref.objects[0].value().clone();
                        let graph =
                            schema.get_graph(graph_name.as_str())
                        .map_err(|e| BindError::External(Box::new(e)))?
                        .ok_or_else(|| BindError::GraphNotExists(graph_name.to_string()))?;
                        Ok(BoundGraphExpr::Ref (
                            GraphCatalogRef {
                                name: graph_name.clone(),
                                graph_ref: graph,
                            }
                        ))
                    },
                    GraphRef::Parameter(param) => {
                        return Err(BindError::NotSupported("Parameter".to_string()));
                    }
                    GraphRef::Home => {
                        return Err(BindError::NotSupported("Home".to_string()));
                    }
                }
            },
            GraphExpr::Current => {
                Ok(BoundGraphExpr::Ref(GraphCatalogRef {
                    name: "current".to_smolstr(),
                    graph_ref: self.graph.clone(),
                }))
            }
            GraphExpr::Object(obj) => {
                return Err(BindError::NotSupported("Object expression".to_string()));
            }
        }
    }


    pub(crate) fn resolve_value(&self, value:&Value) -> Result<BoundValue, BindError> {
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
                                    |e| BindError::ErrorCur
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
                                    |e| BindError::ErrorCur
                                )?;
                                Ok(BoundValue::Literal(BoundLiteral::Temporal(BoundTemporal::Date(value))))
                            },
                            TemporalLiteralKind::Time => {
                                let value = NaiveTime::parse_from_str(temporal.literal.value(), "%H:%M:%S%.f")
                                    .map_err(|e| BindError::ErrorCur)?;
                                Ok(BoundValue::Literal(BoundLiteral::Temporal(BoundTemporal::Time(value))))
                            },
                            TemporalLiteralKind::Datetime => {
                                let value = NaiveDateTime::parse_from_str(temporal.literal.value(), "%Y-%m-%d %H:%M:%S%.f")
                                    .map_err(|e| BindError::ErrorCur)?;
                                Ok(BoundValue::Literal(BoundLiteral::Temporal(BoundTemporal::DatTime(value))))
                            },
                            TemporalLiteralKind::Timestamp => {
                                let value = DateTime::parse_from_rfc3339(temporal.literal.value()).map_err(
                                    |e| BindError::ErrorCur
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
                                return Err(BindError::NotSupported("Duration".to_string()));
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

    pub(crate) fn resolve_expr(&self, expr: &Expr) -> Result<BoundExpr, BindError> {
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