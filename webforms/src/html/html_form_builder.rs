//! Module to build HtmlForms

use crate::html::{HtmlAttribute, HtmlField, HtmlFieldBuilder};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct HtmlFormBuilder {
    pub fields: HashMap<&'static str, HtmlFieldBuilder>,
}

impl HtmlFormBuilder {
    pub fn field<S: AsRef<str>>(&self, field: S, attrs: &HashSet<HtmlAttribute>) -> HtmlField {
        let name = field.as_ref();
        let field = match self.fields.get(name) {
            Some(f) => f,
            None => panic!("WebForms - No field with name {}", name),
        };

        field.build(Some(attrs.clone()))
    }

    pub fn field2<S: AsRef<str>>(&self, field: S) -> HtmlField {
        let name = field.as_ref();
        let field = match self.fields.get(name) {
            Some(f) => f,
            None => panic!("WebForms - No field with name {}", name),
        };

        field.build(None)
    }

    pub fn builder<S: AsRef<str>>(&mut self, field: S) -> &mut HtmlFieldBuilder {
        match self.fields.get_mut(field.as_ref()) {
            Some(field) => field,
            None => panic!("WebForms - No field with name {}", field.as_ref()),
        }
    }
}
