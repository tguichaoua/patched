use patched::Patch;

#[derive(Patch)]
union Foo {
    a: u64,
    b: bool,
}

fn main() {}
