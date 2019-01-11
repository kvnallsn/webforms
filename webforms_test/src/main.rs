use lazy_static::lazy_static;
use regex::Regex;
use webforms::validate::{ValidateError, ValidateForm};

#[derive(ValidateForm)]
#[validate_regex(user_re = r"^mark$")]
struct LoginForm<'a> {
    #[validate(min_length = 3)]
    #[validate(max_length = 10)]
    #[validate(compiled_regex = "user_re")]
    pub username: &'a str,

    #[validate(min_length = 8)]
    pub password: &'a str,

    #[validate_match(password)]
    pub password2: &'a str,

    #[validate(email)]
    pub email: &'a str,

    #[validate(min_value = 18)]
    pub age: i32,
}

fn main() {
    let form = LoginForm {
        username: "mike",
        password: "a",
        password2: "aa",
        email: "mike@mail.com",
        age: 17,
    };

    match form.validate() {
        Ok(_) => println!("Validate Success!"),
        Err(errs) => {
            for err in errs {
                println!("{}", err);
            }
        }
    };
}
