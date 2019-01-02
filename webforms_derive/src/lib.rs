//! Macro implementations for WebForms
#![recursion_limit = "128"]

mod validate;
extern crate proc_macro;

use crate::proc_macro::TokenStream;
use syn;

/// Derives the ValidateForm trait from for a given struct
///
/// Different types have different available validate tags.  Using an
/// invalid attribute tag on an type (e.g., max_length on an int type)
/// will cause the compiler to panic.
///
/// Type: String
/// * `min_length` - Minimum length of the string
/// * `max_length` - Maximum length of the string
/// * `regex` - Input must match the supplied regular expression
/// * `email` - Special regex to validate an email address
///
/// Using either the `regex` or `email` attributes requires your crate
/// to depend on both the regex and lazy_static crates.  lazy_static is
/// required to minimize the number of times a given regex is compiled
///
/// Type: Integer
/// * `min_value` - Minimum value of this int
/// * `max_value` - Maxium value of this int
///
/// # Example
///
/// ```compile_fail
/// #[derive(ValidateForm)]
/// struct LoginForm {
///     /// Username must between 4 and 16 characters
///     #[validate(min_length = 4)]
///     #[validate(max_length = 16)]
///     pub username: String,
///
///     ///Email must conform to the email regex provided
///     #[validate(email)]
///     pub email: String,
/// }
/// ```
#[proc_macro_derive(ValidateForm, attributes(validate, validate_regex))]
pub fn validate_macro_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput =
        syn::parse(input).expect("failed to parse ValidateForm macro input");

    validate::impl_validate_macro(ast)
}
