//! Context closely related to dependency cloning.

pub use self::{
    owned::{CloneOwned, CloneOwnedWith},
    r#mut::{CloneMut, CloneMutWith},
    r#ref::{CloneRef, CloneRefWith},
};

mod r#mut;
mod owned;
mod r#ref;
