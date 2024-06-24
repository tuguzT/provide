//! Define different ways to provide some dependency.
//!
//! See [crate] documentation for more.

pub use self::{
    provide::{
        ProvideMutWith, ProvideRefWith, ProvideWith, TryProvideMutWith, TryProvideRefWith,
        TryProvideWith,
    },
    with::With,
};

mod provide;
mod with;
