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

    /// Provides dependency by *value* with additional context provided by the caller,
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
    use crate::{
        context::{
            clone::{CloneMut, CloneMutWith, CloneOwned, CloneOwnedWith, CloneRef, CloneRefWith},
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

    impl<T, U> ProvideWith<T, CloneOwned> for U
    where
        T: Clone,
        U: Provide<T>,
        U::Remainder: With<T>,
    {
        type Remainder = <U::Remainder as With<T>>::Output;

        fn provide_with(self, _: CloneOwned) -> (T, Self::Remainder) {
            let (dependency, remainder) = self.provide();
            let remainder = remainder.with(dependency.clone());
            (dependency, remainder)
        }
    }

    impl<T, U, C> ProvideWith<T, CloneOwnedWith<C>> for U
    where
        T: Clone,
        U: ProvideWith<T, C>,
        U::Remainder: With<T>,
    {
        type Remainder = <U::Remainder as With<T>>::Output;

        fn provide_with(self, context: CloneOwnedWith<C>) -> (T, Self::Remainder) {
            let context = context.into_inner();
            let (dependency, remainder) = self.provide_with(context);
            let remainder = remainder.with(dependency.clone());
            (dependency, remainder)
        }
    }

    impl<T, U> ProvideWith<T, CloneRef> for U
    where
        T: Clone,
        U: ProvideRef<T>,
    {
        type Remainder = U;

        fn provide_with(self, _: CloneRef) -> (T, Self::Remainder) {
            let dependency = self.provide_ref().clone();
            (dependency, self)
        }
    }

    impl<T, U, C> ProvideWith<T, CloneRefWith<C>> for U
    where
        T: Clone,
        U: ProvideRefWith<T, C>,
    {
        type Remainder = U;

        fn provide_with(self, context: CloneRefWith<C>) -> (T, Self::Remainder) {
            let context = context.into_inner();
            let dependency = self.provide_ref_with(context).clone();
            (dependency, self)
        }
    }

    impl<T, U> ProvideWith<T, CloneMut> for U
    where
        T: Clone,
        U: ProvideMut<T>,
    {
        type Remainder = U;

        fn provide_with(mut self, _: CloneMut) -> (T, Self::Remainder) {
            let dependency = self.provide_mut().clone();
            (dependency, self)
        }
    }

    impl<T, U, C> ProvideWith<T, CloneMutWith<C>> for U
    where
        T: Clone,
        U: ProvideMutWith<T, C>,
    {
        type Remainder = U;

        fn provide_with(mut self, context: CloneMutWith<C>) -> (T, Self::Remainder) {
            let context = context.into_inner();
            let dependency = self.provide_mut_with(context).clone();
            (dependency, self)
        }
    }
}
