
use uuid::Uuid;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use Lazy;
use models::collections::{EntityCollection, Ids, IterMut};
use models::file::File;
use connection::Connection;

pub struct FileCollection {
    latest: Option<Uuid>,
    files: HashMap<Uuid, Lazy<File>>,
    connection: Rc<RefCell<Connection>>,
}

impl FileCollection {
    pub fn new(conn: Rc<RefCell<Connection>>) -> FileCollection {
        FileCollection {
            latest: None,
            files: HashMap::new(),
            connection: conn,
        }
    }

    pub fn from_iter<'a, T>(iter: T, conn: Rc<RefCell<Connection>>) -> FileCollection
        where T: IntoIterator<Item = &'a Uuid>
    {
        FileCollection {
            latest: None,
            files: iter.into_iter()
                .map(|&id| (id, Lazy::Guid(id)))
                .collect(),
            connection: conn,
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

    fn iter_mut(&mut self) -> IterMut<File> {
        IterMut::new(self.connection.clone(), self.files.values_mut())
    }
}
