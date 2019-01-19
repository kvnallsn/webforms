//! Module to build HtmlForms

use crate::html::HtmlFieldBuilder;
use std::collections::HashMap;

#[derive(Debug)]
pub struct HtmlFormBuilder {
    pub fields: HashMap<&'static str, HtmlFieldBuilder>,
}

impl HtmlFormBuilder {
    pub fn field<S: AsRef<str>>(&self, field: S) -> &HtmlFieldBuilder {
        match self.fields.get(field.as_ref()) {
            Some(field) => field,
            None => panic!("WebForms - No field with name {}", field.as_ref()),
        }
    }

    pub fn field_mut<S: AsRef<str>>(&mut self, field: S) -> &mut HtmlFieldBuilder {
        match self.fields.get_mut(field.as_ref()) {
            Some(field) => field,
            None => panic!("WebForms - No field with name {}", field.as_ref()),
        }
    }
}
