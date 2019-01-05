# Rust WebForms Library

This library supports validating and (in the future) rendering HTML forms for use with Askama or Tera templates.

## Form Validation

To add form validation to a struct, implement or derive the ValidateForm trait.  Validator attributes can either be applied to the struct or to individual fields.

Struct attribute: validate_regex

| Validator    | Type  | Argument Type | Description                                            | Notes |
| ------------ | ----- | ------------- | ------------------------------------------------------ | ----- |
| *identifier* | Ident | Regex         | Creates an identifier that links to the regex provided | 1     |

Notes:

1. Requires the `lazy_static` and `regex` crates as dependencies

Field attribute: validate

| Validator      | Type    | Argument Type | Description                                                             | Notes |
| -------------- | ------- | ------------- | ----------------------------------------------------------------------- | ----- |
| email          | String  | None          | Checks if input is a valid email address                                | 1     |
| phone          | String  | None          | Checks if input is a valid **US** phone number                          | 1     |
| min_length     | String  | Integer       | Checks if input length in characters is greater than the value provided |       |
| max_length     | String  | Integer       | Checks if input length in characters is less than the value provided    |       |
| min_value      | Numeric | Numeric       | Checks if input is greater than the value provided                      | 2     |
| max_value      | Numeric | Numeric       | Checks if input is less than the value provided                         | 2     |
| regex          | String  | Regex         | Checks if input matches the supplied regex                              | 1     |
| compiled_regex | String  | String        | Checks if input matches a regex specified in a struct validator         | 1     |

Notes:

1. Requires the `lazy_static` and `regex` crates as dependencies
2. Can be any numeric type (integer/float) but type must match the field being checked!

### Example

```rust
use webforms::validate::ValidateForm;

#[derive(ValidateForm)]
struct LoginForm {
    #[validate(email)]
    pub email: String,

    #[validate(regex = r"^some_password_regex$")]
    pub password: String,
}
```

This will automatically implement the ValidateForm trait allowing the `validate()` method to be called like so:

```rust
pub fn main() {
    let form = LoginForm {
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
