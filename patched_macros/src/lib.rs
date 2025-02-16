mod expand;
mod internals;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// Derive the [`Patch`] trait.
///
/// This macro will generates a new struct representing the target struct but with all field
/// being an `Option`.
///
/// # Container attributes
///
/// - `#[patch(name = Foo)]`: set the name of the patch struct.
/// - `#[patch(attr = ...)]`: pass an attribute to put on the patch struct.
/// - `#[patch(from)]`: impl the `From` trait on the patch struct.
///
/// # Field attributes
///
/// - `#[patch(with = StructPatch)]`: set the type of the field on the patch struct.
/// - `#[patch(attr = ...)]`: pass an attribute to put on the patch struct's field.
///
/// # Example
///
/// ```
/// # use patched::Patch;
/// #[derive(Patch)]
/// struct Foo {
///     a: u64,
///     #[patch(with = GooPatch)]
///     b: Goo,
/// }
///
/// #[derive(Patch)]
/// struct Goo {
///     a: String,
/// }
/// ```
#[proc_macro_derive(Patch, attributes(patch))]
pub fn derive_patch(item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as DeriveInput);
    expand::derive_patch(&mut input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
