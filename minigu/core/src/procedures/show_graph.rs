use std::sync::Arc;

use arrow::array::StringArray;
use minigu_catalog::provider::SchemaProvider;
use minigu_common::data_chunk::DataChunk;
use minigu_common::data_type::{DataField, DataSchema, LogicalType};
use minigu_context::procedure::Procedure;

// Show graph names in current schema.

pub fn build_procedure() -> Procedure {
    let schema = Arc::new(DataSchema::new(vec![DataField::new(
        "graph_name".into(),
        LogicalType::String,
        false,
    )]));

    Procedure::new(vec![], Some(schema.clone()), move |context, args| {
        assert!(args.is_empty());
        let chunk = if let Some(current_schema) = context.current_schema {
            let names = current_schema.graph_names();
            let names_out = Arc::new(StringArray::from_iter_values(names));
            DataChunk::new(vec![names_out])
        } else {
            DataChunk::new_empty(&schema)
        };
        Ok(vec![chunk])
    })
}
