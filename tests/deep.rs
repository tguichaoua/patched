use patched::Patch;

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

#[test]
fn patch_deep() {
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
