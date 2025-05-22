use std::collections::HashMap;
use std::sync::Arc;

use minigu_common::types::ProcedureId;

use super::Procedure;

#[derive(Debug)]
pub struct ProcedureRegistry {
    next_procedure_id: ProcedureId,
    procedures: HashMap<ProcedureId, Arc<dyn Procedure>>,
}

impl Default for ProcedureRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ProcedureRegistry {
    pub fn new() -> Self {
        Self {
            next_procedure_id: 1,
            procedures: HashMap::new(),
        }
    }

    pub fn register(&mut self, proc: Arc<dyn Procedure>) -> Option<ProcedureId> {
        let id = self.next_procedure_id;
        self.next_procedure_id = self.next_procedure_id.checked_add(1)?;
        self.procedures.insert(id, proc);
        Some(id)
    }

    pub fn get(&self, id: ProcedureId) -> Option<Arc<dyn Procedure>> {
        self.procedures.get(&id).cloned()
    }

    pub fn unregister(&mut self, id: ProcedureId) -> bool {
        self.procedures.remove(&id).is_some()
    }
}
