//! Validate macro implementation

use crate::proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use rand::Rng;
use std::collections::HashMap;
use syn;

mod validators;

/// Various kinds of validation types we support along with
/// the necessary critera to validate the actual value
pub(crate) enum ValidateType {
    StringMin(syn::LitInt),
    StringMax(syn::LitInt),
    ValueMin(syn::LitInt),
    ValueMax(syn::LitInt),
    Regex(String),
    Email(String),
    Phone(String),
    Match(syn::Ident),
}

/// Container for a given validation field and all
/// #[validate] attributes applied to it
pub(crate) struct ValidateField<'a> {
    pub field: &'a syn::Field,
    pub attrs: Vec<ValidateType>,
    pub optional: bool,
}

/// ToTokens implementation for ValidateField
///
/// Allows the struct to be used inside a quote! macro
impl<'a> ToTokens for ValidateField<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        validators::write(self, tokens);
    }
}

/// Validation informatoin for the entire struct deriving
/// ValidateFrom
pub(crate) struct ValidateStruct<'a> {
    pub ident: &'a syn::Ident,
    pub regex_tokens: HashMap<String, String>,
    pub fields: Vec<ValidateField<'a>>,
}

/// ToTokens implementation for ValidateStruct
///
/// Allows the struct to be used inside a quote! macro
impl<'a> ToTokens for ValidateStruct<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let fields = &self.fields;

        // If we're using a regex matcher, expand the regex using lazy_static
        // to ensure it only compiles once
        if self.regex_tokens.len() > 0 {
            let regex_tokens = self.regex_tokens.iter().map(|(id, regex)| {
                let rid = syn::Ident::new(&id, Span::call_site());
                quote! {
                    static ref #rid: Regex = Regex::new(&#regex).expect("failed to compile regex");
                }
            });

            tokens.extend(quote! {
                lazy_static! {
                    #(#regex_tokens)*
                }
            });
        }

        tokens.extend(quote! {
            #(#fields)*
        });
    }
}

