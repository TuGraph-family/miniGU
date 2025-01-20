use crate::macros::base;

#[apply(base)]
pub enum Procedure {
    Catalog,
    Data,
    Query,
}

#[apply(base)]
pub struct ProcedureBody {
    
}

#[apply(base)]
pub enum Statement {
    Catalog,
    Data,
    Query,
}