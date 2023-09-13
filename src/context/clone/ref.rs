use core::{
    borrow::{Borrow, BorrowMut},
    ops::{Deref, DerefMut},
};

use crate::with::With;

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
