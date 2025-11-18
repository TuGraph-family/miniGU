use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufReader, BufWriter};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::procedures::gcard_query::degreepiecewise::PiecewiseConstantFunction;
use crate::procedures::gcard_query::error::{GCardError, GCardResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AltKey(pub Vec<String>);

impl fmt::Display for AltKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.join(", "))
    }
}
impl AltKey {
    pub fn sorted(&self) -> AltKey {
        let mut v = self.0.clone();
        v.sort();
        AltKey(v)
    }

    fn normalized(&self) -> Vec<String> {
        let mut v: Vec<String> = self
            .0
            .iter()
            .map(|s| s.to_lowercase())
            .collect();
        v.sort();
        v
    }
}

impl PartialEq for AltKey {
    fn eq(&self, other: &Self) -> bool {
        self.normalized() == other.normalized()
    }
}

impl Hash for AltKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for s in self.normalized() {
            s.hash(state);
        }
    }
}

impl Eq for AltKey {}

pub fn make_alt_key(node_seq: &[String], edge_seq: &[String]) -> AltKey {
    let mut out = Vec::with_capacity(node_seq.len() + edge_seq.len());
    for i in 0..node_seq.len() {
        out.push(node_seq[i].clone());
        if i < edge_seq.len() {
            out.push(edge_seq[i].clone());
        }
    }
    AltKey(out)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompressedDegreeSeq {
    SafeBound {
        function: PiecewiseConstantFunction,
    },
    FastCompressor {
        len: usize,
        base: u64,
        counts: Vec<u64>,
    },
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DegreeSeqGraphCompressed {
    pub edge_set_to_endpoints: HashMap<AltKey, HashMap<String, CompressedDegreeSeq>>,
}

impl DegreeSeqGraphCompressed {
    pub fn new() -> Self {
        Self {
            edge_set_to_endpoints: HashMap::new(),
        }
    }

    pub fn get_piece_func_by_path(
        &self,
        path: &AltKey,
        target_node: &str,
    ) -> PiecewiseConstantFunction {
        if let Some(endpoints) = self.edge_set_to_endpoints.get(&path.sorted()) {
            let func = endpoints
                .iter()
                .find(|(k, _)| k.eq_ignore_ascii_case(target_node))
                .map(|(_, v)| v.clone())
                .expect("not found");
            match func {
                CompressedDegreeSeq::SafeBound { function } => function,
                CompressedDegreeSeq::FastCompressor {
                    len: _,
                    base,
                    counts,
                } => {
                    let total: usize = counts.iter().map(|&c| c as usize).sum();
                    let mut seq = Vec::with_capacity(total);
                    for (i, &c) in counts.iter().enumerate().rev() {
                        if c == 0 {
                            continue;
                        }
                        let upper_f = base.pow(i as u32);
                        let upper_u64 = if upper_f >= u64::MAX {
                            u64::MAX
                        } else {
                            upper_f as u64
                        };
                        seq.extend(std::iter::repeat(upper_u64).take(c as usize));
                    }
                    PiecewiseConstantFunction::from_degree_sequence(seq.as_slice(), 0.01, true)
                        .unwrap()
                }
            }
        } else {
            PiecewiseConstantFunction::empty()
        }
    }

    pub fn num_edge_sets(&self) -> usize {
        self.edge_set_to_endpoints.len()
    }

    pub fn export_bincode<P: AsRef<Path>>(&self, path: P) -> GCardResult<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        bincode::serialize_into(writer, self)
            .map_err(|e| GCardError::InvalidData(format!("Failed to serialize: {}", e)))?;
        Ok(())
    }

    pub fn import_bincode<P: AsRef<Path>>(path: P) -> GCardResult<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let graph = bincode::deserialize_from(reader)
            .map_err(|e| GCardError::InvalidData(format!("Failed to deserialize: {}", e)))?;
        Ok(graph)
    }
}
