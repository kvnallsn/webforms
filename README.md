# Rust WebForms Library

This library supports validating and (in the future) rendering HTML forms for use with Askama or Tera templates.

## Form Validation

To add form validation to a struct, implement or derive the ValidateForm trait.

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

## HTML Generate

TBD

## Information

License: MIT
Author: Kevin Allison
