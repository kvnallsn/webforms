//! Module to build HtmlForms

use crate::html::HtmlFieldBuilder;
use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;

pub struct HtmlFormBuilder<'a> {
    fields: HashMap<&'static str, HtmlFieldBuilder>,
    validated: bool,
    phantom: PhantomData<&'a i32>,
}

impl<'a> HtmlFormBuilder<'a> {
    /// Creates a new HtmlFormBuilder with the specified fields
    /// and validators
    pub fn new() -> HtmlFormBuilder<'a> {
        HtmlFormBuilder {
            fields: HashMap::new(),
            validated: false,
            phantom: PhantomData,
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

    /// Returns true if this form has been sucessfully validated,
    /// false if validation failed or it never occured (i.e., called
    /// `blank_form`)
    pub fn validated(&self) -> bool {
        self.validated
    }

    /// Validates a field's value against a list of closures, setting the
    /// validated field appropriately
    ///
    /// # Arguments
    ///
    /// * `value` - Value of field to validate
    /// * `validators` - Vector of closures to validate against
    pub fn validate_field<T: Debug>(&mut self, value: &T, validators: Vec<Box<&Fn(&T) -> bool>>) {
        self.validated = validators.iter().all(|x| x(value));
    }

    /// Returns all errors that occured during form validation, or
    /// None if no errors occured
    ///
    /// # Arguments
    ///
    /// * `field` - Name of field to retrieve errors for
    pub fn errors<S: AsRef<str>>(&self, _field: S) -> Option<bool> {
        None
    }

    /// Adds a new field builder (and thus field) to this form builder
    pub fn add_field(&mut self, name: &'static str, field: HtmlFieldBuilder) {
        self.fields.insert(name, field);
    }
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
