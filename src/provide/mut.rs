use core::convert::Infallible;

/// Type of provider which provides dependency by mutable reference.
///
/// This trait can be interpreted as an extension of [`AsMut`] trait
/// but with the ability to return not only plain mutable references.
///
/// See [crate] documentation for more.
pub trait ProvideMut<'me, T> {
    /// Provides dependency by mutable reference.
    ///
    /// # Examples
    ///
    /// You can implement this trait for your provider
    /// to provide dependency by mutable reference:
    ///
    /// ```
    /// use provide::ProvideMut;
    ///
    /// struct Provider {
    ///     foo: i32,
    ///     bar: f32,
    /// }
    ///
    /// impl ProvideMut<'_, i32> for Provider {
    ///     fn provide_mut(&mut self) -> i32 {
    ///         let Self { foo, .. } = self;
    ///         *foo
    ///     }
    /// }
    ///
    /// impl<'me> ProvideMut<'me, &'me mut f32> for Provider {
    ///     fn provide_mut(&'me mut self) -> &'me mut f32 {
    ///         let Self { bar, .. } = self;
    ///         bar
    ///     }
    /// }
    ///
    /// let mut provider = Provider { foo: 1, bar: 2.0 };
    ///
    /// let dependency: i32 = provider.provide_mut();
    /// assert_eq!(dependency, 1);
    ///
    /// let dependency: &mut f32 = provider.provide_mut();
    /// assert_eq!(dependency, &mut 2.0);
    /// ```
    ///
    /// You can also provide your dependency via cheap mutable-to-mutable reference conversion
    /// thanks to implementation for all types which implement [`AsRef`]:
    ///
    /// ```
    /// use provide::ProvideMut;
    ///
    /// let mut provider = vec![1, 2, 3];
    /// let dependency = provider.provide_mut();
    /// assert_eq!(dependency, [1, 2, 3]);
    /// ```
    ///
    /// However, due to this blanket implementation, you cannot do this:
    ///
    /// ```compile_fail
    /// use provide::ProvideMut;
    ///
    /// struct GenericProvider<T>(T)
    /// where
    ///     T: ?Sized;
    ///
    /// impl<'me, T> ProvideMut<'me, &'me mut T> for GenericProvider<T>
    /// where
    ///     T: ?Sized,
    /// {
    ///     fn provide_mut(&'me mut self) -> &'me mut T {
    ///         let Self(dependency) = self;
    ///         dependency
    ///     }
    /// }
    /// ```
    ///
    /// Instead, consider using a newtype wrapper to avoid conflicting implementations:
    ///
    /// ```
    /// use provide::ProvideMut;
    ///
    /// struct GenericProvider<T>(T)
    /// where
    ///     T: ?Sized;
    ///
    /// struct Wrapper<T>(T)
    /// where
    ///     T: ?Sized;
    ///
    /// impl<'me, T> ProvideMut<'me, Wrapper<&'me mut T>> for GenericProvider<T>
    /// where
    ///     T: ?Sized,
    /// {
    ///     fn provide_mut(&'me mut self) -> Wrapper<&'me mut T> {
    ///         let Self(dependency) = self;
    ///         Wrapper(dependency)
    ///     }
    /// }
    ///
    /// let mut provider = GenericProvider(1);
    /// let Wrapper(dependency) = provider.provide_mut();
    /// assert_eq!(dependency, &mut 1);
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

/// Type of provider which can provide dependency by mutable reference or fail.
///
/// This trait can be interpreted as an extension of [`AsMut`] trait
/// but with the ability to return not only plain mutable references.
///
/// See [crate] documentation for more.
pub trait TryProvideMut<'me, T> {
    /// The type returned in the event of an error.
    type Error;

    /// Tries to provide dependency by mutable reference.
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
