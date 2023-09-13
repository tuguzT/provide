use core::{
    borrow::{Borrow, BorrowMut},
    fmt::Debug,
    hash::Hash,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::with::With;

/// Context which allows to provide dependency by *creating it from*
/// another dependency by *unique reference*.
///
/// This is possible if:
/// - pointer to another dependency of type `D` implements [`Into`]`<T>`,
/// - provider implements [`ProvideMut`](crate::ProvideMut)`<D>`,
///
/// where `T` is the type of dependency to provide.
pub struct FromDependencyMut<D>(PhantomData<fn() -> D>)
where
    D: ?Sized;

impl<D> FromDependencyMut<D>
where
    D: ?Sized,
{
    /// Creates new from dependency context.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::context::convert::FromDependencyMut;
    ///
    /// todo!()
    /// ```
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<D> Debug for FromDependencyMut<D>
where
    D: ?Sized,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let type_name = core::any::type_name::<D>();
        write!(f, "FromDependencyMut<{type_name}>")
    }
}

impl<D> Default for FromDependencyMut<D>
where
    D: ?Sized,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<D> Clone for FromDependencyMut<D>
where
    D: ?Sized,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<D> Copy for FromDependencyMut<D> where D: ?Sized {}

impl<D> PartialEq for FromDependencyMut<D>
where
    D: ?Sized,
{
    fn eq(&self, other: &Self) -> bool {
        let Self(this) = self;
        let Self(other) = other;
        this == other
    }
}

impl<D> Eq for FromDependencyMut<D> where D: ?Sized {}

impl<D> PartialOrd for FromDependencyMut<D>
where
    D: ?Sized,
{
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        let Self(this) = self;
        let Self(other) = other;
        this.partial_cmp(other)
    }
}

impl<D> Ord for FromDependencyMut<D>
where
    D: ?Sized,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let Self(this) = self;
        let Self(other) = other;
        this.cmp(other)
    }
}

impl<D> Hash for FromDependencyMut<D>
where
    D: ?Sized,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let Self(this) = self;
        this.hash(state)
    }
}

/// Attach additional context to the current context.
impl<D, C> With<C> for FromDependencyMut<D>
where
    D: ?Sized,
{
    type Output = FromDependencyMutWith<D, C>;

    /// Attaches additional context to the current context.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::context::convert::{
    ///     FromDependencyMut,
    ///     FromDependencyMutWith,
    /// };
    ///
    /// todo!()
    /// ```
    fn with(self, context: C) -> Self::Output {
        context.into()
    }
}

/// Context which allows to provide dependency by *creating it from*
/// another dependency by *unique reference*
/// which could be provided with additional context.
///
/// This is possible if:
/// - pointer to another dependency of type `D` implements [`Into`]`<T>`,
/// - provider implements [`ProvideMutWith`](crate::with::ProvideMutWith)`<D, C>`,
///
/// where `T` is the type of dependency to provide.
pub struct FromDependencyMutWith<D, C>
where
    D: ?Sized,
    C: ?Sized,
{
    phantom: PhantomData<fn() -> D>,
    /// Inner context of the current context.
    pub context: C,
}

impl<D, C> FromDependencyMutWith<D, C>
where
    D: ?Sized,
{
    /// Creates self from provided context.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::context::convert::FromDependencyMutWith;
    ///
    /// todo!()
    /// ```
    pub const fn new(context: C) -> Self {
        let phantom = PhantomData;
        Self { phantom, context }
    }

    /// Returns inner context, consuming self.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::context::convert::FromDependencyMutWith;
    ///
    /// todo!()
    /// ```
    pub fn into_inner(self) -> C {
        let Self { context, .. } = self;
        context
    }
}

impl<D, C> From<C> for FromDependencyMutWith<D, C>
where
    D: ?Sized,
{
    fn from(context: C) -> Self {
        Self::new(context)
    }
}

impl<D, C> Debug for FromDependencyMutWith<D, C>
where
    D: ?Sized,
    C: Debug + ?Sized,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self { context, .. } = self;
        let type_name = core::any::type_name::<D>();
        write!(f, "FromDependencyMutWith<{type_name}>({context:?})")
    }
}

impl<D, C> Default for FromDependencyMutWith<D, C>
where
    D: ?Sized,
    C: Default,
{
    fn default() -> Self {
        let context = Default::default();
        Self::new(context)
    }
}

impl<D, C> Clone for FromDependencyMutWith<D, C>
where
    D: ?Sized,
    C: Clone,
{
    fn clone(&self) -> Self {
        let Self { context, .. } = self;
        let context = context.clone();
        Self::new(context)
    }
}

impl<D, C> Copy for FromDependencyMutWith<D, C>
where
    D: ?Sized,
    C: Copy,
{
}

impl<D, C> PartialEq for FromDependencyMutWith<D, C>
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

impl<D, C> Eq for FromDependencyMutWith<D, C>
where
    D: ?Sized,
    C: Eq + ?Sized,
{
}

impl<D, C> PartialOrd for FromDependencyMutWith<D, C>
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

impl<D, C> Ord for FromDependencyMutWith<D, C>
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

impl<D, C> Hash for FromDependencyMutWith<D, C>
where
    D: ?Sized,
    C: Hash + ?Sized,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        let Self { context, .. } = self;
        context.hash(state)
    }
}

impl<D, C> Deref for FromDependencyMutWith<D, C>
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

impl<D, C> DerefMut for FromDependencyMutWith<D, C>
where
    D: ?Sized,
    C: ?Sized,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        let Self { context, .. } = self;
        context
    }
}

impl<D, C, T> AsRef<T> for FromDependencyMutWith<D, C>
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

impl<D, C, T> AsMut<T> for FromDependencyMutWith<D, C>
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

impl<D, C> Borrow<C> for FromDependencyMutWith<D, C>
where
    D: ?Sized,
    C: ?Sized,
{
    fn borrow(&self) -> &C {
        self.deref()
    }
}

impl<D, C> BorrowMut<C> for FromDependencyMutWith<D, C>
where
    D: ?Sized,
    C: ?Sized,
{
    fn borrow_mut(&mut self) -> &mut C {
        self.deref_mut()
    }
}
