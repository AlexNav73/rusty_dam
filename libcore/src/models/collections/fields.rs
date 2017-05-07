
use uuid::Uuid;

use std::collections::HashMap;

use Entity;
use models::collections::{EntityCollection, Ids, Iter};
use models::field::RecordField;
use connection::App;

pub struct FieldCollection<'a> {
    fields: HashMap<Uuid, RecordField<'a>>,
    application: App<'a>,
}

impl<'a> FieldCollection<'a> {
    pub fn new(app: App) -> Self {
        FieldCollection {
            fields: HashMap::new(),
            application: app,
        }
    }

    pub fn add(&mut self, field: RecordField) {
        if !self.fields.contains_key(&field.id()) {
            self.fields.insert(field.id(), field);
        }
    }

    pub fn from_iter<T>(iter: T, app: App) -> Self
        where T: IntoIterator<Item=RecordField<'a>>
    {
        FieldCollection {
            fields: iter.into_iter().map(|f| (f.id(), f)).collect(),
            application: app,
        }
    }
}

impl<'a, 'b> EntityCollection<'a, 'b, RecordField<'a>> for FieldCollection<'a> {
    fn ids(&self) -> Ids<'a, 'b, RecordField<'a>> {
        Ids::new(self.fields.keys())
    }

    fn iter(&self) -> Iter<'a, 'b, RecordField<'a>> {
        Iter::new(self.application.clone(), self.fields.values())
    }
}
