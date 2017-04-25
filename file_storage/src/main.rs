#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::UUID;
use rocket::Data;
use rocket::response::NamedFile;

use std::path::{PathBuf, Path};

#[post("/upload/<record_id>/<file_id>", data="<file>")]
fn upload(record_id: UUID, file_id: UUID, file: Data) -> &'static str {
    "Store"
}

#[get("/download/<file..>")]
fn download(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![upload, download])
        .launch();
}
