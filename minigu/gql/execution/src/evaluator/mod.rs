pub mod binary;
pub mod column_ref;
pub mod constant;
pub mod datum;
pub mod scalar_function;
pub mod unary;

use std::fmt::Debug;

use binary::{Binary, BinaryOp};
use datum::DatumRef;
use minigu_common::data_chunk::DataChunk;
use unary::{Unary, UnaryOp};

use crate::error::ExecutionResult;

pub type BoxedEvaluator = Box<dyn Evaluator>;

pub trait Evaluator: Debug {
    fn evaluate(&self, chunk: &DataChunk) -> ExecutionResult<DatumRef>;

    fn add<E>(self, other: E) -> Binary<Self, E>
    where
        Self: Sized,
        E: Evaluator,
    {
        Binary::new(BinaryOp::Add, self, other)
    }

    fn sub<E>(self, other: E) -> Binary<Self, E>
    where
        Self: Sized,
        E: Evaluator,
    {
        Binary::new(BinaryOp::Sub, self, other)
    }

    fn mul<E>(self, other: E) -> Binary<Self, E>
    where
        Self: Sized,
        E: Evaluator,
    {
        Binary::new(BinaryOp::Mul, self, other)
    }

    fn div<E>(self, other: E) -> Binary<Self, E>
    where
        Self: Sized,
        E: Evaluator,
    {
        Binary::new(BinaryOp::Div, self, other)
    }

    fn rem<E>(self, other: E) -> Binary<Self, E>
    where
        Self: Sized,
        E: Evaluator,
    {
        Binary::new(BinaryOp::Rem, self, other)
    }

    fn neg(self) -> Unary<Self>
    where
        Self: Sized,
    {
        Unary::new(UnaryOp::Neg, self)
    }

    fn not(self) -> Unary<Self>
    where
        Self: Sized,
    {
        Unary::new(UnaryOp::Not, self)
    }

    fn and<E>(self, other: E) -> Binary<Self, E>
    where
        Self: Sized,
        E: Evaluator,
    {
        Binary::new(BinaryOp::And, self, other)
    }

    fn or<E>(self, other: E) -> Binary<Self, E>
    where
        Self: Sized,
        E: Evaluator,
    {
        Binary::new(BinaryOp::Or, self, other)
    }

    fn eq<E>(self, other: E) -> Binary<Self, E>
    where
        Self: Sized,
        E: Evaluator,
    {
        Binary::new(BinaryOp::Eq, self, other)
    }

    fn ne<E>(self, other: E) -> Binary<Self, E>
    where
        Self: Sized,
        E: Evaluator,
    {
        Binary::new(BinaryOp::Ne, self, other)
    }

    fn gt<E>(self, other: E) -> Binary<Self, E>
    where
        Self: Sized,
        E: Evaluator,
    {
        Binary::new(BinaryOp::Gt, self, other)
    }

    fn ge<E>(self, other: E) -> Binary<Self, E>
    where
        Self: Sized,
        E: Evaluator,
    {
        Binary::new(BinaryOp::Ge, self, other)
    }

    fn lt<E>(self, other: E) -> Binary<Self, E>
    where
        Self: Sized,
        E: Evaluator,
    {
        Binary::new(BinaryOp::Lt, self, other)
    }

    fn le<E>(self, other: E) -> Binary<Self, E>
    where
        Self: Sized,
        E: Evaluator,
    {
        Binary::new(BinaryOp::Le, self, other)
    }
}

impl<E> Evaluator for Box<E>
where
    E: Evaluator + ?Sized,
{
    fn evaluate(&self, chunk: &DataChunk) -> ExecutionResult<DatumRef> {
        (**self).evaluate(chunk)
    }
}
