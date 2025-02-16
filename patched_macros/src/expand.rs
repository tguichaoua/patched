use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::internals::{
    ast::{self, Container},
    Context,
};

pub fn derive_patch(input: &mut DeriveInput) -> Result<TokenStream, syn::Error> {
    let context = Context::new();
    let container = match Container::from_ast(&context, input) {
        Some(container) => container,
        None => return Err(context.check().unwrap_err()),
    };
    context.check()?;

    let patch_struct = expand_patch_struct(&container);
    let impl_patch_trait = expand_patch_trait(&container);

    Ok(quote! {
        #patch_struct
        #impl_patch_trait
    })
}

fn expand_patch_trait(container: &Container) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = container.generics.split_for_impl();

    let base_struct_name = container.ident;
    let patch_struct_name = container.attrs.patch_struct_name();

    let patch_fields = container.fields.iter().map(|field| {
        let member = &field.member;
        quote! {
            patched::Patch::patch(&mut self.#member, patch.#member);
        }
    });

    quote! {
        #[automatically_derived]
        impl #impl_generics patched::Patch<#patch_struct_name #ty_generics> for #base_struct_name #ty_generics #where_clause {
            #[inline]
            fn patch(&mut self, patch: #patch_struct_name #ty_generics) {
                #(#patch_fields)*
            }
        }
    }
}

fn expand_patch_struct(container: &Container) -> TokenStream {
    let base_struct_name = container.ident;
    let vis = container.vis;
    let patch_struct_name = container.attrs.patch_struct_name();
    let patch_struct_attributes = container.attrs.path_struct_attributes();
    let generics = container.generics;

    let fields = match container.style {
        crate::internals::ast::Style::Struct => {
            let fields = container.fields.iter().map(|field| {
                let name = &field.member;
                let ty = field.attrs.patch_ty();
                let attrs = field.attrs.path_field_attributes();
                quote! { #(#attrs)* #name: #ty }
            });
            quote! {
                { #(#fields),* }
            }
        }
        crate::internals::ast::Style::Tuple => {
            let fields = container.fields.iter().map(|field| {
                let ty = field.attrs.patch_ty();
                let attrs = field.attrs.path_field_attributes();
                quote! { #(#attrs)* #ty }
            });
            quote! {
                ( #(#fields),* );
            }
        }
        crate::internals::ast::Style::Unit => quote!(;),
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let default_fields = expand_instantiate_fields(container.style, &container.fields, |_| {
        quote! { ::core::default::Default::default() }
    });

    let merge_fields = expand_instantiate_fields(container.style, &container.fields, |field| {
        let name = &field.member;
        quote! { patched::Merge::merge(self.#name, rhs.#name) }
    });

    let impl_from_trait = container.attrs.impl_from_trait().then(|| {
        let from_fields = expand_instantiate_fields(container.style, &container.fields, |field| {
            let name = &field.member;
            quote! { ::core::convert::From::from(value.#name) }
        });

        quote! {
            #[automatically_derived]
            impl #impl_generics ::core::convert::From<#base_struct_name #ty_generics> for #patch_struct_name #ty_generics #where_clause {
                #[inline]
                fn from(value: #base_struct_name #ty_generics) -> Self {
                    Self #from_fields
                }
            }
        }
    });

    quote! {
        #(#patch_struct_attributes)*
        #vis struct #patch_struct_name #generics #fields

        #[automatically_derived]
        impl #impl_generics patched::Merge for #patch_struct_name #ty_generics #where_clause {
            type Output = Self;

            #[inline]
            fn merge(self, rhs: Self) -> Self::Output {
                Self #merge_fields
            }
        }

        #[automatically_derived]
        impl #impl_generics ::core::default::Default for #patch_struct_name #ty_generics #where_clause {
            #[inline]
            fn default() -> Self {
                Self #default_fields
            }
        }

        #impl_from_trait
    }
}

fn expand_instantiate_fields(
    style: ast::Style,
    fields: &[ast::Field],
    mut field_value: impl FnMut(&ast::Field) -> TokenStream,
) -> TokenStream {
    match style {
        ast::Style::Struct => {
            let fields = fields.iter().map(|field| {
                let name = &field.member;
                let value = field_value(field);
                quote! { #name: #value }
            });

            quote!( { #(#fields),* } )
        }
        ast::Style::Tuple => {
            let fields = fields.iter().map(|field| {
                let value = field_value(field);
                quote! { #value }
            });

            quote!( ( #(#fields),* ) )
        }
        ast::Style::Unit => quote!(),
    }
}
