use core::convert::Infallible;

/// Type of provider which provides dependency by *unique reference*.
///
/// This trait can be interpreted as an extension of [`AsMut`] trait
/// but with the ability to return not only plain unique references.
///
/// See [crate] documentation for more.
pub trait ProvideMut<'me, T> {
    /// Provides dependency by *unique reference*.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::ProvideMut;
    ///
    /// todo!()
    /// ```
    fn provide_mut(&'me mut self) -> T;
}

impl<'me, T, U> ProvideMut<'me, &'me mut T> for U
where
    T: ?Sized,
    U: AsMut<T> + ?Sized,
{
    fn provide_mut(&'me mut self) -> &'me mut T {
        self.as_mut()
    }
}

/// Type of provider which can provide dependency by *unique reference* or fail.
///
/// This trait can be interpreted as an extension of [`AsMut`] trait
/// but with the ability to return not only plain unique references.
///
/// See [crate] documentation for more.
pub trait TryProvideMut<'me, T> {
    /// The type returned in the event of an error.
    type Error;

    /// Tries to provide dependency by *unique reference*.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::ProvideMut;
    ///
    /// todo!()
    /// ```
    fn try_provide_mut(&'me mut self) -> Result<T, Self::Error>;
}

impl<'me, T, U> TryProvideMut<'me, T> for U
where
    U: ProvideMut<'me, T> + ?Sized,
{
    type Error = Infallible;

    fn try_provide_mut(&'me mut self) -> Result<T, Self::Error> {
        let provide_mut = self.provide_mut();
        Ok(provide_mut)
    }
}
