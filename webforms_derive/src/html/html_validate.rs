//! Handles the html validation attribute

use crate::{is_option, parse_attribute_list};
use quote::{quote, ToTokens};

#[derive(Clone)]
enum Validator {
    MinValue(syn::LitInt),
    MaxValue(syn::LitInt),
    MinFloat(syn::LitFloat),
}

#[derive(Clone)]
pub(crate) struct HtmlValidate {
    name: Option<proc_macro2::Ident>,
    validators: Vec<proc_macro2::TokenStream>,
    validators2: Vec<Validator>,
    optional: bool,
}

impl HtmlValidate {
    /// Creates a new HtmlField by parsing all attributes attached to the field
    ///
    /// Arguments
    ///
    /// * `field` - Field to parse validators from
    pub fn parse(field: &syn::Field) -> HtmlValidate {
        let mut validator = HtmlValidate {
            name: field.ident.clone(),
            validators: vec![],
            validators2: Vec::new(),
            optional: is_option(&field.ty),
        };

        // Parse the attribute list on this field, looking for the following attributes:
        // * #[html_validate] - Validation criterea for this field
        for attr in &field.attrs {
            if attr.path.is_ident("html_validate") {
                parse_attribute_list(attr, |meta| match meta {
                    syn::Meta::Word(_) => {}
                    syn::Meta::List(_) => {}
                    syn::Meta::NameValue(ref nv) => {
                        if nv.ident == "min" {
                            validator.add_validator2(
                                match nv.lit {
                                    syn::Lit::Int(ref i) => Validator::MinValue(i.clone()),
                                    syn::Lit::Float(ref f) => Validator::MinFloat(f.clone()),
                                    _ => panic!("WebForms - #[html_validate] min specifier requires an int or float argument"),
                            });
                        } else if nv.ident == "max" {
                            validator.add_validator(
                                match nv.lit {
                                    syn::Lit::Int(ref i) => quote! { ::webforms::html::Validator::MaxValue(#i) },
                                    syn::Lit::Float(ref f) => quote! { ::webforms::html::Validator::MaxValue(#f) },
                                    _ => panic!("WebForms - #[html_validate] max specifier requires an int or float argument"),
                            });
                        } else if nv.ident == "minlength" {
                            validator.add_validator(
                                match nv.lit {
                                    syn::Lit::Int(ref i) => quote! { ::webforms::html::Validator::MinLength(#i) },
                                    _ => panic!("WebForms - #[html_validate] minlength specifier requires an int argument"),
                            });
                        } else if nv.ident == "maxlength" {
                            validator.add_validator(
                                match nv.lit {
                                    syn::Lit::Int(ref i) => quote! { ::webforms::html::Validator::MaxLength(#i) },
                                    _ => panic!("WebForms - #[html_validate] maxlength specifier requires an int argument"),
                            });
                        } else if nv.ident == "pattern" {
                            validator.add_validator(
                                match nv.lit {
                                    syn::Lit::Str(ref s) => quote! { ::webforms::html::Validator::Pattern(#s) },
                                    _ => panic!("WebForms - #[html_validate] pattern specifier requires an string or regex argument"),
                            });
                        }
                    }
                });
            }
        }

        validator
    }

    /// Adds a validator to this HtmlValidate struct
    ///
    /// # Arguments
    ///
    /// * `v` - Validator (from Validator Enums) to add
    fn add_validator(&mut self, t: proc_macro2::TokenStream) {
        self.validators.push(t);
    }

    fn add_validator2(&mut self, v: Validator) {
        self.validators2.push(v);
    }
}

impl Validator {
    pub fn write(&self, field: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        match self {
            Validator::MinValue(i) => quote! { #field.clone() > #i },
            Validator::MinFloat(f) => quote! { #field.clone() > #f },
            Validator::MaxValue(i) => quote! { #field.clone() < #i },
            _ => panic!("TESDF"),
        }
    }
}

impl ToTokens for Validator {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {}
}

impl ToTokens for HtmlValidate {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let field = match self.optional {
            true => quote! { self.#name.unwrap() },
            false => quote! { self.#name },
        };

        let v: Vec<_> = self.validators2.iter().map(|v| v.write(&field)).collect();

        //tokens.extend(quote! { ::webforms::html::HtmlValidator::with_fn(|| true); });
        tokens.extend(
            quote! { ::webforms::html::HtmlValidator::with_fns(vec![#(Box::new(&|| #v)),*]) },
        );
        //tokens.extend(quote! {::webforms::html::HtmlValidator::with_validators(vec![#(#v,)*])});
    }
}
