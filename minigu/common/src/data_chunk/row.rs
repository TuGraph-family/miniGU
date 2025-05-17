use std::iter::Enumerate;
use std::ops::Range;

use arrow::array::{ArrayIter, BooleanArray};
use serde::{Deserialize, Serialize};

use super::DataChunk;
use crate::value::{ScalarValue, ScalarValueAccessor};

#[derive(Debug)]
pub struct Rows<'a> {
    pub(super) chunk: &'a DataChunk,
    pub(super) iter: RowIndexIter<'a>,
}

#[derive(Debug)]
pub(super) enum RowIndexIter<'a> {
    Filtered(Enumerate<ArrayIter<&'a BooleanArray>>),
    Unfiltered(Range<usize>),
}

impl Iterator for RowIndexIter<'_> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            RowIndexIter::Filtered(iter) => iter
                .by_ref()
                .filter(|(_, v)| v.unwrap_or_default())
                .map(|(i, _)| i)
                .next(),
            RowIndexIter::Unfiltered(iter) => iter.next(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RowRef<'a> {
    chunk: &'a DataChunk,
    row_index: usize,
}

impl RowRef<'_> {
    #[inline]
    pub fn row_index(&self) -> usize {
        self.row_index
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<ScalarValue> {
        let column = self.chunk.columns.get(index)?;
        Some(column.index(self.row_index))
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.chunk.columns.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.chunk.columns.is_empty()
    }

    #[inline]
    pub fn into_owned(self) -> OwnedRow {
        OwnedRow(self.into_iter().collect())
    }
}

impl IntoIterator for RowRef<'_> {
    type Item = ScalarValue;

    type IntoIter = impl Iterator<Item = Self::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.chunk
            .columns
            .iter()
            .map(move |c| c.index(self.row_index))
    }
}

impl<'a> Iterator for Rows<'a> {
    type Item = RowRef<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let row_index = self.iter.next()?;
        Some(RowRef {
            chunk: self.chunk,
            row_index,
        })
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OwnedRow(Vec<ScalarValue>);

impl OwnedRow {
    #[inline]
    pub fn new(values: Vec<ScalarValue>) -> Self {
        Self(values)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<&ScalarValue> {
        self.0.get(index)
    }

    #[inline]
    pub fn into_inner(self) -> Vec<ScalarValue> {
        self.0
    }
}

impl<'a> From<RowRef<'a>> for OwnedRow {
    #[inline]
    fn from(value: RowRef<'a>) -> Self {
        value.into_owned()
    }
}
