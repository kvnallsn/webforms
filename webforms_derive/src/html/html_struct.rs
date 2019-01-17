//! Implemenation of the HtmlStruct container used when parsing a struct with the #[derive(HtmlForm)] attribute

use crate::{html::HtmlField, parse_attribute_list};
use quote::{quote, ToTokens};
use std::io::Write;

pub(crate) struct HtmlStruct {
    pub name: String,
    pub form: HtmlField,
    pub fields: Vec<HtmlField>,
    pub sumbit: HtmlField,
}

impl HtmlStruct {
    pub fn new(ast: &syn::DeriveInput) -> HtmlStruct {
        let name = ast.ident.to_string();
        let mut hs = HtmlStruct {
            name: name.clone(),
            form: HtmlField::form(name),
            fields: vec![],
            sumbit: HtmlField::submit(),
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
    }

    /// Parses any struct attributes that are attached to the struct
    /// deriving HtmlFrom
    ///
    /// # Arguments
    ///
    /// * `ast` - Abstract Syntax Tree of struct
    fn parse_struct_attributes(&mut self, ast: &syn::DeriveInput) {
        for attr in &ast.attrs {
            if attr.path.is_ident("html_form") {
                let mut form_field = HtmlField::tag("form");
                parse_attribute_list(attr, |meta| match meta {
                    syn::Meta::NameValue(ref nv) => {
                        form_field.parse_pair_attribute(nv.ident.to_string(), &nv.lit)
                    }
                    _ => panic!("WebForms - Failed to parse html_form attribute"),
                });
                self.form = form_field;
            } else if attr.path.is_ident("html_submit") {
                let mut submit_field = HtmlField::tag("input");
                submit_field.add_pair_attribute("type", "submit");
                parse_attribute_list(attr, |meta| match meta {
                    syn::Meta::NameValue(ref nv) => {
                        submit_field.parse_pair_attribute(nv.ident.to_string(), &nv.lit)
                    }
                    _ => panic!("WebForms - Failed to parse html_submit attribute"),
                });
                self.sumbit = submit_field;
            }
        }
    }

    /// Parses all attributes applied to fields on the struct deriving
    /// HtmlForm
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

    fn write(&self) -> String {
        let mut w = Vec::new();
        self.form.write(&mut w, true, false);
        self.fields.iter().for_each(|field| {
            field.write(&mut w, true, true);
        });
        self.sumbit.write(&mut w, true, true);
        write!(&mut w, "</form>").unwrap();

        let s = std::str::from_utf8(&w).unwrap();
        s.to_owned()
    }
}

impl ToTokens for HtmlStruct {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let s = self.write();
        tokens.extend(quote! {
            #s
        })
    }
}
