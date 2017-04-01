
use uuid::Uuid;

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use {Entity, ToDto, FromDto, Load, LoadError};
use es::EsDto;
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
    fn id(&self) -> Uuid {
        self.id
    }

    fn create(app: &App) -> File {
        File {
            id: Uuid::new_v4(),
            path: None,
            connection: app.connection(),
        }
    }
}

impl ToDto for File {
    type Dto = FileDto;

    fn to_dto(&self) -> FileDto {
        FileDto { id: self.id }
    }
}

impl Load for File {
    fn load(_c: Rc<RefCell<Connection>>, _id: Uuid) -> Result<Self, LoadError> {
        unimplemented!()
    }
}

#[derive(Serialize, Deserialize)]
pub struct FileDto {
    id: Uuid,
}

impl EsDto for FileDto {
    fn doc_type() -> &'static str {
        "files"
    }

    fn id(&self) -> Uuid {
        self.id
    }
}

impl FromDto for FileDto {
    type Item = File;

    fn from_dto(self, conn: Rc<RefCell<Connection>>) -> File {
        File {
            id: self.id,
            path: None,
            connection: conn,
        }
    }
}
