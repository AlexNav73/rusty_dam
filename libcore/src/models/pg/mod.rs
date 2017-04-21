
use uuid::Uuid;
use diesel::prelude::*;
use diesel::types::*;
use diesel::pg::types::sql_types;

use std::str::FromStr;
use std::string::ParseError;

use pg::PgClientConnection;
use LoadError;

pub mod schema;
pub mod models;

pub struct ClassificationNamePath {
    path: Vec<String>,
}

impl ClassificationNamePath {
    pub fn from_uuid(pg_conn: PgClientConnection, cid: Uuid) -> Result<Self, LoadError> {
        sql_function!(get_classification_name_path,
                      get_classification_name_path_t,
                      (cls_id: sql_types::Uuid) -> Array<Text>);

        exec_fn!(get_classification_name_path(cid), pg_conn)
            .and_then(|p| Ok(ClassificationNamePath { path: p }))
    }
}

impl ToString for ClassificationNamePath {
    fn to_string(&self) -> String {
        let total_cls_lens = self.path.iter().map(|c| c.len()).sum::<usize>();
        let total_sep = self.path.len();
        let mut res = String::with_capacity(total_cls_lens + total_sep);
        for cls in self.path.iter() {
            res.push_str("/");
            res.push_str(cls);
        }
        res
    }
}

impl FromStr for ClassificationNamePath {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ClassificationNamePath { path: s.split_terminator('/').map(|n| n.into()).collect() })
    }
}
