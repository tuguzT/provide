#![allow(clippy::module_inception)]

use super::{
    clone::{CloneDependencyMutWith, CloneDependencyRefWith, CloneDependencyWith},
    convert::{FromDependencyMutWith, FromDependencyRefWith, FromDependencyWith},
};

/// Extension trait for *context adaptors*.
///
/// If you want to define adaptors for your type of context,
/// you should make new extension trait which derives from the current one.
pub trait Context: Sized {
    /// Allows to clone dependency
    /// after it was provided by *value* with `self` context.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::context::Context;
    ///
    /// todo!()
    /// ```
    fn then_clone<D>(self) -> CloneDependencyWith<D, Self>
    where
        D: ?Sized,
    {
        self.into()
    }

    /// Allows to clone dependency
    /// after it was provided by *shared reference* with `self` context.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::context::Context;
    ///
    /// todo!()
    /// ```
    fn then_clone_ref<D>(self) -> CloneDependencyRefWith<D, Self>
    where
        D: ?Sized,
    {
        self.into()
    }

    /// Allows to clone dependency
    /// after it was provided by *unique reference* with `self` context.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::context::Context;
    ///
    /// todo!()
    /// ```
    fn then_clone_mut<D>(self) -> CloneDependencyMutWith<D, Self>
    where
        D: ?Sized,
    {
        self.into()
    }

    /// Allows to create dependency from another dependency
    /// after it was provided by *value* with `self` context.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::context::Context;
    ///
    /// todo!()
    /// ```
    fn then_from<D>(self) -> FromDependencyWith<D, Self>
    where
        D: ?Sized,
    {
        self.into()
    }

    /// Allows to create dependency from another dependency
    /// after it was provided by *shared reference* with `self` context.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::context::Context;
    ///
    /// todo!()
    /// ```
    fn then_from_ref<D>(self) -> FromDependencyRefWith<D, Self>
    where
        D: ?Sized,
    {
        self.into()
    }

    /// Allows to create dependency from another dependency
    /// after it was provided by *unique reference* with `self` context.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::context::Context;
    ///
    /// todo!()
    /// ```
    fn then_from_mut<D>(self) -> FromDependencyMutWith<D, Self>
    where
        D: ?Sized,
    {
        self.into()
    }
}

impl<T> Context for T {}
