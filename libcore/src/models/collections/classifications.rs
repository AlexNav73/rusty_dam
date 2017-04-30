
use uuid::Uuid;

use std::collections::HashMap;

use Entity;
use models::classification::RecordClassification;
use models::collections::{EntityCollection, Ids, Iter};
use connection::App;

pub struct ClassificationCollection {
    classifications: HashMap<Uuid, RecordClassification>,
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
        where T: IntoIterator<Item = RecordClassification>
    {
        ClassificationCollection {
            classifications: iter.into_iter().map(|c| (c.id(), c)).collect(),
            application: app,
        }
    }
}

impl EntityCollection<RecordClassification> for ClassificationCollection {
    fn ids(&self) -> Ids<RecordClassification> {
        Ids::new(self.classifications.keys())
    }

    fn iter(&self) -> Iter<RecordClassification> {
        Iter::new(self.application.clone(), self.classifications.values())
    }
}
