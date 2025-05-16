use std::num::NonZeroU32;
use smol_str::SmolStr;

/// Identifier associated with a vertex type.
pub type VertexTypeId = NonZeroU32;

/// Identifier associated with a relationship type.
pub type EdgeTypeId = NonZeroU32;

pub type GraphTypeId = NonZeroU32;

pub type LabelId = NonZeroU32;

pub type GraphId = NonZeroU32;

pub type SchemaId = NonZeroU32;

pub type ProcedureId = NonZeroU32;

pub type PropertyId = NonZeroU32;

pub type Ident = SmolStr;


