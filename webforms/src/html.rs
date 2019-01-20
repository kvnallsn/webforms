//! Generates valid HTML code for displaying a form
//!
//! All fields correspond an input tag with matching types.  For example, an i32 field
//! would generated an input tag with the type set to number.

// Import and re-export the macro
pub use webforms_derive::HtmlForm;

#[macro_export]
macro_rules! attrs {
    () => { std::collections::HashSet::new() };
    ($($v:expr)+) => {{
        let mut h: std::collections::HashSet<::webforms::html::HtmlAttribute> = std::collections::HashSet::new();
        $(h.insert(::webforms::html::HtmlAttribute::new_single($v));)+
        h
    }};
    ($($k:expr => $v:expr),+) => {{
        let mut h: std::collections::HashSet<::webforms::html::HtmlAttribute> = std::collections::HashSet::new();
        $(h.insert(::webforms::html::HtmlAttribute::new_pair($k, $v));)+
        h
    }};
    ($($k:expr => $v:expr),+; $($single:expr),+) => {{
        let mut h: std::collections::HashSet<::webforms::html::HtmlAttribute> = std::collections::HashSet::new();
        $(h.insert(::webforms::html::HtmlAttribute::new_pair($k, $v));)+
        $(h.insert(::webforms::html::HtmlAttribute::new_single($single));)+
        h
    }};
}

mod html_attribute;
mod html_field;
mod html_form_builder;

pub use self::html_attribute::HtmlAttribute;
pub use self::html_field::{HtmlField, HtmlFieldBuilder};
pub use self::html_form_builder::HtmlFormBuilder;

/// HtmlForm provides two methods, render_field and render_form. Both provide
/// different ways to accomplish the same goal, rendering a form as valid and safe
/// HTML.
pub trait HtmlForm {
    /// Return the HTML form of this form
    fn form(&self) -> HtmlFormBuilder;
}
