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

mod impls {
    use crate::{
        context::{
            clone::{CloneDependencyRef, CloneDependencyRefWith},
            convert::{FromDependencyRef, FromDependencyRefWith},
            Empty,
        },
        ProvideRef,
    };

    use super::ProvideRefWith;

    impl<'me, T, U> ProvideRefWith<'me, T, Empty> for U
    where
        U: ProvideRef<'me, T> + ?Sized,
    {
        fn provide_ref_with(&'me self, _: Empty) -> T {
            self.provide_ref()
        }
    }

    impl<'me, T, U, D> ProvideRefWith<'me, T, FromDependencyRef<D>> for U
    where
        U: ProvideRef<'me, D> + ?Sized,
        D: Into<T>,
    {
        fn provide_ref_with(&'me self, _: FromDependencyRef<D>) -> T {
            let dependency = self.provide_ref();
            dependency.into()
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

    // TODO clone from reference provided by `Deref` trait
    //      add generic parameter `D` which implements `Deref<Target = T>`
    //      then add it to the `CloneDependencyRef` struct (to be `CloneDependencyRef<D>`)
    impl<'me, T, U> ProvideRefWith<'me, T, CloneDependencyRef> for U
    where
        T: Clone + 'me,
        U: ProvideRef<'me, &'me T> + ?Sized,
    {
        fn provide_ref_with(&'me self, _: CloneDependencyRef) -> T {
            let dependency = self.provide_ref();
            dependency.clone()
        }
    }

    // TODO clone from reference provided by `Deref` trait
    //      add generic parameter `D` which implements `Deref<Target = T>`
    //      then add it to the `CloneDependencyRefWith` struct (to be `CloneDependencyRefWith<D, C>`)
    impl<'me, T, U, C> ProvideRefWith<'me, T, CloneDependencyRefWith<C>> for U
    where
        T: Clone + 'me,
        U: ProvideRefWith<'me, &'me T, C> + ?Sized,
    {
        fn provide_ref_with(&'me self, context: CloneDependencyRefWith<C>) -> T {
            let context = context.into_inner();
            let dependency = self.provide_ref_with(context);
            dependency.clone()
        }
    }
}
