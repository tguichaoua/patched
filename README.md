# Patched

[![github](https://img.shields.io/badge/github-tguichaoua/patched-8da0cb?logo=github)](https://github.com/tguichaoua/patched)
[![Latest version](https://img.shields.io/crates/v/patched.svg)](https://crates.io/crates/patched)
[![Documentation](https://docs.rs/patched/badge.svg)](https://docs.rs/patched)
[![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/tguichaoua/patched/blob/main/LICENSE-MIT)
[![Apache](https://img.shields.io/badge/license-Apache-blue.svg)](https://github.com/tguichaoua/patched/blob/main/LICENSE-APACHE)

A macro that generates patch struct.

```rust
use patched::Patch;

#[derive(Patch)]
struct Foo {
    a: u8,
    b: String,
}

// Will generates

struct FooPatch {
    a: Option<u8>
    b: Option<String>,
}

impl Default for FooPatch {
    /* ... */
}

impl Patch<FooPatch> for Foo {
    /* ... */
}
```

Usage example:

```rs
let mut value = Foo {
    a: 10,
    b: String::from("Hello");
}

value.patch(FooPatch {
    a: Some(99),
    ..Default::default()
});

assert_eq!(
    value,
    Foo {
        a: 99,
        b: String::from("Hello");
    }
);
```
