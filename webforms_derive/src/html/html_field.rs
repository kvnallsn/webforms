//! Implemenation of the HtmlField container used when parsing a field in a struct with the #[derive(HtmlForm)] attribute

use crate::{
    html::{html_input_type, HtmlValidate},
    is_option, parse_attribute_list,
};
use quote::{quote, ToTokens};
use std::collections::{HashMap, HashSet};

pub(crate) struct HtmlField<'a> {
    pub ident: &'a Option<proc_macro2::Ident>,
    pub tag: String,
    pub name: Option<String>,
    pub pair_attrs: HashMap<String, String>,
    pub value_attrs: HashSet<String>,
    pub validators: Vec<HtmlValidate<'a>>,
    pub optional: bool,
}

impl<'a> HtmlField<'a> {
    /// Creates a new HtmlField manually, by specifying a name for the element to have
    /// Note: This name should NOT conflict with other names that may be parsed via
    /// `parse`
    ///
    /// # Arguments
    ///
    /// * `tag` - HTML tag to use for this field
    /// * `name` - Name of this field
    /// * `attrs` - Vector of HtmlFieldAttributes
    pub fn with_name<S: Into<String>>(tag: S, field: &syn::Field) -> HtmlField {
        let name = field.ident.as_ref().map(|i| i.to_string());

        HtmlField {
            ident: &field.ident,
            tag: tag.into(),
            name: name,
            pair_attrs: HashMap::new(),
            value_attrs: HashSet::new(),
            validators: Vec::new(),
            optional: is_option(&field.ty),
        }
    }

    pub fn input(field: &syn::Field) -> HtmlField {
        let mut html_field = HtmlField::with_name("input", field);
        html_field.add_pair_attribute("type", html_input_type(&field.ty));
        if !html_field.optional {
            html_field.add_value_attribute("required");
        }

        html_field
    }

    /// Adds an pair-type attribue (e.g. class='input') to this field
    ///
    /// # Arguments
    ///
    /// * `attr` - Name of the attribute (e.g., "class")
    /// * `value` - Value of this attribute (e.g., "form-inline")
    pub fn add_pair_attribute<S: Into<String>, P: Into<String>>(&mut self, attr: S, value: P) {
        let attr = attr.into();
        let value = value.into();
        self.pair_attrs
            .entry(attr)
            .and_modify(|v| *v = value.clone())
            .or_insert(value);
    }

    /// Parses a literal value received from the syntax tree, then converts it to a
    /// new HtmlFieldAttribute
    ///
    /// # Arguments
    ///
    /// * `attr` - Name of attribute
    /// * `lit` - Value of attribute to parse
    pub fn parse_pair_attribute(&mut self, attr: String, lit: &syn::Lit) {
        let value = match lit {
            syn::Lit::Str(ref s) => s.value(),
            syn::Lit::Int(ref i) => format!("{}", i.value()),
            syn::Lit::Float(ref f) => format!("{}", f.value()),
            //syn::Lit::Bool(ref b) => match b.value { true => "True", false => "False"}),
            _ => panic!("WebForms - failed to parse value for attribute `{}` - must be string, int, float or bool", attr),
        };

        //self.attrs.push(attr);
        self.add_pair_attribute(attr, value);
    }

    /// Adds a new value-type attribute to this field
    ///
    /// # Arguments
    ///
    /// * `value` - Value of this attribute (e.g., "required")
    pub fn add_value_attribute<S: Into<String>>(&mut self, value: S) {
        //self.attrs.push(HtmlFieldAttribute::new_value(value));
        self.value_attrs.insert(value.into());
    }

