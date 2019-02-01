//! #[derive(HtmlForm) macro implementation

use crate::is_option;
use crate::proc_macro::TokenStream;
use lazy_static::lazy_static;
use quote::quote;
use syn;

mod html_defaults;
mod html_field;
mod html_struct;
mod html_validate;

use self::html_defaults::HtmlDefaults;
pub(crate) use self::html_field::HtmlField;
pub(crate) use self::html_struct::HtmlStruct;
pub(crate) use self::html_validate::HtmlValidate;

/// Lazily load the default configurations, if they exist
lazy_static! {
    static ref HTML_DEFAULTS: HtmlDefaults = HtmlDefaults::from_file("webforms_test/webforms.toml");
}

/// Implementation for the HtmlForm macro
pub(crate) fn impl_html_macro(ast: syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;

    let st = HtmlStruct::new(&ast);

    let fields = &st.fields;
    let field_names: Vec<&str> = st
        .fields
        .iter()
        .map(|f| match f.name {
            Some(ref n) => n,
            None => "Unknown",
        })
        .collect();

    let gen = quote! {
        impl #generics ::webforms::html::HtmlForm for #name #generics {

            fn form(&self) -> ::webforms::html::HtmlFormBuilder {
                let mut form = ::webforms::html::HtmlFormBuilder {
                    fields: std::collections::HashMap::new()
                };
                #(form.fields.insert(#field_names, #fields);)*
                form
            }
        }
    };

    gen.into()
}

fn html_input_type_parse_opt(args: &syn::PathArguments, default: &'static str) -> &'static str {
    // Read first arg in path arguments to get type
    let mut ret: &'static str = default;
    if let syn::PathArguments::AngleBracketed(ref brackets) = args {
        if let Some(f) = brackets.args.first() {
            if let syn::GenericArgument::Type(ref t) = f.value() {
                ret = html_input_type(t)
            }
        }
    }

    ret
}

/// Returns the appropriate input type attribute for a given
/// field in a struct deriving HtmlForm.  Returns a string
/// representing the input type to use.  If the type cannot be
/// detected, defaults to the `text` type
///
/// # Arguments
///
/// * `ty` - Type of field
pub(crate) fn html_input_type(ty: &syn::Type) -> &'static str {
    let opt = is_option(ty);

    match ty {
        syn::Type::Path(ref p) => match p.path.segments.last() {
            Some(ref r) if opt => html_input_type_parse_opt(&r.value().arguments, "text"),
            Some(ref r) if HTML_DEFAULTS.has_input_type(&r.value().ident) => {
                HTML_DEFAULTS.get_input_type(&r.value().ident)
            }
            Some(ref r) => {
                let ty = &r.value().ident;

                if ty == "i8"
                    || ty == "i16"
                    || ty == "i32"
                    || ty == "i64"
                    || ty == "i128"
                    || ty == "isize"
                    || ty == "u8"
                    || ty == "u16"
                    || ty == "u32"
                    || ty == "u64"
                    || ty == "u128"
                    || ty == "usize"
                {
                    "number"
                } else {
                    "text"
                }
            }
            None => "text",
        },
        syn::Type::Reference(ref r) => html_input_type(&r.elem),
        _ => "text",
    }
}
