use provide::{with::ProvideRefWith, ProvideRef};

#[test]
fn my_context_ref() {
    struct ProviderWrapper<T>(T)
    where
        T: ?Sized;

    struct WrapOption;

    impl<'me, T, U> ProvideRefWith<'me, Option<T>, WrapOption> for ProviderWrapper<U>
    where
        U: ProvideRef<'me, T>,
    {
        fn provide_ref_with(&'me self, _: WrapOption) -> Option<T> {
            let Self(provider) = self;
            Some(provider.provide_ref())
        }
    }

    let provider = ProviderWrapper("hello");
    let dependency = provider.provide_ref_with(WrapOption);
    assert_eq!(dependency, Some("hello"));
}
