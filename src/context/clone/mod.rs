//! Context closely related to dependency [cloning](core::clone).

pub use self::{
    owned::{CloneDependency, CloneDependencyWith},
    r#mut::{CloneDependencyMut, CloneDependencyMutWith},
    r#ref::{CloneDependencyRef, CloneDependencyRefWith},
};

mod r#mut;
mod owned;
mod r#ref;
