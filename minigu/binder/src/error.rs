use itertools::Itertools;
use miette::Diagnostic;
use minigu_catalog::error::CatalogError;
use minigu_common::data_type::LogicalType;
use minigu_common::error::NotImplemented;
use smol_str::SmolStr;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum BindError {
    #[error("catalog error")]
    Catalog(#[from] CatalogError),

    #[error("not a directory: {0}")]
    NotDirectory(String),

    #[error("not a schema: {0}")]
    NotSchema(String),

    #[error("no such directory or schema at: {0}")]
    DirectoryOrSchemaNotFound(String),

    #[error("current graph not specified")]
    CurrentGraphNotSpecified,

    #[error("current schema not specified")]
    CurrentSchemaNotSpecified,

    #[error("home schema not specified")]
    HomeSchemaNotSpecified,

    #[error("home graph not specified")]
    HomeGraphNotSpecified,

    #[error("procedure not found: {0}")]
    ProcedureNotFound(SmolStr),

    #[error("graph not found: {0}")]
    GraphNotFound(SmolStr),

    #[error("too many objects: {0:?}")]
    InvalidObjectReference(Vec<SmolStr>),

    #[error("procedure without schema: {0}")]
    ProcedureWithoutSchema(SmolStr),

    #[error("yield item not found: {0}")]
    YieldItemNotFound(SmolStr),

    #[error("variable not found: {0}")]
    VariableNotFound(SmolStr),

    #[error("invalid integer: {0}")]
    InvalidInteger(SmolStr),

    #[error(
        "incorrect number or types of arguments for procedure {procedure}: expected [{}], got [{}]",
        expected.iter().map(|t| t.to_string()).join(", "),
        actual.iter().map(|t| t.to_string()).join(", "),
    )]
    IncorrectArguments {
        procedure: SmolStr,
        expected: Vec<LogicalType>,
        actual: Vec<LogicalType>,
    },

    #[error("yield clause not allowed for procedure without data schema: {0}")]
    YieldAfterSchemalessProcedure(SmolStr),

    #[error("data schema not provided for procedure: {0}")]
    DataSchemaNotProvided(SmolStr),

    #[error("incorrect number of yield items: expected {expected}, got {actual}")]
    IncorrectNumberOfYieldItems { expected: usize, actual: usize },

    #[error("no column can be returned in the return statement")]
    NoColumnInReturnStatement,

    #[error("not a catalog-modifying procedure: {0}")]
    #[diagnostic(help(
        "append \"return *\" to the statement if you want to use \"{0}\" as a query procedure"
    ))]
    NotCatalogProcedure(SmolStr),

    // TODO: Remove this error variant
    #[error("unexpected bind error")]
    Unexpected,

    #[error(transparent)]
    #[diagnostic(transparent)]
    NotImplemented(#[from] NotImplemented),
}

pub type BindResult<T> = std::result::Result<T, BindError>;
