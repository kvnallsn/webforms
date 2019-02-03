//! Represents an Html Tag/Field

use crate::html::{HtmlAttribute, HtmlValidator};
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct HtmlFieldBuilder {
    pub tag: String,
    pub name: Option<String>,
    pub attrs: HashSet<HtmlAttribute>,
    pub replace: bool,
}

pub struct HtmlField {
    pub tag: String,
    pub name: Option<String>,
    pub attrs: HashSet<HtmlAttribute>,
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
            replace: false,
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
            replace: false,
        };

        if let Some(ref name) = field.name {
            field
                .attrs
                .replace(HtmlAttribute::new_pair("name", name.to_string()));
        }

        field
    }

    /// Adds `attr` to this fields attribute set, either updating the existing
    /// value (in append mode), or replacing it completely (in replace mode)
    ///
    /// # Arguments
    ///
    /// * `attr` - New HtmlAttribute to add to this field
    fn add_to_attributes(&mut self, attr: HtmlAttribute) {
        if self.replace {
            self.attrs.replace(attr);
        } else {
            self.attrs.insert(match self.attrs.get(&attr) {
                Some(a) => HtmlAttribute::merge(a, &attr),
                None => attr,
            });
        }
    }

    /// Finializes and builds the field contained in this builder. Consumes
    /// the HtmlFieldBuilder and returns an HtmlField
    pub fn finish(self) -> HtmlField {
        HtmlField {
            tag: self.tag,
            name: None,
            attrs: self.attrs,
        }
    }

    /// Switches into replace mode, all values/pairs after this call
    /// will replace (not append) any attributes that are encountered
    pub fn replace(mut self) -> Self {
        self.replace = true;
        self
    }

    /// Switches into append mode, all values/pairs after this call
    /// will append (not replace) any attributes that are encountered
    pub fn append(mut self) -> Self {
        self.replace = false;
        self
    }

    /// Adds a new value attribute (e.g. `checked` or `required`) to this field builder
    ///
    /// # Arugments
    ///
    /// * `value` - Attribute to add
    pub fn value<S: Into<String>>(mut self, value: S) -> Self {
        self.add_to_attributes(HtmlAttribute::new_single(value));
        self
    }

    /// Adds a vector of new value attribute (e.g. `checked` or `required`) to this
    /// field builder
    ///
    /// # Arugments
    ///
    /// * `values` - Vector of new attributes to add
    pub fn values<S: Into<String>>(mut self, values: Vec<String>) -> Self {
        for value in values {
            self.add_to_attributes(HtmlAttribute::new_single(value));
        }
        self
    }

    /// Adds new paired attribute (e.g., class="....") to this field builder
    ///
    /// # Arguments
    ///
    /// * `attr` - Name of the attribute (e.g., "class")
    /// * `value` - Value of the attribute (e.g., "btn btn-large")
    pub fn attr<S: Into<String>, P: Into<String>>(mut self, attr: S, value: P) -> Self {
        self.add_to_attributes(HtmlAttribute::new_pair(attr.into(), value.into()));
        self
    }

    /// Adds a new vector of attributes to the field builder
    ///
    /// # Arguments
    ///
    /// * `h` - Set of new Html Attributes to add (either paired or value)
    pub fn attrs(mut self, h: HashSet<HtmlAttribute>) -> Self {
        for attr in h {
            self.add_to_attributes(attr);
        }
        self
    }

    /// Helper method to set class attribute
    ///
    /// # Arguments
    ///
    /// * `classes` - String of classes to apply to this field
    pub fn class<S: Into<String>>(mut self, classes: S) -> Self {
        self.add_to_attributes(HtmlAttribute::new_pair("class", classes));
        self
    }

    /// Helper method to set required attribute on this field builder
    pub fn required(mut self) -> Self {
        self.add_to_attributes(HtmlAttribute::new_single("required"));
        self
    }

    /// Helper method to unset the required attribute on this field builder
    pub fn optional(mut self) -> Self {
        self.add_to_attributes(HtmlAttribute::new_single("required"));
        self
    }
}

impl std::fmt::Display for HtmlField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<input")?;

        for attr in &self.attrs {
            write!(f, " {}", attr)?;
        }

        write!(f, ">")
    }
}
