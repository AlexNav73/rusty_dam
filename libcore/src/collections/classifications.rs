
use uuid::Uuid;

use std::collections::HashMap;
use std::iter::FromIterator;

use Lazy;
use classification::Classification;
use collections::{EntityCollection, Ids};

pub struct ClassificationCollection {
    classifications: HashMap<Uuid, Lazy<Classification>>,
}

impl ClassificationCollection {
    pub fn new() -> ClassificationCollection {
        ClassificationCollection {
            classifications: HashMap::new()
        }
    }
}

impl EntityCollection<Classification> for ClassificationCollection {
    fn ids(&self) -> Ids<Classification> {
        Ids::new(self.classifications.keys())
    }
}

impl<'a> FromIterator<&'a Uuid> for ClassificationCollection {
    fn from_iter<T>(iter: T) -> Self
        where T: IntoIterator<Item = &'a Uuid>
    {
        ClassificationCollection {
            classifications: iter.into_iter().map(|&id| (id, Lazy::Guid(id))).collect()
        }
    }
}

