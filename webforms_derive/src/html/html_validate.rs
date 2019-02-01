//! Handles the html validation attribute

use crate::{is_option, parse_attribute_list};

/// All possible validators supported.
enum Validator {
    MinValue(u64),
    MaxValue(u64),
    MinFloat(f64),
    MaxFloat(f64),
    MinLength(u64),
    MaxLength(u64),
    Pattern(String),
}

pub(crate) struct HtmlValidate {
    name: String,
    validators: Vec<Validator>,
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
            name: field
                .ident
                .as_ref()
                .expect("HtmlForm - requires named fields")
                .to_string(),
            validators: vec![],
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
                                    syn::Lit::Int(ref i) => Validator::MinValue(i.value()),
                                    syn::Lit::Float(ref f) => Validator::MinFloat(f.value()),
                                    _ => panic!("WebForms - #[html_validate] min specifier requires an int or float argument"),
                            });
                        } else if nv.ident == "max" {
                            validator.add_validator(
                                match nv.lit {
                                    syn::Lit::Int(ref i) => Validator::MaxValue(i.value()),
                                    syn::Lit::Float(ref f) => Validator::MaxFloat(f.value()),
                                    _ => panic!("WebForms - #[html_validate] max specifier requires an int or float argument"),
                            });
                        } else if nv.ident == "minlength" {
                            validator.add_validator(
                                match nv.lit {
                                    syn::Lit::Int(ref i) => Validator::MinLength(i.value()),
                                    _ => panic!("WebForms - #[html_validate] minlength specifier requires an int argument"),
                            });
                        } else if nv.ident == "maxlength" {
                            validator.add_validator(
                                match nv.lit {
                                    syn::Lit::Int(ref i) => Validator::MaxLength(i.value()),
                                    _ => panic!("WebForms - #[html_validate] maxlength specifier requires an int argument"),
                            });
                        } else if nv.ident == "pattern" {
                            validator.add_validator(
                                match nv.lit {
                                    syn::Lit::Str(ref s) => Validator::Pattern(s.value()),
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
    fn add_validator(&mut self, v: Validator) {
        self.validators.push(v);
    }
}
