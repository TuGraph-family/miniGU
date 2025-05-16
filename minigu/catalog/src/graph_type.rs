use std::collections::HashMap;

use minigu_common::types::LabelId;
use smol_str::SmolStr;

use crate::label_set::LabelSet;
use crate::types::{NodeTypeId, RelTypeId};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeType {}

impl NodeType {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RelType {}

impl RelType {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphType {
    name_to_label_id: HashMap<SmolStr, LabelId>,
    key_label_set_to_node_type_id: HashMap<LabelSet, NodeTypeId>,
    node_types: Vec<NodeType>,
    key_label_set_to_rel_type_id: HashMap<LabelSet, RelTypeId>,
    rel_types: Vec<RelType>,
}

impl GraphType {
    #[inline(always)]
    pub fn get_label_id(&self, name: &str) -> Option<LabelId> {
        self.name_to_label_id.get(name).copied()
    }

    #[inline(always)]
    pub fn get_node_type_id(&self, key_label_set: &LabelSet) -> Option<NodeTypeId> {
        self.key_label_set_to_node_type_id
            .get(key_label_set)
            .copied()
    }

    #[inline(always)]
    pub fn get_node_type(&self, id: NodeTypeId) -> &NodeType {
        self.node_types
            .get((id.get() - 1) as usize)
            .expect("node type with `id` should exist")
    }

    #[inline(always)]
    pub fn get_rel_type_id(&self, key_label_set: &LabelSet) -> Option<RelTypeId> {
        self.key_label_set_to_rel_type_id
            .get(key_label_set)
            .copied()
    }

    #[inline(always)]
    pub fn get_rel_type(&self, id: RelTypeId) -> &RelType {
        self.rel_types
            .get((id.get() - 1) as usize)
            .expect("rel type with `id` should exist")
    }
}
