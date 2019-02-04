//! Handles the html validation attribute

use crate::{is_option, parse_attribute_list};
use quote::{quote, ToTokens};

#[derive(Clone)]
enum Validator {
    MinValue(syn::LitInt),
    MaxValue(syn::LitInt),
    MinFloat(syn::LitFloat),
    MaxFloat(syn::LitFloat),
    MinLength(syn::LitInt),
    MaxLength(syn::LitInt),
    Pattern(syn::LitStr),
}

#[derive(Clone)]
pub(crate) struct HtmlValidate<'a> {
    name: Option<proc_macro2::Ident>,
    ty: &'a syn::Type,
    validators: Vec<Validator>,
    optional: bool,
}

impl<'a> HtmlValidate<'a> {
    /// Creates a new HtmlField by parsing all attributes attached to the field
    ///
    /// Arguments
    ///
    /// * `field` - Field to parse validators from
    pub fn parse(field: &'a syn::Field) -> HtmlValidate<'a> {
        let mut validator = HtmlValidate {
            name: field.ident.clone(),
            ty: &field.ty,
            validators: Vec::new(),
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
                            validator.add_validator(
                                match nv.lit {
                                    syn::Lit::Int(ref i) => Validator::MinValue(i.clone()),
                                    syn::Lit::Float(ref f) => Validator::MinFloat(f.clone()),
                                    _ => panic!("WebForms - #[html_validate] min specifier requires an int or float argument"),
                            });
                        } else if nv.ident == "max" {
                            validator.add_validator(
                                match nv.lit {
                                    syn::Lit::Int(ref i) => Validator::MaxValue(i.clone()),
                                    syn::Lit::Float(ref f) => Validator::MaxFloat(f.clone()),
                                    _ => panic!("WebForms - #[html_validate] max specifier requires an int or float argument"),
                            });
                        } else if nv.ident == "minlength" {
                            validator.add_validator(
                                match nv.lit {
                                    syn::Lit::Int(ref i) => Validator::MinLength(i.clone()),
                                    _ => panic!("WebForms - #[html_validate] minlength specifier requires an int argument"),
                            });
                        } else if nv.ident == "maxlength" {
                            validator.add_validator(
                                match nv.lit {
                                    syn::Lit::Int(ref i) => Validator::MaxLength(i.clone()),
                                    _ => panic!("WebForms - #[html_validate] maxlength specifier requires an int argument"),
                            });
                        } else if nv.ident == "pattern" {
                            validator.add_validator(
                                match nv.lit {
                                    syn::Lit::Str(ref s) => Validator::Pattern(s.clone()),
                                    _ => panic!("WebForms - #[html_validate] pattern specifier requires an string or regex argument"),
                            });
                        }
                    }
                });
            }
        }

        validator
    }

    fn add_validator(&mut self, v: Validator) {
        self.validators.push(v);
    }
}

impl Validator {
    pub fn write(
        &self,
        _name: &Option<proc_macro2::Ident>,
        optional: bool,
    ) -> proc_macro2::TokenStream {
        // If the type is optional, validate as such
        let field = match optional {
            true => quote! {opt},
            false => quote! {x},
        };

        let cond = match self {
            Validator::MinValue(i) => quote! { #field >= &#i },
            Validator::MinFloat(f) => quote! { #field >= &#f },
            Validator::MaxValue(i) => quote! { #field <= &#i },
            Validator::MaxFloat(f) => quote! { #field <= &#f },
            Validator::MinLength(i) => quote! {#field.len() >= #i},
            Validator::MaxLength(i) => quote! {#field.len() <= #i},
            Validator::Pattern(s) => quote! { true },
        };

        match optional {
            true => quote! {
                match &x {
                    Some(opt) => {#cond},
                    None => true,
                }
            },
            false => cond,
        }
    }
}

impl<'a> ToTokens for HtmlValidate<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        //let ty = self.ty;
        let v: Vec<_> = self
            .validators
            .iter()
            .map(|v| v.write(name, self.optional))
            .collect();

        let ts = quote! { vec![#(Box::new(&|x| #v)),*] };
        //println!("{}", ts);
        tokens.extend(ts);
    }
}
