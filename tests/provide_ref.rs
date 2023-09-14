use provide::ProvideRef;

#[test]
fn manual_impl() {
    struct Provider {
        foo: i32,
        bar: f32,
    }

    impl ProvideRef<'_, i32> for Provider {
        fn provide_ref(&self) -> i32 {
            let Self { foo, .. } = self;
            *foo
        }
    }

    impl<'me> ProvideRef<'me, &'me f32> for Provider {
        fn provide_ref(&'me self) -> &'me f32 {
            let Self { bar, .. } = self;
            bar
        }
    }

    let provider = Provider { foo: 1, bar: 2.0 };

    let dependency: i32 = provider.provide_ref();
    assert_eq!(dependency, 1);

    let dependency: &f32 = provider.provide_ref();
    assert_eq!(dependency, &2.0);
}

#[test]
fn from_impl() {
    struct Provider {
        foo: i32,
        bar: f32,
    }

    impl AsRef<i32> for Provider {
        fn as_ref(&self) -> &i32 {
            let Self { foo, .. } = self;
            foo
        }
    }

    impl AsRef<f32> for Provider {
        fn as_ref(&self) -> &f32 {
            let Self { bar, .. } = self;
            bar
        }
    }

    let provider = Provider { foo: 1, bar: 2.0 };

    let dependency: &i32 = provider.provide_ref();
    assert_eq!(dependency, &1);

    let dependency: &f32 = provider.provide_ref();
    assert_eq!(dependency, &2.0);
}

#[test]
fn generic_impl() {
    struct GenericProvider<T>(T)
    where
        T: ?Sized;

    struct Wrapper<T>(T)
    where
        T: ?Sized;

    impl<'me, T> ProvideRef<'me, Wrapper<&'me T>> for GenericProvider<T>
    where
        T: ?Sized,
    {
        fn provide_ref(&'me self) -> Wrapper<&'me T> {
            let Self(dependency) = self;
            Wrapper(dependency)
        }
    }

    let provider = GenericProvider(1);
    let Wrapper(dependency) = provider.provide_ref();
    assert_eq!(dependency, &1);
}
