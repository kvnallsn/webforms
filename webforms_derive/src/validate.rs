//! Validate macro implementation

use crate::proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use std::collections::HashMap;
use syn;

mod validators;

/// Various kinds of validation types we support along with
/// the necessary critera to validate the actual value
pub(crate) enum ValidateType {
    StringMin(syn::LitInt),
    StringMax(syn::LitInt),
    ValueMin(syn::LitInt),
    ValueMax(syn::LitInt),
    Regex(String),
}

/// Container for a given validation field and all
/// #[validate] attributes applied to it
pub(crate) struct ValidateFieldInfo<'a> {
    pub field: &'a syn::Field,
    pub attrs: Vec<ValidateType>,
    pub optional: bool,
}

impl<'a> ToTokens for ValidateFieldInfo<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        validators::write(self, tokens);
    }
}

/// Validation informatoin for the entire struct deriving
/// ValidateFrom
pub(crate) struct ValidateInfo<'a> {
    pub regex_tokens: HashMap<String, String>,
    pub fields: Vec<ValidateFieldInfo<'a>>,
}

impl<'a> ToTokens for ValidateInfo<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let fields = &self.fields;

        // If we're using a regex matcher, expand the regex using lazy_static
        // to ensure it only compiles once
        if self.regex_tokens.len() > 0 {
            let regex_tokens = self.regex_tokens.iter().map(|(id, regex)| {
                let rid = syn::Ident::new(&id, Span::call_site());
                quote! {
                    static ref #rid: Regex = Regex::new(&#regex).expect("failed to compile regex");
                }
            });

            tokens.extend(quote! {
                lazy_static! {
                    #(#regex_tokens)*
                }
            });
        }

        tokens.extend(quote! {
            #(#fields)*
        });
    }
}

fn parse_meta_attr(meta: &syn::Meta, struct_info: &mut ValidateInfo, info: &mut ValidateFieldInfo) {
    match meta {
        syn::Meta::Word(ref w) => parse_word_attr(w, struct_info, info),
        syn::Meta::List(ref l) => parse_list_attr(l, struct_info, info),
        syn::Meta::NameValue(ref nv) => parse_namevalue_attr(nv, struct_info, info),
    }
}

fn parse_word_attr(
    ident: &syn::Ident,
    struct_info: &mut ValidateInfo,
    info: &mut ValidateFieldInfo,
) {
    if ident == "email" {
        crate::validate::validators::validate_email(struct_info, info);
    } else if ident == "phone" {
        crate::validate::validators::validate_phone_number(struct_info, info);
    } else if ident == "optional" {
        info.optional = true;
    }
}

fn parse_list_attr(
    list: &syn::MetaList,
    struct_info: &mut ValidateInfo,
    info: &mut ValidateFieldInfo,
) {
    for nested in list.nested.iter() {
        match nested {
            syn::NestedMeta::Meta(m) => parse_meta_attr(m, struct_info, info),
            _ => {}
        };
    }
}

fn parse_namevalue_attr(
    nv: &syn::MetaNameValue,
    struct_info: &mut ValidateInfo,
    info: &mut ValidateFieldInfo,
) {
    if nv.ident == "min_length" {
        match nv.lit {
            syn::Lit::Int(ref i) => info.attrs.push(ValidateType::StringMin(i.clone())),
            _ => panic!("min_length requires an integer argument"),
        }
    } else if nv.ident == "max_length" {
        match nv.lit {
            syn::Lit::Int(ref i) => info.attrs.push(ValidateType::StringMax(i.clone())),
            _ => panic!("max_length requires an integer argument"),
        }
    } else if nv.ident == "min_value" {
        match nv.lit {
            syn::Lit::Int(ref i) => info.attrs.push(ValidateType::ValueMin(i.clone())),
            _ => panic!("min_value requires an integer argument"),
        }
    } else if nv.ident == "max_value" {
        match nv.lit {
            syn::Lit::Int(ref i) => info.attrs.push(ValidateType::ValueMax(i.clone())),
            _ => panic!("max_value requires an integer argument"),
        }
    } else if nv.ident == "regex" {
        match nv.lit {
            syn::Lit::Str(ref s) => {
                crate::validate::validators::validate_regex(s, struct_info, info)
            }
            _ => panic!("regex requires a string argument"),
        }
    } else {
        println!("Unknown ident: {}", nv.ident.to_string());
    }
}

pub(crate) fn impl_validate_macro(ast: syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    let fields = match ast.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(fields),
            ..
        }) => fields.named,
        _ => panic!("ValidateForm only defined on data structs!"),
    };

    let mut validate_info = ValidateInfo {
        regex_tokens: HashMap::new(),
        fields: vec![],
    };

    for field in fields.iter() {
        let mut info = ValidateFieldInfo {
            field: field,
            attrs: vec![],
            optional: false,
        };

        for attr in &field.attrs {
            if attr.path.is_ident("validate") {
                parse_meta_attr(
                    &attr
                        .parse_meta()
                        .expect("Failed to parse webform attribute"),
                    &mut validate_info,
                    &mut info,
                );
            }
        }

        validate_info.fields.push(info);
    }

    let gen = quote! {
        impl #generics ValidateForm for #name #generics {
            fn validate(&self) -> Result<(), Vec<ValidateError>> {

                let mut v: Vec<ValidateError> = Vec::new();

                #validate_info

                match v.len() {
                    0 => Ok(()),
                    _ => Err(v),
                }
            }
        }
    };

    gen.into()
}
