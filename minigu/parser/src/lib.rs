#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "512"]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[macro_use(apply)]
extern crate macro_rules_attribute;

pub mod ast;
pub mod error;
mod lexer;
mod macros;
mod parser;
mod span;
mod unescape;

#[cfg(not(feature = "std"))]
mod imports {
    pub(crate) use alloc::borrow::Cow;
    pub(crate) use alloc::boxed::Box;
    pub(crate) use alloc::vec;
    pub(crate) use alloc::vec::Vec;
}
#[cfg(feature = "std")]
mod imports {
    pub(crate) use std::borrow::Cow;
    pub(crate) use std::boxed::Box;
    pub(crate) use std::vec;
    pub(crate) use std::vec::Vec;
}
