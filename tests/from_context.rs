use provide::{
    context::convert::{FromDependency, FromDependencyMut, FromDependencyRef},
    with::{ProvideMutWith, ProvideRefWith, ProvideWith},
};

#[test]
fn by_value() {
    let provider = 1;
    let (dependency, _): (f64, _) = provider.provide_with(FromDependency::default());
    assert_eq!(dependency, 1.0);
}

#[test]
fn by_ref() {
    let provider = "hello";
    let dependency: String = provider.provide_ref_with(FromDependencyRef::<&str>::new());
    assert_eq!(dependency, "hello".to_string());
}

#[test]
fn by_mut() {
    let mut provider = [1, 2, 3, 4, 5];

    let dependency: Vec<_> = provider.provide_mut_with(FromDependencyMut::new());
    assert_eq!(dependency, vec![1, 2, 3, 4, 5]);

    let dependency: Vec<_> = provider.provide_mut_with(FromDependencyRef::new());
    assert_eq!(dependency, vec![1, 2, 3, 4, 5]);
}
