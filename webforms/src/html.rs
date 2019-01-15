//! Generates valid HTML code for displaying a form
//!
//! All fields correspond an input tag with matching types.  For example, an i32 field
//! would generated an input tag with the type set to number.

// Import and re-export the macro
pub use webforms_derive::HtmlForm;

/// HtmlForm provides two methods, render_field and render_form. Both provide
/// different ways to accomplish the same goal, rendering a form as valid and safe
/// HTML.
pub trait HtmlForm {
    /// Renders the specified field and nothing else.  Useful if you want to
    /// split the form up accross different HTML tags (like divs, etc) or want
    /// more control over how the form renders
    ///
    /// # Arguments
    ///
    /// * `field` - Name of field to render
    fn render_field<S: AsRef<str>>(&self, field: S) -> String;

    /// Renders the whole form at once according to all the attribute tags provided
    fn render_form(&self) -> String;
}
