use serde::Serialize;

use crate::object_ref::SchemaRef;
use crate::statement::catalog::LinearBoundCatalogModifyingStatement;

#[derive(Debug, Serialize)]
pub struct BoundProcedure {
    // pub binding_variable_def: BindingVariableDefBlock,
    pub statement: BoundStatement,
    pub next_statement: Vec<BoundNextStatement>,
}

#[derive(Debug, Serialize)]
pub enum BoundStatement {
    Catalog(LinearBoundCatalogModifyingStatement),
    // Query(BoundCompositeQueryStatement),
    // Data(BoundLinearDataModifyingStatement),
}

#[derive(Debug, Serialize)]
pub struct BoundNextStatement {
    pub yield_clause: Option<Vec<usize>>,
    pub statement: BoundStatement,
}
