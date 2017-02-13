
use uuid::Uuid;

use std::path::Path;
use std::slice::Iter;
use std::iter::FromIterator;

use {Entity, Lazy, Document};

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
struct FileDto {}

impl Document<File> for FileDto {
    fn doc_type() -> &'static str {
        "file"
    }

    fn map(self) -> File {
        // TODO: Proper impl
        unimplemented!()
    }
}

impl Entity for File {
    fn id(&self) -> Uuid {
        self.id
    }
}

pub struct FileCollection {
    files: Vec<Lazy<File>>,
}

impl FileCollection {
    pub fn new() -> FileCollection {
        FileCollection { files: Vec::new() }
    }

    pub fn latest(&self) -> Option<&Lazy<File>> {
        self.files.iter().last()
    }

    pub fn iter<'a>(&'a self) -> FileIter<'a> {
        FileIter { inner: self.files.iter() }
    }
}

pub struct FileIter<'a> {
    inner: Iter<'a, Lazy<File>>,
}

impl<'a> Iterator for FileIter<'a> {
    type Item = &'a Lazy<File>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a> IntoIterator for &'a FileCollection {
    type Item = &'a Lazy<File>;
    type IntoIter = FileIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        FileIter { inner: self.files.iter() }
    }
}

impl<'a> FromIterator<&'a Uuid> for FileCollection {
    fn from_iter<T>(iter: T) -> Self
        where T: IntoIterator<Item = &'a Uuid>
    {
        FileCollection { files: iter.into_iter().map(|id| id.into()).collect() }
    }
}
