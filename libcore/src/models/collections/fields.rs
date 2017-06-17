
use uuid::Uuid;

use std::collections::HashMap;
use std::ops::{ Index, IndexMut };

use Entity;
use models::collections::{EntityCollection, Ids, Iter};
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

    pub fn add(&mut self, field: RecordField) {
        if !self.fields.contains_key(&field.id()) {
            self.fields.insert(field.id(), field);
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

    fn iter(&self) -> Iter<RecordField> {
        Iter::new(self.application.clone(), self.fields.values())
    }
}

impl<'idx> Index<&'idx Uuid> for FieldCollection {
    type Output = RecordField;

    fn index<'a>(&'a self, index: &'idx Uuid) -> &'a Self::Output {
        // TODO: Proper error handeling
        self.fields.get(index).expect("Field with this ID is not present in collection")
    }
}

impl<'idx> IndexMut<&'idx Uuid> for FieldCollection {
    fn index_mut<'a>(&'a mut self, index: &'idx Uuid) -> &'a mut Self::Output {
        // TODO: Proper error handeling
        self.fields.get_mut(index).expect("Field with this ID is not present in collection")
    }
}
