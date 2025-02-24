//! This module contains the definition of [`Span`].

use core::num::TryFromIntError;
use core::ops::Range;

/// Span within a GQL string.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    pub const fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    pub fn join(self, other: Self) -> Self {
        let Self { start, end } = self;
        let Self {
            start: other_start,
            end: other_end,
        } = other;
        Self {
            start: start.min(other_start),
            end: end.max(other_end),
        }
    }
}

impl From<Range<u32>> for Span {
    fn from(Range { start, end }: Range<u32>) -> Self {
        Self { start, end }
    }
}

impl TryFrom<Range<usize>> for Span {
    type Error = TryFromIntError;

    fn try_from(value: Range<usize>) -> Result<Self, Self::Error> {
        let start = value.start.try_into()?;
        let end = value.end.try_into()?;
        Ok(Self { start, end })
    }
}

impl From<Span> for Range<u32> {
    fn from(Span { start, end }: Span) -> Self {
        Self { start, end }
    }
}
