use provide::{
    with::{ProvideMutWith, ProvideRefWith, ProvideWith},
    ProvideRef,
};

#[test]
fn empty() {
    let provider = 1;
    let (dependency, _): (i32, _) = provider.provide_with(());
    assert_eq!(dependency, 1);
}

#[test]
fn empty_ref() {
    let provider = "hello";
    let dependency: &str = provider.provide_ref_with(());
    assert_eq!(dependency, "hello");
}

#[test]
fn empty_mut() {
    let mut provider = vec![1, 2, 3, 4, 5];
    let dependency: &mut [_] = provider.provide_mut_with(());
    assert_eq!(dependency, &mut [1, 2, 3, 4, 5]);
}

#[test]
fn my_context_ref() {
    struct GenericProvider<T>(T)
    where
        T: ?Sized;

    struct WrapOption;

    impl<'me, T, U> ProvideRefWith<'me, Option<T>, WrapOption> for GenericProvider<U>
    where
        U: ProvideRef<'me, T> + ?Sized,
    {
        fn provide_ref_with(&'me self, _: WrapOption) -> Option<T> {
            let Self(provider) = self;
            let dependency = provider.provide_ref();
            Some(dependency)
        }
    }

    let provider = GenericProvider("hello");
    let dependency = provider.provide_ref_with(WrapOption);
    assert_eq!(dependency, Some("hello"));
}
