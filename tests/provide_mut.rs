use provide::ProvideMut;

#[test]
fn manual_impl() {
    struct Provider {
        foo: i32,
        bar: f32,
    }

    impl ProvideMut<'_, i32> for Provider {
        fn provide_mut(&mut self) -> i32 {
            let Self { foo, .. } = self;
            *foo
        }
    }

    impl<'me> ProvideMut<'me, &'me mut f32> for Provider {
        fn provide_mut(&'me mut self) -> &'me mut f32 {
            let Self { bar, .. } = self;
            bar
        }
    }

    let mut provider = Provider { foo: 1, bar: 2.0 };

    let dependency: i32 = provider.provide_mut();
    assert_eq!(dependency, 1);

    let dependency: &mut f32 = provider.provide_mut();
    assert_eq!(dependency, &mut 2.0);
}

#[test]
fn from_impl() {
    struct Provider {
        foo: i32,
        bar: f32,
    }

    impl AsMut<i32> for Provider {
        fn as_mut(&mut self) -> &mut i32 {
            let Self { foo, .. } = self;
            foo
        }
    }

    impl AsMut<f32> for Provider {
        fn as_mut(&mut self) -> &mut f32 {
            let Self { bar, .. } = self;
            bar
        }
    }

    let mut provider = Provider { foo: 1, bar: 2.0 };

    let dependency: &mut i32 = provider.provide_mut();
    assert_eq!(dependency, &mut 1);

    let dependency: &mut f32 = provider.provide_mut();
    assert_eq!(dependency, &mut 2.0);
}

#[test]
fn generic_impl() {
    struct GenericProvider<T>(T)
    where
        T: ?Sized;

    struct Wrapper<T>(T)
    where
        T: ?Sized;

    impl<'me, T> ProvideMut<'me, Wrapper<&'me mut T>> for GenericProvider<T>
    where
        T: ?Sized,
    {
        fn provide_mut(&'me mut self) -> Wrapper<&'me mut T> {
            let Self(dependency) = self;
            Wrapper(dependency)
        }
    }

    let mut provider = GenericProvider(1);
    let Wrapper(dependency) = provider.provide_mut();
    assert_eq!(dependency, &mut 1);
}
