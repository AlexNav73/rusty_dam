
use rocket_contrib::Template;
use rocket::response::Redirect;
use rocket::request::Form;
use rocket::http::{Cookie, Cookies};
use rocket::Data;
use libcore::{App, LoadError};

use std::io::Write;

use {APIKey, Config};

#[derive(Serialize)]
struct Content {
    message: String
}

#[get("/")]
fn index(key: APIKey) -> Template {
    let message = Content { message: "Hello".to_string() };
    Template::render("templates/index", &message)
}

#[get("/", rank = 2)]
fn index_anon() -> Redirect {
    Redirect::to("/security/login")
}

