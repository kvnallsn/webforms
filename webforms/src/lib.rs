//! Handles generating HTML forms/validating form input

mod validate;
pub use crate::validate::{ValidateForm, ValidateError};
pub use webforms_derive::ValidateForm;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
