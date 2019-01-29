//! Describes an Html Tag Attribtue

use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Eq)]
pub enum HtmlAttribute {
    Single(String),
    Pair(String, String),
}

impl HtmlAttribute {
    /// Constructs a new single (value) attribute
    ///
    /// # Arguments
    ///
    /// * `value` - Value of this attribute (e.g. "required")
    pub fn new_single<S: Into<String>>(value: S) -> HtmlAttribute {
        HtmlAttribute::Single(value.into())
    }

    /// Constructs a new pair attribute
    ///
    /// # Arguments
    ///
    /// * `name` - Name of attribute (e.g., "id")
    /// * `value` - Value of attribute (e.g., "username")
    pub fn new_pair<S: Into<String>, P: Into<String>>(name: S, value: P) -> HtmlAttribute {
        HtmlAttribute::Pair(name.into(), value.into())
    }

    /// Merges two HtmlAttributes into one attribute.  This assumes the attrs
    /// are the same and only updates the values.  Nothing is done for
    /// value (single) attributes and returns and panics if types mismatch
    pub fn merge(a: &HtmlAttribute, b: &HtmlAttribute) -> HtmlAttribute {
        match a {
            HtmlAttribute::Single(_) => match b {
                HtmlAttribute::Single(_) => a.clone(),
                _ => panic!("HtmlAttribute: merge failed: single -> pair"),
            },
            HtmlAttribute::Pair(ref attr, ref av) => match b {
                HtmlAttribute::Pair(_, ref bv) => {
                    HtmlAttribute::new_pair(attr.clone(), format!("{} {}", av, bv))
                }
                _ => panic!("HtmlAttribute: merge failed: pair -> single"),
            },
        }
    }

    /// Human readable form of this HtmlAttribute
    pub fn render(&self) -> String {
        match &self {
            HtmlAttribute::Single(ref val) => format!(" {}", val),
            HtmlAttribute::Pair(ref name, ref val) => format!(" {}='{}'", name, val),
        }
    }

    /// Update appends the value provided to this attribute.  For single (value) attributes,
    /// this method does nothing.
    ///
    /// # Arguments
    ///
    /// * `value` - Value to append for the attribute pair
    pub fn update<S: Into<String>>(&self, value: S) {
        match self {
            HtmlAttribute::Single(_) => {}  // Do Nothing
            HtmlAttribute::Pair(_, _) => {} // Update the valu
        }
    }
}

impl PartialEq for HtmlAttribute {
    fn eq(&self, other: &HtmlAttribute) -> bool {
        match &self {
            HtmlAttribute::Single(ref v1) => match other {
                HtmlAttribute::Single(ref v2) => v1 == v2,
                _ => false,
            },
            HtmlAttribute::Pair(ref n1, _) => match other {
                HtmlAttribute::Pair(ref n2, _) => n1 == n2,
                _ => false,
            },
        }
    }
}

impl Hash for HtmlAttribute {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            HtmlAttribute::Single(ref v) => v.hash(state),
            HtmlAttribute::Pair(ref a, _) => a.hash(state),
        }
    }
}

impl std::fmt::Display for HtmlAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HtmlAttribute::Single(ref value) => write!(f, "{}", value),
            HtmlAttribute::Pair(ref attr, ref value) => write!(f, "{}='{}'", attr, value),
        }
    }
}
