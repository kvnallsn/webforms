//! All validation code goes here

use crate::validate::{ValidateField, ValidateType};
use proc_macro2::Span;
use quote::quote;
use syn;

pub(crate) fn write(info: &ValidateField, tokens: &mut proc_macro2::TokenStream) {
    let name = &info.field.ident;
    let mut stream = proc_macro2::TokenStream::new();
    for attr in &info.attrs {
        let field = match info.optional {
            true => quote! {
                opt
            },
            false => quote! {
                self.#name
            },
        };

        let refs = match info.optional {
            true => quote! {&},
            false => quote! {},
        };

        stream.extend(match attr {
            ValidateType::StringMin(min) => {
                quote! {
                    if #field.len() < #min {
                        v.push(ValidateError::InputTooShort { field: stringify!(#name), min: #min });
                    }
                }
            },
            ValidateType::StringMax(max) => {
                quote! {
                    if #field.len() > #max {
                        v.push(ValidateError::InputTooLong { field: stringify!(#name), max: #max });
                    }
                }
            },
            ValidateType::ValueMin(min) => {
                quote! {
                    if #field < #refs #min {
                        v.push(ValidateError::TooSmall { field: stringify!(#name), min: #min });
                    }
                }
            },
            ValidateType::ValueMax(max) => {
                quote! {
                    if #field > #refs #max {
                        v.push(ValidateError::TooLarge { field: stringify!(#name), max: #max });
                    }
                }
            },
            ValidateType::Regex(id) => {
                let rid = syn::Ident::new(&id, Span::call_site());
                quote! {
                    if !#rid.is_match(&#field) {
                        v.push(ValidateError::InvalidRegex { field: stringify!(#name) })
                    }
                }
            },
            ValidateType::Email(id) => {
                let rid = syn::Ident::new(&id, Span::call_site());
                quote! {
                    if !#rid.is_match(&#field) {
                        v.push(ValidateError::InvalidEmail { field: stringify!(#name) })
                    }
                }
            },
            ValidateType::Phone(id) => {
                let rid = syn::Ident::new(&id, Span::call_site());
                quote! {
                    if !#rid.is_match(&#field) {
                        v.push(ValidateError::InvalidPhoneNumber { field: stringify!(#name) })
                    }
                }
            }
        });
    }

    tokens.extend(match info.optional {
        true => quote! {
            match self.#name.as_ref() {
                Some(opt) => {#stream},
                None => {},
            }
        },
        false => stream,
    });
}
