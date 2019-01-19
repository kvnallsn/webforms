//! #[derive(HtmlForm) macro implementation

use crate::proc_macro::TokenStream;
use lazy_static::lazy_static;
use quote::quote;
use std::collections::HashSet;
use syn;

mod html_defaults;
mod html_field;
mod html_struct;

use self::html_defaults::HtmlDefault;
pub(crate) use self::html_field::HtmlField;
pub(crate) use self::html_struct::HtmlStruct;

/// Lazily load the default configurations, if they exist
lazy_static! {
    static ref HTML_DEFAULTS: HashSet<HtmlDefault> =
        HtmlDefault::from_file("webforms_test/webforms.toml");
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
    let form_str = st.write();

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
