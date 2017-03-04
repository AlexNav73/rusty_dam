
use uuid::Uuid;

use std::collections::HashMap;
use std::iter::FromIterator;

use Lazy;
use collections::{EntityCollection, Ids};
use field::Field;

pub struct FieldCollection {
    fields: HashMap<Uuid, Lazy<Field>>,
}

impl FieldCollection {
    pub fn new() -> FieldCollection {
        FieldCollection {
            fields: HashMap::new(),
        }
    }
}

impl EntityCollection<Field> for FieldCollection {
    fn ids(&self) -> Ids<Field> {
        Ids::new(self.fields.keys())
    }
}

impl<'a> FromIterator<&'a Uuid> for FieldCollection {
    fn from_iter<T>(iter: T) -> Self
        where T: IntoIterator<Item = &'a Uuid>
    {
        FieldCollection { 
            fields: iter.into_iter().map(|&id| (id, Lazy::Guid(id))).collect()
        }
    }
}

