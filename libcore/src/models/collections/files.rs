
use uuid::Uuid;

use std::collections::HashMap;

use Entity;
use models::collections::{EntityCollection, Ids, Iter};
use models::file::File;
use connection::App;

pub struct FileCollection<'a> {
    latest: Option<Uuid>,
    files: HashMap<Uuid, File<'a>>,
    application: App<'a>,
}

impl<'a> FileCollection<'a> {
    pub fn new(app: App) -> Self {
        FileCollection {
            latest: None,
            files: HashMap::new(),
            application: app,
        }
    }

    pub fn from_iter<T>(iter: T, app: App) -> FileCollection
        where T: IntoIterator<Item=File<'a>>
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

impl<'a, 'b> EntityCollection<'a, 'b, File<'a>> for FileCollection<'a> {
    fn ids(&self) -> Ids<'a, 'b, File<'a>> {
        Ids::new(self.files.keys())
    }

    fn iter(&self) -> Iter<'a, 'b, File<'a>> {
        Iter::new(self.application.clone(), self.files.values())
    }
}
