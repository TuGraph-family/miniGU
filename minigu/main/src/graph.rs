use std::collections::HashMap;
use std::sync::Arc;

use minigu_common::types::GraphId;
use minigu_storage::memory::MemoryGraph;

pub type GraphRef = Arc<()>;

pub struct GraphRegistry {
    next_graph_id: GraphId,
    graphs: HashMap<GraphId, Arc<MemoryGraph>>,
}

impl Default for GraphRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphRegistry {
    pub fn new() -> Self {
        Self {
            next_graph_id: 1,
            graphs: HashMap::new(),
        }
    }

    // pub fn register(&mut self, graph: GraphRef) -> Option<GraphId> {
    //     let id = self.next_graph_id;
    //     self.next_graph_id = self.next_graph_id.checked_add(1)?;
    //     self.graphs.insert(id, graph);
    //     Some(id)
    // }

    // pub fn get(&self, id: GraphId) -> Option<GraphRef> {
    //     self.graphs.get(&id).cloned()
    // }

    pub fn unregister(&mut self, id: GraphId) -> bool {
        self.graphs.remove(&id).is_some()
    }
}
