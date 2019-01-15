//! Implemenation of the attributes on an html input tag

use quote::{quote, ToTokens};

pub(crate) enum HtmlFieldAttribute {
    Value(String),
    Pair(String, String),
}

impl HtmlFieldAttribute {
    pub fn new_pair<S: Into<String>, P: Into<String>>(attr: S, value: P) -> HtmlFieldAttribute {
        HtmlFieldAttribute::Pair(attr.into(), value.into())
    }

    pub fn new_value<S: Into<String>>(value: S) -> HtmlFieldAttribute {
        HtmlFieldAttribute::Value(value.into())
    }
}

impl ToTokens for HtmlFieldAttribute {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            HtmlFieldAttribute::Value(s) => tokens.extend(quote! {
                format!("{}", #s)
            }),
            HtmlFieldAttribute::Pair(attr, value) => tokens.extend(quote! {
                format!("{}='{}'", #attr, #value)
            }),
        }
    }
}
