
use uuid::Uuid;

use std::collections::HashMap;

use Entity;
use models::classification::RecordClassification;
use models::collections::{EntityCollection, Ids, Iter};
use connection::App;

pub struct ClassificationCollection<'a> {
    classifications: HashMap<Uuid, RecordClassification<'a>>,
    application: App<'a>,
}

impl<'a> ClassificationCollection<'a> {
    pub fn new(app: App) -> Self {
        ClassificationCollection {
            classifications: HashMap::new(),
            application: app,
        }
    }

    pub fn add(&mut self, cls: RecordClassification) {
        self.classifications.insert(cls.id(), cls);
    }

    pub fn from_iter<T>(iter: T, app: App) -> ClassificationCollection
        where T: IntoIterator<Item=RecordClassification<'a>>
    {
        ClassificationCollection {
            classifications: iter.into_iter().map(|c| (c.id(), c)).collect(),
            application: app,
        }
    }
}

impl<'a, 'b> EntityCollection<'a, 'b, RecordClassification<'a>> for ClassificationCollection<'a> {
    fn ids(&self) -> Ids<'a, 'b, RecordClassification<'a>> {
        Ids::new(self.classifications.keys())
    }

    fn iter(&self) -> Iter<'a, 'b, RecordClassification<'a>> {
        Iter::new(self.application.clone(), self.classifications.values())
    }
}
