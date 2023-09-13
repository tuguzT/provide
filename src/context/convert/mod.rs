//! Context closely related to dependency [type conversions](core::convert).

pub use self::{
    owned::{FromDependency, FromDependencyWith},
    r#ref::{FromDependencyRef, FromDependencyRefWith},
};

mod owned;
mod r#ref;
