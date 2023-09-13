//! Define different ways to provide some dependency.
//!
//! See [crate] documentation for more.

use core::ops::{Deref, DerefMut};

use crate::{
    context::{CloneOwned, CloneOwnedWith, CloneRef, CloneRefWith, Empty},
    deref::DerefWrapper,
    Provide, ProvideMut, ProvideRef,
};

/// Type of provider which can be created from provided dependency.
///
/// This trait can be used to emulate extension of self type with dependency type,
/// where the [output](With::Output) is product type consisting of self and provided dependency.
///
/// See [crate] documentation for more.
pub trait With<T> {
    /// Type of new provider with provided dependency.
    type Output;

    /// Creates new provider from the self and provided dependency.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::with::With;
    ///
    /// todo!()
    /// ```
    fn with(self, dependency: T) -> Self::Output;
}

impl<T> With<T> for () {
    type Output = T;

    fn with(self, dependency: T) -> Self::Output {
        dependency
    }
}

/// Type of provider which provides dependency by *value*,
/// but with additional context provided by the caller.
///
/// This trait is very similar to [`Provide`] trait.
/// However, this trait allows to retrieve additional context provided by the caller,
/// so it is possible to *define many ways* of how dependency can be provided.
///
/// See [crate] documentation for more.
pub trait ProvideWith<T, C> {
    /// Remaining part of the provider after providing dependency by value.
    type Remainder;

    /// Provides dependency by *value* with additional context provided by the caller,
    /// also returning [remaining part](ProvideWith::Remainder) of the provider.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::with::ProvideWith;
    ///
    /// todo!()
    /// ```
    fn provide_with(self, context: C) -> (T, Self::Remainder);
}

impl<T, U> ProvideWith<T, Empty> for U
where
    U: Provide<T>,
{
    type Remainder = U::Remainder;

    fn provide_with(self, _: Empty) -> (T, Self::Remainder) {
        self.provide()
    }
}

impl<T, U> ProvideWith<T, CloneOwned> for U
where
    T: Clone,
    U: Provide<T>,
    U::Remainder: With<T>,
{
    type Remainder = <U::Remainder as With<T>>::Output;

    fn provide_with(self, _: CloneOwned) -> (T, Self::Remainder) {
        let (dependency, remainder) = self.provide();
        let remainder = remainder.with(dependency.clone());
        (dependency, remainder)
    }
}

impl<T, U, C> ProvideWith<T, CloneOwnedWith<C>> for U
where
    T: Clone,
    U: ProvideWith<T, C>,
    U::Remainder: With<T>,
{
    type Remainder = <U::Remainder as With<T>>::Output;

    fn provide_with(self, context: CloneOwnedWith<C>) -> (T, Self::Remainder) {
        let context = context.into_inner();
        let (dependency, remainder) = self.provide_with(context);
        let remainder = remainder.with(dependency.clone());
        (dependency, remainder)
    }
}

impl<T, U> ProvideWith<T, CloneRef> for U
where
    T: Clone,
    U: ProvideRef<T>,
{
    type Remainder = U;

    fn provide_with(self, _: CloneRef) -> (T, Self::Remainder) {
        let dependency = self.provide_ref().clone();
        (dependency, self)
    }
}

impl<T, U, C> ProvideWith<T, CloneRefWith<C>> for U
where
    T: Clone,
    U: ProvideRefWith<T, C>,
{
    type Remainder = U;

    fn provide_with(self, context: CloneRefWith<C>) -> (T, Self::Remainder) {
        let context = context.into_inner();
        let dependency = self.provide_ref_with(context).clone();
        (dependency, self)
    }
}

/// Type of provider which provides dependency by *shared reference*,
/// but with additional context provided by the caller.
///
/// This trait is very similar to [`ProvideRef`] trait.
/// However, this trait allows to retrieve additional context provided by the caller,
/// so it is possible to *define many ways* of how dependency can be provided.
///
/// See [crate] documentation for more.
pub trait ProvideRefWith<T, C>
where
    T: ?Sized,
{
    /// Type of shared reference to provided dependency.
    type Ref<'me>: Deref<Target = T>
    where
        Self: 'me,
        T: 'me;

    /// Provides dependency by *shared reference*
    /// with additional context provided by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::with::ProvideRefWith;
    ///
    /// todo!()
    /// ```
    fn provide_ref_with<'me>(&'me self, context: C) -> Self::Ref<'me>
    where
        T: 'me;
}

