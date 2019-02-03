//! Module to build HtmlForms

use crate::html::{HtmlFieldBuilder, HtmlValidator};
use std::collections::HashMap;

pub struct HtmlFormBuilder<'a> {
    fields: HashMap<&'static str, HtmlFieldBuilder>,
    validators: HashMap<&'static str, HtmlValidator<'a>>,
}

impl<'a> HtmlFormBuilder<'a> {
    /// Creates a new HtmlFormBuilder with the specified fields
    /// and validators
    pub fn new() -> HtmlFormBuilder<'a> {
        HtmlFormBuilder {
            fields: HashMap::new(),
            validators: HashMap::new(),
        }
    }

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

    /// Iterates over all fields in this form builder, validating
    /// them against the critera specified in the #[html_validate]
    /// attribute
    pub fn validate(&self) -> bool {
        for (_, validator) in &self.validators {
            validator.validate();
        }

        false
    }

    /// Returns all errors that occured during form validation, or
    /// None if no errors occured
    ///
    /// # Arguments
    ///
    /// * `field` - Name of field to retrieve errors for
    pub fn errors<S: AsRef<str>>(&self, field: S) -> Option<bool> {
        None
    }

    /// Adds a new field builder (and thus field) to this form builder
    pub fn add_field(&mut self, name: &'static str, field: HtmlFieldBuilder) {
        self.fields.insert(name, field);
    }

    // Adds a new field validator to a given field
    //pub fn add_validator(&mut self, name: &'static str, validator: HtmlValidator) {
    //    self.validators.insert(name, validator);
    //}
}

impl<'a> std::fmt::Display for HtmlFormBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (_, field) in &self.fields {
            let builder = field.clone();
            write!(f, "{}\n", builder.finish())?;
        }

        Ok(())
    }
}
