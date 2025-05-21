use std::collections::HashMap;
use std::sync::Arc;

use minigu_common::types::ProcedureId;
use minigu_execution::executor::Executor;

use super::{Procedure, ProcedureRef};

#[derive(Debug)]
pub struct ProcedureRegistry {
    next_procedure_id: ProcedureId,
    procedures: HashMap<ProcedureId, ProcedureRef>,
}

impl Default for ProcedureRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ProcedureRegistry {
    pub fn new() -> Self {
        Self {
            next_procedure_id: ProcedureId::new(1).expect("procedure id should be non-zero"),
            procedures: HashMap::new(),
        }
    }

    pub fn add(&mut self, proc: ProcedureRef) -> ProcedureId {
        let id = self.next_procedure_id;
        self.next_procedure_id;
        self.procedures.insert(id, proc);
        id
    }

    pub fn get(&self, id: ProcedureId) -> Option<ProcedureRef> {
        todo!()
    }

    pub fn remove(&mut self, id: ProcedureId) -> bool {
        todo!()
    }
}
