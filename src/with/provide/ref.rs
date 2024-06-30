use core::{convert::Infallible, ops::Deref};

use crate::{
    context::{clone::CloneDependencyRefWith, convert::FromDependencyRefWith, Empty},
    ProvideRef,
};

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

impl<'me, T, U, D, C> ProvideRefWith<'me, T, FromDependencyRefWith<D, C>> for U
where
    U: ProvideRefWith<'me, D, C> + ?Sized,
    D: Into<T>,
{
    fn provide_ref_with(&'me self, context: FromDependencyRefWith<D, C>) -> T {
        let context = context.into_inner();
        let dependency = (*self).provide_ref_with(context);
        dependency.into()
    }
}

impl<'me, T, U, D, C> ProvideRefWith<'me, T, CloneDependencyRefWith<D, C>> for U
where
    T: Clone,
    U: ProvideRefWith<'me, D, C> + ?Sized,
    D: Deref<Target = T>,
{
    fn provide_ref_with(&'me self, context: CloneDependencyRefWith<D, C>) -> T {
        let context = context.into_inner();
        let dependency = self.provide_ref_with(context);
        dependency.clone()
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
