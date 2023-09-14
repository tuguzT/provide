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
    U: AsMut<T> + ?Sized,
{
    fn provide_mut(&'me mut self) -> &'me mut T {
        self.as_mut()
    }
}
