
use uuid::Uuid;

use std::collections::HashMap;

use Lazy;
use models::collections::{EntityCollection, Ids, IterMut};
use models::field::Field;
use connection::App;

pub struct FieldCollection {
    fields: HashMap<Uuid, Lazy<Field>>,
    application: App,
}

impl FieldCollection {
    pub fn new(app: App) -> FieldCollection {
        FieldCollection {
            fields: HashMap::new(),
            application: app,
        }
    }

    pub fn from_iter<'a, T>(iter: T, app: App) -> Self
        where T: IntoIterator<Item = Uuid>
    {
        FieldCollection {
            fields: iter.into_iter()
                .map(|id| (id, Lazy::Guid(id)))
                .collect(),
            application: app,
        }
    }
}

impl EntityCollection<Field> for FieldCollection {
    fn ids(&self) -> Ids<Field> {
        Ids::new(self.fields.keys())
    }

    fn iter_mut(&mut self) -> IterMut<Field> {
        IterMut::new(self.application.clone(), self.fields.values_mut())
    }
}
