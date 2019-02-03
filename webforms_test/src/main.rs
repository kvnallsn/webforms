use askama::Template;
use lazy_static::lazy_static;
use regex::Regex;
use std::{fs::File, io::Write};
use webforms::{
    attrs,
    html::{HtmlForm, HtmlFormBuilder},
    validate::{ValidateError, ValidateForm},
};

#[derive(HtmlForm)]
//#[validate_regex(user_re = r"^mark$")]
struct LoginForm<'a> {
    //#[validate(min_length = 3, max_length = 10, compiled_regex = "user_re")]
    #[html_validate(pattern = "^[a-z]{7}$")]
    #[html_input(text, class = "input-text", placeholder = "Username", required)]
    pub username: &'a str,

    //#[validate(min_length = 8)]
    #[html_input(password, beer = "coors")]
    pub password: &'a str,

    //#[validate_match(password)]
    #[html_input(password)]
    pub password2: &'a str,

    //#[validate(email)]
    #[html_input(email)]
    pub email: &'a str,

    #[html_validate(min = 18)]
    //#[validate(optional)]
    pub age: Option<i32>,
}

#[derive(Template)]
#[template(path = "hello.html", print = "code")]
struct HelloTemplate<'a> {
    pub name: &'a str,
    pub form: HtmlFormBuilder<'a>,
}

fn main() {
    let form = LoginForm {
        username: "mike",
        password: "a",
        password2: "aa",
        email: "mike@mail.com",
        age: Some(17),
    };

    println!("\n-------- VALIDATE TEST ----------\n");

    match form.validate_form() {
        true => println!("Validate Success!"),
        false => println!("Validate Failure!")
    };

    println!("\n---------- HTML TEST ------------\n");

    let f = form.form();
    let f2 = f
        .builder("username")
        .value("beer")
        .attr("drink", "beer")
        .finish();
    println!("{}", f2);

    println!("\n---------- FORM TEST ------------\n");
    println!("{}", f);

    println!("\n--------- RENDER TEST -----------\n");
    let template = HelloTemplate {
        name: "WebForm",
        form: form.form(),
    };

    let mut fp = File::create("test.html").unwrap();
    fp.write(template.render().unwrap().as_bytes()).unwrap();
}