    /// Creates a new HtmlField by parsing all attributes attached to the field
    pub fn parse(field: &syn::Field) -> HtmlField {
        let mut f = HtmlField::input(field);

        for attr in &field.attrs {
            if attr.path.is_ident("html_attrs") {
                // Applies the list of attributes to this tag
                parse_attribute_list(attr, |meta| match meta {
                    syn::Meta::Word(ref ident) => f.add_value_attribute(ident.to_string()),
                    syn::Meta::List(_) => panic!(""),
                    syn::Meta::NameValue(ref nv) => {
                        f.parse_pair_attribute(nv.ident.to_string(), &nv.lit)
                    }
                });
            } else if attr.path.is_ident("html_input") {
                // Parses the #[html_input] attribute.  This attribute controls the
                // <input> tag for the form.  The first argument MUST be a type
                // (e.g., number, text, etc.) as specified in the html spec.  The rest
                // of the arguments are attributes that will be applied to the tag.

                let meta = attr
                    .parse_meta()
                    .expect("HtmlForm - failed to parse html attribue for field");

                let list = match meta {
                    syn::Meta::List(ref list) => list,
                    _ => panic!("HtmlForm - failed to parse html_type attribute for field (meta)"),
                };

                // First argument is required to be the input field type
                match list.nested.first() {
                    Some(p) => match p.value() {
                        syn::NestedMeta::Meta(m) => match m {
                            syn::Meta::Word(ref ty) => {
                                f.add_pair_attribute("type", ty.to_string());
                            }
                            _ => panic!(
                                "HtmlForm - #[html_input] requires first argument to be type"
                            ),
                        },
                        _ => panic!("HtmlForm - #[html_input] invalid first argument"),
                    },
                    None => panic!(
                        "HtmlForm - #[html_input] requires at least one argument (input type)"
                    ),
                }

                // Parse rest of list as normal
                for attr in list.nested.iter().skip(1) {
                    let attr = match attr {
                        syn::NestedMeta::Meta(m) => m,
                        _ => panic!(
                            "HtmlForms - #[html_input] - invalid syntax after first argument"
                        ),
                    };

                    match attr {
                        syn::Meta::Word(ref ident) => f.add_value_attribute(ident.to_string()),
                        syn::Meta::List(_) => {
                            panic!("HtmlForms - #[html_input] Nested lists not allowed")
                        }
                        syn::Meta::NameValue(ref nv) => {
                            f.parse_pair_attribute(nv.ident.to_string(), &nv.lit)
                        }
                    }
                }
            } else if attr.path.is_ident("html_validate") {
                // Parses the validation critera and inserts what is available into the
                // input tag.  Not all name/value pairs are supported in html.  Those
                // that are not are quietly ignored here
                parse_attribute_list(attr, |meta| match meta {
                    syn::Meta::Word(_) => {}
                    syn::Meta::List(_) => {}
                    syn::Meta::NameValue(ref nv) => {
                        // First handle setting the right attributes for the html tag itself
                        if nv.ident == "min"
                            || nv.ident == "max"
                            || nv.ident == "maxlength"
                            || nv.ident == "pattern"
                        {
                            let val = match nv.lit {
                                syn::Lit::Int(ref i) => format!("{}", i.value()),
                                syn::Lit::Float(ref f) => format!("{}", f.value()),
                                syn::Lit::Str(ref s) => s.value(),
                                _ => panic!("WebForms - #[html_validate] invalid min/max/maxlength/pattern attribute on field '{}'", ""),
                            };
                            f.add_pair_attribute(nv.ident.to_string(), val);
                        } else if nv.ident == "regex" {
                            // This is a pre-compiled regex, look to struct info to load
                        }

                        // Next build the validators that can be run server side
                    }
                });
            }
        }

        f
    }
}

impl<'a> ToTokens for HtmlField<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let tag = &self.tag;
        let name = match self.name {
            Some(ref n) => quote! { Some(#n) },
            None => quote! { None },
        };
        let pairs: Vec<_> = self
            .pair_attrs
            .iter()
            .map(|(k, v)| {
                quote! {#k => #v}
            })
            .collect();

        let values: Vec<_> = self.value_attrs.iter().collect();

        tokens.extend(quote! {{
            let mut attrs = ::webforms::attrs!(#(#pairs),*);
            #(attrs.insert(::webforms::html::HtmlAttribute::new_single(#values));)*
            ::webforms::html::HtmlFieldBuilder::with_attrs(#tag, #name, attrs)
        }})
    }
}
