use std::ops::Deref;
use std::sync::Arc;

use minigu_catalog::provider::{
    DirectoryProvider, EdgeTypeProvider, GraphProvider, GraphTypeProvider, ProcedureProvider,
    SchemaProvider, VertexTypeProvider,
};
use serde::Serialize;
use smol_str::SmolStr;

pub type SchemaRef = ObjectRef<dyn SchemaProvider>;
pub type DirectoryRef = ObjectRef<dyn DirectoryProvider>;
pub type GraphRef = ObjectRef<dyn GraphProvider>;
pub type GraphTypeRef = ObjectRef<dyn GraphTypeProvider>;
pub type VertexTypeRef = ObjectRef<dyn VertexTypeProvider>;
pub type EdgeTypeRef = ObjectRef<dyn EdgeTypeProvider>;
pub type ProcedureRef = ObjectRef<dyn ProcedureProvider>;

#[derive(Debug, Serialize)]
pub struct ObjectRef<T: ?Sized> {
    pub name: SmolStr,
    #[serde(skip)]
    pub object: Arc<T>,
}

impl<T: ?Sized> ObjectRef<T> {
    pub fn new(name: SmolStr, object: Arc<T>) -> Self {
        Self { name, object }
    }

    pub fn name(&self) -> &SmolStr {
        &self.name
    }
}

impl<T: ?Sized> Deref for ObjectRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}
