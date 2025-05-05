use std::collections::HashMap;
use std::sync::Arc;

use arrow::array::{Array, ArrayRef, StringArray};
use minigu_common::types::{VertexId, VertexIdArray};

use super::{ExpandSource, VertexPropertySource};
use crate::error::ExecutionResult;

type AdjList = Arc<(Vec<VertexId>, Vec<String>)>;

#[derive(Debug, Clone)]
pub struct MockExpandSourceBuilder {
    adj_lists: HashMap<VertexId, (Vec<VertexId>, Vec<String>)>,
    max_array_size: usize,
}

impl MockExpandSourceBuilder {
    pub fn new(max_array_size: usize) -> Self {
        Self {
            adj_lists: HashMap::new(),
            max_array_size,
        }
    }

    pub fn add_vertex(mut self, vertex: VertexId) -> Self {
        self.adj_lists
            .entry(vertex)
            .or_insert_with(|| (vec![], vec![]));
        self
    }

    pub fn add_edge(mut self, src: VertexId, dst: VertexId, prop: String) -> Self {
        let (neighbors, props) = self.adj_lists.get_mut(&src).unwrap();
        neighbors.push(dst);
        props.push(prop);
        self
    }

    pub fn build(self) -> MockExpandSource {
        MockExpandSource {
            adj_lists: self
                .adj_lists
                .into_iter()
                .map(|(k, v)| (k, Arc::new(v)))
                .collect(),
            max_array_size: self.max_array_size,
        }
    }
}

/// A mock expand source that maps each vertex to its neighbors and the corresponding String-typed
/// edge properties.
///
/// This should be used for testing purposes only.
#[derive(Debug, Clone)]
pub struct MockExpandSource {
    adj_lists: HashMap<VertexId, AdjList>,
    max_array_size: usize,
}

pub struct ExpandIter {
    neighbors_props: AdjList,
    offset: usize,
    max_array_size: usize,
}

impl Iterator for ExpandIter {
    type Item = ExecutionResult<Vec<ArrayRef>>;

    fn next(&mut self) -> Option<Self::Item> {
        let (neighbors, props) = &*self.neighbors_props;
        if self.offset >= neighbors.len() {
            return None;
        }
        let neighbors = VertexIdArray::from_iter_values(
            neighbors
                .iter()
                .skip(self.offset)
                .take(self.max_array_size)
                .copied(),
        );
        let props =
            StringArray::from_iter_values(props.iter().skip(self.offset).take(self.max_array_size));
        self.offset += self.max_array_size;
        Some(Ok(vec![Arc::new(neighbors), Arc::new(props)]))
    }
}

impl ExpandSource for MockExpandSource {
    type ExpandIter = ExpandIter;

    fn expand_from_vertex(&self, vertex: VertexId) -> Option<Self::ExpandIter> {
        self.adj_lists.get(&vertex).map(|adj_list| ExpandIter {
            neighbors_props: adj_list.clone(),
            offset: 0,
            max_array_size: self.max_array_size,
        })
    }
}

#[derive(Debug, Clone)]
pub struct MockVertexPropertySource {
    vertex_properties: HashMap<VertexId, String>,
}

impl MockVertexPropertySource {
    pub fn new() -> Self {
        Self {
            vertex_properties: HashMap::new(),
        }
    }

    pub fn add_vertex_property(&mut self, vertex: VertexId, property: String) {
        self.vertex_properties.insert(vertex, property);
    }
}

impl VertexPropertySource for MockVertexPropertySource {
    fn scan_vertex_properties(&self, vertices: &VertexIdArray) -> ExecutionResult<Vec<ArrayRef>> {
        assert!(!vertices.is_nullable());
        let properties = StringArray::from_iter(
            vertices
                .values()
                .iter()
                .map(|v| self.vertex_properties.get(v)),
        );
        Ok(vec![Arc::new(properties)])
    }
}
