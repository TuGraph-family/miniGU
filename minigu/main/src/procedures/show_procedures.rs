use std::sync::Arc;

use arrow::array::{ArrayRef, StringArray};
use itertools::Itertools;
use minigu_catalog::provider::SchemaProvider;
use minigu_common::data_chunk;
use minigu_common::data_chunk::DataChunk;
use minigu_common::data_type::{DataField, DataSchema, LogicalType};
use minigu_context::procedure::Procedure;

/// Show all procedures in current schema.
pub fn build_procedure() -> Procedure {
    let schema = Arc::new(DataSchema::new(vec![
        DataField::new("name".into(), LogicalType::String, false),
        DataField::new("params".into(), LogicalType::String, false),
    ]));
    Procedure::new(vec![], Some(schema.clone()), move |context, args| {
        assert!(args.is_empty());
        let chunk = if let Some(current_schema) = context.current_schema {
            let names = current_schema.procedure_names();
            let procedures: Vec<_> = names
                .iter()
                .map(|name| current_schema.get_procedure(name))
                .try_collect()?;
            let parameters = procedures.into_iter().map(|p| {
                p.expect("procedure should exist")
                    .parameters()
                    .iter()
                    .map(|p| p.to_string())
                    .join(", ")
            });
            let names = Arc::new(StringArray::from_iter_values(names));
            let parameters = Arc::new(StringArray::from_iter_values(parameters));
            DataChunk::new(vec![names, parameters])
        } else {
            DataChunk::new_empty(&schema)
        };
        Ok(vec![chunk])
    })
}
