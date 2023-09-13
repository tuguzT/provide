use core::{
    borrow::{Borrow, BorrowMut},
    ops::{Deref, DerefMut},
};

use crate::with::With;

/// Context which allows to provide dependency by *cloning* a *value*.
///
/// This is possible if:
/// - type of dependency `T` implements [`Clone`],
/// - provider implements [`Provide`](crate::Provide)`<T>`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CloneDependency;

/// Attach additional context to the current context.
impl<T> With<T> for CloneDependency {
    type Output = CloneDependencyWith<T>;

    /// Attaches additional context to the current context.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::context::clone::{
    ///     CloneDependency,
    ///     CloneDependencyWith,
    /// };
    ///
    /// todo!()
    /// ```
    fn with(self, context: T) -> Self::Output {
        context.into()
    }
}

/// Context which allows to provide dependency by *cloning* a *value*
/// which could be provided with additional context.
///
/// This is possible if:
/// - type of dependency `T` implements [`Clone`],
/// - provider implements [`ProvideWith`](crate::with::ProvideWith)`<T, C>`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CloneDependencyWith<C>(pub C)
where
    C: ?Sized;

impl<C> CloneDependencyWith<C> {
    /// Creates self from provided context.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::context::clone::CloneDependencyWith;
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
    /// use provide::context::clone::CloneDependencyWith;
    ///
    /// todo!()
    /// ```
    pub fn into_inner(self) -> C {
        let Self(context) = self;
        context
    }
}

impl<C> From<C> for CloneDependencyWith<C> {
    fn from(context: C) -> Self {
        Self::new(context)
    }
}

impl<C> Deref for CloneDependencyWith<C>
where
    C: ?Sized,
{
    type Target = C;

    fn deref(&self) -> &Self::Target {
        let Self(context) = self;
        context
    }
}

impl<C> DerefMut for CloneDependencyWith<C>
where
    C: ?Sized,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        let Self(context) = self;
        context
    }
}

impl<C, T> AsRef<T> for CloneDependencyWith<C>
where
    C: ?Sized,
    T: ?Sized,
    <Self as Deref>::Target: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        self.deref().as_ref()
    }
}

impl<C, T> AsMut<T> for CloneDependencyWith<C>
where
    C: ?Sized,
    T: ?Sized,
    <Self as Deref>::Target: AsMut<T>,
{
    fn as_mut(&mut self) -> &mut T {
        self.deref_mut().as_mut()
    }
}

impl<C> Borrow<C> for CloneDependencyWith<C>
where
    C: ?Sized,
{
    fn borrow(&self) -> &C {
        self.deref()
    }
}

impl<C> BorrowMut<C> for CloneDependencyWith<C>
where
    C: ?Sized,
{
    fn borrow_mut(&mut self) -> &mut C {
        self.deref_mut()
    }
}
