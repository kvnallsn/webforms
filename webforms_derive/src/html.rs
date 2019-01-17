//! #[derive(HtmlForm) macro implementation

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

mod html_field;
mod html_struct;

pub(crate) use self::html_field::HtmlField;
pub(crate) use self::html_struct::HtmlStruct;

/// Implementation for the HtmlForm macro
pub(crate) fn impl_html_macro(ast: syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;

    let st = HtmlStruct::new(&ast);

    let gen = quote! {
        impl #generics HtmlForm for #name #generics {
            fn render_field<S: AsRef<str>>(&self, field: S) -> String {
                "".to_owned()
            }

            fn render_form(&self) -> &'static str {
                #st
            }
        }
    };

    gen.into()
}
