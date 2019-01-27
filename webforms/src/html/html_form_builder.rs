//! Module to build HtmlForms

use crate::html::{HtmlAttribute, HtmlField, HtmlFieldBuilder};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct HtmlFormBuilder {
    pub fields: HashMap<&'static str, HtmlFieldBuilder>,
}

impl HtmlFormBuilder {
    /// Builds and returns a field with, attaching the specified attributes to the field.
    /// Attributes are normalled added using the `attrs!` macro.
    ///
    /// # Arguments
    ///
    /// * `field` - Name of field to build, as string
    /// * `attrs` - Set of attributes to add to this field.
    pub fn field<S: AsRef<str>>(&self, field: S, attrs: &HashSet<HtmlAttribute>) -> HtmlField {
        let name = field.as_ref();
        let field = match self.fields.get(name) {
            Some(f) => f,
            None => panic!("WebForms - No field with name {}", name),
        };

        field.build(Some(attrs.clone()))
    }

    /// Same as `field`, but builds a field with no additional attributes.AsMut
    ///
    /// # Arguments
    ///
    /// * `field` - Name of field to build
    pub fn field_no_attrs<S: AsRef<str>>(&self, field: S) -> HtmlField {
        let name = field.as_ref();
        let field = match self.fields.get(name) {
            Some(f) => f,
            None => panic!("WebForms - No field with name {}", name),
        };

        field.build(None)
    }

    /// Returns a Builder than can build a new HtmlField in-place. Useful when
    /// mutable references are allowed.AsMut
    ///
    /// # Arguments
    ///
    /// * `field` - Name of field to build
    pub fn builder<S: AsRef<str>>(&mut self, field: S) -> &mut HtmlFieldBuilder {
        match self.fields.get_mut(field.as_ref()) {
            Some(field) => field,
            None => panic!("WebForms - No field with name {}", field.as_ref()),
        }
    }
}
