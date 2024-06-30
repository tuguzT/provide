use core::convert::Infallible;

use crate::{context::Empty, ProvideRef};

/// Type of provider which provides dependency by *shared reference*,
/// but with additional context provided by the caller.
///
/// This trait is very similar to [`ProvideRef`](crate::ProvideRef) trait.
/// However, this trait allows to retrieve additional context provided by the caller,
/// so it is possible to *define many ways* of how dependency can be provided.
///
/// See [crate] documentation for more.
pub trait ProvideRefWith<'me, T, C> {
    /// Provides dependency by *shared reference*
    /// with additional context provided by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::with::ProvideRefWith;
    ///
    /// todo!()
    /// ```
    fn provide_ref_with(&'me self, context: C) -> T;
}

impl<'me, T, U> ProvideRefWith<'me, T, Empty> for U
where
    U: ProvideRef<'me, T> + ?Sized,
{
    fn provide_ref_with(&'me self, _: Empty) -> T {
        self.provide_ref()
    }
}

/// Type of provider which can provide dependency by *shared reference*,
/// but with additional context provided by the caller, or fail.
///
/// This trait is very similar to [`TryProvideRef`](crate::TryProvideRef) trait.
/// However, this trait allows to retrieve additional context provided by the caller,
/// so it is possible to *define many ways* of how dependency can be provided.
///
/// See [crate] documentation for more.
pub trait TryProvideRefWith<'me, T, C> {
    /// The type returned in the event of an error.
    type Error;

    /// Tries to provide dependency by *shared reference*
    /// with additional context provided by the caller on success.
    ///
    /// # Examples
    ///
    /// ```
    /// use provide::with::TryProvideRefWith;
    ///
    /// todo!()
    /// ```
    fn try_provide_ref_with(&'me self, context: C) -> Result<T, Self::Error>;
}

impl<'me, T, U, C> TryProvideRefWith<'me, T, C> for U
where
    U: ProvideRefWith<'me, T, C> + ?Sized,
{
    type Error = Infallible;

    fn try_provide_ref_with(&'me self, context: C) -> Result<T, Self::Error> {
        let provide_ref_with = self.provide_ref_with(context);
        Ok(provide_ref_with)
    }
}
