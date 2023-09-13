//! Context closely related to dependency [type conversions](core::convert).

pub use self::{
    owned::{FromDependency, FromDependencyWith},
    r#mut::{FromDependencyMut, FromDependencyMutWith},
    r#ref::{FromDependencyRef, FromDependencyRefWith},
};

mod r#mut;
mod owned;
mod r#ref;
