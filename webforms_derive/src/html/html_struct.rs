//! Implemenation of the HtmlStruct container used when parsing a struct with the #[derive(HtmlForm)] attribute

use crate::html::{HtmlField, HtmlValidate};

pub(crate) struct HtmlStruct {
    pub name: String,
    pub fields: Vec<HtmlField>,
    pub validators: Vec<HtmlValidate>,
}

impl HtmlStruct {
    pub fn new(ast: &syn::DeriveInput) -> HtmlStruct {
        let name = ast.ident.to_string();
        let mut hs = HtmlStruct {
            name: name.clone(),
            fields: vec![],
            validators: vec![],
        };
        hs.parse(ast);
        hs
    }

    /// Parses a struct with the #[derive(HtmlForm)] attribute.  This is
    /// utlity method to parse all struct and field attributes.
    ///
    /// # Arguments
    ///
    /// * `ast` - The abstract syntax tree to parse
    fn parse(&mut self, ast: &syn::DeriveInput) {
        self.parse_struct_attributes(ast);
        self.parse_fields(ast);
        self.parse_validators(ast);
    }

    /// Parses any struct attributes that are attached to the struct
    /// deriving HtmlFrom
    ///
    /// # Arguments
    ///
    /// * `ast` - Abstract Syntax Tree of struct
    fn parse_struct_attributes(&mut self, ast: &syn::DeriveInput) {
        for attr in &ast.attrs {
            if attr.path.is_ident("html_regex") {}
        }
    }

    /// Parses all attributes applied to fields on the struct
    /// deriving HtmlForm
    ///
    /// # Arguments
    ///
    /// * `ast` - Abstract Syntax Tree of struct
    fn parse_fields(&mut self, ast: &syn::DeriveInput) {
        let fields = match ast.data {
            syn::Data::Struct(syn::DataStruct {
                fields: syn::Fields::Named(ref fields),
                ..
            }) => &fields.named,
            _ => panic!("HtmlForm only defined on data structs!"),
        };

        self.fields = fields
            .iter()
            .map(|field| HtmlField::parse(&field))
            .collect();
    }

    /// Parses and builds the validators that will be used after
    /// the form is submitted via the `validate(&self)` method
    ///
    /// # Arguments
    ///
    /// * `ast` - Abstract Syntax Tree of struct
    fn parse_validators(&mut self, ast: &syn::DeriveInput) {
        let fields = match ast.data {
            syn::Data::Struct(syn::DataStruct {
                fields: syn::Fields::Named(ref fields),
                ..
            }) => &fields.named,
            _ => panic!("HtmlForm only defined on data structs!"),
        };

        self.validators = fields
            .iter()
            .map(|field| HtmlValidate::parse(&field))
            .collect();
    }
}
