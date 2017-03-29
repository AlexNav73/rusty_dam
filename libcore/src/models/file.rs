
use uuid::Uuid;

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use {Entity, Document};
use connection::{App, Connection};

pub enum FileError {
    NotAFile,
    PathDoesNotExists,
}

pub struct File {
    id: Uuid,
    path: Option<String>,
    connection: Rc<RefCell<Connection>>,
}

impl File {
    pub fn file_stem(&self) -> &str {
        match self.path {
            Some(ref p) => Path::new(p).file_stem().unwrap().to_str().unwrap(),
            None => {
                // TODO: Proper impl
                unimplemented!()
            }
        }
    }
}

impl Entity for File {
    type Dto = FileDto;

    fn id(&self) -> Uuid {
        self.id
    }

    fn map(&self) -> FileDto {
        FileDto { id: self.id }
    }

    fn create(app: &App) -> File {
        File {
            id: Uuid::new_v4(),
            path: None,
            connection: app.connection(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct FileDto {
    id: Uuid,
}

impl Document<File> for FileDto {
    fn doc_type() -> &'static str {
        "files"
    }

    fn map(self, conn: Rc<RefCell<Connection>>) -> File {
        File {
            id: self.id,
            path: None,
            connection: conn,
        }
    }
}
