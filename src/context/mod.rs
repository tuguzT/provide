//! Types of context used to represent different ways to provide some dependency.
//!
//! See [crate] documentation for more.

pub mod clone;

/// Context which represents no meaningful context.
pub type Empty = ();