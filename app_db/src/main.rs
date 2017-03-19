
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

mod fields;
mod schema;

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    use self::schema::fields::dsl::*;
    use fields::Fields;

    let connection = establish_connection();
    let results = fields.filter(id.eq(0))
        .limit(5)
        .load::<Fields>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} fields", results.len());
    for post in results {
        println!("{}", post.id);
        println!("----------\n");
        println!("{}", post.name);
    }
}

