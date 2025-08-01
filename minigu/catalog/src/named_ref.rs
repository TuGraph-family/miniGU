use std::fmt::{Debug, Display};
use std::ops::Deref;
use std::sync::Arc;

use serde::Serialize;
use smol_str::SmolStr;

use crate::provider::{
    EdgeTypeProvider, GraphProvider, GraphTypeProvider, ProcedureProvider, VertexTypeProvider,
};

pub type NamedGraphRef = NamedRef<dyn GraphProvider>;
pub type NamedGraphTypeRef = NamedRef<dyn GraphTypeProvider>;
pub type NamedVertexTypeRef = NamedRef<dyn VertexTypeProvider>;
pub type NamedEdgeTypeRef = NamedRef<dyn EdgeTypeProvider>;
pub type NamedProcedureRef = NamedRef<dyn ProcedureProvider>;

#[derive(Serialize)]
pub struct NamedRef<T: ?Sized> {
    name: SmolStr,
    #[serde(skip)]
    object: Arc<T>,
}

impl<T: ?Sized> NamedRef<T> {
    #[inline]
    pub fn new(name: SmolStr, object: Arc<T>) -> Self {
        Self { name, object }
    }

    #[inline]
    pub fn name(&self) -> &SmolStr {
        &self.name
    }

    #[inline]
    pub fn object(&self) -> &Arc<T> {
        &self.object
    }
}

impl<T: ?Sized> Deref for NamedRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl<T: ?Sized> Debug for NamedRef<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl<T: ?Sized> Display for NamedRef<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl<T: ?Sized> Clone for NamedRef<T> {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            object: self.object.clone(),
        }
    }
}
