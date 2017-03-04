
use uuid::Uuid;

use std::collections::HashMap;
use std::iter::FromIterator;

use Lazy;
use file::File;
use collections::{EntityCollection, Ids};

pub struct FileCollection {
    latest: Option<Uuid>,
    files: HashMap<Uuid, Lazy<File>>,
}

impl FileCollection {
    pub fn new() -> FileCollection {
        // TODO: Proper impl
        FileCollection {
            latest: None,
            files: HashMap::new(),
        }
    }

    // pub fn latest(&self) -> File {
    // self.files[&self.latest.unwrap()] // TODO: Error handling
    // .unwrap(self.connection)
    // .expect("File not found")
    // }
}

impl EntityCollection<File> for FileCollection {
    fn ids(&self) -> Ids<File> {
        Ids::new(self.files.keys())
    }
}

impl<'a> FromIterator<&'a Uuid> for FileCollection {
    fn from_iter<T>(iter: T) -> Self
        where T: IntoIterator<Item = &'a Uuid>
    {
        FileCollection {
            latest: None,
            files: iter.into_iter().map(|&id| (id, Lazy::Guid(id))).collect(),
        }
    }
}

