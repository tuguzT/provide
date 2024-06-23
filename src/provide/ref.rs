use core::convert::Infallible;

/// Type of provider which provides dependency by *shared reference*.
///
/// This trait can be interpreted as an extension of [`AsRef`] trait
/// but with the ability to return not only plain shared references.
///
/// See [crate] documentation for more.
pub trait ProvideRef<'me, T> {
    /// Provides dependency by *shared reference*.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::ProvideRef;
    ///
    /// todo!()
    /// ```
    fn provide_ref(&'me self) -> T;
}

impl<'me, T, U> ProvideRef<'me, &'me T> for U
where
    T: ?Sized,
    U: AsRef<T> + ?Sized,
{
    fn provide_ref(&'me self) -> &'me T {
        self.as_ref()
    }
}

/// Type of provider which can provide dependency by *shared reference* or fail.
///
/// This trait can be interpreted as an extension of [`AsRef`] trait
/// but with the ability to return not only plain shared references.
///
/// See [crate] documentation for more.
pub trait TryProvideRef<'me, T> {
    /// The type returned in the event of an error.
    type Error;

    /// Tries to provide dependency by *shared reference*.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::TryProvideRef;
    ///
    /// todo!()
    /// ```
    fn try_provide_ref(&'me self) -> Result<T, Self::Error>;
}

impl<'me, T, U> TryProvideRef<'me, T> for U
where
    U: ProvideRef<'me, T> + ?Sized,
{
    type Error = Infallible;

    fn try_provide_ref(&'me self) -> Result<T, Self::Error> {
        let provide_ref = self.provide_ref();
        Ok(provide_ref)
    }
}
