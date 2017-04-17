
use uuid::Uuid;
use diesel::prelude::*;
use diesel::types::*;
use diesel::pg::types::sql_types;

use pg::PgDto;
use connection::{App, Connection};
use LoadError;

pub mod schema;
pub mod models;

use self::schema::classifications::dsl::*;
use self::models::*;

sql_function!(get_classification_name_path, get_classification_name_path_t, (cls_id: sql_types::Uuid) -> Array<Text>);

pub fn get_name_path(app: &App, cls_id: Uuid) -> Result<Vec<String>, LoadError> {
    let conn = app.connection();
    let pg_conn = conn.borrow_mut().pg().connect();
    ::diesel::select(get_classification_name_path(cls_id))
        .first::<Vec<String>>(&*pg_conn)
        .map_err(|_| LoadError::NotFound)
}

pub fn get_cls_by_id(app: &App, cls_id: Uuid) -> Result<Classification, LoadError> {
    let conn = app.connection();
    let pg_conn = conn.borrow_mut().pg().connect();
    classifications
        .filter(id.eq(cls_id))
        .first::<Classification>(&*pg_conn)
        .map_err(|_| LoadError::NotFound)
}

