#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate rocket;
extern crate rocket_contrib;
extern crate uuid;
extern crate libcore;

use uuid::Uuid;
use rocket::request::{self, FromRequest};
use rocket::outcome::Outcome;
use rocket::http::Status;
use rocket::Request;
use libcore::{App, Configuration};

mod controllers;

use controllers::account::{ SESSION_KEY_NAME, SESSION_LOGIN_NAME };

struct Config;

impl Configuration for Config {
    fn id(&self) -> String {
        "".into()
    }

    fn es_index_name(&self) -> String {
        "rusty_dam".into()
    }

    fn es_url(&self) -> String {
        "http://192.168.99.100:32769/".into()
    }
}

pub struct APIKey(App);

impl<'a, 'r> FromRequest<'a, 'r> for APIKey {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<APIKey, ()> {
        let session_key = request.cookies().find(SESSION_KEY_NAME);
        let session_login = request.cookies().find(SESSION_LOGIN_NAME);
        if let Some(ref key) = session_key {
            if let Ok(session_id) = Uuid::parse_str(key.value()) {
                let mut app = App::new(Config);
                let login = session_login.map(|n| n.value().to_owned()).unwrap_or("".to_string());
                if let Ok(_) = app.connect_to_session(session_id, login) {
                    return Outcome::Success(APIKey(app));
                } else {
                    return Outcome::Failure((Status::BadRequest, ()));
                }
            }
        }

        Outcome::Forward(())
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![
               controllers::home::index,
               controllers::home::index_anon,
               controllers::static_files::all
        ])
        .mount("/security", routes![
               controllers::account::login,
               controllers::account::login_post,
        ]).launch();
}
