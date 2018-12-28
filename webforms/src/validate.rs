//! Validates a stuct's fields according the attributes applied to each field.
//! 
//! Provides a derive macro to auto-implement the ValidateForm trait that
//! supports the `validate(&self)` method.  Current attributes (and support types):
//! 
//! | attribute | field type | value type | description | Notes |
//! | --------- | -----------| ---------- | ----------- | ----- |
//! | min_length | String | Integer | Checks if input meets a required minimum length | |
//! | max_length | String | Integer | Checks if input is under or equal to a maximum length | |
//! | regex | String | String |  Checks if input is a match against the supplied regex | 1 |
//! | email | String | String | Checks if input matches an email address (via regex) | 1 |
//! | min_value | Integer/Float | Integer/Float | Checks if input is greater than or equal to specified value | |
//! | max_value | Integer/Float | Integer/Float | Checks if input is less than or euqal to specified value | |
//! 
//! Notes:
//! 1 - Requires crate to depend on `regex` and `lazy_static` crates and import them.  See below for example.
//! 
//! # Example
//! 
//! ```
//! use lazy_static::lazy_static;
//! use regex::Regex;
//! use webforms::validate::{ValidateForm, ValidateError};
//! 
//! #[derive(ValidateForm)]
//! struct LoginForm {
//! 
//!      #[validate(min_length = 3)]
//!      #[validate(max_length = 20)]
//!      pub username: String,
//! 
//!      #[validate(email)]
//!      pub email: String,
//! }
//! ```

/// Import and re-export the macro
pub use webforms_derive::ValidateForm;

/// Errors that can appear if validation fails
#[derive(Debug)]
pub enum ValidateError {
    /// Input was too short (< min_length)
    InputTooShort { field: &'static str, min: i64 },

    /// Input was too long (> max_length)
    InputTooLong { field: &'static str, max: i64 },

    /// Minimum value for an integer field
    TooSmall { field: &'static str, min: i64 },

    /// Maximum value for an integer field
    TooLarge { field: &'static str, max: i64 },

    /// Input contained invalid characters (invalid)
    InvalidCharacters { field: &'static str },

    /// The email entered does not match our email regex
    InvalidEmail { field: &'static str },

    /// The field failed the user-passed regex
    InvalidRegex { field: &'static str },
}

/// Validates a form according to attributes set via #[validate] attribute
/// on a given struct.  The attributes are set on the individual fields in
/// a struct.
pub trait ValidateForm {
    /// Performs form validation, retuns Ok if validation passed, or a vector
    /// of errors if validation failed
    fn validate(&self) -> Result<(), Vec<ValidateError>>;
}
