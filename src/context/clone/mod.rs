//! Context closely related to dependency [cloning](Clone).

pub use self::{
    owned::{CloneOwned, CloneOwnedWith},
    r#mut::{CloneMut, CloneMutWith},
    r#ref::{CloneRef, CloneRefWith},
};

mod r#mut;
mod owned;
mod r#ref;
