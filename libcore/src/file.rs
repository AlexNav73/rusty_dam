
use uuid::Uuid;

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use {Entity, Document};
use connection::Connection;

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
    pub fn new<P: AsRef<Path>>(_path: P) -> Result<File, FileError> {
        unimplemented!()
/*        let path = path.as_ref();*/

        //match (path.exists(), path.is_file()) {
            //(true, true) => {
                //Ok(File {
                    //id: Uuid::new_v4(),
                    //path: Some(path.to_str().ok_or(FileError::PathDoesNotExists)?.to_string()),
                //})
            //}
            //(false, _) => Err(FileError::PathDoesNotExists),
            //(true, false) => Err(FileError::NotAFile),
        /*}*/
    }

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

#[derive(Serialize, Deserialize)]
pub struct FileDto {
    id: Uuid,
}

impl Document<File> for FileDto {
    fn doc_type() -> &'static str {
        "file"
    }

    fn map(self, conn: Rc<RefCell<Connection>>) -> File {
        File {
            id: self.id,
            path: None,
            connection: conn,
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
}

