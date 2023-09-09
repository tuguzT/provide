//! Truly zero cost dependency injection — in safe and stable Rust.
//!
//! This crate defines a concept of **providers** — types which provide some dependency
//! by value, shared or unique reference.

// TODO crate documentation

#![warn(clippy::all)]
#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![no_std]

pub use self::{
    provide::{Provide, ProvideMut, ProvideRef},
    with::With,
};

mod provide;
mod with;
