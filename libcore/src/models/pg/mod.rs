
use uuid::Uuid;
use diesel::prelude::*;
use diesel::types::*;
use diesel::pg::types::sql_types;

use std::str::FromStr;
use std::string::ParseError;

use pg::PgDto;
use connection::App;
use LoadError;

pub mod schema;
pub mod models;

use self::schema::classifications::dsl::*;
use self::models::*;

sql_function!(get_classification_name_path, get_classification_name_path_t, (cls_id: sql_types::Uuid) -> Array<Text>);

pub struct ClassificationNamePath {
    path: Vec<String>
}

impl ClassificationNamePath {
    pub fn form_uuid(app: &mut App, cid: Uuid) -> Result<Self, LoadError> {
        let pg_conn = app.pg().connect();
        ::diesel::select(get_classification_name_path(cid))
            .first::<Vec<String>>(&*pg_conn)
            .and_then(|p| Ok(ClassificationNamePath { path: p }))
            .map_err(|_| LoadError::NotFound)
    }
}

impl ToString for ClassificationNamePath {
    fn to_string(&self) -> String {
        let total_cls_lens = self.path.iter().map(|c| c.len()).sum::<usize>();
        let total_sep = self.path.len();
        let mut res = String::with_capacity(total_cls_lens + total_sep - 1);
        res.extend(self.path.iter().map(|x| x.as_str()));
        res
    }
}

impl FromStr for ClassificationNamePath {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ClassificationNamePath {
            path: s.split_terminator('/').map(|n| n.into()).collect()
        })
    }
}

pub fn get_cls_by_id(mut app: App, cls_id: Uuid) -> Result<Classification, LoadError> {
    let pg_conn = app.pg().connect();
    classifications
        .filter(id.eq(cls_id))
        .first::<Classification>(&*pg_conn)
        .map_err(|_| LoadError::NotFound)
}
