# Rust WebForms Library

This library supports validating and (in the future) rendering HTML forms for use with Askama or Tera templates.

## Table of Contents

- [Rust WebForms Library](#rust-webforms-library)
  - [Table of Contents](#table-of-contents)
  - [Form Validation](#form-validation)
    - [Struct Attibutes](#struct-attibutes)
      - [`#[validate_regex(...)]`](#validateregex)
      - [Struct Attribute Example](#struct-attribute-example)
    - [Field Attributes](#field-attributes)
      - [`#[validate(...)]`](#validate)
      - [`#[validate_match(...)]`](#validatematch)
      - [`#[validate_regex(...)]` (Field)](#validateregex-field)
      - [Field Attribute Example](#field-attribute-example)
    - [Using Geneated Code](#using-geneated-code)
  - [HTML Generation](#html-generation)
  - [Information](#information)

## Form Validation

To add form validation to a struct, implement or derive the ValidateForm trait.  Validator attributes can either be applied to the struct or to individual fields.

### Struct Attibutes

#### `#[validate_regex(...)]`

| Validator    | Type  | Argument Type | Description                                            | Notes |
| ------------ | ----- | ------------- | ------------------------------------------------------ | ----- |
| *identifier* | Ident | Regex         | Creates an identifier that links to the regex provided | 1, 2  |

Notes:

1. Requires the `lazy_static` and `regex` crates as dependencies
2. *identifer* is any user-specified string.  This will be turned into an identifier than can be used with the `#[validate(compiled_regex = "...")]` field attribute
  
#### Struct Attribute Example

The following example compiles a regex name `pw_regex` and allows it to be used multiple times later in the form while only being compiled once.

```rust
use webforms::validate::ValidateForm;

#[derive(ValidateForm)]
#[validate_regex(pw_regex = r"^a_regex_string$")]
struct RegisterForm {
    ...
    #[validate(compiled_regex = "pw_regex")]
    pub password1: String,

    #[validate(compiled_regex = "pw_regex")]
    pub password2: String,
    ...
}
```

### Field Attributes

#### `#[validate(...)]`

| Validator    | Type    | Argument Type | Description                                                             | Notes |
| ------------ | ------- | ------------- | ----------------------------------------------------------------------- | ----- |
| `email`      | String  | None          | Checks if input is a valid email address                                | 1     |
| `phone`      | String  | None          | Checks if input is a valid **US** phone number                          | 1     |
| `min_length` | String  | Integer       | Checks if input length in characters is greater than the value provided |       |
| `max_length` | String  | Integer       | Checks if input length in characters is less than the value provided    |       |
| `min_value`  | Numeric | Numeric       | Checks if input is greater than the value provided                      | 2     |
| `max_value`  | Numeric | Numeric       | Checks if input is less than the value provided                         | 2     |
| `regex`      | String  | Regex         | Checks if input matches the supplied regex                              | 1     |

Notes:

1. Requires the `lazy_static` and `regex` crates as dependencies
2. Can be any numeric type (integer/float) but type must match the field being checked!

#### `#[validate_match(...)]`

| Argument | Type     | Argument Type   | Description                                                       | Notes |
| -------- | -------- | --------------- | ----------------------------------------------------------------- | ----- |
| *field*  | *Varies* | Field in Struct | Checks if this field matches the value specified in another field | 1     |

1. Type can vary, but must exactly match the field indicated in the attribute

#### `#[validate_regex(...)]` (Field)

| Argument | Type   | Argument Type | Description                                                                     | Notes |
| -------- | ------ | ------------- | ------------------------------------------------------------------------------- | ----- |
| *regex*  | String | Variable Name | Checks if this field matches the compiled regex stated in the struct attributes | 1     |

1. Requires the `lazy_static` and `regex` crates as dependencies
  
#### Field Attribute Example

```rust
use webforms::validate::ValidateForm;

#[derive(ValidateForm)]
struct UpdateProfileForm {
    #[validate(email)]
    pub email: String,

    #[validate(regex = r"^some_password_regex$")]
    pub password: String,

    #[validate_match(password)]
    pub password2: String,

    #[validate(phone)]
    pub phone: String,

    #[validate(min_value = 18)]
    pub age: u8;
}
```

### Using Geneated Code

This will automatically implement the ValidateForm trait allowing the `validate()` method to be called like so:

```rust
pub fn main() {
    let form = RegisterForm {
        ...
    };

    match form.validate() {
        Ok(_) => println!("success!"),
        Err(errs) => {
            for err in errs {
                println!("{:?}", err);
            }
        }
    }
}
```

validate() returns Ok(()) if validation suceeded or a vector of ValidationError types, each describing what field failed validation.

## HTML Generation

TODO: Goal is to implement a method (perhans `render()`) that can be called from templating libraries to render a form to HTML

## Information

License: MIT

Author: Kevin Allison
