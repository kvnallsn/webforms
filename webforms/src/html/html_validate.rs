//! Common attribute validation critera

pub struct ValidateFunction<'a, T> {
    field: String,
    validators: Vec<Box<&'a Fn(&T) -> bool>>,
}

impl<'a, T> ValidateFunction<'a, T> {
    pub fn new<S: Into<String>>(
        field: S,
        fns: Vec<Box<&'a Fn(&T) -> bool>>,
    ) -> ValidateFunction<'a, T> {
        ValidateFunction {
            field: field.into(),
            validators: fns,
        }
    }

    pub fn field(&self) -> &str {
        &self.field
    }

    pub fn validate(&self, value: &T) -> bool {
        self.validators.iter().all(|x| x(value))
    }
}