impl<'a> ValidateStruct<'a> {
    fn new(ident: &'a syn::Ident) -> ValidateStruct<'a> {
        ValidateStruct {
            ident: ident,
            regex_tokens: HashMap::new(),
            fields: vec![],
        }
    }

    /// Helper method to parse all struct attributes then parse all
    /// attributes attached to fields in the structs.
    ///
    /// Calls `self.parse_struct_attributes` theh `self.parse_field_attributes`
    ///
    /// Arguments:
    /// * `ast` - Syntax Tree obtained from parsing input with syn
    fn parse(&mut self, ast: &'a syn::DeriveInput) {
        self.parse_struct_attributes(ast);
        self.parse_field_attributes(ast);
    }

    /// Parses all attributes attached to a struct that derives ValidateForm
    /// Examples include: #[validate_regex]
    ///
    /// Arguments:
    /// * `ast` - Syntax Tree obtained from parsing input with syn
    fn parse_struct_attributes(&mut self, ast: &'a syn::DeriveInput) {
        for attr in &ast.attrs {
            if attr.path.is_ident("validate_regex") {
                // Compile a regex expression
                let meta = &attr
                    .parse_meta()
                    .expect("Failed to parse validate_regex attribute");

                self.parse_validate_regex_attr(&meta);
            }
        }
    }

    /// Parses attributes on files attached to this struct.  Examples
    /// include: #[validate]
    ///
    /// # Arguments
    /// * `ast` - Syntax Tree obtained from parsing input with syn
    fn parse_field_attributes(&mut self, ast: &'a syn::DeriveInput) {
        let fields = match ast.data {
            syn::Data::Struct(syn::DataStruct {
                fields: syn::Fields::Named(ref fields),
                ..
            }) => &fields.named,
            _ => panic!("ValidateForm only defined on data structs!"),
        };

        for field in fields.iter() {
            let mut info = ValidateField::new(field);

            for attr in &field.attrs {
                if attr.path.is_ident("validate") {
                    let meta = &attr
                        .parse_meta()
                        .expect("Failed to parse webform validate attribute");
                    info.parse_validate_attribute(meta, self);
                } else if attr.path.is_ident("validate_match") {
                    let meta = &attr
                        .parse_meta()
                        .expect("Failed to parse webform validate attribute");
                    info.parse_validate_match_attribute(meta);
                } else if attr.path.is_ident("validat") {

                }
            }

            self.fields.push(info);
        }
    }

    /// Parses the #[validate_regex] attribute applied to structs
    ///
    /// # Arguments
    /// * `meta` - The parsed meta argument to extract the compiled regex from
    fn parse_validate_regex_attr(&mut self, meta: &syn::Meta) {
        match meta {
            syn::Meta::List(ref list) => {
                for nested in list.nested.iter() {
                    match nested {
                        syn::NestedMeta::Meta(m) => self.parse_validate_regex_attr(m),
                        _ => panic!("Unsupported list attribute"),
                    }
                }
            }
            syn::Meta::NameValue(ref nv) => {
                // nv.ident is the name of the regex we are going to compile,
                // it's value must be a Literal String
                match nv.lit {
                    syn::Lit::Str(ref s) => {
                        let k = nv.ident.to_string();
                        if !self.regex_tokens.contains_key(&k) {
                            self.regex_tokens.insert(k, s.value());
                        } else {
                            panic!("ValidateForm: regex with id `{}` already defined!", k);
                        }
                    }
                    _ => panic!(
                    "ValidateForm: compiling a regex via validate_regex requires a string argument"
                ),
                }
            }
            _ => panic!("Only List meta supported for struct info"),
        }
    }
}

impl<'a> ValidateField<'a> {
    /// Creates a new ValidateField structure.  Contains
    /// all necessary information to generate a validation statement for
    /// ensuring contents contained meets specified critera
    ///
    /// # Arguments
    /// * `field` - The field (member in struct) for this validator
    fn new(field: &'a syn::Field) -> ValidateField<'a> {
        ValidateField {
            field: field,
            attrs: vec![],
            optional: false,
        }
    }

    fn parse_validate_match_attribute(&mut self, meta: &syn::Meta) {
        match meta {
            syn::Meta::Word(ref w) => {
                self.attrs.push(ValidateType::Match(w.clone()));
            },
            syn::Meta::List(ref list) => {
                for nested in list.nested.iter() {
                    match nested {
                        syn::NestedMeta::Meta(m) => self.parse_validate_match_attribute(m),
                        _ => panic!(""),
                    }
                }
            },
            _ => panic!("")
        }
    }

    /// Parses the #[validate] attribute on a given field in a
    /// Data Struct that derives ValidateForm
    ///
    /// # Arguments
    /// * `struct_info` - Containing parent validation structure
    fn parse_validate_attribute(&mut self, meta: &syn::Meta, struct_info: &mut ValidateStruct<'a>) {
        match meta {
            syn::Meta::Word(ref w) => {
                if w == "email" {
                    let id = "form_regex_email".to_owned();
                    let regex = r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$".to_owned();

                    if !struct_info.regex_tokens.contains_key(&id) {
                        struct_info.regex_tokens.insert(id.clone(), regex);
                    }

                    self.attrs.push(ValidateType::Email(id));
                } else if w == "phone" {
                    let id = "form_regex_us_phone".to_owned();
                    let regex = r"^(\+\d{1,2}\s)?\(?\d{3}\)?[\s.-]?\d{3}[\s.-]?\d{4}$".to_owned();

                    if !struct_info.regex_tokens.contains_key(&id) {
                        struct_info.regex_tokens.insert(id.clone(), regex);
                    }

                    self.attrs.push(ValidateType::Phone(id));
                } else if w == "optional" {
                    self.optional = true;
                }
            }
            syn::Meta::List(ref list) => {
                for nested in list.nested.iter() {
                    match nested {
                        syn::NestedMeta::Meta(m) => self.parse_validate_attribute(m, struct_info),
                        _ => panic!("ValidateForm: Unsupported validate attribute"),
                    }
                }
            }
            syn::Meta::NameValue(ref nv) => {
                if nv.ident == "min_length" {
                    match nv.lit {
                        syn::Lit::Int(ref i) => self.attrs.push(ValidateType::StringMin(i.clone())),
                        _ => panic!("min_length requires an integer argument"),
                    }
                } else if nv.ident == "max_length" {
                    match nv.lit {
                        syn::Lit::Int(ref i) => self.attrs.push(ValidateType::StringMax(i.clone())),
                        _ => panic!("max_length requires an integer argument"),
                    }
                } else if nv.ident == "min_value" {
                    match nv.lit {
                        syn::Lit::Int(ref i) => self.attrs.push(ValidateType::ValueMin(i.clone())),
                        _ => panic!("min_value requires an integer argument"),
                    }
                } else if nv.ident == "max_value" {
                    match nv.lit {
                        syn::Lit::Int(ref i) => self.attrs.push(ValidateType::ValueMax(i.clone())),
                        _ => panic!("max_value requires an integer argument"),
                    }
                } else if nv.ident == "regex" {
                    match nv.lit {
                        syn::Lit::Str(ref s) => {
                            let regex = s.value();
                            let mut rng = rand::thread_rng();
                            let id = format!(
                                "form_regex_{}_{}",
                                self.field.ident.as_ref().expect("").to_string(),
                                rng.gen::<u32>()
                            );

                            if !struct_info.regex_tokens.contains_key(&id) {
                                struct_info.regex_tokens.insert(id.clone(), regex);
                            } else {
                                panic!("ValidateForm: regex `{}` already defined!", id);
                            }

                            self.attrs.push(ValidateType::Regex(id));
                        }
                        _ => panic!("regex requires a string argument"),
                    }
                } else if nv.ident == "compiled_regex" {
                    match nv.lit {
                        syn::Lit::Str(ref s) => {
                            let regex = s.value();
                            if struct_info.regex_tokens.contains_key(&regex) {
                                self.attrs.push(ValidateType::Regex(regex));
                            } else {
                                panic!("compiled_regex requires a pre-compiled regex via a `validate_regex` struct attribute");
                            }
                        }
                        _ => panic!("compiled_regex requires a string argumente"),
                    }
                } else {
                    println!("Unknown attribute: {}", nv.ident.to_string());
                }
            }
        }
    }
}

pub(crate) fn impl_validate_macro(ast: syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;

    let mut validate_info = ValidateStruct::new(name);
    validate_info.parse(&ast);

    let gen = quote! {
        impl #generics ValidateForm for #name #generics {
            fn validate(&self) -> Result<(), Vec<ValidateError>> {

                let mut v: Vec<ValidateError> = Vec::new();

                #validate_info

                match v.len() {
                    0 => Ok(()),
                    _ => Err(v),
                }
            }
        }
    };

    gen.into()
}
