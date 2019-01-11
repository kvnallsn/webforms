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
//! | email | String | None | Checks if input matches an email address (via regex) | 1 |
//! | phone | String | None | Checks if input matches a phone number (via regex) | 2 |
//! | min_value | Integer/Float | Integer/Float | Checks if input is greater than or equal to specified value | |
//! | max_value | Integer/Float | Integer/Float | Checks if input is less than or euqal to specified value | |
//!
//! Notes:
//! * 1 - Requires crate to depend on `regex` and `lazy_static` crates and import them.  See below for example.
//! * 2 - Currently only matches on US phone numbers
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
//!      #[validate(email)]
//!      pub email: String,
//!
//!      #[validate(min_length = 8)]
//!      #[validate(max_length = 20)]
//!      pub password: String,
//! }
//!
//! fn main() {
//!    let form = LoginForm {
//!        email: "test@someemail.com".to_owned(),
//!        password: "itsasecret".to_owned(),
//!    };
//!
//!    if let Err(e) = form.validate() {
//!        println!("{:?}", e);
//!    }
//! }
//! ```

use std::fmt::{self, Display};
// Import and re-export the macro
pub use webforms_derive::ValidateForm;

// Errors that can appear if validation fails
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

    /// The phone number entered does not match our regex
    InvalidPhoneNumber { field: &'static str },

    /// The field failed the user-passed regex
    InvalidRegex { field: &'static str },

    /// Two fields do not match
    FieldMismatch { field: &'static str },
}

impl Display for ValidateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidateError::InputTooShort { field, min } => {
                write!(f, "{}: input too short. ({} min length)", field, min)
            }

            ValidateError::InputTooLong { field, max } => {
                write!(f, "{}: input too long. ({} max length)", field, max)
            }
            ValidateError::TooSmall { field, min } => write!(
                f,
                "{}: input below required minimum. ({} minimum)",
                field, min
            ),
            ValidateError::TooLarge { field, max } => write!(
                f,
                "{}: input above maximum allowed. ({} maximum)",
                field, max
            ),
            ValidateError::InvalidCharacters { field } => {
                write!(f, "{}: contains invalid characters", field)
            }
            ValidateError::InvalidEmail { field } => {
                write!(f, "{}: not a valid email address", field)
            }
            ValidateError::InvalidPhoneNumber { field } => {
                write!(f, "{}: not a valid U.S. phone number", field)
            }
            ValidateError::InvalidRegex { field } => {
                write!(f, "{}: does not match required input", field)
            }
            ValidateError::FieldMismatch { field } => {
                write!(f, "{}: does not match other field", field)
            }
        }
    }
}

/// Validates a form according to attributes set via #[validate] attribute
/// on a given struct.  The attributes are set on the individual fields in
/// a struct.
pub trait ValidateForm {
    /// Performs form validation, retuns Ok if validation passed, or a vector
    /// of errors if validation failed
    fn validate(&self) -> Result<(), Vec<ValidateError>>;
}

#[cfg(test)]
mod tests {
    use crate::validate::{ValidateError, ValidateForm};
    use lazy_static::lazy_static;
    use regex::Regex;

    #[derive(ValidateForm)]
    #[validate_regex(compiled_re = r"^100 Mike Rd$")]
    struct TestForm<'a> {
        #[validate(min_length = 3)]
        #[validate(max_length = 20)]
        pub username: &'a str,

        #[validate(email)]
        pub email: &'a str,

        #[validate(regex = r"^[a-z]{8}\d{3}!$")]
        pub some_string: &'a str,

        #[validate_match(some_string)]
        pub some_string_2: &'a str,

        #[validate(phone)]
        pub phone: &'a str,

        #[validate(min_value = 18)]
        #[validate(max_value = 65)]
        pub age: i16,

        #[validate_regex(compiled_re)]
        pub address: &'a str,

        #[validate(optional)]
        #[validate(min_value = 50)]
        pub opt_number: Option<i16>,

        #[validate(optional)]
        #[validate(min_length = 5)]
        pub opt_owned_string: Option<String>,

