use std::sync::Arc;

use arrow::array::{ArrayRef, StringArray};
use itertools::Itertools;
use minigu_catalog::provider::SchemaProvider;
use minigu_catalog::txn::error::CatalogTxnError;
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
    Procedure::new(vec![], Some(schema.clone()), move |mut context, args| {
        assert!(args.is_empty());
        let current_schema_opt = context.current_schema.clone();
        let schema_local = schema.clone();
        let chunks = context.with_statement_txn(|txn| {
            let chunk = if let Some(current_schema) = current_schema_opt.clone() {
                let names = current_schema.procedure_names(txn);
                let procedures: Vec<_> = names
                    .iter()
                    .map(|name| {
                        current_schema
                            .get_procedure(name, txn)
                            .map_err(|e| CatalogTxnError::External(Box::new(e)))
                    })
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
                DataChunk::new_empty(&schema_local)
            };
            Ok(vec![chunk])
        })?;
        Ok(chunks)
    })
}
