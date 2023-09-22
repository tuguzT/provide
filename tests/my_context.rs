use provide::with::{ProvideMutWith, ProvideRefWith};

// #[test]
// fn by_value() {
//     use provide::with::ProvideWith;

//     struct GenericProvider<T>(T)
//     where
//         T: ?Sized;

//     #[derive(Default)]
//     struct WrapOptionWith<C>(C)
//     where
//         C: ?Sized;

//     impl<T, U, C> ProvideWith<Option<T>, WrapOptionWith<C>> for GenericProvider<U>
//     where
//         U: ProvideWith<T, C>,
//     {
//         type Remainder = U::Remainder;

//         fn provide_with(self, context: WrapOptionWith<C>) -> (Option<T>, Self::Remainder) {
//             let Self(provider) = self;
//             let WrapOptionWith(context) = context;
//             let (dependency, remainder) = provider.provide_with(context);
//             let dependency = Some(dependency);
//             (dependency, remainder)
//         }
//     }

//     let provider = GenericProvider(1);
//     let context = WrapOptionWith::<()>::default();
//     // TODO fix somehow
//     // error[E0275]: overflow evaluating the requirement `by_value::GenericProvider<{integer}>: provide::Provide<_>`
//     //   --> tests\my_context.rs:44:36
//     //    |
//     // 44 |     let (dependency, _) = provider.provide_with(context);
//     //    |                                    ^^^^^^^^^^^^
//     //    |
//     //    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`my_context`)
//     //    = note: required for `by_value::GenericProvider<{integer}>` to implement `provide::with::ProvideWith<_, provide::context::convert::FromDependency<_>>`
//     //    = note: 127 redundant requirements hidden
//     //    = note: required for `by_value::GenericProvider<{integer}>` to implement `provide::with::ProvideWith<_, provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::clone::CloneDependencyWith<provide::context::convert::FromDependency<_>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`
//     let (dependency, _) = provider.provide_with(context);
//     assert_eq!(dependency, Some(1));
// }

#[test]
fn by_ref() {
    struct GenericProvider<T>(T)
    where
        T: ?Sized;

    #[derive(Default)]
    struct WrapOptionWith<C>(C)
    where
        C: ?Sized;

    impl<'me, T, U, C> ProvideRefWith<'me, Option<T>, WrapOptionWith<C>> for GenericProvider<U>
    where
        U: ProvideRefWith<'me, T, C> + ?Sized,
    {
        fn provide_ref_with(&'me self, context: WrapOptionWith<C>) -> Option<T> {
            let Self(provider) = self;
            let WrapOptionWith(context) = context;
            let dependency = provider.provide_ref_with(context);
            Some(dependency)
        }
    }

    let provider = GenericProvider("hello");
    let context = WrapOptionWith::<()>::default();
    let dependency = provider.provide_ref_with(context);
    assert_eq!(dependency, Some("hello"));
}

#[test]
fn by_mut() {
    struct GenericProvider<T>(T)
    where
        T: ?Sized;

    #[derive(Default)]
    struct WrapOptionWith<C>(C)
    where
        C: ?Sized;

    impl<'me, T, U, C> ProvideMutWith<'me, Option<T>, WrapOptionWith<C>> for GenericProvider<U>
    where
        U: ProvideMutWith<'me, T, C> + ?Sized,
    {
        fn provide_mut_with(&'me mut self, context: WrapOptionWith<C>) -> Option<T> {
            let Self(provider) = self;
            let WrapOptionWith(context) = context;
            let dependency = provider.provide_mut_with(context);
            Some(dependency)
        }
    }

    let mut provider = GenericProvider([1, 2, 3, 4, 5]);
    let context = WrapOptionWith::<()>::default();
    let dependency = provider.provide_mut_with(context);
    assert_eq!(dependency, Some([1, 2, 3, 4, 5].as_mut_slice()));
}
