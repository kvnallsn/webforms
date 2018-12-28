//! Form validation, custom derive macro to auto-generate a validate method

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
}

/// Validates a form according to attributes set via #[validate] attribute
/// on a given struct
pub trait ValidateForm {
    /// Performs form validation, retuns Ok if validation passed, or a list
    /// of errors if validation failed
    fn validate(&self) -> Result<(), Vec<ValidateError>>;
}
