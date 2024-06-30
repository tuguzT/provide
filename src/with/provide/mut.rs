use core::convert::Infallible;

use crate::{context::Empty, ProvideMut};

/// Type of provider which provides dependency by *unique reference*,
/// but with additional context provided by the caller.
///
/// This trait is very similar to [`ProvideMut`](crate::ProvideMut) trait.
/// However, this trait allows to retrieve additional context provided by the caller,
/// so it is possible to *define many ways* of how dependency can be provided.
///
/// See [crate] documentation for more.
pub trait ProvideMutWith<'me, T, C> {
    /// Provides dependency by *unique reference*
    /// with additional context provided by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::with::ProvideMutWith;
    ///
    /// todo!()
    /// ```
    fn provide_mut_with(&'me mut self, context: C) -> T;
}

impl<'me, T, U> ProvideMutWith<'me, T, Empty> for U
where
    U: ProvideMut<'me, T> + ?Sized,
{
    fn provide_mut_with(&'me mut self, _: Empty) -> T {
        self.provide_mut()
    }
}

/// Type of provider which can provide dependency by *unique reference*,
/// but with additional context provided by the caller, or fail.
///
/// This trait is very similar to [`TryProvideMut`](crate::TryProvideMut) trait.
/// However, this trait allows to retrieve additional context provided by the caller,
/// so it is possible to *define many ways* of how dependency can be provided.
///
/// See [crate] documentation for more.
pub trait TryProvideMutWith<'me, T, C> {
    /// The type returned in the event of an error.
    type Error;

    /// Tries to provide dependency by *unique reference*
    /// with additional context provided by the caller on success.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::with::TryProvideMutWith;
    ///
    /// todo!()
    /// ```
    fn try_provide_mut_with(&'me mut self, context: C) -> Result<T, Self::Error>;
}

impl<'me, T, U, C> TryProvideMutWith<'me, T, C> for U
where
    U: ProvideMutWith<'me, T, C> + ?Sized,
{
    type Error = Infallible;

    fn try_provide_mut_with(&'me mut self, context: C) -> Result<T, Self::Error> {
        let provide_mut_with = self.provide_mut_with(context);
        Ok(provide_mut_with)
    }
}
