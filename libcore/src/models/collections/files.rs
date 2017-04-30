
use uuid::Uuid;

use std::collections::HashMap;

use Entity;
use models::collections::{EntityCollection, Ids, Iter};
use models::file::File;
use connection::App;

pub struct FileCollection {
    latest: Option<Uuid>,
    files: HashMap<Uuid, File>,
    application: App,
}

impl FileCollection {
    pub fn new(app: App) -> FileCollection {
        FileCollection {
            latest: None,
            files: HashMap::new(),
            application: app,
        }
    }

    pub fn from_iter<'a, T>(iter: T, app: App) -> FileCollection
        where T: IntoIterator<Item = File>
    {
        FileCollection {
            latest: None,
            files: iter.into_iter().map(|f| (f.id(), f)).collect(),
            application: app,
        }
    }

    // pub fn latest(&self) -> File {
    // self.files[&self.latest.unwrap()] // TODO: Error handling
    // .unwrap(self.application)
    // .expect("File not found")
    // }
}

impl EntityCollection<File> for FileCollection {
    fn ids(&self) -> Ids<File> {
        Ids::new(self.files.keys())
    }

    fn iter(&self) -> Iter<File> {
        Iter::new(self.application.clone(), self.files.values())
    }
}
