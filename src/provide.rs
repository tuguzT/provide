use core::ops::{Deref, DerefMut};

/// Type of provider which provides dependency *by **value***.
///
/// Type of dependency is determined from the generic type parameter `T`.
///
/// See [crate] documentation for more.
pub trait Provide<T> {
    /// Remaining part of the provider after providing dependency by value.
    type Remainder;

    /// Provides dependency *by **value***, also returning remaining part of the provider.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::Provide;
    ///
    /// todo!()
    /// ```
    fn provide(self) -> (T, Self::Remainder);
}

/// Type of provider which provides dependency *by **shared** reference*.
///
/// Type of dependency is determined from the generic type parameter `T`.
///
/// See [crate] documentation for more.
pub trait ProvideRef<T>
where
    T: ?Sized,
{
    /// Type of shared reference to provided dependency.
    type Ref<'me>: Deref<Target = T>
    where
        Self: 'me;

    /// Provides dependency *by **shared** reference*.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::ProvideRef;
    ///
    /// todo!()
    /// ```
    fn provide_ref(&self) -> Self::Ref<'_>;
}

/// Type of provider which provides dependency *by **unique** reference*.
///
/// Type of dependency is determined from the generic type parameter `T`.
///
/// See [crate] documentation for more.
pub trait ProvideMut<T>
where
    T: ?Sized,
{
    /// Type of unique reference to provided dependency.
    type Mut<'me>: DerefMut<Target = T>
    where
        Self: 'me;

    /// Provides dependency *by **unique** reference*.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::ProvideMut;
    ///
    /// todo!()
    /// ```
    fn provide_mut(&mut self) -> Self::Mut<'_>;
}
