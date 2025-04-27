use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use std::ops::Range;
use std::sync::Arc;
use std::result;
use gql_parser::error::TokenError;
use crate::{impl_binder_already_exist_error_display, impl_binder_not_exist_error_display};


#[cfg(feature = "miette")]
use miette::Diagnostic;
use thiserror::Error;
pub type Result<T> = result::Result<T, Error>;

#[derive(Error, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub enum Error {
    #[error("schema not exists {0:?}")]
    SchemaNotExists(String),
    #[error("schema already exists {0:?}")]
    SchemaAlreadyExists(String),
    #[error("graph type not exists {0:?}")]
    GraphTypeNotExists(String),
    #[error("graph type already exists {0:?}")]
    GraphTypeAlreadyExists(String),
    #[error("graph not exists {0:?}")]
    GraphNotExists(String),
    #[error("graph already exists {0:?}")]
    GraphAlreadyExists(String),
    #[error("procedure {0} not exists")]
    ProcedureNotExists(String),
    #[error("node type not exists {0:?}")]
    NodeTypeNotExists(String),
    #[error("edge type not exists {0:?}")]
    EdgeTypeNotExists(String),
    #[error("invalid field {0:?}")]
    InvalidField(String),
    #[error("duplicate field name {0:?}")]
    DuplicateField(String),
    #[error("not support operation {0:?}")]
    NotSupported(String),
    #[error("tmp error")]
    ErrorCur
}

// #[derive(Debug, Clone, PartialEq, Eq)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// pub struct BinderError<T> {
//     input: Arc<str>,
//     span: Range<usize>,
//     position: (usize, usize),
//     #[cfg_attr(feature = "serde", serde(skip))]
//     _marker: PhantomData<T>,
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotSupportError {
    pub operation_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidFieldError {
    pub field_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidKeyError {
    pub key_name: String,
}



// impl<T> BinderError<T> {
//     pub fn input(&self) -> &Arc<str> {
//         &self.input
//     }
// 
//     pub fn span(&self) -> &Range<usize> {
//         &self.span
//     }
// 
//     pub fn position(&self) -> (usize, usize) {
//         self.position
//     }
// }

impl NotSupportError {
    pub fn operation_name(&self) -> &String {
        &self.operation_name
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemaNotExistsError {
    pub schema_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemaAlreadyExistsError {
    pub schema_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphTypeNotExistsError {
    pub type_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphTypeAlreadyExists {
    pub type_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphNotExistsError {
    pub graph_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphAlreadyExists {
    pub graph_name: String,
}

// impl_binder_not_exist_error_display!(
//     (SchemaNotExists, "schema"),
//     (GraphNotExists, "graph"),
//     (GraphTypeNotExists, "graph type"),
// );
//
// impl_binder_already_exist_error_display!(
//     (SchemaAlreadyExists, "schema"),
//     (GraphAlreadyExists, "graph"),
//     (GraphTypeAlreadyExists, "graph type"),
// );

impl Display for NotSupportError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let operation = &self.operation_name();
        write!(f,
            "{operation} is not support yet"
        )
    }
}
// 
// impl<T> core::error::Error for BinderError<T> where BinderError<T>: Debug + Display {}
impl core::error::Error for NotSupportError {}
// 
// #[cfg(feature = "miette")]
// impl<T> Diagnostic for BinderError<T> where BinderError<T>: core::error::Error {
//     fn source_code(&self) -> Option<&dyn miette::SourceCode> {
//         Some(&self.input)
//     }
// 
//     fn labels(&self) -> Option<Box<dyn Iterator<Item = miette::LabeledSpan> + '_>> {
//         Some(Box::new(core::iter::once(
//             miette::LabeledSpan::new_with_span(Some("here".into()), self.span.clone()),
//         )))
//     }
// }

#[cfg(feature = "miette")]
impl Diagnostic for NotSupportError {
    fn source_code(&self) -> Option<&dyn miette::SourceCode> {
        Some(&self.operation_name)
    }
}

