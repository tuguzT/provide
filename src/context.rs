//! Types of context used to represent different ways to provide some dependency.
//!
//! See [crate] documentation for more.

use core::{
    borrow::{Borrow, BorrowMut},
    ops::{Deref, DerefMut},
};

use crate::with::With;

/// Context which represents no meaningful context.
pub type Empty = ();

/// Context which allows to provide dependency by *cloning*
/// if type of dependency implements [`Clone`]
/// and provider implements [`Provide`](crate::Provide)`<_>`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CloneOwned;

impl<T> With<T> for CloneOwned {
    type Output = CloneOwnedWith<T>;

    fn with(self, dependency: T) -> Self::Output {
        dependency.into()
    }
}

/// Context which allows to provide dependency by *cloning*
/// if type of dependency implements [`Clone`]
/// and provider implements [`ProvideWith`](crate::with::ProvideWith)`<_, C>`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CloneOwnedWith<C>(pub C)
where
    C: ?Sized;

impl<C> CloneOwnedWith<C> {
    /// Creates self from provided context.
    pub const fn new(context: C) -> Self {
        Self(context)
    }

    /// Returns inner context, consuming self.
    pub fn into_inner(self) -> C {
        let Self(context) = self;
        context
    }
}

impl<C> From<C> for CloneOwnedWith<C> {
    fn from(context: C) -> Self {
        Self::new(context)
    }
}

impl<C> Deref for CloneOwnedWith<C>
where
    C: ?Sized,
{
    type Target = C;

    fn deref(&self) -> &Self::Target {
        let Self(context) = self;
        context
    }
}

impl<C> DerefMut for CloneOwnedWith<C>
where
    C: ?Sized,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        let Self(context) = self;
        context
    }
}

impl<C, T> AsRef<T> for CloneOwnedWith<C>
where
    C: ?Sized,
    T: ?Sized,
    <Self as Deref>::Target: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        self.deref().as_ref()
    }
}

impl<C, T> AsMut<T> for CloneOwnedWith<C>
where
    C: ?Sized,
    T: ?Sized,
    <Self as Deref>::Target: AsMut<T>,
{
    fn as_mut(&mut self) -> &mut T {
        self.deref_mut().as_mut()
    }
}

impl<T> Borrow<T> for CloneOwnedWith<T>
where
    T: ?Sized,
{
    fn borrow(&self) -> &T {
        self.deref()
    }
}

impl<T> BorrowMut<T> for CloneOwnedWith<T>
where
    T: ?Sized,
{
    fn borrow_mut(&mut self) -> &mut T {
        self.deref_mut()
    }
}

/// Context which allows to provide dependency by *cloning*
/// if type of dependency implements [`Clone`]
/// and provider implements [`ProvideRef`](crate::ProvideRef)`<_>`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CloneRef;

impl<T> With<T> for CloneRef {
    type Output = CloneRefWith<T>;

    fn with(self, dependency: T) -> Self::Output {
        dependency.into()
    }
}

/// Context which allows to provide dependency by *cloning*
/// if type of dependency implements [`Clone`]
/// and provider implements [`ProvideRefWith`](crate::with::ProvideRefWith)`<_, C>`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CloneRefWith<C>(pub C)
where
    C: ?Sized;

impl<C> CloneRefWith<C> {
    /// Creates self from provided context.
    pub const fn new(context: C) -> Self {
        Self(context)
    }

    /// Returns inner context, consuming self.
    pub fn into_inner(self) -> C {
        let Self(context) = self;
        context
    }
}

impl<C> From<C> for CloneRefWith<C> {
    fn from(context: C) -> Self {
        Self::new(context)
    }
}

impl<C> Deref for CloneRefWith<C>
where
    C: ?Sized,
{
    type Target = C;

    fn deref(&self) -> &Self::Target {
        let Self(context) = self;
        context
    }
}

impl<C> DerefMut for CloneRefWith<C>
where
    C: ?Sized,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        let Self(context) = self;
        context
    }
}

impl<C, T> AsRef<T> for CloneRefWith<C>
where
    C: ?Sized,
    T: ?Sized,
    <Self as Deref>::Target: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        self.deref().as_ref()
    }
}

impl<C, T> AsMut<T> for CloneRefWith<C>
where
    C: ?Sized,
    T: ?Sized,
    <Self as Deref>::Target: AsMut<T>,
{
    fn as_mut(&mut self) -> &mut T {
        self.deref_mut().as_mut()
    }
}

impl<T> Borrow<T> for CloneRefWith<T>
where
    T: ?Sized,
{
    fn borrow(&self) -> &T {
        self.deref()
    }
}

impl<T> BorrowMut<T> for CloneRefWith<T>
where
    T: ?Sized,
{
    fn borrow_mut(&mut self) -> &mut T {
        self.deref_mut()
    }
}

// TODO `CloneMut`, `CloneMutWith<C>`
