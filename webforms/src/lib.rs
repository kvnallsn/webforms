//! A crate for deriving multiple form helper traits useful when working with web forms.
//! 
//! Currently impleted traits:
//! * `ValidateForm` - Checks each annotated field for requirement list in the field attributes.
//! * `HtmlForm` - Produces valid html input fields for each field in a form
//! 
//! See each module for examples
//! 
//! # Features
//! * `validate` - Enables the ValidateForm trait and derive macro
//! * `html` - Enables the HtmlForm trait and derive macro

#[cfg(feature = "validate")]
pub mod validate;

#[cfg(feature = "html")]
pub mod html;
