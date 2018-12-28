//! Validate macro implementation

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

fn parse_meta(field: &syn::Field, meta: &syn::Meta, tokens: &mut proc_macro2::TokenStream) {
    //match attr.parse_meta().expect("Unknown webform attribute") {
    match meta {
        syn::Meta::Word(ref w) => parse_word_attr(field, w, tokens),
        syn::Meta::List(ref l) => parse_list_attr(field, l, tokens),
        syn::Meta::NameValue(ref nv) => parse_namevalue_attr(field, nv, tokens),
    }
}

fn parse_word_attr(_field: &syn::Field, _ident: &syn::Ident, _tokens: &mut proc_macro2::TokenStream) {
    // Do nothing...for now
}

fn parse_list_attr(field: &syn::Field, list: &syn::MetaList, tokens: &mut proc_macro2::TokenStream) {
    for nested in list.nested.iter() {
        match nested {
            syn::NestedMeta::Meta(m) => parse_meta(field, m, tokens),
            _ => {},
        };
    }
}

fn parse_namevalue_attr(field: &syn::Field, nv: &syn::MetaNameValue, tokens: &mut proc_macro2::TokenStream) {
    let name = &field.ident;
    if nv.ident == "min_length" {
        match nv.lit {
            syn::Lit::Int(ref i) => tokens.extend(quote! {
                if self.#name.len() < #i {
                    v.push(ValidateError::InputTooShort { field: stringify!(#name), min: #i });
                }
            }),
            _ => panic!("min_length requires an integer argument")
        }
    } else if nv.ident == "max_length" {
        match nv.lit {
            syn::Lit::Int(ref i) => tokens.extend(quote! {
                if self.#name.len() > #i {
                    v.push(ValidateError::InputTooLong { field: stringify!(#name), max: #i });
                }
            }),
            _ => panic!("max_length requires an integer argument")
        }
    } else if nv.ident == "min_value" {
        match nv.lit {
            syn::Lit::Int(ref i) => tokens.extend(quote! {
                if self.#name < #i {
                    v.push(ValidateError::TooSmall{ field: stringify!(#name), min: #i });
                }
            }),
            _ => panic!("min_value requires an integer argument")
        }
    } else if nv.ident == "max_value" {
        match nv.lit {
            syn::Lit::Int(ref i) => tokens.extend(quote! {
                if self.#name > #i {
                    v.push(ValidateError::TooLarge { field: stringify!(#name), max: #i });
                }
            }),
            _ => panic!("max_value requires an integer argument")
        }
    } else {
        println!("Unknown ident: {}", nv.ident.to_string());
    }
}

pub(crate) fn impl_validate_macro(ast: syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = match ast.data {
        syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(fields), .. }) => fields.named,
        _ => panic!("ValidateForm only defined on data structs!"),
    };

    let reqs = fields.iter().map(|field| {
        let mut tokens = proc_macro2::TokenStream::new();
        for attr in &field.attrs {
            if attr.path.is_ident("validate") {
                parse_meta(field, &attr.parse_meta().expect("Unknown web form attribute"), &mut tokens);
            }
        }
        tokens
    });

    let gen = quote! {
        use webforms::ValidateError;

        impl<'a> ValidateForm for #name<'a> {
            fn validate(&self) -> Result<(), Vec<ValidateError>> {
                let mut v: Vec<ValidateError> = Vec::new();

                #(#reqs)*

                println!("{:?}", v);
                Err(v)
            }
        }
    };

    gen.into()
}
