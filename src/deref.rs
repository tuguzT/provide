//! Utilities for dealing with dereferencing.

use core::{
    borrow::{Borrow, BorrowMut},
    ops::{Deref, DerefMut},
};

/// Noop smart pointer which holds the value on which it points to.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DerefWrapper<T>(pub T)
where
    T: ?Sized;

impl<T> DerefWrapper<T> {
    /// Creates self from provided value.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::deref::DerefWrapper;
    ///
    /// todo!()
    /// ```
    pub const fn new(value: T) -> Self {
        Self(value)
    }

    /// Returns inner value, consuming self.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::deref::DerefWrapper;
    ///
    /// todo!()
    /// ```
    pub fn into_inner(self) -> T {
        let Self(value) = self;
        value
    }
}

impl<T> From<T> for DerefWrapper<T> {
    fn from(context: T) -> Self {
        Self::new(context)
    }
}

impl<T> Deref for DerefWrapper<T>
where
    T: ?Sized,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let Self(context) = self;
        context
    }
}

impl<T> DerefMut for DerefWrapper<T>
where
    T: ?Sized,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        let Self(context) = self;
        context
    }
}

impl<T, U> AsRef<U> for DerefWrapper<T>
where
    T: ?Sized,
    U: ?Sized,
    <Self as Deref>::Target: AsRef<U>,
{
    fn as_ref(&self) -> &U {
        self.deref().as_ref()
    }
}

impl<T, U> AsMut<U> for DerefWrapper<T>
where
    T: ?Sized,
    U: ?Sized,
    <Self as Deref>::Target: AsMut<U>,
{
    fn as_mut(&mut self) -> &mut U {
        self.deref_mut().as_mut()
    }
}

impl<T> Borrow<T> for DerefWrapper<T>
where
    T: ?Sized,
{
    fn borrow(&self) -> &T {
        self.deref()
    }
}

impl<T> BorrowMut<T> for DerefWrapper<T>
where
    T: ?Sized,
{
    fn borrow_mut(&mut self) -> &mut T {
        self.deref_mut()
    }
}
