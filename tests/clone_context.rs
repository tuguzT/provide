use provide::{
    context::clone::{CloneDependency, CloneDependencyMut, CloneDependencyRef},
    with::{ProvideMutWith, ProvideRefWith, ProvideWith},
};

#[test]
fn by_value() {
    let provider = vec![1, 2, 3, 4, 5];
    let (dependency, provider): (Vec<_>, _) = provider.provide_with(CloneDependency::default());
    assert_eq!(dependency, provider);
}

#[test]
fn by_ref() {
    let provider = vec![1, 2, 3, 4, 5];
    let dependency: Vec<_> = provider.provide_ref_with(CloneDependencyRef::new());
    assert_eq!(dependency, provider);
}

#[test]
fn by_mut() {
    let mut provider = vec![1, 2, 3, 4, 5];

    let dependency: Vec<_> = provider.provide_mut_with(CloneDependencyMut::new());
    assert_eq!(dependency, provider);

    let dependency: Vec<_> = provider.provide_mut_with(CloneDependencyRef::new());
    assert_eq!(dependency, provider);
}
