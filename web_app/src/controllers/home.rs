
use rocket_contrib::Template;
use rocket::response::Redirect;

use APIKey;

#[derive(Serialize)]
struct Content {
    message: String
}

#[get("/")]
fn index(_key: APIKey) -> Template {
    let message = Content { message: "Hello".to_string() };
    Template::render("templates/index", &message)
}

#[get("/", rank = 2)]
fn index_anon() -> Redirect {
    Redirect::to("/security/login")
}

