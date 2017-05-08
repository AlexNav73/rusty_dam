
use rocket_contrib::Template;
use rocket::response::Redirect;
use rocket::request::Form;
use rocket::Data;

use std::io::Write;

use APIKey;

#[derive(Serialize)]
struct Content {
    message: String
}

#[derive(Debug, FromForm)]
struct Credentials {
    login: String,
    password: String
}

#[get("/")]
fn index(key: APIKey) -> Template {
    let message = Content { message: "Hello".to_string() };
    Template::render("templates/index", &message)
}

#[get("/", rank = 2)]
fn index_anon() -> Redirect {
    Redirect::to("/login")
}

#[get("/login")]
fn login() -> Template {
    let message = Content { message: "Login".to_string() };
    Template::render("templates/login", &message)
}

#[post("/login", data = "<cred>")]
fn login_post(cred: Form<Credentials>) -> String {
    format!("{:?}", cred.into_inner())
}

#[get("/home")]
fn home() -> &'static str {
    "Home"
}

