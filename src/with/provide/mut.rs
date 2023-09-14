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
    use crate::{
        context::{
            clone::{
                CloneDependencyMut, CloneDependencyMutWith, CloneDependencyRef,
                CloneDependencyRefWith,
            },
            convert::{
                FromDependencyMut, FromDependencyMutWith, FromDependencyRef, FromDependencyRefWith,
            },
            Empty,
        },
        with::ProvideRefWith,
        ProvideMut, ProvideRef,
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

    impl<'me, T, U, D> ProvideMutWith<'me, T, FromDependencyRef<D>> for U
    where
        U: ProvideRef<'me, D> + ?Sized,
        D: Into<T>,
    {
        fn provide_mut_with(&'me mut self, _: FromDependencyRef<D>) -> T {
            let dependency = self.provide_ref();
            dependency.into()
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

    impl<'me, T, U, D> ProvideMutWith<'me, T, FromDependencyMut<D>> for U
    where
        U: ProvideMut<'me, D> + ?Sized,
        D: Into<T>,
    {
        fn provide_mut_with(&'me mut self, _: FromDependencyMut<D>) -> T {
            let dependency = self.provide_mut();
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

    // TODO clone from reference provided by `Deref` trait
    //      add generic parameter `D` which implements `Deref<Target = T>`
    //      then add it to the `CloneDependencyRef` struct (to be `CloneDependencyRef<D>`)
    impl<'me, T, U> ProvideMutWith<'me, T, CloneDependencyRef> for U
    where
        T: Clone + 'me,
        U: ProvideRef<'me, &'me T> + ?Sized,
    {
        fn provide_mut_with(&'me mut self, _: CloneDependencyRef) -> T {
            let dependency = self.provide_ref();
            dependency.clone()
        }
    }

    // TODO clone from reference provided by `Deref` trait
    //      add generic parameter `D` which implements `Deref<Target = T>`
    //      then add it to the `CloneDependencyRefWith` struct (to be `CloneDependencyRefWith<D, C>`)
    impl<'me, T, U, C> ProvideMutWith<'me, T, CloneDependencyRefWith<C>> for U
    where
        T: Clone + 'me,
        U: ProvideRefWith<'me, &'me T, C> + ?Sized,
    {
        fn provide_mut_with(&'me mut self, context: CloneDependencyRefWith<C>) -> T {
            let context = context.into_inner();
            let dependency = (*self).provide_ref_with(context);
            dependency.clone()
        }
    }

    // TODO clone from reference provided by `Deref` trait
    //      add generic parameter `D` which implements `Deref<Target = T>`
    //      then add it to the `CloneDependencyMut` struct (to be `CloneDependencyMut<D>`)
    impl<'me, T, U> ProvideMutWith<'me, T, CloneDependencyMut> for U
    where
        T: Clone + 'me,
        U: ProvideMut<'me, &'me T> + ?Sized,
    {
        fn provide_mut_with(&'me mut self, _: CloneDependencyMut) -> T {
            let dependency = self.provide_mut();
            dependency.clone()
        }
    }

    // TODO clone from reference provided by `Deref` trait
    //      add generic parameter `D` which implements `Deref<Target = T>`
    //      then add it to the `CloneDependencyMutWith` struct (to be `CloneDependencyMutWith<D>`)
    impl<'me, T, U, C> ProvideMutWith<'me, T, CloneDependencyMutWith<C>> for U
    where
        T: Clone + 'me,
        U: ProvideMutWith<'me, &'me T, C> + ?Sized,
    {
        fn provide_mut_with(&'me mut self, context: CloneDependencyMutWith<C>) -> T {
            let context = context.into_inner();
            let dependency = self.provide_mut_with(context);
            dependency.clone()
        }
    }
}
