use core::{
    borrow::{Borrow, BorrowMut},
    fmt::Debug,
    hash::Hash,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::{
    context::Empty,
    with::{ProvideMutWith, ProvideRefWith, ProvideWith, With},
};

/// Context which allows to provide dependency by *cloning* from *shared reference*.
///
/// This is possible if:
/// - type of dependency to provide `T` implements [`Clone`],
/// - type of unique reference `D` implements [`Deref`]`<`[`Target`](Deref::Target)` = T>`,
/// - provider implements [`ProvideRef`](crate::ProvideRef)`<'_, D>`.
pub type CloneDependencyRef<D> = CloneDependencyRefWith<D, Empty>;

impl<D> CloneDependencyRef<D>
where
    D: ?Sized,
{
    /// Creates self with empty context.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::context::clone::CloneDependencyRef;
    ///
    /// todo!()
    /// ```
    pub const fn new() -> Self {
        Self::with(())
    }
}

/// Context which allows to provide dependency by *cloning* from *shared reference*
/// with additional context.
///
/// This is possible if:
/// - type of dependency to provide `T` implements [`Clone`],
/// - type of unique reference `D` implements [`Deref`]`<`[`Target`](Deref::Target)` = T>`,
/// - provider implements [`ProvideRefWith`](crate::with::ProvideRefWith)`<'_, D, C>`.
pub struct CloneDependencyRefWith<D, C>
where
    D: ?Sized,
    C: ?Sized,
{
    phantom: PhantomData<fn() -> D>,
    /// Inner context of the current context.
    pub context: C,
}

impl<D, C> CloneDependencyRefWith<D, C>
where
    D: ?Sized,
{
    /// Creates self with provided context.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::context::clone::CloneDependencyRefWith;
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
    /// use provide::context::clone::CloneDependencyRefWith;
    ///
    /// todo!()
    /// ```
    pub fn into_inner(self) -> C {
        let Self { context, .. } = self;
        context
    }
}

impl<D, C, T> With<T> for CloneDependencyRefWith<D, C>
where
    D: ?Sized,
    C: With<T>,
{
    type Output = CloneDependencyRefWith<D, C::Output>;

    /// Attaches additional context to the current context.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::context::clone::CloneDependencyRefWith;
    ///
    /// todo!()
    /// ```
    fn with(self, dependency: T) -> Self::Output {
        let context = self.into_inner();
        let context = context.with(dependency);
        context.into()
    }
}

impl<D, C> From<C> for CloneDependencyRefWith<D, C>
where
    D: ?Sized,
{
    fn from(context: C) -> Self {
        Self::with(context)
    }
}

impl<D, C> Debug for CloneDependencyRefWith<D, C>
where
    D: ?Sized,
    C: Debug + ?Sized,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self { context, .. } = self;
        let type_name = core::any::type_name::<D>();
        write!(f, "CloneDependencyRefWith<{type_name}>({context:?})")
    }
}

impl<D, C> Default for CloneDependencyRefWith<D, C>
where
    D: ?Sized,
    C: Default,
{
    fn default() -> Self {
        let context = Default::default();
        Self::with(context)
    }
}

impl<D, C> Clone for CloneDependencyRefWith<D, C>
where
    D: ?Sized,
    C: Clone,
{
    fn clone(&self) -> Self {
        let Self { context, .. } = self;
        let context = context.clone();
        Self::with(context)
    }
}

impl<D, C> Copy for CloneDependencyRefWith<D, C>
where
    D: ?Sized,
    C: Copy,
{
}

impl<D, C> PartialEq for CloneDependencyRefWith<D, C>
where
    D: ?Sized,
    C: PartialEq + ?Sized,
{
    fn eq(&self, other: &Self) -> bool {
        let Self { context: this, .. } = self;
        let Self { context: other, .. } = other;
        this == other
    }
}

impl<D, C> Eq for CloneDependencyRefWith<D, C>
where
    D: ?Sized,
    C: Eq + ?Sized,
{
}

impl<D, C> PartialOrd for CloneDependencyRefWith<D, C>
where
    D: ?Sized,
    C: PartialOrd + ?Sized,
{
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        let Self { context: this, .. } = self;
        let Self { context: other, .. } = other;
        this.partial_cmp(other)
    }
}

impl<D, C> Ord for CloneDependencyRefWith<D, C>
where
    D: ?Sized,
    C: Ord + ?Sized,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let Self { context: this, .. } = self;
        let Self { context: other, .. } = other;
        this.cmp(other)
    }
}

impl<D, C> Hash for CloneDependencyRefWith<D, C>
where
    D: ?Sized,
    C: Hash + ?Sized,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let Self { context, .. } = self;
        context.hash(state)
    }
}

impl<D, C> Deref for CloneDependencyRefWith<D, C>
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

impl<D, C> DerefMut for CloneDependencyRefWith<D, C>
where
    D: ?Sized,
    C: ?Sized,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        let Self { context, .. } = self;
        context
    }
}

impl<D, C, T> AsRef<T> for CloneDependencyRefWith<D, C>
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

impl<D, C, T> AsMut<T> for CloneDependencyRefWith<D, C>
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

impl<D, C> Borrow<C> for CloneDependencyRefWith<D, C>
where
    D: ?Sized,
    C: ?Sized,
{
    fn borrow(&self) -> &C {
        self.deref()
    }
}

impl<D, C> BorrowMut<C> for CloneDependencyRefWith<D, C>
where
    D: ?Sized,
    C: ?Sized,
{
    fn borrow_mut(&mut self) -> &mut C {
        self.deref_mut()
    }
}

impl<T, U, D, C> ProvideWith<T, CloneDependencyRefWith<D, C>> for U
where
    T: Clone,
    U: for<'any> ProvideRefWith<'any, D, C>,
    D: Deref<Target = T>,
{
    type Remainder = U;

    fn provide_with(self, context: CloneDependencyRefWith<D, C>) -> (T, Self::Remainder) {
        let context = context.into_inner();
        let dependency = self.provide_ref_with(context).clone();
        (dependency, self)
    }
}

impl<'me, T, U, D, C> ProvideRefWith<'me, T, CloneDependencyRefWith<D, C>> for U
where
    T: Clone,
    U: ProvideRefWith<'me, D, C> + ?Sized,
    D: Deref<Target = T>,
{
    fn provide_ref_with(&'me self, context: CloneDependencyRefWith<D, C>) -> T {
        let context = context.into_inner();
        let dependency = self.provide_ref_with(context);
        dependency.clone()
    }
}

impl<'me, T, U, D, C> ProvideMutWith<'me, T, CloneDependencyRefWith<D, C>> for U
where
    T: Clone,
    U: ProvideRefWith<'me, D, C> + ?Sized,
    D: Deref<Target = T>,
{
    fn provide_mut_with(&'me mut self, context: CloneDependencyRefWith<D, C>) -> T {
        let context = context.into_inner();
        let dependency = (*self).provide_ref_with(context);
        dependency.clone()
    }
}