        #[validate(optional)]
        #[validate(min_length = 5)]
        pub opt_ref_string: Option<&'a str>,
    }

    impl<'a> Default for TestForm<'a> {
        fn default() -> Self {
            TestForm {
                username: "Mike",
                email: "mike@test.com",
                some_string: "password123!",
                some_string_2: "password123!",
                phone: "+1 111-111-1111",
                age: 25,
                address: "100 Mike Rd",
                opt_number: Some(90),
                opt_owned_string: Some("Maryland".to_owned()),
                opt_ref_string: Some("Maryland"),
            }
        }
    }

    #[test]
    fn test_all_valid() {
        let form = TestForm {
            ..Default::default()
        };

        let res = form.validate();
        assert!(res.is_ok());
    }

    #[test]
    fn test_username_too_short() {
        let form = TestForm {
            username: "a",
            ..Default::default()
        };

        let res = form.validate();
        assert!(res.is_err());
        let errs = res.unwrap_err();
        assert_eq!(errs.len(), 1);

        match errs[0] {
            ValidateError::InputTooShort { field: _, min: _ } => {}
            _ => panic!("Wrong Error for Too Short"),
        }
    }

    #[test]
    fn test_username_too_long() {
        let form = TestForm {
            username: "aaaaa aaaaa aaaaa aaaaa",
            ..Default::default()
        };

        let res = form.validate();
        assert!(res.is_err());
        let errs = res.unwrap_err();
        assert_eq!(errs.len(), 1);

        match errs[0] {
            ValidateError::InputTooLong { field: _, max: _ } => {}
            _ => panic!("Wrong Error for Too Long"),
        }
    }

    #[test]
    fn test_invalid_email() {
        let form = TestForm {
            email: "test@test",
            ..Default::default()
        };

        let res = form.validate();
        assert!(res.is_err());
        let errs = res.unwrap_err();
        assert_eq!(errs.len(), 1);

        match errs[0] {
            ValidateError::InvalidEmail { field: _ } => {}
            _ => panic!("Wrong Error for Invalid Email"),
        }
    }

    #[test]
    fn test_invalid_some_string() {
        let form = TestForm {
            some_string: "password123",
            some_string_2: "password123",
            ..Default::default()
        };

        let res = form.validate();
        assert!(res.is_err());
        let errs = res.unwrap_err();
        assert_eq!(errs.len(), 1);

        match errs[0] {
            ValidateError::InvalidRegex { field: _ } => {}
            _ => panic!("Wrong Error for Invalid Regex"),
        }
    }

    #[test]
    fn test_invalid_some_string_mismatch() {
        let form = TestForm {
            some_string_2: "wrong_field",
            ..Default::default()
        };

        let res = form.validate();
        assert!(res.is_err());
        let errs = res.unwrap_err();
        assert_eq!(errs.len(), 1);

        match errs[0] {
            ValidateError::FieldMismatch { field: _ } => {}
            _ => panic!("Wrong Error for Field Mismatch"),
        }
    }

    #[test]
    fn test_invalid_phone() {
        let form = TestForm {
            phone: "1-111-1111",
            ..Default::default()
        };

        let res = form.validate();
        assert!(res.is_err());
        let errs = res.unwrap_err();
        assert_eq!(errs.len(), 1);

        match errs[0] {
            ValidateError::InvalidPhoneNumber { field: _ } => {}
            _ => panic!("Wrong Error for Invalid Phone Number"),
        }
    }

    #[test]
    fn test_age_too_small() {
        let form = TestForm {
            age: 10,
            ..Default::default()
        };

        let res = form.validate();
        assert!(res.is_err());
        let errs = res.unwrap_err();
        assert_eq!(errs.len(), 1);

        match errs[0] {
            ValidateError::TooSmall { field: _, min: _ } => {}
            _ => panic!("Wrong Error for Too Small"),
        }
    }

    #[test]
    fn test_age_too_large() {
        let form = TestForm {
            age: 70,
            ..Default::default()
        };

        let res = form.validate();
        assert!(res.is_err());
        let errs = res.unwrap_err();
        assert_eq!(errs.len(), 1);

        match errs[0] {
            ValidateError::TooLarge { field: _, max: _ } => {}
            _ => panic!("Wrong Error for Too Large"),
        }
    }

    #[test]
    fn test_optional_none() {
        let form = TestForm {
            opt_number: None,
            ..Default::default()
        };

        let res = form.validate();
        assert!(res.is_ok());
    }

    #[test]
    fn test_optional_some_too_low() {
        let form = TestForm {
            opt_number: Some(10),
            ..Default::default()
        };

        let res = form.validate();
        assert!(res.is_err());
        let errs = res.unwrap_err();
        assert_eq!(errs.len(), 1);

        match errs[0] {
            ValidateError::TooSmall { field: _, min: _ } => {}
            _ => panic!("Wrong Error for Too Small"),
        }
    }
}
