mod expand_source;
#[cfg(test)]
pub(crate) mod mock;

mod property_scan_source;

use std::sync::Arc;

use arrow::array::ArrayRef;
use auto_impl::auto_impl;
use minigu_common::types::{LabelId, PropertyId, VertexId, VertexIdArray};

use crate::error::ExecutionResult;
use crate::executor::vertex_scan::VertexScanBuilder;
use crate::executor::{Executor, IntoExecutor};

/// The output type of [`VertexSource`].
pub type VertexSourceOutput = ExecutionResult<Arc<VertexIdArray>>;

/// A trait for sources that can be scanned to get vertex IDs.
///
/// This has been automatically implemented for all types of `IntoIterator<Item =
/// VertexSourceOutput>`.
pub trait VertexSource: Iterator<Item = VertexSourceOutput> {
    fn scan_vertex(self) -> impl Executor
    where
        Self: Sized,
    {
        VertexScanBuilder::new(self).into_executor()
    }
}

impl<I> VertexSource for I where I: Iterator<Item = VertexSourceOutput> {}

/// A trait for sources that map vertex IDs to (multiple) property value columns.
#[auto_impl(&, Box, Arc)]
pub trait VertexPropertySource {
    fn scan_vertex_properties(
        &self,
        vertices: &VertexIdArray,
        property_list: &Vec<PropertyId>,
    ) -> ExecutionResult<Vec<ArrayRef>>;
}

/// A trait for sources that map a vertex to its neighbors and (possibly) properties of the
/// corresponding edges.
#[auto_impl(&, Box, Arc)]
pub trait ExpandSource {
    type ExpandIter: Iterator<Item = ExecutionResult<Vec<ArrayRef>>>;

    /// Returns an iterator over the neighbors and (possibly) properties of the given vertex, if
    /// the vertex exists. Otherwise, return `None`.
    ///
    /// # Parameters
    /// - `vertex`: The source vertex ID to expand from
    /// - `edge_labels`: Optional filter for edge labels. If `Some(labels)`, only neighbors
    ///   connected by edges with labels in `labels` are returned.
    /// - `target_vertex_labels`: Optional filter for target vertex labels. If `Some(labels)`, only
    ///   neighbors with labels matching `labels` are returned.
    ///
    /// # Notes
    /// The following two cases should be handled correctly:
    /// - The vertex does not exists.
    /// - The vertex exists but it has no neighbor.
    ///
    /// For the first case, this method should return `None`. For the second case, this method
    /// should return an iterator that yields no output.
    fn expand_from_vertex(
        &self,
        vertex: VertexId,
        edge_labels: Option<Vec<Vec<LabelId>>>,
        target_vertex_labels: Option<Vec<Vec<LabelId>>>,
    ) -> Option<Self::ExpandIter>;
}

pub type BoxedExpandSource =
    Box<dyn ExpandSource<ExpandIter = Box<dyn Iterator<Item = ExecutionResult<Vec<ArrayRef>>>>>>;
