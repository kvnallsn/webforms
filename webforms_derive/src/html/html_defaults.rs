//! Module to contain any default attributes that should be written
//! when an html tag is encountered.  These can be overriden by
//! using the #[html()] attribute for a specific field

use serde_derive::Deserialize;
use std::{
    collections::{BTreeMap, HashSet},
    fs::File,
    io::Read,
    path::Path,
};

#[derive(Eq, Hash, Debug)]
pub struct HtmlDefault {
    pub tag: String,
    pub attrs: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct TomlTest {
    tags: BTreeMap<String, BTreeMap<String, String>>,
}

impl HtmlDefault {
    /// Creates a new HtmlDefault structure with the specified
    /// tag and empty attributes map
    ///
    /// # Arguments
    ///
    /// * `tag` - Name of type tag this default corresponds to
    pub fn new<S: Into<String>>(tag: S) -> HtmlDefault {
        HtmlDefault {
            tag: tag.into(),
            attrs: vec![],
        }
    }

    /// Loads a set of HtmlDefault structs from a given file
    ///
    /// # Arguments
    ///
    /// * `path` - Location of file to load
    pub fn from_file<P: AsRef<Path>>(path: P) -> HashSet<HtmlDefault> {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let c: TomlTest = toml::from_str(&contents).unwrap();
        dbg!(c);
        HashSet::new()
    }
}

impl PartialEq for HtmlDefault {
    fn eq(&self, other: &HtmlDefault) -> bool {
        self.tag == other.tag
    }
}
