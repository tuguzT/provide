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

mod impls {
    use core::ops::Deref;

    use crate::{
        context::{
            clone::{CloneDependencyMutWith, CloneDependencyRefWith},
            convert::{FromDependencyMutWith, FromDependencyRefWith},
            Empty,
        },
        with::ProvideRefWith,
        ProvideMut,
    };

    use super::ProvideMutWith;

    impl<'me, T, U> ProvideMutWith<'me, T, Empty> for U
    where
        U: ProvideMut<'me, T> + ?Sized,
    {
        fn provide_mut_with(&'me mut self, _: Empty) -> T {
            self.provide_mut()
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
