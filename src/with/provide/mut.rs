use core::ops::DerefMut;

/// Type of provider which provides dependency by *unique reference*,
/// but with additional context provided by the caller.
///
/// This trait is very similar to [`ProvideMut`](crate::ProvideMut) trait.
/// However, this trait allows to retrieve additional context provided by the caller,
/// so it is possible to *define many ways* of how dependency can be provided.
///
/// See [crate] documentation for more.
pub trait ProvideMutWith<T, C>
where
    T: ?Sized,
{
    /// Type of unique reference to provided dependency.
    type Mut<'me>: DerefMut<Target = T>
    where
        Self: 'me,
        T: 'me;

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
    fn provide_mut_with<'me>(&'me mut self, context: C) -> Self::Mut<'me>
    where
        T: 'me;
}

mod impls {
    use crate::{
        context::{
            clone::{
                CloneDependencyMut, CloneDependencyMutWith, CloneDependencyRef,
                CloneDependencyRefWith,
            },
            Empty,
        },
        deref::DerefWrapper,
        with::ProvideRefWith,
        ProvideMut, ProvideRef,
    };

    use super::ProvideMutWith;

    impl<T, U> ProvideMutWith<T, Empty> for U
    where
        T: ?Sized,
        U: ProvideMut<T> + ?Sized,
    {
        type Mut<'me> = U::Mut<'me>
        where
            Self: 'me,
            T: 'me;

        fn provide_mut_with<'me>(&'me mut self, _: Empty) -> Self::Mut<'me>
        where
            T: 'me,
        {
            self.provide_mut()
        }
    }

    impl<T, U> ProvideMutWith<T, CloneDependencyRef> for U
    where
        T: Clone,
        U: ProvideRef<T> + ?Sized,
    {
        type Mut<'me> = DerefWrapper<T>
        where
            Self: 'me,
            T: 'me;

        fn provide_mut_with<'me>(&'me mut self, _: CloneDependencyRef) -> Self::Mut<'me>
        where
            T: 'me,
        {
            let dependency = self.provide_ref().clone();
            dependency.into()
        }
    }

    impl<T, U, C> ProvideMutWith<T, CloneDependencyRefWith<C>> for U
    where
        T: Clone,
        U: ProvideRefWith<T, C> + ?Sized,
    {
        type Mut<'me> = DerefWrapper<T>
        where
            Self: 'me,
            T: 'me;

        fn provide_mut_with<'me>(
            &'me mut self,
            context: CloneDependencyRefWith<C>,
        ) -> Self::Mut<'me>
        where
            T: 'me,
        {
            let context = context.into_inner();
            let dependency = (*self).provide_ref_with(context).clone();
            dependency.into()
        }
    }

    impl<T, U> ProvideMutWith<T, CloneDependencyMut> for U
    where
        T: Clone,
        U: ProvideMut<T> + ?Sized,
    {
        type Mut<'me> = DerefWrapper<T>
        where
            Self: 'me,
            T: 'me;

        fn provide_mut_with<'me>(&'me mut self, _: CloneDependencyMut) -> Self::Mut<'me>
        where
            T: 'me,
        {
            let dependency = self.provide_mut().clone();
            dependency.into()
        }
    }

    impl<T, U, C> ProvideMutWith<T, CloneDependencyMutWith<C>> for U
    where
        T: Clone,
        U: ProvideMutWith<T, C> + ?Sized,
    {
        type Mut<'me> = DerefWrapper<T>
        where
            Self: 'me,
            T: 'me;

        fn provide_mut_with<'me>(
            &'me mut self,
            context: CloneDependencyMutWith<C>,
        ) -> Self::Mut<'me>
        where
            T: 'me,
        {
            let context = context.into_inner();
            let dependency = self.provide_mut_with(context).clone();
            dependency.into()
        }
    }
}
