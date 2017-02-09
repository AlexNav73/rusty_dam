
use uuid::Uuid;

use std::path::Path;

use Entity;

pub enum FileError {
    NotAFile,
    PathDoesNotExists,
}

pub struct File {
    id: Uuid,
    path: String
}

impl File {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<File, FileError> {
        let path = path.as_ref();

        match (path.exists(), path.is_file()) {
            (true, true) => Ok(File { 
                id: Uuid::new_v4(), 
                path: path.to_str().ok_or(FileError::PathDoesNotExists)?.to_string()
            }),
            (false, _) => Err(FileError::PathDoesNotExists),
            (true, false) => Err(FileError::NotAFile)
        }
    }

    pub fn file_stem(&self) -> &str {
        Path::new(&self.path).file_stem().unwrap().to_str().unwrap()
    }
}

impl Entity for File {
    fn id(&self) -> Uuid { self.id }
}

