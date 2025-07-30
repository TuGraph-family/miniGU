use minigu_common::data_type::DataSchemaRef;
use serde::Serialize;

use super::catalog::BoundCatalogModifyingStatement;
use super::query::BoundCompositeQueryStatement;

#[derive(Debug, Clone, Serialize)]
pub struct BoundProcedure {
    pub statement: BoundStatement,
    pub next_statements: Vec<BoundNextStatement>,
}

impl BoundProcedure {
    pub fn schema(&self) -> Option<DataSchemaRef> {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum BoundStatement {
    Catalog(Vec<BoundCatalogModifyingStatement>),
    Query(BoundCompositeQueryStatement),
    // Data(BoundLinearDataModifyingStatement),
}

#[derive(Debug, Clone, Serialize)]
pub struct BoundNextStatement {
    pub yield_column_indices: Vec<usize>,
    pub statement: BoundStatement,
}
