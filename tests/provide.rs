use provide::Provide;

#[test]
fn self_impl() {
    let provider = 1;
    let (dependency, _) = provider.provide();
    assert_eq!(dependency, 1);
}

#[test]
fn manual_impl() {
    struct Provider {
        foo: i32,
        bar: f32,
    }

    impl Provide<i32> for Provider {
        type Remainder = f32;

        fn provide(self) -> (i32, Self::Remainder) {
            let Self { foo, bar } = self;
            (foo, bar)
        }
    }

    let provider = Provider { foo: 1, bar: 2.0 };
    let (dependency, _): (i32, _) = provider.provide();
    assert_eq!(dependency, 1);
}

#[test]
fn generic_impl() {
    struct GenericProvider<T>(T);

    struct Wrapper<T>(T);

    impl<T> Provide<Wrapper<T>> for GenericProvider<T> {
        type Remainder = ();

        fn provide(self) -> (Wrapper<T>, Self::Remainder) {
            let Self(dependency) = self;
            (Wrapper(dependency), ())
        }
    }

    #[derive(Debug, PartialEq)]
    struct MyDependency {
        foo: i32,
        bar: f32,
    }

    let dependency = MyDependency { foo: 1, bar: 2.0 };
    let provider = GenericProvider(dependency);

    let (Wrapper(dependency), _): (Wrapper<_>, _) = provider.provide();
    assert_eq!(dependency, MyDependency { foo: 1, bar: 2.0 });
}
