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

    pub fn build<'a>(&'a self, attrs: Option<HashSet<HtmlAttribute>>) -> HtmlField<'a> {
        HtmlField {
            tag: &self.tag,
            name: &self.name,
            attrs: &self.attrs,
            late_attrs: attrs,
        }
    }

    pub fn value<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.attrs.replace(HtmlAttribute::new_single(value));
        self
    }

    pub fn values<S: Into<String>>(&mut self, values: Vec<String>) -> &mut Self {
        for value in values {
            self.attrs.replace(HtmlAttribute::new_single(value));
        }
        self
    }

    pub fn attr<S: Into<String>, P: Into<String>>(&mut self, attr: S, value: P) -> &mut Self {
        self.attrs.replace(HtmlAttribute::new_pair(attr, value));
        self
    }

    pub fn attrs(&mut self, h: HashSet<HtmlAttribute>) -> &mut Self {
        self.attrs.extend(h);
        self
    }

    pub fn class<S: Into<String>>(&mut self, classes: S) -> &mut Self {
        self.attrs
            .replace(HtmlAttribute::new_pair("class", classes));
        self
    }

    pub fn required(&mut self) -> &mut Self {
        self.attrs.replace(HtmlAttribute::new_single("required"));
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
