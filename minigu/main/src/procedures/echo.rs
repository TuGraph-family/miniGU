use std::error::Error;
use std::sync::Arc;

use minigu_common::data_chunk;
use minigu_common::data_type::{DataField, DataSchema, LogicalType};
use minigu_common::value::ScalarValue;
use minigu_context::procedure::Procedure;
use minigu_context::session::SessionContext;

/// Echo the input string.
pub fn build_procedure() -> Procedure {
    let parameters = vec![LogicalType::String];
    let schema = Arc::new(DataSchema::new(vec![DataField::new(
        "output".into(),
        LogicalType::String,
        false,
    )]));
    Procedure::new(parameters, Some(schema), |_context, args| {
        assert_eq!(args.len(), 1);
        let arg = args[0]
            .try_as_string()
            .expect("arg must be a string")
            .clone();
        Ok(vec![data_chunk!((Utf8, [arg]))])
    })
}
