use core::ops::{Deref, DerefMut};

/// Type of provider which provides dependency *by **value***.
///
/// Type of dependency is determined from the generic type parameter `T`.
///
/// This trait can be interpreted as an extension of [`Into`] trait
/// but with the ability to return remaining part of the provider to be used later
/// or in chain to retrieve more dependencies.
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

impl<T, U> Provide<T> for U
where
    U: Into<T>,
{
    type Remainder = ();

    fn provide(self) -> (T, Self::Remainder) {
        let dependency = self.into();
        (dependency, ())
    }
}

/// Type of provider which provides dependency *by **shared** reference*.
///
/// Type of dependency is determined from the generic type parameter `T`.
///
/// This trait can be interpreted as an extension of [`AsRef`] trait
/// but with the ability to return not only plain shared references.
///
/// See [crate] documentation for more.
pub trait ProvideRef<T>
where
    T: ?Sized,
{
    /// Type of shared reference to provided dependency.
    type Ref<'me>: Deref<Target = T>
    where
        Self: 'me,
        T: 'me;

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

impl<T, U> ProvideRef<T> for U
where
    T: ?Sized,
    U: AsRef<T>,
{
    type Ref<'me> = &'me T
    where
        Self: 'me,
        T: 'me;

    fn provide_ref(&self) -> Self::Ref<'_> {
        self.as_ref()
    }
}

/// Type of provider which provides dependency *by **unique** reference*.
///
/// Type of dependency is determined from the generic type parameter `T`.
///
/// This trait can be interpreted as an extension of [`AsMut`] trait
/// but with the ability to return not only plain unique references.
///
/// See [crate] documentation for more.
pub trait ProvideMut<T>
where
    T: ?Sized,
{
    /// Type of unique reference to provided dependency.
    type Mut<'me>: DerefMut<Target = T>
    where
        Self: 'me,
        T: 'me;

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

impl<T, U> ProvideMut<T> for U
where
    T: ?Sized,
    U: AsMut<T>,
{
    type Mut<'me> = &'me mut T
    where
        Self: 'me,
        T: 'me;

    fn provide_mut(&mut self) -> Self::Mut<'_> {
        self.as_mut()
    }
}
