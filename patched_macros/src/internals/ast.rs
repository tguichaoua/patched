use syn::{punctuated::Punctuated, Generics, Ident, Member, Token, Visibility};

use super::{attr, Context};

pub struct Container<'a> {
    pub attrs: attr::Container,
    pub vis: &'a Visibility,
    pub ident: &'a Ident,
    pub generics: &'a Generics,
    pub style: Style,
    pub fields: Vec<Field>,
}

pub struct Field {
    pub member: Member,
    pub attrs: attr::Field,
}

#[derive(Copy, Clone)]
pub enum Style {
    /// Named fields.
    Struct,
    /// Unnamed fields.
    Tuple,
    /// No fields.
    Unit,
}

impl<'a> Container<'a> {
    /// Convert the raw Syn ast into a parsed container object, collecting errors in `cx`.
    pub fn from_ast(cx: &Context, item: &'a syn::DeriveInput) -> Option<Container<'a>> {
        let attrs = attr::Container::from_ast(cx, item);

        let (style, fields) = match &item.data {
            syn::Data::Struct(data) => struct_from_ast(cx, &data.fields),
            syn::Data::Enum(_) => {
                cx.error_spanned_by(item, "Patch does not support derive for enums");
                return None;
            }
            syn::Data::Union(_) => {
                cx.error_spanned_by(item, "Patch does not support derive for unions");
                return None;
            }
        };

        Some(Container {
            attrs,
            vis: &item.vis,
            ident: &item.ident,
            generics: &item.generics,
            style,
            fields,
        })
    }
}

fn struct_from_ast(cx: &Context, fields: &syn::Fields) -> (Style, Vec<Field>) {
    match fields {
        syn::Fields::Named(fields) => (Style::Struct, fields_from_ast(cx, &fields.named)),
        syn::Fields::Unnamed(fields) => (Style::Tuple, fields_from_ast(cx, &fields.unnamed)),
        syn::Fields::Unit => (Style::Unit, Vec::new()),
    }
}

fn fields_from_ast(cx: &Context, fields: &Punctuated<syn::Field, Token![,]>) -> Vec<Field> {
    fields
        .iter()
        .enumerate()
        .map(|(i, field)| Field {
            member: match &field.ident {
                Some(ident) => syn::Member::Named(ident.clone()),
                None => syn::Member::Unnamed(i.into()),
            },
            attrs: attr::Field::from_ast(cx, field),
        })
        .collect()
}
