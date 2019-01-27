//! Implemenation of the HtmlField container used when parsing a field in a struct with the #[derive(HtmlForm)] attribute

use crate::{html::html_input_type, is_option, parse_attribute_list};
use quote::{quote, ToTokens};
use std::collections::{HashMap, HashSet};

pub(crate) struct HtmlField {
    pub tag: String,
    pub name: Option<String>,
    pub pair_attrs: HashMap<String, String>,
    pub value_attrs: HashSet<String>,
}

impl HtmlField {
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
        let mut f = HtmlField::input(
            field
                .ident
                .as_ref()
                .expect("HtmlForm - requires named fields")
                .to_string(),
            &field.ty,
        );

        for attr in &field.attrs {
            if attr.path.is_ident("html") {
                parse_attribute_list(attr, |meta| match meta {
                    syn::Meta::Word(ref ident) => f.add_value_attribute(ident.to_string()),
                    syn::Meta::List(_) => panic!(""),
                    syn::Meta::NameValue(ref nv) => {
                        f.parse_pair_attribute(nv.ident.to_string(), &nv.lit)
                    }
                });
            } else if attr.path.is_ident("html_input_type") {
                parse_attribute_list(attr, |meta| {
                    match meta {
                        syn::Meta::Word(ref ident) => f.add_pair_attribute("type", ident.to_string()),
                        _ => panic!("HtmlForm - #[html_input_type(...)] requires only a keyword type (e.g., text, password)")
                    }
                });
            }
        }

        f
    }

    /// Creates a new HtmlField manually, by specifying a name for the element to have
    /// Note: This name should NOT conflict with other names that may be parsed via
    /// `parse`
    ///
    /// # Arguments
    ///
    /// * `tag` - HTML tag to use for this field
    /// * `name` - Name of this field
    /// * `attrs` - Vector of HtmlFieldAttributes
    pub fn with_name<S: Into<String>, P: Into<String>>(tag: S, name: P) -> HtmlField {
        HtmlField {
            tag: tag.into(),
            name: Some(name.into()),
            pair_attrs: HashMap::new(),
            value_attrs: HashSet::new(),
        }
    }

    pub fn new<S: Into<String>>(tag: S) -> HtmlField {
        HtmlField {
            tag: tag.into(),
            name: None,
            pair_attrs: HashMap::new(),
            value_attrs: HashSet::new(),
        }
    }

    pub fn tag<S: Into<String>>(tag: S) -> HtmlField {
        HtmlField::new(tag)
    }

    pub fn input<S: Into<String>>(name: S, ty: &syn::Type) -> HtmlField {
        let mut field = HtmlField::with_name("input", name);
        field.add_pair_attribute("type", html_input_type(ty));
        if !is_option(ty) {
            field.add_value_attribute("required");
        }

        field
    }

    /// Creates a default form tag for the form
    ///
    /// # Arguments
    /// * `name` - Name of this form
    /// * `method` - What method to use (e.g, GET, POST) when submitting this form
    pub fn form<S: Into<String>>(_name: S) -> HtmlField {
        HtmlField::tag("form")
    }

    /// Creates a default submit field for a form
    pub fn submit() -> HtmlField {
        HtmlField::tag("input")
    }

    /// Writes this HTML field/tag to the specified vector
    ///
    /// # Arguments
    ///
    /// * `w` - Vector of u8's to write to
    /// * `newline` - True to print a newline character after the tag
    /// * `indent` - True to indent (via a tab) this field
    pub fn write(&self, writer: &mut std::io::Write, newline: bool, indent: bool) {
        if indent {
            write!(writer, "\t").unwrap();
        }

        write!(writer, "<{}", &self.tag).unwrap();
        if let Some(name) = &self.name {
            write!(writer, " name='{}'", name).unwrap();
        }

        self.pair_attrs
            .iter()
            .for_each(|(attr, value)| write!(writer, " {}='{}'", attr, value).unwrap());

        self.value_attrs
            .iter()
            .for_each(|value| write!(writer, " {}", value).unwrap());

        write!(writer, ">").unwrap();
        if newline {
            write!(writer, "\n").unwrap();
        }
    }
}

impl ToTokens for HtmlField {
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

        tokens.extend(quote! {{
            let attrs = ::webforms::attrs!(#(#pairs),*);
            ::webforms::html::HtmlFieldBuilder::with_attrs(#tag, #name, attrs)
        }})
    }
}
