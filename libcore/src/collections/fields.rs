
use uuid::Uuid;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use Lazy;
use collections::{EntityCollection, Ids, IterMut};
use connection::Connection;
use field::Field;

pub struct FieldCollection {
    fields: HashMap<Uuid, Lazy<Field>>,
    connection: Rc<RefCell<Connection>>,
}

impl FieldCollection {
    pub fn new(conn: Rc<RefCell<Connection>>) -> FieldCollection {
        FieldCollection {
            fields: HashMap::new(),
            connection: conn,
        }
    }

    pub fn from_iter<'a, T>(iter: T, conn: Rc<RefCell<Connection>>) -> Self
        where T: IntoIterator<Item = &'a Uuid>
    {
        FieldCollection {
            fields: iter.into_iter().map(|&id| (id, Lazy::Guid(id))).collect(),
            connection: conn,
        }
    }
}

impl EntityCollection<Field> for FieldCollection {
    fn ids(&self) -> Ids<Field> {
        Ids::new(self.fields.keys())
    }

    fn iter_mut(&mut self) -> IterMut<Field> {
        IterMut::new(self.connection.clone(), self.fields.values_mut())
    }
}
