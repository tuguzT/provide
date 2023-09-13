use core::ops::Deref;

/// Type of provider which provides dependency by *shared reference*,
/// but with additional context provided by the caller.
///
/// This trait is very similar to [`ProvideRef`](crate::ProvideRef) trait.
/// However, this trait allows to retrieve additional context provided by the caller,
/// so it is possible to *define many ways* of how dependency can be provided.
///
/// See [crate] documentation for more.
pub trait ProvideRefWith<T, C>
where
    T: ?Sized,
{
    /// Type of shared reference to provided dependency.
    type Ref<'me>: Deref<Target = T>
    where
        Self: 'me,
        T: 'me;

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
    fn provide_ref_with<'me>(&'me self, context: C) -> Self::Ref<'me>
    where
        T: 'me;
}

mod impls {
    use crate::{
        context::{
            clone::{CloneRef, CloneRefWith},
            Empty,
        },
        deref::DerefWrapper,
        ProvideRef,
    };

    use super::ProvideRefWith;

    impl<T, U> ProvideRefWith<T, Empty> for U
    where
        T: ?Sized,
        U: ProvideRef<T> + ?Sized,
    {
        type Ref<'me> = U::Ref<'me>
        where
            Self: 'me,
            T: 'me;

        fn provide_ref_with<'me>(&'me self, _: Empty) -> Self::Ref<'me>
        where
            T: 'me,
        {
            self.provide_ref()
        }
    }

    impl<T, U> ProvideRefWith<T, CloneRef> for U
    where
        T: Clone,
        U: ProvideRef<T> + ?Sized,
    {
        type Ref<'me> = DerefWrapper<T>
        where
            Self: 'me,
            T: 'me;

        fn provide_ref_with<'me>(&'me self, _: CloneRef) -> Self::Ref<'me>
        where
            T: 'me,
        {
            let dependency = self.provide_ref().clone();
            dependency.into()
        }
    }

    impl<T, U, C> ProvideRefWith<T, CloneRefWith<C>> for U
    where
        T: Clone,
        U: ProvideRefWith<T, C> + ?Sized,
    {
        type Ref<'me> = DerefWrapper<T>
        where
            Self: 'me,
            T: 'me;

        fn provide_ref_with<'me>(&'me self, context: CloneRefWith<C>) -> Self::Ref<'me>
        where
            T: 'me,
        {
            let context = context.into_inner();
            let dependency = self.provide_ref_with(context).clone();
            dependency.into()
        }
    }
}