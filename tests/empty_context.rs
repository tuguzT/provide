use provide::with::{ProvideMutWith, ProvideRefWith, ProvideWith};

#[test]
fn by_value() {
    let provider = 1;
    let (dependency, _): (i32, _) = ProvideWith::<_, ()>::provide_with(provider, ());
    assert_eq!(dependency, 1);
}

#[test]
fn by_ref() {
    let provider = "hello";
    let dependency: &str = provider.provide_ref_with(());
    assert_eq!(dependency, "hello");
}

#[test]
fn by_mut() {
    let mut provider = [1, 2, 3, 4, 5];
    let dependency = provider.provide_mut_with(());
    assert_eq!(dependency, &mut [1, 2, 3, 4, 5]);
}
