//! Implemenation of the HtmlField container used when parsing a field in a struct with the #[derive(HtmlForm)] attribute

use crate::html::HtmlFieldAttribute;

use quote::{quote, ToTokens};

pub(crate) struct HtmlField {
    pub tag: String,
    pub name: String,
    pub attrs: Vec<HtmlFieldAttribute>,
}

impl HtmlField {
    /// Parses the #[html()] attribute when deriving an HtmlForm trait
    ///
    /// # Arguments
    ///
    /// * `attr` - Attribute to parse
    fn parse_html_attr(&mut self, attr: &syn::Attribute) {
        let meta = attr.parse_meta().expect(&format!(
            "HtmlForm - failed to parse html attribue for field {:?}",
            self.name
        ));

        let list = match meta {
            syn::Meta::List(ref list) => list,
            _ => panic!(
                "HtmlForm - failed to parse html attribute for field {:?} (meta)",
                self.name
            ),
        };

        for attr in list.nested.iter() {
            let attr = match attr {
                syn::NestedMeta::Meta(m) => m,
                _ => panic!(
                    "HtmlForm - failed to parse html sub-attribute for field {:?}",
                    self.name
                ),
            };

            match attr {
                syn::Meta::Word(ref ident) => self.add_value_attribute(ident.to_string()),
                syn::Meta::List(_) => panic!("{:?}: List", self.name),
                syn::Meta::NameValue(ref nv) => {
                    self.parse_pair_attribute(nv.ident.to_string(), &nv.lit)
                }
            }
        }
    }

    /// Parses the type of input to use.  Because `type` is a reserved keyword in Rust, a
    /// special attribute is required to set the type of the type of the input box.
    fn parse_html_type_attribute(&mut self, attr: &syn::Attribute) {
        let meta = attr.parse_meta().expect(&format!(
            "HtmlForm - failed to parse html attribue for field {:?}",
            self.name
        ));

        let list = match meta {
            syn::Meta::List(ref list) => list,
            _ => panic!(
                "HtmlForm - failed to parse html_type attribute for field {:?} (meta)",
                self.name
            ),
        };

        for attr in list.nested.iter() {
            let attr = match attr {
                syn::NestedMeta::Meta(m) => m,
                _ => panic!(
                    "HtmlForm - failed to parse html_type sub-attribute for field {:?}",
                    self.name
                ),
            };

            let field_attr = match attr {
                syn::Meta::Word(ref ident) => HtmlFieldAttribute::new_pair("type".to_string(), ident.to_string()),
                _ => panic!("HtmlForm - #[html_input_type(...)] requires only a keyword type (e.g., text, password)")
            };

            self.attrs.push(field_attr);
        }
    }

    /// Adds an pair-type attribue (e.g. class='input') to this field
    ///
    /// # Arguments
    ///
    /// * `attr` - Name of the attribute (e.g., "class")
    /// * `value` - Value of this attribute (e.g., "form-inline")
    #[allow(dead_code)]
    pub fn add_pair_attribute<S: Into<String>, P: Into<String>>(&mut self, attr: S, value: S) {
        self.attrs.push(HtmlFieldAttribute::new_pair(attr, value));
    }

    /// Parses a literal value received from the syntax tree, then converts it to a
    /// new HtmlFieldAttribute
    ///
    /// # Arguments
    ///
    /// * `attr` - Name of attribute
    /// * `lit` - Value of attribute to parse
    pub fn parse_pair_attribute(&mut self, attr: String, lit: &syn::Lit) {
        let attr = match lit {
            syn::Lit::Str(ref s) => HtmlFieldAttribute::new_pair(attr, s.value()),
            syn::Lit::Int(ref i) => HtmlFieldAttribute::new_pair(attr, format!("{}", i.value())),
            syn::Lit::Float(ref f) => HtmlFieldAttribute::new_pair(attr, format!("{}", f.value())),
            syn::Lit::Bool(ref b) => HtmlFieldAttribute::new_pair(attr, match b.value { true => "True", false => "False"}),
            _ => panic!("WebForms - failed to parse value for attribute `{}` - must be string, int, float or bool", attr),
        };

        self.attrs.push(attr);
    }

    /// Adds a new value-type attribute to this field
    ///
    /// # Arguments
    ///
    /// * `value` - Value of this attribute (e.g., "required")
    #[allow(dead_code)]
    pub fn add_value_attribute(&mut self, value: String) {
        self.attrs.push(HtmlFieldAttribute::new_value(value));
    }

    /// Creates a new HtmlField by parsing all attributes attached to the field
    pub fn parse(field: &syn::Field) -> HtmlField {
        let mut f = HtmlField::new(
            "input",
            field
                .ident
                .as_ref()
                .expect("HtmlFrom - requires named fields")
                .to_string(),
            vec![],
        );

        for attr in &field.attrs {
            if attr.path.is_ident("html") {
                f.parse_html_attr(attr);
            } else if attr.path.is_ident("html_input_type") {
                f.parse_html_type_attribute(attr);
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
    pub fn new<S: Into<String>, P: Into<String>>(
        tag: S,
        name: P,
        attrs: Vec<HtmlFieldAttribute>,
    ) -> HtmlField {
        HtmlField {
            tag: tag.into(),
            name: name.into(),
            attrs: attrs,
        }
    }

    pub fn tag<S: Into<String>, P: Into<String>>(tag: S, name: P) -> HtmlField {
        HtmlField::new(tag, name, vec![])
    }

    /// Creates a default form tag for the form
    ///
    /// # Arguments
    /// * `name` - Name of this form
    /// * `method` - What method to use (e.g, GET, POST) when submitting this form
    pub fn form<S: Into<String>>(name: S) -> HtmlField {
        HtmlField::new("form", name, vec![])
    }

    /// Creates a default submit field for a form
    pub fn submit() -> HtmlField {
        HtmlField::new(
            "input",
            "submit",
            vec![
                HtmlFieldAttribute::new_pair("type", "submit"),
                HtmlFieldAttribute::new_pair("value", "Submit"),
            ],
        )
    }
}

impl ToTokens for HtmlField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let tag = &self.tag;
        let name = &self.name;
        let attrs = &self.attrs;

        let mut fmt = "<{} name='{}'".to_owned();
        for _ in 0..self.attrs.len() {
            fmt.push_str(" {}");
        }
        fmt.push_str(">");

        tokens.extend(quote! {
            format!(#fmt, #tag, #name, #(#attrs),*)
        });
    }
}
