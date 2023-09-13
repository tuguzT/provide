use core::ops::DerefMut;

/// Type of provider which provides dependency by *unique reference*.
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

    /// Provides dependency by *unique reference*.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::ProvideMut;
    ///
    /// todo!()
    /// ```
    fn provide_mut<'me>(&'me mut self) -> Self::Mut<'me>
    where
        T: 'me;
}

impl<T, U> ProvideMut<T> for U
where
    T: ?Sized,
    U: AsMut<T> + ?Sized,
{
    type Mut<'me> = &'me mut T
    where
        Self: 'me,
        T: 'me;

    fn provide_mut<'me>(&'me mut self) -> Self::Mut<'me>
    where
        T: 'me,
    {
        self.as_mut()
    }
}
