use core::convert::Infallible;

/// Type of provider which provides dependency by reference.
///
/// This trait can be interpreted as an extension of [`AsRef`] trait
/// but with the ability to return not only plain references.
///
/// See [crate] documentation for more.
pub trait ProvideRef<'me, T> {
    /// Provides dependency by reference.
    ///
    /// # Examples
    ///
    /// You can implement this trait for your provider
    /// to provide dependency by reference:
    ///
    /// ```
    /// use provide::ProvideRef;
    ///
    /// struct Provider {
    ///     foo: i32,
    ///     bar: f32,
    /// }
    ///
    /// impl ProvideRef<'_, i32> for Provider {
    ///     fn provide_ref(&self) -> i32 {
    ///         let Self { foo, .. } = self;
    ///         *foo
    ///     }
    /// }
    ///
    /// impl<'me> ProvideRef<'me, &'me f32> for Provider {
    ///     fn provide_ref(&'me self) -> &'me f32 {
    ///         let Self { bar, .. } = self;
    ///         bar
    ///     }
    /// }
    ///
    /// let provider = Provider { foo: 1, bar: 2.0 };
    ///
    /// let dependency: i32 = provider.provide_ref();
    /// assert_eq!(dependency, 1);
    ///
    /// let dependency: &f32 = provider.provide_ref();
    /// assert_eq!(dependency, &2.0);
    /// ```
    ///
    /// You can also provide your dependency via cheap reference-to-reference conversion
    /// thanks to implementation for all types which implement [`AsRef`]:
    ///
    /// ```
    /// use provide::ProvideRef;
    ///
    /// let provider = vec![1, 2, 3];
    /// let dependency = provider.provide_ref();
    /// assert_eq!(dependency, [1, 2, 3]);
    /// ```
    ///
    /// However, due to this blanket implementation, you cannot do this:
    ///
    /// ```compile_fail
    /// use provide::ProvideRef;
    ///
    /// struct GenericProvider<T>(T)
    /// where
    ///     T: ?Sized;
    ///
    /// impl<'me, T> ProvideRef<'me, &'me T> for GenericProvider<T>
    /// where
    ///     T: ?Sized,
    /// {
    ///     fn provide_ref(&'me self) -> &'me T {
    ///         let Self(dependency) = self;
    ///         dependency
    ///     }
    /// }
    /// ```
    ///
    /// Instead, consider using a newtype wrapper to avoid conflicting implementations:
    ///
    /// ```
    /// use provide::ProvideRef;
    ///
    /// struct GenericProvider<T>(T)
    /// where
    ///     T: ?Sized;
    ///
    /// struct Wrapper<T>(T)
    /// where
    ///     T: ?Sized;
    ///
    /// impl<'me, T> ProvideRef<'me, Wrapper<&'me T>> for GenericProvider<T>
    /// where
    ///     T: ?Sized,
    /// {
    ///     fn provide_ref(&'me self) -> Wrapper<&'me T> {
    ///         let Self(dependency) = self;
    ///         Wrapper(dependency)
    ///     }
    /// }
    ///
    /// let provider = GenericProvider(1);
    /// let Wrapper(dependency) = provider.provide_ref();
    /// assert_eq!(dependency, &1);
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

/// Type of provider which can provide dependency by reference or fail.
///
/// This trait can be interpreted as an extension of [`AsRef`] trait
/// but with the ability to return not only plain references.
///
/// See [crate] documentation for more.
pub trait TryProvideRef<'me, T> {
    /// The type returned in the event of an error.
    type Error;

    /// Tries to provide dependency by reference.
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
