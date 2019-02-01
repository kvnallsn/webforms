//! Macro implementations for WebForms
#![recursion_limit = "128"]

mod html;
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
#[proc_macro_derive(ValidateForm, attributes(validate, validate_regex, validate_match))]
pub fn validate_macro_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput =
        syn::parse(input).expect("failed to parse ValidateForm macro input");

    validate::impl_validate_macro(ast)
}

/// Derives the HtmlForm trait for a given struct
///
/// Will generate valid and complient HTML for a struct that can be used
/// with various templating languages (Tera, Askama, etc) to render forms
/// onto webpages
#[proc_macro_derive(HtmlForm, attributes(html_attrs, html_input, html_validate))]
pub fn html_macro_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).expect("failed to parse HtmlForm macro input");

    html::impl_html_macro(ast)
}

/// Parses an attribute list in the form #[attribute(list)] and applies the given
/// function to nested meta attributes
///
/// # Arguments
///
/// * `attr` - Attribute to parse
/// * `f` - Function to run over extracted meta arguments
pub(crate) fn parse_attribute_list<F>(attr: &syn::Attribute, mut f: F)
where
    F: FnMut(&syn::Meta),
{
    let meta = attr
        .parse_meta()
        .expect("HtmlForm - failed to parse html attribue for field");

    let list = match meta {
        syn::Meta::List(ref list) => list,
        _ => panic!("HtmlForm - failed to parse html_type attribute for field (meta)"),
    };

    for attr in list.nested.iter() {
        let attr = match attr {
            syn::NestedMeta::Meta(m) => m,
            _ => panic!("WebForms"),
        };

        f(attr);
    }
}

/// Detects whether a type is wrapped in a Option<>. Returns true
/// is the field is an option
///
/// # Arguments
///
/// * `type` - Type to determine if it's an option
pub(crate) fn is_option(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(ref p) => {
            let mut opt = false;
            for segment in p.path.segments.iter() {
                if segment.ident == "Option" {
                    opt = true;
                    break;
                }
            }
            opt
        }
        syn::Type::Reference(ref r) => is_option(&r.elem),
        _ => false,
    }
}
