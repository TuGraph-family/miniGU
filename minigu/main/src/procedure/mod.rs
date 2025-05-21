use std::fmt::Debug;
use std::sync::Arc;

pub mod registry;
pub mod result;

pub type ProcedureRef = Arc<dyn Procedure>;
pub trait Procedure: Debug + Send + Sync {

}
