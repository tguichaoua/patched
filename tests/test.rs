use patched::Patch;

#[test]
fn patch() {
    #[derive(Patch, PartialEq, Eq, Debug)]
    struct Foo {
        a: u32,
        b: bool,
        c: String,
    }

    let mut value = Foo {
        a: 50,
        b: true,
        c: String::from("Hello"),
    };

    value.patch(FooPatch {
        a: Some(10),
        b: Some(false),
        ..Default::default()
    });

    assert_eq!(
        value,
        Foo {
            a: 10,
            b: false,
            c: String::from("Hello"),
        }
    );
}

#[test]
fn patch_deep() {
    #[derive(Patch, PartialEq, Eq, Debug)]
    struct Foo {
        a: u32,
        #[patch(with = GooPatch)]
        b: Goo,
        #[patch(with = HooPatch)]
        c: Hoo,
    }

    #[derive(Patch, PartialEq, Eq, Debug)]
    struct Goo {
        a: bool,
        b: u8,
    }

    #[derive(Patch, PartialEq, Eq, Debug)]
    struct Hoo {
        a: String,
    }

    let mut value = Foo {
        a: 12,
        b: Goo { a: true, b: 65 },
        c: Hoo {
            a: String::from("Hello"),
        },
    };

    value.patch(FooPatch {
        b: GooPatch {
            b: Some(100),
            ..Default::default()
        },
        c: HooPatch {
            a: Some(String::from("World!")),
        },
        ..Default::default()
    });

    assert_eq!(
        value,
        Foo {
            a: 12,
            b: Goo { a: true, b: 100 },
            c: Hoo {
                a: String::from("World!"),
            },
        }
    );
}

#[test]
fn from_impl() {
    #[derive(Patch, Debug, PartialEq, Eq)]
    #[patch_attr(derive(Debug, PartialEq, Eq))]
    #[patch(from)]
    struct Foo {
        a: u32,
        b: String,
    }

    let value = Foo {
        a: 53,
        b: String::from("Hello"),
    };

    let patch = FooPatch::from(value);

    assert_eq!(
        patch,
        FooPatch {
            a: Some(53),
            b: Some(String::from("Hello"))
        }
    );
}
