use serde::Serialize;

use super::catalog::BoundCatalogModifyingStatement;
use super::query::BoundCompositeQueryStatement;

#[derive(Debug, Clone, Serialize)]
pub struct BoundProcedure {
    pub statement: BoundStatement,
    pub next_statements: Vec<BoundNextStatement>,
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
