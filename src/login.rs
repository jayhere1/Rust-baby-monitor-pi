use rocket::request::Form;
use rocket::response::{content::Html, Redirect};
use rocket_contrib::templates::Template;

#[derive(FromForm)]
struct LoginForm {
    username: String,
    password: String,
}

#[get("/")]
fn index() -> Template {
    let context = (); // You can pass data to the template context here
    Template::render("index", context)
}

#[post("/login", data = "<form>")]
fn login(form: Form<LoginForm>) -> Redirect {
    let username = form.username.to_string();
    let password = form.password.to_string();

    // Perform authentication logic here
    // For example, you can compare the username and password with predefined values
    if username == "myuser" && password == "mypassword" {
        // Redirect to the video streaming page on successful login
        Redirect::to("/stream")
    } else {
        // Redirect back to the home page on failed login
        Redirect::to("/")
    }
}

