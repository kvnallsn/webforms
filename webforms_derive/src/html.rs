//! #[derive(HtmlForm) macro implementation

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

macro_rules! attribute_list {
    ($attr:expr) => {{
        let meta = $attr
            .parse_meta()
            .expect("HtmlForm - failed to parse html attribue for field");

        match meta {
            syn::Meta::List(ref list) => list.clone(),
            _ => panic!("HtmlForm - failed to parse html_type attribute for field (meta)"),
        }
    }};
}

mod html_field;
mod html_field_attribute;
mod html_struct;

pub(crate) use self::html_field::HtmlField;
pub(crate) use self::html_field_attribute::HtmlFieldAttribute;
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

            fn render_form(&self) -> String {
                #st
            }
        }
    };

    gen.into()
}
