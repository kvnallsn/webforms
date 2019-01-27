//! Represents an Html Tag/Field

use crate::html::HtmlAttribute;
use std::collections::HashSet;

#[derive(Debug)]
pub struct HtmlFieldBuilder {
    pub tag: String,
    pub name: Option<String>,
    pub attrs: HashSet<HtmlAttribute>,
}

pub struct HtmlField<'a> {
    pub tag: &'a str,
    pub name: &'a Option<String>,
    pub attrs: &'a HashSet<HtmlAttribute>,
    pub late_attrs: Option<HashSet<HtmlAttribute>>,
}

impl HtmlFieldBuilder {
    /// Builds and returns a new HtmlFieldBuilder with the specified tag and
    /// ame (if present)
    ///
    /// # Arguments
    ///
    /// * `tag` - Html tag (e.g., 'input') if this field
    /// * `name` - An optional name attribute for this field
    pub fn new<S: Into<String>, P: Into<String>>(tag: S, name: Option<P>) -> Self {
        let mut field = HtmlFieldBuilder {
            tag: tag.into(),
            name: name.map(|s| s.into()),
            attrs: HashSet::new(),
        };

        if let Some(ref name) = field.name {
            field
                .attrs
                .replace(HtmlAttribute::new_pair("name", name.to_string()));
        }

        field
    }

    /// Same as `new`, but additionally accepts a set of attributes to add to this field.
    ///
    /// # Arguments
    ///
    /// * `tag` - Html tag (e.g., 'input') if this field
    /// * `name` - An optional name attribute for this field
    /// * `attrs` - Set of attributes to apply to this tag.
    pub fn with_attrs<S: Into<String>, P: Into<String>>(
        tag: S,
        name: Option<P>,
        attrs: HashSet<HtmlAttribute>,
    ) -> Self {
        let mut field = HtmlFieldBuilder {
            tag: tag.into(),
            name: name.map(|s| s.into()),
            attrs: attrs,
        };

        if let Some(ref name) = field.name {
            field
                .attrs
                .replace(HtmlAttribute::new_pair("name", name.to_string()));
        }

        field
    }

    /// Finializes and builds the field contained in this builder.  Optionally, more
    /// attributes can be attached here and will take precedence over an attributes
    /// passed in the `with_attrs` construction method
    ///
    /// # Arguments
    ///
    /// * `attrs` - Optional list of more attributes to take precedence over current attrs
    pub fn build<'a>(&'a self, attrs: Option<HashSet<HtmlAttribute>>) -> HtmlField<'a> {
        HtmlField {
            tag: &self.tag,
            name: &self.name,
            attrs: &self.attrs,
            late_attrs: attrs,
        }
    }

    /// Adds a new value attribute (e.g. `checked` or `required`) to this field builder
    ///
    /// # Arugments
    ///
    /// * `value` - Attribute to add
    pub fn value<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.attrs.replace(HtmlAttribute::new_single(value));
        self
    }

    /// Adds a vector of new value attribute (e.g. `checked` or `required`) to this
    /// field builder
    ///
    /// # Arugments
    ///
    /// * `values` - Vector of new attributes to add
    pub fn values<S: Into<String>>(&mut self, values: Vec<String>) -> &mut Self {
        for value in values {
            self.attrs.replace(HtmlAttribute::new_single(value));
        }
        self
    }

    /// Adds new paired attribute (e.g., class="....") to this field builder
    ///
    /// # Arguments
    ///
    /// * `attr` - Name of the attribute (e.g., "class")
    /// * `value` - Value of the attribute (e.g., "btn btn-large")
    pub fn attr<S: Into<String>, P: Into<String>>(&mut self, attr: S, value: P) -> &mut Self {
        self.attrs.replace(HtmlAttribute::new_pair(attr, value));
        self
    }

    /// Adds a new vector of attributes to the field builder
    ///
    /// # Arguments
    ///
    /// * `h` - Set of new Html Attributes to add (either paired or value)
    pub fn attrs(&mut self, h: HashSet<HtmlAttribute>) -> &mut Self {
        self.attrs.extend(h);
        self
    }

    /// Helper method to set class attribute
    ///
    /// # Arguments
    ///
    /// * `classes` - String of classes to apply to this field
    pub fn class<S: Into<String>>(&mut self, classes: S) -> &mut Self {
        self.attrs
            .replace(HtmlAttribute::new_pair("class", classes));
        self
    }

    /// Helper method to set required attribute on this field builder
    pub fn required(&mut self) -> &mut Self {
        self.attrs.replace(HtmlAttribute::new_single("required"));
        self
    }

    /// Helper method to unset the required attribute on this field builder
    pub fn optional(&mut self) -> &mut Self {
        self.attrs.remove(&HtmlAttribute::new_single("required"));
        self
    }
}

impl<'a> std::fmt::Display for HtmlField<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<input")?;

        // Write the attributes out, if there is a collision between the default
        // set and the specified set, pick the specified set
        match self.late_attrs {
            Some(ref late_attrs) => {
                for attr in self.attrs {
                    if !late_attrs.contains(attr) {
                        write!(f, " {}", attr)?;
                    }
                }

                for attr in late_attrs {
                    write!(f, " {}", attr)?;
                }
            }
            None => {
                for attr in self.attrs {
                    write!(f, " {}", attr)?;
                }
            }
        }

        write!(f, ">")
    }
}
