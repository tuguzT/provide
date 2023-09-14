/// Type of provider which provides dependency by *value*,
/// but with additional context provided by the caller.
///
/// This trait is very similar to [`Provide`](crate::Provide) trait.
/// However, this trait allows to retrieve additional context provided by the caller,
/// so it is possible to *define many ways* of how dependency can be provided.
///
/// See [crate] documentation for more.
pub trait ProvideWith<T, C> {
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
    fn provide_with(self, context: C) -> (T, Self::Remainder);
}

mod impls {
    use core::ops::Deref;

    use crate::{
        context::{
            clone::{
                CloneDependency, CloneDependencyMut, CloneDependencyMutWith, CloneDependencyRef,
                CloneDependencyRefWith, CloneDependencyWith,
            },
            convert::{
                FromDependency, FromDependencyMut, FromDependencyMutWith, FromDependencyRef,
                FromDependencyRefWith, FromDependencyWith,
            },
            Empty,
        },
        with::{ProvideMutWith, ProvideRefWith, With},
        Provide, ProvideMut, ProvideRef,
    };

    use super::ProvideWith;

    impl<T, U> ProvideWith<T, Empty> for U
    where
        U: Provide<T>,
    {
        type Remainder = U::Remainder;

        fn provide_with(self, _: Empty) -> (T, Self::Remainder) {
            self.provide()
        }
    }

    impl<T, U, D> ProvideWith<T, FromDependency<D>> for U
    where
        U: Provide<D>,
        D: Into<T>,
    {
        type Remainder = U::Remainder;

        fn provide_with(self, _: FromDependency<D>) -> (T, Self::Remainder) {
            let (dependency, remainder) = self.provide();
            let dependency = dependency.into();
            (dependency, remainder)
        }
    }

    impl<T, U, D, C> ProvideWith<T, FromDependencyWith<D, C>> for U
    where
        U: ProvideWith<D, C>,
        D: Into<T>,
    {
        type Remainder = U::Remainder;

        fn provide_with(self, context: FromDependencyWith<D, C>) -> (T, Self::Remainder) {
            let context = context.into_inner();
            let (dependency, remainder) = self.provide_with(context);
            let dependency = dependency.into();
            (dependency, remainder)
        }
    }

    impl<T, U, D> ProvideWith<T, FromDependencyRef<D>> for U
    where
        U: for<'any> ProvideRef<'any, D>,
        D: Into<T>,
    {
        type Remainder = U;

        fn provide_with(self, _: FromDependencyRef<D>) -> (T, Self::Remainder) {
            let dependency = self.provide_ref();
            let dependency = dependency.into();
            (dependency, self)
        }
    }

    impl<T, U, D, C> ProvideWith<T, FromDependencyRefWith<D, C>> for U
    where
        U: for<'any> ProvideRefWith<'any, D, C>,
        D: Into<T>,
    {
        type Remainder = U;

        fn provide_with(self, context: FromDependencyRefWith<D, C>) -> (T, Self::Remainder) {
            let context = context.into_inner();
            let dependency = self.provide_ref_with(context);
            let dependency = dependency.into();
            (dependency, self)
        }
    }

    impl<T, U, D> ProvideWith<T, FromDependencyMut<D>> for U
    where
        U: for<'any> ProvideMut<'any, D>,
        D: Into<T>,
    {
        type Remainder = U;

        fn provide_with(mut self, _: FromDependencyMut<D>) -> (T, Self::Remainder) {
            let dependency = self.provide_mut();
            let dependency = dependency.into();
            (dependency, self)
        }
    }

    impl<T, U, D, C> ProvideWith<T, FromDependencyMutWith<D, C>> for U
    where
        U: for<'any> ProvideMutWith<'any, D, C>,
        D: Into<T>,
    {
        type Remainder = U;

        fn provide_with(mut self, context: FromDependencyMutWith<D, C>) -> (T, Self::Remainder) {
            let context = context.into_inner();
            let dependency = self.provide_mut_with(context);
            let dependency = dependency.into();
            (dependency, self)
        }
    }

    impl<T, U> ProvideWith<T, CloneDependency> for U
    where
        T: Clone,
        U: Provide<T>,
        U::Remainder: With<T>,
    {
        type Remainder = <U::Remainder as With<T>>::Output;

        fn provide_with(self, _: CloneDependency) -> (T, Self::Remainder) {
            let (dependency, remainder) = self.provide();
            let remainder = remainder.with(dependency.clone());
            (dependency, remainder)
        }
    }

    impl<T, U, C> ProvideWith<T, CloneDependencyWith<C>> for U
    where
        T: Clone,
        U: ProvideWith<T, C>,
        U::Remainder: With<T>,
    {
        type Remainder = <U::Remainder as With<T>>::Output;

        fn provide_with(self, context: CloneDependencyWith<C>) -> (T, Self::Remainder) {
            let context = context.into_inner();
            let (dependency, remainder) = self.provide_with(context);
            let remainder = remainder.with(dependency.clone());
            (dependency, remainder)
        }
    }

    impl<T, U, D> ProvideWith<T, CloneDependencyRef<D>> for U
    where
        T: Clone,
        U: for<'any> ProvideRef<'any, D>,
        D: Deref<Target = T>,
    {
        type Remainder = U;

        fn provide_with(self, _: CloneDependencyRef<D>) -> (T, Self::Remainder) {
            let dependency = self.provide_ref().clone();
            (dependency, self)
        }
    }

    impl<T, U, D, C> ProvideWith<T, CloneDependencyRefWith<D, C>> for U
    where
        T: Clone,
        U: for<'any> ProvideRefWith<'any, D, C>,
        D: Deref<Target = T>,
    {
        type Remainder = U;

        fn provide_with(self, context: CloneDependencyRefWith<D, C>) -> (T, Self::Remainder) {
            let context = context.into_inner();
            let dependency = self.provide_ref_with(context).clone();
            (dependency, self)
        }
    }

    impl<T, U, D> ProvideWith<T, CloneDependencyMut<D>> for U
    where
        T: Clone,
        U: for<'any> ProvideMut<'any, D>,
        D: Deref<Target = T>,
    {
        type Remainder = U;

        fn provide_with(mut self, _: CloneDependencyMut<D>) -> (T, Self::Remainder) {
            let dependency = self.provide_mut().clone();
            (dependency, self)
        }
    }

    impl<T, U, D, C> ProvideWith<T, CloneDependencyMutWith<D, C>> for U
    where
        T: Clone,
        U: for<'any> ProvideMutWith<'any, D, C>,
        D: Deref<Target = T>,
    {
        type Remainder = U;

        fn provide_with(mut self, context: CloneDependencyMutWith<D, C>) -> (T, Self::Remainder) {
            let context = context.into_inner();
            let dependency = self.provide_mut_with(context).clone();
            (dependency, self)
        }
    }
}
