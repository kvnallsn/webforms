//! Handles the html validation attribute

use crate::{is_option, parse_attribute_list};
use quote::{quote, ToTokens};
use std::collections::HashMap;

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
    errors: HashMap<&'static str, String>,
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
            errors: HashMap::new(),
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
            } else if attr.path.is_ident("html_error") {
                parse_attribute_list(attr, |meta| match meta {
                    syn::Meta::Word(_) => {}
                    syn::Meta::List(_) => {}
                    syn::Meta::NameValue(ref nv) => {
                        if nv.ident == "min" {
                            match nv.lit {
                                syn::Lit::Str(ref s) => {
                                    validator.add_error_msg("min", s.value());
                                }
                                _ => {}
                            }
                        } else if nv.ident == "max" {
                            match nv.lit {
                                syn::Lit::Str(ref s) => {
                                    validator.add_error_msg("max", s.value());
                                }
                                _ => {}
                            }
                        } else if nv.ident == "minlength" {
                            match nv.lit {
                                syn::Lit::Str(ref s) => {
                                    validator.add_error_msg("minlength", s.value());
                                }
                                _ => {}
                            }
                        } else if nv.ident == "maxlength" {
                            match nv.lit {
                                syn::Lit::Str(ref s) => {
                                    validator.add_error_msg("maxlength", s.value());
                                }
                                _ => {}
                            }
                        } else if nv.ident == "pattern" {
                            match nv.lit {
                                syn::Lit::Str(ref s) => {
                                    validator.add_error_msg("pattern", s.value());
                                }
                                _ => {}
                            }
                        }
                    }
                });
            }
        }

        validator
    }

    /// Adds a validator to this Validation container
    fn add_validator(&mut self, v: Validator) {
        self.validators.push(v);
    }

    /// Adds an error message to the hash map
    fn add_error_msg(&mut self, key: &'static str, msg: String) {
        self.errors.insert(key, msg);
    }
}

impl Validator {
    /// Converts this validator to a TokenStream that can be inserted
    /// into the derived trait.  If the field is an optional field,
    /// it will properly destructure for the comparison
    ///
    /// # Arguments
    /// * `name` - Currently unused
    /// * `optional` - True if this is an optional type, false otherwise
    pub fn write(
        &self,
        _name: &Option<proc_macro2::Ident>,
        optional: bool,
        errors: &HashMap<&'static str, String>,
    ) -> proc_macro2::TokenStream {
        // If the type is optional, validate as such
        let field = match optional {
            true => quote! {opt},
            false => quote! {x},
        };

        let err_msg = self.get_error(errors);

        let cond = match self {
            Validator::MinValue(i) => quote! { #field >= &#i },
            Validator::MinFloat(f) => quote! { #field >= &#f },
            Validator::MaxValue(i) => quote! { #field <= &#i },
            Validator::MaxFloat(f) => quote! { #field <= &#f },
            Validator::MinLength(i) => quote! {#field.len() >= #i},
            Validator::MaxLength(i) => quote! {#field.len() <= #i},
            Validator::Pattern(s) => quote! { true },
        };

        let check = quote! {
            match #cond {
                true => Ok(()),
                false => Err(#err_msg),
            }
        };

        match optional {
            true => quote! {
                match &x {
                    Some(opt) => {#check},
                    None => Ok(()),
                }
            },
            false => check,
        }
    }

    fn get_name(&self) -> &'static str {
        match self {
            Validator::MinValue(_) => "min",
            Validator::MinFloat(_) => "min",
            Validator::MaxValue(_) => "max",
            Validator::MaxFloat(_) => "max",
            Validator::MinLength(_) => "minlength",
            Validator::MaxLength(_) => "maxlength",
            Validator::Pattern(_) => "pattern",
        }
    }

    fn get_error(&self, map: &HashMap<&'static str, String>) -> String {
        match map.get(self.get_name()) {
            Some(v) => v.clone(),
            None => self.default_error(),
        }
    }

    /// Default error strings to use when validation fails.  These
    /// can be overriden using the #[html_error] attribute
    fn default_error(&self) -> String {
        match self {
            Validator::MinValue(i) => format!("Minimum value is {}", i.value()),
            Validator::MinFloat(f) => format!("Minimum value is {}", f.value()),
            Validator::MaxValue(i) => format!("Maximum value is {}", i.value()),
            Validator::MaxFloat(f) => format!("Maximum value is {}", f.value()),
            Validator::MinLength(i) => format!("Must be at least {} characters long", i.value()),
            Validator::MaxLength(i) => format!("Maximum length is {}", i.value()),
            Validator::Pattern(s) => format!("Did not match pattern: {}", s.value()),
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
            .map(|v| v.write(name, self.optional, &self.errors))
            .collect();

        let ts = quote! { ::webforms::html::FieldValidator::new(stringify!(#name), vec![#(Box::new(&|x| #v)),*]) };
        //println!("{}", ts);
        tokens.extend(ts);
    }
}
