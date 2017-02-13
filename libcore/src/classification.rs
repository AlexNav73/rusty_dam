
use uuid::Uuid;

use std::slice::Iter;
use std::iter::FromIterator;

use {Entity, Lazy, Document};

pub struct Classification {
    id: Uuid,
}

#[derive(Serialize, Deserialize)]
struct ClassificationDto {}

impl Document<Classification> for ClassificationDto {
    fn doc_type() -> &'static str {
        "classification"
    }

    fn map(self) -> Classification {
        // TODO: Proper impl
        unimplemented!()
    }
}

impl Entity for Classification {
    fn id(&self) -> Uuid {
        self.id
    }
}

pub struct ClassificationCollection {
    classifications: Vec<Lazy<Classification>>,
}

impl ClassificationCollection {
    pub fn new() -> ClassificationCollection {
        ClassificationCollection { classifications: Vec::new() }
    }

    pub fn iter<'a>(&'a self) -> ClassificationIter<'a> {
        ClassificationIter { inner: self.classifications.iter() }
    }
}

pub struct ClassificationIter<'a> {
    inner: Iter<'a, Lazy<Classification>>,
}

impl<'a> Iterator for ClassificationIter<'a> {
    type Item = &'a Lazy<Classification>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a> IntoIterator for &'a ClassificationCollection {
    type Item = &'a Lazy<Classification>;
    type IntoIter = ClassificationIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ClassificationIter { inner: self.classifications.iter() }
    }
}

impl<'a> FromIterator<&'a Uuid> for ClassificationCollection {
    fn from_iter<T>(iter: T) -> Self
        where T: IntoIterator<Item = &'a Uuid>
    {
        ClassificationCollection { classifications: iter.into_iter().map(|id| id.into()).collect() }
    }
}
