//! Contains the definition of [`Spanned`], an AST wrapper providing span information.

use core::ops::Range;

use crate::imports::{Box, Vec};
use crate::macros::base;

/// A wrapper around a value that contains its span information.
///
/// This is typically used to construct accurate and user-friendly diagnostics during semantic
/// analysis.
#[apply(base)]
pub struct Spanned<T>(pub T, pub Range<usize>);

impl<T> Spanned<T> {
    /// Returns the span of the value.
    #[inline(always)]
    pub fn span(&self) -> Range<usize> {
        self.1.clone()
    }

    /// Returns the inner value.
    #[inline(always)]
    pub fn value(&self) -> &T {
        &self.0
    }

    /// Takes a closure and applies it to the inner value while preserving the span.
    #[inline(always)]
    pub fn map<F, O>(self, f: F) -> Spanned<O>
    where
        F: FnOnce(T) -> O,
    {
        let Spanned(value, span) = self;
        Spanned(f(value), span)
    }
}

/// Type alias for vectors of spanned values.
pub type VecSpanned<T> = Vec<Spanned<T>>;

/// Type alias for boxed spanned values.
pub type BoxSpanned<T> = Box<Spanned<T>>;

/// Type alias for optional spanned values.
pub type OptSpanned<T> = Option<Spanned<T>>;
