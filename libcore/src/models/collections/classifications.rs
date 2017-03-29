
use uuid::Uuid;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use Lazy;
use models::classification::Classification;
use models::collections::{EntityCollection, Ids, IterMut};
use connection::Connection;

pub struct ClassificationCollection {
    classifications: HashMap<Uuid, Lazy<Classification>>,
    connection: Rc<RefCell<Connection>>,
}

impl ClassificationCollection {
    pub fn new(conn: Rc<RefCell<Connection>>) -> ClassificationCollection {
        ClassificationCollection {
            classifications: HashMap::new(),
            connection: conn,
        }
    }

    pub fn from_iter<'a, T>(iter: T, conn: Rc<RefCell<Connection>>) -> ClassificationCollection
        where T: IntoIterator<Item = &'a Uuid>
    {
        ClassificationCollection {
            classifications: iter.into_iter()
                .map(|&id| (id, Lazy::Guid(id)))
                .collect(),
            connection: conn,
        }
    }
}

impl EntityCollection<Classification> for ClassificationCollection {
    fn ids(&self) -> Ids<Classification> {
        Ids::new(self.classifications.keys())
    }

    fn iter_mut(&mut self) -> IterMut<Classification> {
        IterMut::new(self.connection.clone(), self.classifications.values_mut())
    }
}
