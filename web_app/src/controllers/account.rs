
use rocket_contrib::Template;
use rocket::response::Redirect;
use rocket::request::Form;
use rocket::http::{Cookie, Cookies};
use rocket::Data;
use libcore::{App, LoadError};

use std::io::Write;

use {APIKey, Config};

pub const SESSION_KEY_NAME: &str = "rusty_key";
pub const SESSION_LOGIN_NAME: &str = "rusty_login";

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
fn login_post(cookies: &Cookies, cred: Form<Credentials>) -> Result<Redirect, LoadError> {
    let cred = cred.into_inner();
    if !cred.login.is_empty() || !cred.password.is_empty() {
        let mut app = App::new(Config);
        let sid = app.login(&cred.login, &cred.password)
            .and_then(|_| Ok(app.session_id()))?
            .unwrap();

        cookies.add(Cookie::new(SESSION_KEY_NAME, sid.to_string()));
        cookies.add(Cookie::new(SESSION_LOGIN_NAME, cred.login));
        Ok(Redirect::to("/"))
    } else {
        Ok(Redirect::to("/login"))
    }
}

#[get("/home")]
fn home() -> &'static str {
    "Home"
}

