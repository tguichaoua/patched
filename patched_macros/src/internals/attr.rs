use proc_macro2::TokenStream;
use quote::{format_ident, ToTokens};
use syn::{parse_quote, Attribute, Ident};

use crate::internals::symbol::*;

use super::Context;

/* -------------------------------------------------------------------------- */

pub(crate) struct Attr<'c, T> {
    cx: &'c Context,
    name: Symbol,
    tokens: TokenStream,
    value: Option<T>,
}

impl<'c, T> Attr<'c, T> {
    fn none(cx: &'c Context, name: Symbol) -> Self {
        Attr {
            cx,
            name,
            tokens: TokenStream::new(),
            value: None,
        }
    }

    fn set<A: ToTokens>(&mut self, obj: A, value: T) {
        let tokens = obj.into_token_stream();

        if self.value.is_some() {
            let msg = format!("duplicate serde attribute `{}`", self.name);
            self.cx.error_spanned_by(tokens, msg);
        } else {
            self.tokens = tokens;
            self.value = Some(value);
        }
    }

    pub(crate) fn get(self) -> Option<T> {
        self.value
    }
}

struct BoolAttr<'c>(Attr<'c, ()>);

impl<'c> BoolAttr<'c> {
    fn none(cx: &'c Context, name: Symbol) -> Self {
        BoolAttr(Attr::none(cx, name))
    }

    fn set_true<A: ToTokens>(&mut self, obj: A) {
        self.0.set(obj, ());
    }

    fn get(&self) -> bool {
        self.0.value.is_some()
    }
}

pub(crate) struct VecAttr<T> {
    first_dup_tokens: TokenStream,
    values: Vec<T>,
}

impl<T> VecAttr<T> {
    fn none() -> Self {
        VecAttr {
            first_dup_tokens: TokenStream::new(),
            values: Vec::new(),
        }
    }

    fn insert<A: ToTokens>(&mut self, obj: A, value: T) {
        if self.values.len() == 1 {
            self.first_dup_tokens = obj.into_token_stream();
        }
        self.values.push(value);
    }

    pub(crate) fn get(self) -> Vec<T> {
        self.values
    }
}

/* -------------------------------------------------------------------------- */

pub struct Container {
    patch_struct_name: Ident,
    path_struct_attributes: Vec<Attribute>,
    impl_from_trait: bool,
}

impl Container {
    pub fn from_ast(cx: &Context, item: &syn::DeriveInput) -> Self {
        let mut patch_struct_name = Attr::none(cx, NAME);
        let mut path_struct_attributes = VecAttr::none();
        let mut impl_from_trait = BoolAttr::none(cx, FROM);

        for attr in &item.attrs {
            if attr.path() != PATCH {
                continue;
            }

            if let syn::Meta::List(meta) = &attr.meta {
                if meta.tokens.is_empty() {
                    continue;
                }
            }

            if let Err(error) = attr.parse_nested_meta(|meta| {
                if meta.path == NAME {
                    // #[patch(name = Foo)]
                    patch_struct_name.set(&meta.path, meta.value()?.parse()?);
                } else if meta.path == ATTR {
                    // #[patch(attr = ...)]
                    let attr_meta: syn::Meta = meta.value()?.parse()?;
                    path_struct_attributes.insert(
                        &meta.path,
                        Attribute {
                            pound_token: Default::default(),
                            style: syn::AttrStyle::Outer,
                            bracket_token: Default::default(),
                            meta: attr_meta,
                        },
                    );
                } else if meta.path == FROM {
                    impl_from_trait.set_true(&meta.path);
                } else {
                    let path = meta.path.to_token_stream().to_string().replace(' ', "");
                    return Err(
                        meta.error(format_args!("unknown patch variant attribute `{}`", path))
                    );
                }

                Ok(())
            }) {
                cx.syn_error(error);
            }
        }

        Container {
            patch_struct_name: patch_struct_name
                .get()
                .unwrap_or_else(|| format_ident!("{}Patch", item.ident)),
            path_struct_attributes: path_struct_attributes.get(),
            impl_from_trait: impl_from_trait.get(),
        }
    }

    pub fn patch_struct_name(&self) -> &Ident {
        &self.patch_struct_name
    }

    pub fn path_struct_attributes(&self) -> &[Attribute] {
        &self.path_struct_attributes
    }

    pub fn impl_from_trait(&self) -> bool {
        self.impl_from_trait
    }
}

/* -------------------------------------------------------------------------- */

pub struct Field {
    patch_ty: syn::Type,
    path_field_attributes: Vec<Attribute>,
}

impl Field {
    pub fn from_ast(cx: &Context, field: &syn::Field) -> Self {
        let mut with = Attr::none(cx, WITH);
        let mut path_field_attributes = VecAttr::none();

        for attr in &field.attrs {
            if attr.path() != PATCH {
                continue;
            }

            if let syn::Meta::List(meta) = &attr.meta {
                if meta.tokens.is_empty() {
                    continue;
                }
            }

            if let Err(error) = attr.parse_nested_meta(|meta| {
                if meta.path == WITH {
                    // #[patch(with = FooPatch)]
                    with.set(&meta.path, meta.value()?.parse()?);
                } else if meta.path == ATTR {
                    // #[patch(attr = ...)]
                    let attr_meta: syn::Meta = meta.value()?.parse()?;
                    path_field_attributes.insert(
                        &meta.path,
                        Attribute {
                            pound_token: Default::default(),
                            style: syn::AttrStyle::Outer,
                            bracket_token: Default::default(),
                            meta: attr_meta,
                        },
                    );
                } else {
                    let path = meta.path.to_token_stream().to_string().replace(' ', "");
                    return Err(
                        meta.error(format_args!("unknown patch variant attribute `{}`", path))
                    );
                }

                Ok(())
            }) {
                cx.syn_error(error);
            }
        }

        Field {
            patch_ty: with.get().unwrap_or_else(|| {
                let ty = &field.ty;
                parse_quote!(::core::option::Option::<#ty>)
            }),
            path_field_attributes: path_field_attributes.get(),
        }
    }

    pub fn patch_ty(&self) -> &syn::Type {
        &self.patch_ty
    }

    pub fn path_field_attributes(&self) -> &[Attribute] {
        &self.path_field_attributes
    }
}

/* -------------------------------------------------------------------------- */
