
use uuid::Uuid;

use std::collections::HashMap;

use Entity;
use models::collections::{EntityCollection, Ids, IterMut};
use models::field::RecordField;
use connection::App;

pub struct FieldCollection {
    fields: HashMap<Uuid, RecordField>,
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
        where T: IntoIterator<Item = RecordField>
    {
        FieldCollection {
            fields: iter.into_iter().map(|f| (f.id(), f)).collect(),
            application: app,
        }
    }
}

impl EntityCollection<RecordField> for FieldCollection {
    fn ids(&self) -> Ids<RecordField> {
        Ids::new(self.fields.keys())
    }

    fn iter_mut(&mut self) -> IterMut<RecordField> {
        IterMut::new(self.application.clone(), self.fields.values_mut())
    }
}
