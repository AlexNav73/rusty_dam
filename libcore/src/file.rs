
use uuid::Uuid;

use std::path::Path;
use std::slice::Iter;
use std::iter::FromIterator;
use std::collections::HashMap;

use {Lazy, Entity, Document};

pub enum FileError {
    NotAFile,
    PathDoesNotExists,
}

pub struct File {
    id: Uuid,
    path: String,
}

impl File {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<File, FileError> {
        let path = path.as_ref();

        match (path.exists(), path.is_file()) {
            (true, true) => {
                Ok(File {
                    id: Uuid::new_v4(),
                    path: path.to_str().ok_or(FileError::PathDoesNotExists)?.to_string(),
                })
            }
            (false, _) => Err(FileError::PathDoesNotExists),
            (true, false) => Err(FileError::NotAFile),
        }
    }

    pub fn file_stem(&self) -> &str {
        Path::new(&self.path).file_stem().unwrap().to_str().unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct FileDto {}

impl Document<File> for FileDto {
    fn doc_type() -> &'static str {
        "file"
    }

    fn map(self) -> File {
        unimplemented!()
    }
}

impl Entity for File {
    type Dto = FileDto;

    fn id(&self) -> Uuid {
        self.id
    }

    fn map(&self) -> FileDto {
        unimplemented!()
    }
}

pub struct FileCollection {
    //latest: Option<Uuid>,
    //files: HashMap<Uuid, Lazy1<File>>
}

impl FileCollection {
    pub fn new() -> FileCollection {
        // TODO: Proper impl
        //FileCollection { latest: None, files: HashMap::new() }
        FileCollection {}
    }

    //pub fn latest(&self) -> File {
        //self.files[&self.latest.unwrap()] // TODO: Error handling
            //.unwrap(self.connection)
            //.expect("File not found")
    //}
}

impl<'a> FromIterator<&'a Uuid> for FileCollection {
    fn from_iter<T>(iter: T) -> Self
        where T: IntoIterator<Item = &'a Uuid>
    {
        //FileCollection { files: iter.into_iter().map(|id| id.into()).collect() }
        FileCollection {}
    }
}

