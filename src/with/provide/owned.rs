use core::convert::Infallible;

use crate::{context::Empty, Provide};

/// Type of provider which provides dependency by *value*,
/// but with additional context provided by the caller.
///
/// This trait is very similar to [`Provide`](crate::Provide) trait.
/// However, this trait allows to retrieve additional context provided by the caller,
/// so it is possible to *define many ways* of how dependency can be provided.
///
/// See [crate] documentation for more.
pub trait ProvideWith<T, C>: Sized {
    /// Remaining part of the provider after providing dependency by value.
    type Remainder;

    /// Provides dependency by *value*
    /// with additional context provided by the caller,
    /// also returning [remaining part](ProvideWith::Remainder) of the provider.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::with::ProvideWith;
    ///
    /// todo!()
    /// ```
    #[must_use = "this call returns dependency and remaining part of the provider"]
    fn provide_with(self, context: C) -> (T, Self::Remainder);
}

impl<T, U> ProvideWith<T, Empty> for U
where
    U: Provide<T>,
{
    type Remainder = U::Remainder;

    fn provide_with(self, _: Empty) -> (T, Self::Remainder) {
        self.provide()
    }
}

/// Type of provider which can provide dependency by *value*,
/// but with additional context provided by the caller, or fail.
///
/// This trait is very similar to [`TryProvide`](crate::TryProvide) trait.
/// However, this trait allows to retrieve additional context provided by the caller,
/// so it is possible to *define many ways* of how dependency can be provided.
///
/// See [crate] documentation for more.
pub trait TryProvideWith<T, C>: Sized {
    /// Remaining part of the provider after providing dependency by value.
    type Remainder;

    /// The type returned in the event of an error.
    type Error;

    /// Tries to provide dependency by *value*
    /// with additional context provided by the caller,
    /// also returning [remaining part](TryProvideWith::Remainder) of the provider on success.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::with::TryProvideWith;
    ///
    /// todo!()
    /// ```
    fn try_provide_with(self, context: C) -> Result<(T, Self::Remainder), Self::Error>;
}

impl<T, U, C> TryProvideWith<T, C> for U
where
    U: ProvideWith<T, C>,
{
    type Remainder = U::Remainder;

    type Error = Infallible;

    fn try_provide_with(self, context: C) -> Result<(T, Self::Remainder), Self::Error> {
        let provide_with = self.provide_with(context);
        Ok(provide_with)
    }
}