impl<T, U> ProvideRefWith<T, Empty> for U
where
    T: ?Sized,
    U: ProvideRef<T> + ?Sized,
{
    type Ref<'me> = U::Ref<'me>
    where
        Self: 'me,
        T: 'me;

    fn provide_ref_with<'me>(&'me self, _: Empty) -> Self::Ref<'me>
    where
        T: 'me,
    {
        self.provide_ref()
    }
}

impl<T, U> ProvideRefWith<T, CloneRef> for U
where
    T: Clone,
    U: ProvideRef<T> + ?Sized,
{
    type Ref<'me> = DerefWrapper<T>
    where
        Self: 'me,
        T: 'me;

    fn provide_ref_with<'me>(&'me self, _: CloneRef) -> Self::Ref<'me>
    where
        T: 'me,
    {
        let dependency = self.provide_ref().clone();
        dependency.into()
    }
}

impl<T, U, C> ProvideRefWith<T, CloneRefWith<C>> for U
where
    T: Clone,
    U: ProvideRefWith<T, C> + ?Sized,
{
    type Ref<'me> = DerefWrapper<T>
    where
        Self: 'me,
        T: 'me;

    fn provide_ref_with<'me>(&'me self, context: CloneRefWith<C>) -> Self::Ref<'me>
    where
        T: 'me,
    {
        let context = context.into_inner();
        let dependency = self.provide_ref_with(context).clone();
        dependency.into()
    }
}

/// Type of provider which provides dependency by *unique reference*,
/// but with additional context provided by the caller.
///
/// This trait is very similar to [`ProvideMut`] trait.
/// However, this trait allows to retrieve additional context provided by the caller,
/// so it is possible to *define many ways* of how dependency can be provided.
///
/// See [crate] documentation for more.
pub trait ProvideMutWith<T, C>
where
    T: ?Sized,
{
    /// Type of unique reference to provided dependency.
    type Mut<'me>: DerefMut<Target = T>
    where
        Self: 'me,
        T: 'me;

    /// Provides dependency by *unique reference*
    /// with additional context provided by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::with::ProvideMutWith;
    ///
    /// todo!()
    /// ```
    fn provide_mut_with<'me>(&'me mut self, context: C) -> Self::Mut<'me>
    where
        T: 'me;
}

impl<T, U> ProvideMutWith<T, Empty> for U
where
    T: ?Sized,
    U: ProvideMut<T> + ?Sized,
{
    type Mut<'me> = U::Mut<'me>
    where
        Self: 'me,
        T: 'me;

    fn provide_mut_with<'me>(&'me mut self, _: Empty) -> Self::Mut<'me>
    where
        T: 'me,
    {
        self.provide_mut()
    }
}

impl<T, U> ProvideMutWith<T, CloneRef> for U
where
    T: Clone,
    U: ProvideRef<T> + ?Sized,
{
    type Mut<'me> = DerefWrapper<T>
    where
        Self: 'me,
        T: 'me;

    fn provide_mut_with<'me>(&'me mut self, _: CloneRef) -> Self::Mut<'me>
    where
        T: 'me,
    {
        let dependency = self.provide_ref().clone();
        dependency.into()
    }
}

impl<T, U, C> ProvideMutWith<T, CloneRefWith<C>> for U
where
    T: Clone,
    U: ProvideRefWith<T, C> + ?Sized,
{
    type Mut<'me> = DerefWrapper<T>
    where
        Self: 'me,
        T: 'me;

    fn provide_mut_with<'me>(&'me mut self, context: CloneRefWith<C>) -> Self::Mut<'me>
    where
        T: 'me,
    {
        let context = context.into_inner();
        let dependency = (*self).provide_ref_with(context).clone();
        dependency.into()
    }
}
