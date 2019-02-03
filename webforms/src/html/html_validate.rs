//! Common attribute validation critera

/// All possible validators supported.
#[derive(Debug)]
pub enum Validator {
    MinValue(u64),
    MaxValue(u64),
    MinFloat(f64),
    MaxFloat(f64),
    MinLength(u64),
    MaxLength(u64),
    Pattern(&'static str),
}

pub struct HtmlValidator<'a> {
    validators: Vec<Box<&'a Fn() -> bool>>,
}

impl<'a> HtmlValidator<'a> {
    pub fn new() -> HtmlValidator<'a> {
        HtmlValidator { validators: vec![] }
    }

    pub fn with_fns(fns: Vec<Box<&'a Fn() -> bool>>) -> HtmlValidator<'a> {
        HtmlValidator { validators: fns }
    }

    pub fn with_fn(f: impl Fn() -> bool) {}

    pub fn validate(&self) -> bool {
        false
    }
}

pub trait FieldValidator {
    fn validate(&self) -> bool;
}

pub struct IntMinValidator;

impl FieldValidator for IntMinValidator {
    fn validate(&self) -> bool {
        false
    }
}
