use core::convert::Infallible;

/// Type of provider which provides dependency by *value*.
///
/// This trait can be interpreted as an extension of [`Into`] trait
/// but with the ability to return remaining part of the provider to be used later
/// or in chain to retrieve more dependencies.
///
/// See [crate] documentation for more.
pub trait Provide<T>: Sized {
    /// Remaining part of the provider after providing dependency by value.
    type Remainder;

    /// Provides dependency by *value*, also returning
    /// [remaining part](Provide::Remainder) of the provider.
    ///
    /// # Examples
    ///
    /// You can implement this trait for your provider
    /// to provide dependency by value:
    ///
    /// ```
    /// use provide::Provide;
    ///
    /// struct Provider {
    ///     foo: i32,
    ///     bar: f32,
    /// }
    ///
    /// impl Provide<i32> for Provider {
    ///     type Remainder = f32;
    ///
    ///     fn provide(self) -> (i32, Self::Remainder) {
    ///         let Self { foo, bar } = self;
    ///         (foo, bar)
    ///     }
    /// }
    ///
    /// let provider = Provider { foo: 1, bar: 2.0 };
    /// let (dependency, _): (i32, _) = provider.provide();
    /// assert_eq!(dependency, 1);
    /// ```
    ///
    /// You can also provide your provider itself as dependency
    /// thanks to implementation for all types which implement [`Into`]:
    ///
    /// ```
    /// use provide::Provide;
    ///
    /// struct Provider(i32);
    ///
    /// impl From<Provider> for i32 {
    ///     fn from(provider: Provider) -> Self {
    ///         let Provider(dependency) = provider;
    ///         dependency
    ///     }
    /// }
    ///
    /// let provider = Provider(1);
    /// let (dependency, _): (i32, _) = provider.provide();
    /// assert_eq!(dependency, 1);
    /// ```
    ///
    /// However, due to this blanket implementation, you cannot do this:
    ///
    /// ```compile_fail
    /// use provide::Provide;
    ///
    /// // Provides generic values.
    /// struct GenericProvider<T>(T);
    ///
    /// impl<T> Provide<T> for GenericProvider<T> {
    ///     type Remainder = ();
    ///
    ///     fn provide(self) -> (T, Self::Remainder) {
    ///         let Self(dependency) = self;
    ///         (dependency, ())
    ///     }
    /// }
    /// ```
    ///
    /// Instead, consider using a newtype wrapper to avoid conflicting implementations:
    ///
    /// ```
    /// use provide::Provide;
    ///
    /// // Provides generic values.
    /// struct GenericProvider<T>(T);
    ///
    /// // Newtype for dependency provided by `GenericProvider`.
    /// struct Wrapper<T>(T);
    ///
    /// impl<T> Provide<Wrapper<T>> for GenericProvider<T> {
    ///     type Remainder = ();
    ///
    ///     fn provide(self) -> (Wrapper<T>, Self::Remainder) {
    ///         let Self(dependency) = self;
    ///         (Wrapper(dependency), ())
    ///     }
    /// }
    ///
    /// // Type of our dependency.
    /// #[derive(Debug, PartialEq)]
    /// struct MyDependency {
    ///     foo: i32,
    ///     bar: f32,
    /// }
    ///
    /// let dependency = MyDependency { foo: 1, bar: 2.0 };
    /// let provider = GenericProvider(dependency);
    ///
    /// let (Wrapper(dependency), _): (Wrapper<_>, _) = provider.provide();
    /// assert_eq!(dependency, MyDependency { foo: 1, bar: 2.0 });
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

/// Type of provider which can provide dependency by *value* or fail.
///
/// This trait can be interpreted as an extension of [`TryInto`] trait
/// but with the ability to return remaining part of the provider on success to be used later
/// or in chain to retrieve more dependencies.
///
/// See [crate] documentation for more.
pub trait TryProvide<T>: Sized {
    /// Remaining part of the provider after providing dependency by value.
    type Remainder;

    /// The type returned in the event of an error.
    type Error;

    /// Tries to provide dependency by *value*, also returning
    /// [remaining part](TryProvide::Remainder) of the provider on success.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::TryProvide;
    ///
    /// todo!()
    /// ```
    fn try_provide(self) -> Result<(T, Self::Remainder), Self::Error>;
}

impl<T, U> TryProvide<T> for U
where
    U: Provide<T>,
{
    type Remainder = U::Remainder;

    type Error = Infallible;

    fn try_provide(self) -> Result<(T, Self::Remainder), Self::Error> {
        let provide = self.provide();
        Ok(provide)
    }
}
