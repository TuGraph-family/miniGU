use std::error::Error;
use std::sync::Arc;

use minigu_common::data_chunk::{self, DataChunk};
use minigu_common::data_type::{DataField, DataSchema, LogicalType};
use minigu_common::value::ScalarValue;
use minigu_context::procedure::ProcedureContainer;
use minigu_context::session::SessionContext;

pub fn echo() -> ProcedureContainer {
    let parameters = vec![LogicalType::String];
    let schema = Arc::new(DataSchema::new(vec![DataField::new(
        "output".into(),
        LogicalType::String,
        false,
    )]));
    ProcedureContainer::new(parameters, Some(schema), |_context, args| {
        assert_eq!(args.len(), 1);
        let arg = args[0].try_as_string().expect("arg must be a string");
        todo!()

    })
}
