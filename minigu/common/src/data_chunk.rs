use std::fmt::{self, Display};
use std::iter::Enumerate;
use std::ops::Range;

use arrow::array::{Array, ArrayIter, ArrayRef, AsArray, BooleanArray};
use arrow::compute;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::value::{IndexScalarValue, ScalarValue};

#[derive(Debug, Clone, PartialEq)]
pub struct DataChunk {
    columns: Vec<ArrayRef>,
    filter: Option<BooleanArray>,
}

impl DataChunk {
    #[inline]
    pub fn new(columns: Vec<ArrayRef>) -> Self {
        assert!(!columns.is_empty(), "columns must not be empty");
        assert!(
            columns.iter().map(|c| c.len()).all_equal(),
            "all columns must have the same length"
        );
        Self {
            columns,
            filter: None,
        }
    }

    #[inline]
    pub fn with_filter(self, filter: BooleanArray) -> Self {
        assert_eq!(
            self.len(),
            filter.len(),
            "filter must have the same length as the data chunk"
        );
        Self {
            filter: Some(filter),
            ..self
        }
    }

    #[inline]
    pub fn unfiltered(self) -> Self {
        Self {
            filter: None,
            ..self
        }
    }

    #[inline]
    pub fn cardinality(&self) -> usize {
        if let Some(filter) = &self.filter {
            filter.true_count()
        } else {
            self.len()
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.columns[0].len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn columns(&self) -> &[ArrayRef] {
        &self.columns
    }

    /// Compacts the data chunk by eagerly applying the filter to each column.
    ///
    /// # Panics
    ///
    /// Panics if the filter is not applied successfully.
    #[inline]
    pub fn compact(&mut self) {
        if let Some(filter) = self.filter.take() {
            self.columns = self
                .columns
                .iter()
                .map(|column| {
                    compute::kernels::filter::filter(column, &filter)
                        .expect("filter should be applied successfully")
                })
                .collect();
        }
    }

    /// Returns a zero-copy slice of this chunk with the indicated offset and length.
    ///
    /// # Panics
    ///
    /// Panics if the offset and length are out of bounds.
    pub fn slice(&self, offset: usize, length: usize) -> Self {
        let columns = self
            .columns
            .iter()
            .map(|c| c.slice(offset, length))
            .collect();
        let filter = self.filter.as_ref().map(|f| f.slice(offset, length));
        Self { columns, filter }
    }

    #[inline]
    pub fn filter(&self) -> Option<&BooleanArray> {
        self.filter.as_ref()
    }

    #[inline]
    pub fn rows(&self) -> Rows<'_> {
        let iter = if let Some(filter) = self.filter() {
            RowIndexIter::Filtered(filter.iter().enumerate())
        } else {
            RowIndexIter::Unfiltered(0..self.len())
        };
        Rows { chunk: self, iter }
    }

    /// Extends the data chunk horizontally, i.e., appends columns to the right.
    ///
    /// # Panics
    ///
    /// Panics if the lengths of the new columns are not equal to the length of the data
    /// chunk.
    #[inline]
    pub fn append_columns<I>(&mut self, columns: I)
    where
        I: IntoIterator<Item = ArrayRef>,
    {
        self.columns.extend(columns);
        assert!(
            self.columns.iter().all(|c| c.len() == self.len()),
            "all columns must have the same length"
        );
    }

    /// Concatenates multiple data chunks vertically into a single data chunk.
    ///
    /// # Notes
    /// `compact` is called automatically for each input chunk before concatenation, that is, the
    /// resulting data chunk is guaranteed to be unfiltered.
    pub fn concat<I>(chunks: I) -> Self
    where
        I: IntoIterator<Item = DataChunk>,
    {
        let mut chunks = chunks.into_iter().collect_vec();
        assert!(!chunks.is_empty(), "chunks must not be empty");
        assert!(
            chunks.iter().map(|chunk| chunk.columns.len()).all_equal(),
            "all chunks must have the same number of columns"
        );
        chunks.iter_mut().for_each(|chunk| chunk.compact());
        let num_columns = chunks[0].columns.len();
        let columns = (0..num_columns)
            .map(|i| {
                compute::kernels::concat::concat(
                    &chunks
                        .iter()
                        .map(|chunk| chunk.columns[i].as_ref())
                        .collect_vec(),
                )
                .expect("concatenation should be successful")
            })
            .collect();
        Self {
            columns,
            filter: None,
        }
    }

    /// Takes rows from the data chunk by given indices and creates a new data chunk from these
    /// rows.
    ///
    /// # Panics
    ///
    /// Panics if any of the indices is out of bounds.
    pub fn take(&self, indices: &dyn Array) -> Self {
        let columns = compute::take_arrays(&self.columns, indices, None)
            .expect("`take_arrays` should be successful");
        let filter = self
            .filter
            .as_ref()
            .map(|f| compute::take(f, indices, None).expect("`take` should be successful"))
            .map(|f| f.as_boolean().clone());
        Self { columns, filter }
    }
}

impl FromIterator<DataChunk> for DataChunk {
    #[inline]
    fn from_iter<T: IntoIterator<Item = DataChunk>>(iter: T) -> Self {
        DataChunk::concat(iter)
    }
}

#[derive(Debug)]
pub struct Rows<'a> {
    chunk: &'a DataChunk,
    iter: RowIndexIter<'a>,
}

