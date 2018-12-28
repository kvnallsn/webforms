
use webforms::ValidateForm;

#[derive(ValidateForm)]
struct LoginForm<'a> {
    #[validate(min_length = 3)]
    #[validate(max_length = 10)]
    pub username: &'a str,

    #[validate(min_length = 8)] 
    pub password: &'a str,

    #[validate(min_value = 18)]
    pub age: i32,
}


fn main() {
    let form = LoginForm {
        username: "mike",
        password: "a",
        age: 17,
    };

    match form.validate() {
        Ok(_) => println!("Validate Success!"),
        Err(e) => println!("Errors: {:?}", e),
    };
}
