use core::{
    borrow::{Borrow, BorrowMut},
    ops::{Deref, DerefMut},
};

use crate::with::With;

/// Context which allows to provide dependency by *cloning* from *unique reference*.
///
/// This is possible if:
/// - type of dependency `T` implements [`Clone`],
/// - provider implements [`ProvideMut`](crate::ProvideMut)`<T>`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CloneDependencyMut;

/// Attach additional context to the current context.
impl<T> With<T> for CloneDependencyMut {
    type Output = CloneDependencyMutWith<T>;

    /// Attaches additional context to the current context.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::context::clone::{
    ///     CloneDependencyMut,
    ///     CloneDependencyMutWith,
    /// };
    ///
    /// todo!()
    /// ```
    fn with(self, context: T) -> Self::Output {
        context.into()
    }
}

/// Context which allows to provide dependency by *cloning* from *unique reference*
/// which could be provided with additional context.
///
/// This is possible if:
/// - type of dependency `T` implements [`Clone`],
/// - provider implements [`ProvideMutWith`](crate::with::ProvideMutWith)`<T, C>`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CloneDependencyMutWith<C>(pub C)
where
    C: ?Sized;

impl<C> CloneDependencyMutWith<C> {
    /// Creates self from provided context.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::context::clone::CloneDependencyMutWith;
    ///
    /// todo!()
    /// ```
    pub const fn new(context: C) -> Self {
        Self(context)
    }

    /// Returns inner context, consuming self.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::context::clone::CloneDependencyMutWith;
    ///
    /// todo!()
    /// ```
    pub fn into_inner(self) -> C {
        let Self(context) = self;
        context
    }
}

impl<C> From<C> for CloneDependencyMutWith<C> {
    fn from(context: C) -> Self {
        Self::new(context)
    }
}

impl<C> Deref for CloneDependencyMutWith<C>
where
    C: ?Sized,
{
    type Target = C;

    fn deref(&self) -> &Self::Target {
        let Self(context) = self;
        context
    }
}

impl<C> DerefMut for CloneDependencyMutWith<C>
where
    C: ?Sized,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        let Self(context) = self;
        context
    }
}

impl<C, T> AsRef<T> for CloneDependencyMutWith<C>
where
    C: ?Sized,
    T: ?Sized,
    <Self as Deref>::Target: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        self.deref().as_ref()
    }
}

impl<C, T> AsMut<T> for CloneDependencyMutWith<C>
where
    C: ?Sized,
    T: ?Sized,
    <Self as Deref>::Target: AsMut<T>,
{
    fn as_mut(&mut self) -> &mut T {
        self.deref_mut().as_mut()
    }
}

impl<C> Borrow<C> for CloneDependencyMutWith<C>
where
    C: ?Sized,
{
    fn borrow(&self) -> &C {
        self.deref()
    }
}

impl<C> BorrowMut<C> for CloneDependencyMutWith<C>
where
    C: ?Sized,
{
    fn borrow_mut(&mut self) -> &mut C {
        self.deref_mut()
    }
}
