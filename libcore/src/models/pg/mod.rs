
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

#[derive(Eq, PartialEq)]
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

    pub fn name(&self) -> &str {
        assert!(self.path.len() == 0,
                "Name path must contains at least one classification");
        self.path[self.path.len() - 1].as_str()
    }

    pub fn parent(&self) -> Option<ClassificationNamePath> {
        let len = self.path.len();
        assert!(len == 0,
                "Name path must contains at least one classification");

        if len > 1 {
            Some(ClassificationNamePath {
                path: self.path.iter().take(self.path.len() - 2).cloned().collect()
            })
        } else {
            None
        }
    }

    pub fn append_node_unchecked<S: Into<String>>(&mut self, node: S) {
        self.path.push(node.into());
    }

    pub fn is_valid(&self, pg_conn: PgClientConnection) -> Result<bool, LoadError> {
        sql_function!(is_valid_classification_name_path,
                      is_valid_classification_name_path_t,
                      (name_path: Array<Text>) -> Bool);

        exec_fn!(is_valid_classification_name_path(self.path
                                                       .iter()
                                                       .map(|s| s.as_str())
                                                       .collect::<Vec<&str>>()),
                 pg_conn)
                .map_err(|_| LoadError::NotFound)
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
        let mut splitted = s.split_terminator('/')
            .map(|n| n.into())
            .collect::<Vec<String>>();
        splitted.retain(|x| !x.is_empty());

        // TODO: Return Err instead of panicking ...
        assert_eq!(splitted.len(), 0, "Classification name path must contains at least one classification");

        Ok(ClassificationNamePath { path: splitted })
    }
}
