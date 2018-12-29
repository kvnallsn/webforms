//! Validate macro implementation

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

mod validators;

fn parse_meta(field: &syn::Field, meta: &syn::Meta, tokens: &mut proc_macro2::TokenStream) {
    //match attr.parse_meta().expect("Unknown webform attribute") {
    match meta {
        syn::Meta::Word(ref w) => parse_word_attr(field, w, tokens),
        syn::Meta::List(ref l) => parse_list_attr(field, l, tokens),
        syn::Meta::NameValue(ref nv) => parse_namevalue_attr(field, nv, tokens),
    }
}

fn parse_word_attr(field: &syn::Field, ident: &syn::Ident, tokens: &mut proc_macro2::TokenStream) {
    if ident == "email" {
            crate::validate::validators::validate_email(field, tokens);
    } else if ident == "phone" {
            crate::validate::validators::validate_phone_number(field, tokens);
    }
}

fn parse_list_attr(field: &syn::Field, list: &syn::MetaList, tokens: &mut proc_macro2::TokenStream) {
    for nested in list.nested.iter() {
        match nested {
            syn::NestedMeta::Meta(m) => parse_meta(field, m, tokens),
            _ => {},
        };
    }
}

fn parse_namevalue_attr(field: &syn::Field, nv: &syn::MetaNameValue, tokens: &mut proc_macro2::TokenStream) {
    if nv.ident == "min_length" {
        match nv.lit {
            syn::Lit::Int(ref i) => crate::validate::validators::validate_min_length(field, i, tokens),
            _ => panic!("min_length requires an integer argument")
        }
    } else if nv.ident == "max_length" {
        match nv.lit {
            syn::Lit::Int(ref i) => crate::validate::validators::validate_max_length(field, i, tokens),
            _ => panic!("max_length requires an integer argument")
        }
    } else if nv.ident == "min_value" {
        match nv.lit {
            syn::Lit::Int(ref i) => crate::validate::validators::validate_min_value(field, i, tokens),
            _ => panic!("min_value requires an integer argument")
        }
    } else if nv.ident == "max_value" {
        match nv.lit {
            syn::Lit::Int(ref i) => crate::validate::validators::validate_max_value(field, i, tokens),
            _ => panic!("max_value requires an integer argument")
        }
    } else if nv.ident == "regex" {
        match nv.lit {
            syn::Lit::Str(ref s) => crate::validate::validators::validate_regex(field, s, tokens),
            _ => panic!("regex requires a string argument")
        }
    } else {
        println!("Unknown ident: {}", nv.ident.to_string());
    }
}

pub(crate) fn impl_validate_macro(ast: syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    let fields = match ast.data {
        syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(fields), .. }) => fields.named,
        _ => panic!("ValidateForm only defined on data structs!"),
    };

    let reqs = fields.iter().map(|field| {
        let mut tokens = proc_macro2::TokenStream::new();
        for attr in &field.attrs {
            if attr.path.is_ident("validate")  {
                parse_meta(field, &attr.parse_meta().expect("Unknown web form attribute"), &mut tokens);
            }
        }
        tokens
    });

    let gen = quote! {
        impl #generics ValidateForm for #name #generics {
            fn validate(&self) -> Result<(), Vec<ValidateError>> {
                let mut v: Vec<ValidateError> = Vec::new();

                #(#reqs)*

                match v.len() {
                    0 => Ok(()),
                    _ => Err(v),
                }
            }
        }
    };

    gen.into()
}
