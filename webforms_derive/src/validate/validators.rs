//! All validation code goes here

use crate::validate::{ValidateFieldInfo, ValidateInfo, ValidateType};
use proc_macro2::Span;
use quote::quote;
use rand::Rng;
use syn;

pub(crate) fn write(info: &ValidateFieldInfo, tokens: &mut proc_macro2::TokenStream) {
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
                quote! {
                    if !#id.is_match(&self.#name) {
                        v.push(ValidateError::InvalidRegex { field: stringify!(#name) })
                    }
                }
            }
            _ => panic!(""),
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

/// Validates an email address using the regular expression below
///
/// In order to use this, the using crate MUST have regex and lazy_static listed as dependancies
pub(crate) fn validate_email(struct_info: &mut ValidateInfo, info: &mut ValidateFieldInfo) {
    let id = syn::Ident::new("form_regex_email", Span::call_site());
    struct_info.regex_tokens.extend(quote! {
        static ref #id: Regex = Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").expect("failed to create email regex to validate form");
    });

    info.attrs.push(ValidateType::Regex(id));
}

/// Validates against a US Phone number
///
/// Required Dependancies: regex, lazy_static
pub(crate) fn validate_phone_number(struct_info: &mut ValidateInfo, info: &mut ValidateFieldInfo) {
    let id = syn::Ident::new("form_regex_phone", Span::call_site());
    struct_info.regex_tokens.extend(quote! {
        static ref #id: Regex = Regex::new(r"^(\+\d{1,2}\s)?\(?\d{3}\)?[\s.-]?\d{3}[\s.-]?\d{4}$").expect("failed to create phone number regex to validate form");
    });

    info.attrs.push(ValidateType::Regex(id));
}

/// Validates an email address using the regular expression below
///
/// In order to use this, the using crate MUST have regex and lazy_static listed as dependancies
pub(crate) fn validate_regex(
    regex: &syn::LitStr,
    struct_info: &mut ValidateInfo,
    info: &mut ValidateFieldInfo,
) {
    let name = &info.field.ident;
    let r = regex.value();
    let mut rng = rand::thread_rng();
    let id = syn::Ident::new(
        &format!(
            "form_regex_{}_{}",
            name.as_ref().unwrap().to_string(),
            rng.gen::<u32>()
        ),
        Span::call_site(),
    );

    struct_info.regex_tokens.extend(quote! {
        static ref #id: Regex = Regex::new(&#r).expect("failed to create email regex to validate form");
    });

    info.attrs.push(ValidateType::Regex(id));
}
