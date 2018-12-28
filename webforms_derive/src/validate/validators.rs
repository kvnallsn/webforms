//! All validation code goes here

use quote::quote;
use proc_macro2::Span;
use rand::Rng;
use syn;

/// Validates an email address using the regular expression below
/// 
/// In order to use this, the using crate MUST have regex and lazy_static listed as dependancies
pub(crate) fn validate_email(field: &syn::Field, tokens: &mut proc_macro2::TokenStream) {
    let name = &field.ident;
    tokens.extend(quote! {
        lazy_static! {
            static ref form_regex_email: Regex = Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").expect("failed to create email regex to validate form");
        }

        if !form_regex_email.is_match(self.#name) {
            v.push(ValidateError::InvalidEmail { field: stringify!(#name) })
        }
    });
}

/// Validates an email address using the regular expression below
/// 
/// In order to use this, the using crate MUST have regex and lazy_static listed as dependancies
pub(crate) fn validate_regex(field: &syn::Field, regex: &syn::LitStr, tokens: &mut proc_macro2::TokenStream) {
    let name = &field.ident;
    let r = regex.value();
    let mut rng = rand::thread_rng();
    let id = syn::Ident::new(&format!("form_regex_{}_{}", name.as_ref().unwrap().to_string(), rng.gen::<u32>()), Span::call_site());
    tokens.extend(quote! {
        lazy_static! {
            static ref #id: Regex = Regex::new(&#r).expect("failed to create email regex to validate form");
        }

        if !#id.is_match(self.#name) {
            v.push(ValidateError::InvalidRegex { field: stringify!(#name) })
        }
    });
}

/// Validates a string has a minimum length
pub(crate) fn validate_min_length(field: &syn::Field, min: &syn::LitInt, tokens: &mut proc_macro2::TokenStream) {
    let name = &field.ident;
    tokens.extend(quote! {
        if self.#name.len() < #min {
            v.push(ValidateError::InputTooShort { field: stringify!(#name), min: #min });
        }
    });
}

/// Validates a string has a maximum length
pub(crate) fn validate_max_length(field: &syn::Field, max: &syn::LitInt, tokens: &mut proc_macro2::TokenStream) {
    let name = &field.ident;
    tokens.extend(quote! {
        if self.#name.len() > #max {
            v.push(ValidateError::InputTooLong { field: stringify!(#name), max: #max });
        }
    });
}

/// Validates an integer has a minimum value
pub(crate) fn validate_min_value(field: &syn::Field, min: &syn::LitInt, tokens: &mut proc_macro2::TokenStream) {
    let name = &field.ident;
    tokens.extend(quote! {
        if self.#name < #min {
            v.push(ValidateError::TooSmall { field: stringify!(#name), min: #min });
        }
    });
}

/// Validates an integerr has a maximum value
pub(crate) fn validate_max_value(field: &syn::Field, max: &syn::LitInt, tokens: &mut proc_macro2::TokenStream) {
    let name = &field.ident;
    tokens.extend(quote! {
        if self.#name < #max {
            v.push(ValidateError::TooLarge { field: stringify!(#name), max: #max });
        }
    });
}
