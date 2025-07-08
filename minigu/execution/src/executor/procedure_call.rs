use minigu_catalog::provider::ProcedureRef;
use minigu_common::value::ScalarValue;
use minigu_context::procedure::Procedure;
use minigu_context::session::SessionContext;

use super::utils::gen_try;
use super::{Executor, IntoExecutor};

pub struct ProcedureCallBuilder {
    procedure: ProcedureRef,
    session_context: SessionContext,
    args: Vec<ScalarValue>,
}

impl ProcedureCallBuilder {
    pub fn new(
        procedure: ProcedureRef,
        session_context: SessionContext,
        args: Vec<ScalarValue>,
    ) -> Self {
        Self {
            procedure,
            session_context,
            args,
        }
    }
}

impl IntoExecutor for ProcedureCallBuilder {
    type IntoExecutor = impl Executor;

    fn into_executor(self) -> Self::IntoExecutor {
        gen move {
            let ProcedureCallBuilder {
                procedure,
                session_context,
                args,
            } = self;
            let procedure = procedure
                .as_any()
                .downcast_ref::<Procedure>()
                .expect("the underlying type of procedure ref should be Procedure");
            for chunk in gen_try!(procedure.call(session_context, args)) {
                yield Ok(chunk)
            }
        }
        .into_executor()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use itertools::Itertools;
    use minigu_catalog::memory::MemoryCatalog;
    use minigu_catalog::memory::schema::MemorySchemaCatalog;
    use minigu_catalog::provider::DirectoryOrSchema;
    use minigu_common::data_chunk;
    use minigu_common::data_type::{DataField, DataSchema, LogicalType};
    use minigu_context::database::DatabaseContext;
    use rayon::ThreadPoolBuilder;

    use super::*;

    fn build_test_procedure() -> Procedure {
        let schema = Arc::new(DataSchema::new(vec![DataField::new(
            "a".into(),
            LogicalType::Int32,
            false,
        )]));
        Procedure::new(vec![], Some(schema), |_, _| {
            let chunk = data_chunk!((Int32, [1, 2, 3]));
            Ok(vec![chunk])
        })
    }

    #[test]
    fn test_procedure_call() {
        let root = Arc::new(MemorySchemaCatalog::new(None));
        let catalog = MemoryCatalog::new(DirectoryOrSchema::Schema(root));
        let runtime = ThreadPoolBuilder::new().build().unwrap();
        let database = Arc::new(DatabaseContext::new(catalog, runtime));
        let context = SessionContext::new(database);
        let procedure = Arc::new(build_test_procedure());
        let procedure_call = ProcedureCallBuilder::new(procedure, context, vec![]);
        let chunk: data_chunk::DataChunk = procedure_call
            .into_executor()
            .into_iter()
            .try_collect()
            .unwrap();
        assert_eq!(chunk, data_chunk!((Int32, [1, 2, 3])));
    }
}
