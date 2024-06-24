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
    fn provide_with(self, context: C) -> (T, Self::Remainder);
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

mod impls {
    use core::{convert::Infallible, ops::Deref};

    use crate::{
        context::{
            clone::{CloneDependencyMutWith, CloneDependencyRefWith, CloneDependencyWith},
            convert::{FromDependencyMutWith, FromDependencyRefWith, FromDependencyWith},
            Empty,
        },
        with::{ProvideMutWith, ProvideRefWith, With},
        Provide,
    };

    use super::{ProvideWith, TryProvideWith};

    impl<T, U> ProvideWith<T, Empty> for U
    where
        U: Provide<T>,
    {
        type Remainder = U::Remainder;

        fn provide_with(self, _: Empty) -> (T, Self::Remainder) {
            self.provide()
        }
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

    impl<T, U, D, C> ProvideWith<T, CloneDependencyWith<D, C>> for U
    where
        U: ProvideWith<D, C>,
        U::Remainder: With<D>,
        D: Into<T> + Clone,
    {
        type Remainder = <U::Remainder as With<D>>::Output;

        fn provide_with(self, context: CloneDependencyWith<D, C>) -> (T, Self::Remainder) {
            let context = context.into_inner();
            let (original_dependency, remainder) = self.provide_with(context);
            let dependency = original_dependency.clone().into();
            let remainder = remainder.with(original_dependency);
            (dependency, remainder)
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
