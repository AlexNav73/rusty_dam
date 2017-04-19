
use uuid::Uuid;

use std::collections::HashMap;

use Lazy;
use models::classification::Classification;
use models::collections::{EntityCollection, Ids, IterMut};
use connection::App;

pub struct ClassificationCollection {
    classifications: HashMap<Uuid, Lazy<Classification>>,
    application: App,
}

impl ClassificationCollection {
    pub fn new(app: App) -> ClassificationCollection {
        ClassificationCollection {
            classifications: HashMap::new(),
            application: app,
        }
    }

    pub fn from_iter<'a, T>(iter: T, app: App) -> ClassificationCollection
        where T: IntoIterator<Item = Uuid>
    {
        ClassificationCollection {
            classifications: iter.into_iter()
                .map(|id| (id, Lazy::Guid(id)))
                .collect(),
            application: app,
        }
    }
}

impl EntityCollection<Classification> for ClassificationCollection {
    fn ids(&self) -> Ids<Classification> {
        Ids::new(self.classifications.keys())
    }

    fn iter_mut(&mut self) -> IterMut<Classification> {
        IterMut::new(self.application.clone(), self.classifications.values_mut())
    }
}
