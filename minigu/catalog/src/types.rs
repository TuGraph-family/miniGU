use std::num::NonZeroU32;
use std::result;
use smallvec::SmallVec;
use minigu_common::error::MiniGuError;
use smol_str::SmolStr;

pub type VertexTypeId = NonZeroU32;

pub type EdgeTypeId = NonZeroU32;

pub type GraphTypeId = NonZeroU32;

pub type LabelId = NonZeroU32;

pub type GraphId = NonZeroU32;

pub type SchemaId = NonZeroU32;

pub type ProcedureId = NonZeroU32;

pub type PropertyId = NonZeroU32;

pub type Ident = SmolStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LabelSet(SmallVec<[minigu_common::types::LabelId; 4]>);

pub type Result<T, E = MiniGuError> = result::Result<T, E>;
