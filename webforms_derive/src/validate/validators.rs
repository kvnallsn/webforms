//! All validation code goes here

use crate::validate::{ValidateField, ValidateType};
use proc_macro2::Span;
use quote::quote;
use syn;

pub(crate) fn write(info: &ValidateField, tokens: &mut proc_macro2::TokenStream) {
    let name = &info.field.ident;
    let mut stream = proc_macro2::TokenStream::new();
    for attr in &info.attrs {
        stream.extend(match attr {
            ValidateType::StringMin(min) => {
                quote! {
                    if self.#name.len() < #min {
                        v.push(ValidateError::InputTooShort { field: stringify!(#name), min: #min });
                    }
                }
            },
            ValidateType::StringMax(max) => {
                quote! {
                    if self.#name.len() > #max {
                        v.push(ValidateError::InputTooLong { field: stringify!(#name), max: #max });
                    }
                }
            },
            ValidateType::ValueMin(min) => {
                quote! {
                    if self.#name < #min {
                        v.push(ValidateError::TooSmall { field: stringify!(#name), min: #min });
                    }
                }
            },
            ValidateType::ValueMax(max) => {
                quote! {
                    if self.#name > #max {
                        v.push(ValidateError::TooLarge { field: stringify!(#name), max: #max });
                    }
                }
            },
            ValidateType::Regex(id) => {
                let rid = syn::Ident::new(&id, Span::call_site());
                quote! {
                    if !#rid.is_match(&self.#name) {
                        v.push(ValidateError::InvalidRegex { field: stringify!(#name) })
                    }
                }
            },
            ValidateType::Email(id) => {
                let rid = syn::Ident::new(&id, Span::call_site());
                quote! {
                    if !#rid.is_match(&self.#name) {
                        v.push(ValidateError::InvalidEmail { field: stringify!(#name) })
                    }
                }
            },
            ValidateType::Phone(id) => {
                let rid = syn::Ident::new(&id, Span::call_site());
                quote! {
                    if !#rid.is_match(&self.#name) {
                        v.push(ValidateError::InvalidPhoneNumber { field: stringify!(#name) })
                    }
                }
            }
        });
    }

    tokens.extend(match info.optional {
        true => quote! {
            match #name {
                Some(s) => {#stream},
                None => {},
            }
        },
        false => stream,
    });
}
