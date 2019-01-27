//! Module to contain any default attributes that should be written
//! when an html tag is encountered.  These can be overriden by
//! using the #[html()] attribute for a specific field

use serde_derive::Deserialize;
use std::{collections::BTreeMap, fs::File, io::Read, path::Path};

#[derive(Deserialize, Debug)]
pub struct HtmlDefaults {
    pub tags: BTreeMap<String, BTreeMap<String, String>>,
    types: BTreeMap<String, String>,
}

impl HtmlDefaults {
    /// Loads a set of HtmlDefault structs from a given TOML file
    ///
    /// The TOML file is broken down into two (2) different sections:
    /// `[tags]` and `[types]`
    ///
    /// The `[tags]` section allows for assigning default attributes to
    /// a given html tag every time it is encounted.
    ///
    /// The `[types]` section is specific to `<input>` tags.  It allows
    /// for mapping between a given Rust type (e.g., i32) and a corresponding
    /// html input type (e.g., "number").
    ///
    /// An example file applying the class "form" to all form tags and
    /// mapping the i32 class to the number html type
    ///
    /// ```toml
    /// [tags]
    /// [tags.form]
    /// class = "form"
    ///
    /// [types]
    /// i32 = "number"
    /// ```
    ///
    /// # Arguments
    ///
    /// * `path` - Location of file to load
    pub fn from_file<P: AsRef<Path>>(path: P) -> HtmlDefaults {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let c: HtmlDefaults = toml::from_str(&contents).unwrap();
        c
    }

    /// Checks to see if the type contained in id has a registered default
    /// type in the config TOML file
    ///
    /// # Arguments
    ///
    /// * `id` - Identity (e.g., variable) to check type of
    pub fn has_input_type(&self, id: &syn::Ident) -> bool {
        self.types.contains_key(&id.to_string())
    }

    /// Returns the default html input type associated with this rust type
    ///
    /// # Arguments
    ///
    /// * `id` - Identity (e.g., variable) to check type of
    pub fn get_input_type(&self, id: &syn::Ident) -> &str {
        self.types
            .get(&id.to_string())
            .expect("default type missing!")
    }
}