#[derive(Debug)]
enum RowIndexIter<'a> {
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
}

impl<'a> From<RowRef<'a>> for OwnedRow {
    #[inline]
    fn from(value: RowRef<'a>) -> Self {
        value.into_owned()
    }
}

#[macro_export]
macro_rules! data_chunk {
    ($(($type:ident, [$($values:expr),*])),*) => {
        {
            let columns: Vec<arrow::array::ArrayRef> = vec![$(
                arrow::array::create_array!($type, [$($values),*]),
            )*];
            $crate::data_chunk::DataChunk::new(columns)
        }
    };
    ({ $($filter_values:expr),* }, $(($type:ident, [$($values:expr),*])),*) => {
        {
            let columns: Vec<arrow::array::ArrayRef> = vec![$(
                arrow::array::create_array!($type, [$($values),*]),
            )*];
            let filter = arrow::buffer::BooleanBuffer::from_iter([$($filter_values),*]);
            $crate::data_chunk::DataChunk::new(columns).with_filter(filter.into())
        }
    };
}

impl Display for DataChunk {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        assert!(
            self.filter.is_none(),
            "only unfiltered data chunk can be displayed"
        );
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use arrow::array::create_array;

    use super::*;

    #[test]
    fn test_rows_1() {
        let chunk = data_chunk!((Int32, [1, 2, 3]), (Utf8, ["abc", "def", "ghi"]));
        let rows: Vec<_> = chunk.rows().map(|r| r.into_owned()).collect();
        let expected = vec![
            OwnedRow::new(vec![1i32.into(), "abc".into()]),
            OwnedRow::new(vec![2i32.into(), "def".into()]),
            OwnedRow::new(vec![3i32.into(), "ghi".into()]),
        ];
        assert_eq!(rows, expected);
    }

    #[test]
    fn test_rows_2() {
        let chunk = data_chunk!(
            { true, false, true },
            (Int32, [1, 2, 3]),
            (Utf8, ["abc", "def", "ghi"])
        );
        let rows: Vec<_> = chunk.rows().map(|r| r.into_owned()).collect();
        let expected = vec![
            OwnedRow::new(vec![1i32.into(), "abc".into()]),
            OwnedRow::new(vec![3i32.into(), "ghi".into()]),
        ];
        assert_eq!(rows, expected);
    }

    #[test]
    fn test_slice() {
        let chunk = data_chunk!(
            { true, false, true },
            (Int32, [1, 2, 3]),
            (Utf8, ["abc", "def", "ghi"])
        );
        let sliced = chunk.slice(1, 1);
        let expected = data_chunk!({ false }, (Int32, [2]), (Utf8, ["def"]));
        assert_eq!(sliced, expected);
    }

    #[test]
    #[should_panic]
    fn test_slice_out_of_bounds_1() {
        let chunk = data_chunk!(
            { true, false, true },
            (Int32, [1, 2, 3]),
            (Utf8, ["abc", "def", "ghi"])
        );
        let _sliced = chunk.slice(2, 2);
    }

    #[test]
    #[should_panic]
    fn test_slice_out_of_bounds_2() {
        let chunk = data_chunk!(
            { true, false, true },
            (Int32, [1, 2, 3]),
            (Utf8, ["abc", "def", "ghi"])
        );
        let _sliced = chunk.slice(3, 1);
    }

    #[test]
    fn test_compact() {
        let mut chunk = data_chunk!(
            { false, true, false },
            (Int32, [1, 2, 3]),
            (Utf8, ["abc", "def", "ghi"])
        );
        chunk.compact();
        let expected = data_chunk!((Int32, [2]), (Utf8, ["def"]));
        assert_eq!(chunk, expected);
    }

    #[test]
    fn test_concat() {
        let chunk1 =
            data_chunk!( {false, true, true}, (Int32, [1, 2, 3]), (Utf8, ["aaa", "bbb", "ccc"]));
        let chunk2 = data_chunk!((Int32, [4, 5, 6]), (Utf8, ["ddd", "eee", "fff"]));
        let expected = data_chunk!(
            (Int32, [2, 3, 4, 5, 6]),
            (Utf8, ["bbb", "ccc", "ddd", "eee", "fff"])
        );
        assert_eq!(DataChunk::concat([chunk1, chunk2]), expected);
    }

    #[test]
    fn test_take() {
        let chunk =
            data_chunk!( {false, true, true}, (Int32, [1, 2, 3]), (Utf8, ["abc", "def", "ghi"]));
        let indices = create_array!(Int32, [0, 2]);
        let taken = chunk.take(indices.as_ref());
        let expected = data_chunk!( {false, true}, (Int32, [1, 3]), (Utf8, ["abc", "ghi"]));
        assert_eq!(taken, expected);
    }
}
