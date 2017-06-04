
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

#[derive(Debug, FromForm)]
struct Credentials {
    login: String,
    password: String
}

#[derive(Serialize)]
struct Content {
    message: String
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
        app.login(&cred.login, &cred.password)
            .map(|_| app.session_id().unwrap())
            .map(move |sid| {
                let mut session = Cookie::new(SESSION_KEY_NAME, sid.to_string());
                let mut username = Cookie::new(SESSION_LOGIN_NAME, cred.login);

                session.set_path("/");
                username.set_path("/");

                cookies.add(session);
                cookies.add(username);
                Redirect::to("/")
            })
            .or(Ok(Redirect::to("/login")))
    } else {
        Ok(Redirect::to("/login"))
    }
}

