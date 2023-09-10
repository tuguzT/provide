//! Define different ways to provide some dependency.
//!
//! See [crate] documentation for more.

use core::ops::{Deref, DerefMut};

use crate::{context::Empty, Provide, ProvideMut, ProvideRef};

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

/// Type of provider which provides dependency by *shared reference*,
/// but with additional context provided by the caller.
///
/// This trait is very similar to [`ProvideRef`] trait.
/// However, this trait allows to retrieve additional context provided by the caller,
/// so it is possible to *define many ways* of how dependency can be provided.
///
/// See [crate] documentation for more.
pub trait ProvideWithRef<T, C>
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
    /// use provide::with::ProvideWithRef;
    ///
    /// todo!()
    /// ```
    fn provide_with_ref(&self, context: C) -> Self::Ref<'_>;
}

impl<T, U> ProvideWithRef<T, Empty> for U
where
    T: ?Sized,
    U: ProvideRef<T> + ?Sized,
{
    type Ref<'me> = U::Ref<'me>
    where
        Self: 'me,
        T: 'me;

    fn provide_with_ref(&self, _: Empty) -> Self::Ref<'_> {
        self.provide_ref()
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
pub trait ProvideWithMut<T, C>
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
    /// use provide::with::ProvideWithMut;
    ///
    /// todo!()
    /// ```
    fn provide_with_mut(&mut self, context: C) -> Self::Mut<'_>;
}

impl<T, U> ProvideWithMut<T, Empty> for U
where
    T: ?Sized,
    U: ProvideMut<T> + ?Sized,
{
    type Mut<'me> = U::Mut<'me>
    where
        Self: 'me,
        T: 'me;

    fn provide_with_mut(&mut self, _: Empty) -> Self::Mut<'_> {
        self.provide_mut()
    }
}
