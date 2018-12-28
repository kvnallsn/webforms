//! Macro implementations for WebForms

mod validate;
extern crate proc_macro;

use crate::proc_macro::TokenStream;
use syn;


#[proc_macro_derive(ValidateForm, attributes(validate))]
pub fn validate_macro_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).expect("failed to parse ValidateForm macro input");

    validate::impl_validate_macro(ast)
}
