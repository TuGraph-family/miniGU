//! AST definitions for *session management*.

use crate::imports::Vec;
use crate::macros::{base, ext};

#[apply(base)]
pub struct SessionActivity<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub set: Vec<SessionSet<'a>>,
    pub reset: Vec<SessionReset<'a>>,
}

#[apply(base)]
pub enum SessionSet<'a> {
    Schema(SessionSetSchema),
    Graph(SessionSetGraph),
    #[cfg_attr(feature = "serde", serde(borrow))]
    TimeZone(SessionSetTimeZone<'a>),
    Parameter(SessionSetParameter),
}

#[apply(base)]
pub struct SessionSetSchema {}

#[apply(base)]
pub struct SessionSetGraph {}

#[apply(base)]
pub struct SessionSetTimeZone<'a> {
    pub value: &'a str,
}

#[apply(ext)]
pub enum SessionSetParameter {
    Graph,
    Table,
    Value,
}

#[apply(base)]
pub enum SessionReset<'a> {
    All,
    Schema,
    Graph,
    TimeZone,
    Parameter(&'a str),
}
