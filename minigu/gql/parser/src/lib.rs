#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "512"]
// TODO: Remove this once the implementation is complete.
#![allow(unused_macros, unused)]
#![deny(clippy::undocumented_unsafe_blocks)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[macro_use(apply)]
extern crate macro_rules_attribute;

pub mod ast;
pub mod error;
mod lexer;
mod macros;
mod parser;
pub mod span;
mod unescape;

pub use lexer::TokenKind;
pub use parser::{ParseOptions, Token, parse_gql, tokenize, tokenize_full};

#[cfg(not(feature = "std"))]
mod imports {
    pub(crate) use alloc::boxed::Box;
    pub(crate) use alloc::sync::Arc;
    pub(crate) use alloc::vec::Vec;
}
#[cfg(feature = "std")]
mod imports {
    pub(crate) use std::boxed::Box;
    pub(crate) use std::sync::Arc;
    pub(crate) use std::vec::Vec;
}
