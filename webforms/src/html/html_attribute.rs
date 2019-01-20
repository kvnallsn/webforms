//! Describes an Html Tag Attribtue

use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Eq)]
pub enum HtmlAttribute {
    Single(String),
    Pair(String, String),
}

impl HtmlAttribute {
    pub fn new_single<S: Into<String>>(value: S) -> HtmlAttribute {
        HtmlAttribute::Single(value.into())
    }

    pub fn new_pair<S: Into<String>, P: Into<String>>(name: S, value: P) -> HtmlAttribute {
        HtmlAttribute::Pair(name.into(), value.into())
    }

    pub fn render(&self) -> String {
        match &self {
            HtmlAttribute::Single(ref val) => format!(" {}", val),
            HtmlAttribute::Pair(ref name, ref val) => format!(" {}='{}'", name, val),
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
