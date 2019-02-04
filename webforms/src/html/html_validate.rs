//! Common attribute validation critera

use std::collections::HashMap;

type CheckFn<T> = Fn(&T) -> std::result::Result<(), &'static str>;
//type CheckFn<T> = Fn(T) -> bool;

pub struct FieldValidator<'a, T> {
    field: &'static str,
    validators: Vec<Box<&'a CheckFn<T>>>,
}

impl<'a, T> FieldValidator<'a, T> {
    pub fn new(field: &'static str, fns: Vec<Box<&'a CheckFn<T>>>) -> FieldValidator<'a, T> {
        FieldValidator {
            field: field,
            validators: fns,
        }
    }

    pub fn field(&self) -> &str {
        &self.field
    }

    pub fn validate(&self, value: &T, errors: &mut HashMap<&'static str, String>) -> bool {
        self.validators.iter().all(|x| match x(value) {
            Ok(_) => true,
            Err(e) => {
                errors.entry(self.field).or_insert(e.to_string());
                false
            }
        })
    }
}
