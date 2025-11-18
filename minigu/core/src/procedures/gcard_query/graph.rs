use std::collections::{HashMap, HashSet};

use minigu_common::types::{EdgeId, VertexId};

use crate::procedures::gcard_query::types::QueryVertex;

pub trait Endpoints {
    fn src(&self) -> VertexId;
    fn dst(&self) -> VertexId;
}

#[derive(Debug, Clone, Default)]
pub struct GraphSkeleton<E> {
    pub vertices: HashMap<VertexId, QueryVertex>,
    pub edges: HashMap<EdgeId, E>,
    pub outgoing_edges: HashMap<VertexId, Vec<EdgeId>>,
    pub incoming_edges: HashMap<VertexId, Vec<EdgeId>>,
}

impl<E> GraphSkeleton<E> {
    pub fn get_degree(&self, vertex_id: VertexId) -> usize {
        let out = self
            .outgoing_edges
            .get(&vertex_id)
            .map(|v| v.len())
            .unwrap_or(0);
        let inc = self
            .incoming_edges
            .get(&vertex_id)
            .map(|v| v.len())
            .unwrap_or(0);
        out + inc
    }

    pub fn get_outgoing_edges(&self, vertex_id: VertexId) -> Vec<&E> {
        self.outgoing_edges
            .get(&vertex_id)
            .map(|ids| ids.iter().filter_map(|id| self.edges.get(id)).collect())
            .unwrap_or_default()
    }

    pub fn get_incoming_edges(&self, vertex_id: VertexId) -> Vec<&E> {
        self.incoming_edges
            .get(&vertex_id)
            .map(|ids| ids.iter().filter_map(|id| self.edges.get(id)).collect())
            .unwrap_or_default()
    }

    pub fn get_vertex(&self, vertex_id: VertexId) -> Option<&QueryVertex> {
        self.vertices.get(&vertex_id)
    }

    pub fn get_edge(&self, edge_id: EdgeId) -> Option<&E> {
        self.edges.get(&edge_id)
    }

    pub fn find_pivot_nodes(&self) -> HashSet<VertexId> {
        self.vertices
            .keys()
            .filter(|&vid| self.get_degree(*vid) >= 3)
            .copied()
            .collect()
    }
}

impl<E: Endpoints> GraphSkeleton<E> {
    pub fn get_neighbors(&self, vertex_id: VertexId) -> Vec<VertexId> {
        let mut out = Vec::new();
        for e in self.get_outgoing_edges(vertex_id) {
            out.push(e.dst());
        }
        for e in self.get_incoming_edges(vertex_id) {
            out.push(e.src());
        }
        out
    }

    pub fn get_neighbor_edges(&self, vertex_id: VertexId) -> Vec<EdgeId> {
        let mut out = Vec::new();
        if let Some(outgoing_ids) = self.outgoing_edges.get(&vertex_id) {
            out.extend(outgoing_ids.iter().copied());
        }

        if let Some(incoming_ids) = self.incoming_edges.get(&vertex_id) {
            out.extend(incoming_ids.iter().copied());
        }
        out
    }
}
