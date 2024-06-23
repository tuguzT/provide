use core::{
    borrow::{Borrow, BorrowMut},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::{context::Empty, with::With};

/// Context which allows to provide dependency by *cloning* a *value*.
///
/// This is possible if:
/// - type of dependency `T` implements [`Clone`],
/// - provider implements [`Provide`](crate::Provide)`<T>`.
pub type CloneDependency<D> = CloneDependencyWith<D, Empty>;

impl<D> CloneDependency<D>
where
    D: ?Sized,
{
    /// Creates self with empty context.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::context::clone::CloneDependency;
    ///
    /// todo!()
    /// ```
    pub const fn new() -> Self {
        Self::with(())
    }
}

/// Context which allows to provide dependency by *cloning* a *value*
/// with additional context.
///
/// This is possible if:
/// - type of dependency `T` implements [`Clone`],
/// - provider implements [`ProvideWith`](crate::with::ProvideWith)`<T, C>`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CloneDependencyWith<D, C>
where
    D: ?Sized,
    C: ?Sized,
{
    phantom: PhantomData<fn() -> D>,
    /// Inner context of the current context.
    pub context: C,
}

impl<D, C> CloneDependencyWith<D, C>
where
    D: ?Sized,
{
    /// Creates self with provided context.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::context::clone::CloneDependencyWith;
    ///
    /// todo!()
    /// ```
    pub const fn with(context: C) -> Self {
        let phantom = PhantomData;
        Self { phantom, context }
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
        let Self { context, .. } = self;
        context
    }
}

impl<D, C, T> With<T> for CloneDependencyWith<D, C>
where
    D: ?Sized,
    C: With<T>,
{
    type Output = CloneDependencyWith<D, C::Output>;

    /// Attaches additional context to the current context.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::context::clone::CloneDependencyWith;
    ///
    /// todo!()
    /// ```
    fn with(self, dependency: T) -> Self::Output {
        let context = self.into_inner();
        let context = context.with(dependency);
        context.into()
    }
}

impl<D, C> From<C> for CloneDependencyWith<D, C>
where
    D: ?Sized,
{
    fn from(context: C) -> Self {
        Self::with(context)
    }
}

impl<D, C> Deref for CloneDependencyWith<D, C>
where
    D: ?Sized,
    C: ?Sized,
{
    type Target = C;

    fn deref(&self) -> &Self::Target {
        let Self { context, .. } = self;
        context
    }
}

impl<D, C> DerefMut for CloneDependencyWith<D, C>
where
    D: ?Sized,
    C: ?Sized,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        let Self { context, .. } = self;
        context
    }
}

impl<D, C, T> AsRef<T> for CloneDependencyWith<D, C>
where
    D: ?Sized,
    C: ?Sized,
    T: ?Sized,
    <Self as Deref>::Target: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        self.deref().as_ref()
    }
}

impl<D, C, T> AsMut<T> for CloneDependencyWith<D, C>
where
    D: ?Sized,
    C: ?Sized,
    T: ?Sized,
    <Self as Deref>::Target: AsMut<T>,
{
    fn as_mut(&mut self) -> &mut T {
        self.deref_mut().as_mut()
    }
}

impl<D, C> Borrow<C> for CloneDependencyWith<D, C>
where
    D: ?Sized,
    C: ?Sized,
{
    fn borrow(&self) -> &C {
        self.deref()
    }
}

impl<D, C> BorrowMut<C> for CloneDependencyWith<D, C>
where
    D: ?Sized,
    C: ?Sized,
{
    fn borrow_mut(&mut self) -> &mut C {
        self.deref_mut()
    }
}
