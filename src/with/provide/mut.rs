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

mod impls {
    use core::{convert::Infallible, ops::Deref};

    use crate::{
        context::{
            clone::{CloneDependencyMutWith, CloneDependencyRefWith},
            convert::{FromDependencyMutWith, FromDependencyRefWith},
            Empty,
        },
        with::ProvideRefWith,
        ProvideMut,
    };

    use super::{ProvideMutWith, TryProvideMutWith};

    impl<'me, T, U> ProvideMutWith<'me, T, Empty> for U
    where
        U: ProvideMut<'me, T> + ?Sized,
    {
        fn provide_mut_with(&'me mut self, _: Empty) -> T {
            self.provide_mut()
        }
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

    impl<'me, T, U, D, C> ProvideMutWith<'me, T, FromDependencyRefWith<D, C>> for U
    where
        U: ProvideRefWith<'me, D, C> + ?Sized,
        D: Into<T>,
    {
        fn provide_mut_with(&'me mut self, context: FromDependencyRefWith<D, C>) -> T {
            let context = context.into_inner();
            let dependency = (*self).provide_ref_with(context);
            dependency.into()
        }
    }

    impl<'me, T, U, D, C> ProvideMutWith<'me, T, FromDependencyMutWith<D, C>> for U
    where
        U: ProvideMutWith<'me, D, C> + ?Sized,
        D: Into<T>,
    {
        fn provide_mut_with(&'me mut self, context: FromDependencyMutWith<D, C>) -> T {
            let context = context.into_inner();
            let dependency = self.provide_mut_with(context);
            dependency.into()
        }
    }

    impl<'me, T, U, D, C> ProvideMutWith<'me, T, CloneDependencyRefWith<D, C>> for U
    where
        T: Clone,
        U: ProvideRefWith<'me, D, C> + ?Sized,
        D: Deref<Target = T>,
    {
        fn provide_mut_with(&'me mut self, context: CloneDependencyRefWith<D, C>) -> T {
            let context = context.into_inner();
            let dependency = (*self).provide_ref_with(context);
            dependency.clone()
        }
    }

    impl<'me, T, U, D, C> ProvideMutWith<'me, T, CloneDependencyMutWith<D, C>> for U
    where
        T: Clone,
        U: ProvideMutWith<'me, D, C> + ?Sized,
        D: Deref<Target = T>,
    {
        fn provide_mut_with(&'me mut self, context: CloneDependencyMutWith<D, C>) -> T {
            let context = context.into_inner();
            let dependency = self.provide_mut_with(context);
            dependency.clone()
        }
    }
}
