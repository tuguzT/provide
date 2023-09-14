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
