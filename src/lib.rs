//! Truly zero cost dependency injection — in safe and stable Rust.
//!
//! This crate defines some key concepts:
//! - **providers** are types which provide some dependency by value, shared or unique reference
//! - **context** types represent different ways to provide some dependency
//! - **injectors** are types which create requested values from contained dependencies

// TODO crate documentation

#![warn(clippy::all)]
#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![no_std]

pub use self::{
    provide::{Provide, ProvideMut, ProvideRef},
    with::{ProvideWith, ProvideWithMut, ProvideWithRef, With},
};

mod provide;
mod with;
