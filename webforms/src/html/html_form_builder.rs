//! Module to build HtmlForms

use crate::html::{HtmlAttribute, HtmlField, HtmlFieldBuilder};
use std::{
    collections::{HashMap, HashSet},
    ops::{Index, IndexMut},
};

#[derive(Debug)]
pub struct HtmlFormBuilder {
    pub fields: HashMap<&'static str, HtmlFieldBuilder>,
}

impl HtmlFormBuilder {
    /// Returns a Builder than can build a new HtmlField in-place. Useful when
    /// mutable references are allowed.AsMut
    ///
    /// # Arguments
    ///
    /// * `field` - Name of field to build
    pub fn builder<S: AsRef<str>>(&self, field: S) -> HtmlFieldBuilder {
        match self.fields.get(field.as_ref()) {
            Some(field) => field.clone(),
            None => panic!("WebForms - No field with name {}", field.as_ref()),
        }
    }
}

impl Index<&'static str> for HtmlFormBuilder {
    type Output = HtmlFieldBuilder;

    fn index(&self, field: &'static str) -> &HtmlFieldBuilder {
        match self.fields.get(field) {
            Some(field) => field,
            None => panic!(""),
        }
    }
}

impl IndexMut<&'static str> for HtmlFormBuilder {
    fn index_mut(&mut self, field: &'static str) -> &mut HtmlFieldBuilder {
        match self.fields.get_mut(field) {
            Some(field) => field,
            None => panic!("WebForms - No field with name {}", field),
        }
    }
}
