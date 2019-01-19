use askama::Template;
use lazy_static::lazy_static;
use regex::Regex;
use webforms::{
    attrs,
    html::{HtmlForm, HtmlFormBuilder},
    validate::{ValidateError, ValidateForm},
};

#[derive(ValidateForm, HtmlForm)]
#[validate_regex(user_re = r"^mark$")]
#[html_form(method = "POST", action = "#", class = 2)]
#[html_submit(class = "btn", value = "Next")]
struct LoginForm<'a> {
    #[validate(min_length = 3)]
    #[validate(max_length = 10)]
    #[validate(compiled_regex = "user_re")]
    #[html(class = "input-textfield", placeholder = "Username", required)]
    #[html_input_type(password)]
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

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    pub name: &'a str,
    pub form: HtmlFormBuilder,
}

fn main() {
    let form = LoginForm {
        username: "mike",
        password: "a",
        password2: "aa",
        email: "mike@mail.com",
        age: 17,
    };

    println!("\n-------- VALIDATE TEST ----------\n");

    match form.validate() {
        Ok(_) => println!("Validate Success!"),
        Err(errs) => {
            for err in errs {
                println!("{}", err);
            }
        }
    };

    println!("\n---------- HTML TEST ------------\n");

    let f = form.form();
    //f.field_mut("username").attrs(attrs!("class" => "input-text"));
    let f2 = f.field("username").build(attrs!("class" => "input-text"));
    println!("{}", f2);

    println!("\n--------- RENDER TEST -----------\n");
    let template = HelloTemplate {
        name: "WebForm",
        form: form.form(),
    };

    println!("{}", template.render().unwrap());
}
