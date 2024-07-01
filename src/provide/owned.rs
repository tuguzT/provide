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
