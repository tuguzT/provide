use core::ops::Deref;

/// Type of provider which provides dependency by *shared reference*.
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

    /// Provides dependency by *shared reference*.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::ProvideRef;
    ///
    /// todo!()
    /// ```
    fn provide_ref<'me>(&'me self) -> Self::Ref<'me>
    where
        T: 'me;
}

impl<T, U> ProvideRef<T> for U
where
    T: ?Sized,
    U: AsRef<T> + ?Sized,
{
    type Ref<'me> = &'me T
    where
        Self: 'me,
        T: 'me;

    fn provide_ref<'me>(&'me self) -> Self::Ref<'me>
    where
        T: 'me,
    {
        self.as_ref()
    }
}
