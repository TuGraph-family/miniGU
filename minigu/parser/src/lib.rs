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
pub(crate) use alloc::borrow::Cow;
#[cfg(not(feature = "std"))]
pub(crate) use alloc::boxed::Box;
#[cfg(not(feature = "std"))]
pub(crate) use alloc::vec::Vec;
#[cfg(feature = "std")]
pub(crate) use std::borrow::Cow;
#[cfg(feature = "std")]
pub(crate) use std::boxed::Box;
#[cfg(feature = "std")]
pub(crate) use std::vec::Vec;
