//! Types of context used to represent different ways to provide some dependency.
//!
//! See [crate] documentation for more.

pub use self::context::Context;

pub mod clone;
pub mod convert;

mod context;

/// Context which represents no meaningful context.
pub type Empty = ();
