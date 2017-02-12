
use {Entity, Lazy};

use uuid::Uuid;

use std::slice::Iter;
use std::iter::FromIterator;

pub struct Field {
    id: Uuid,
}

impl Entity for Field {
    fn id(&self) -> Uuid {
        self.id
    }
}

pub struct FieldCollection {
    fields: Vec<Lazy<Field>>,
}

impl FieldCollection {
    pub fn new() -> FieldCollection {
        FieldCollection { fields: Vec::new() }
    }

    pub fn iter<'a>(&'a self) -> FieldIter<'a> {
        FieldIter { inner: self.fields.iter() }
    }
}

pub struct FieldIter<'a> {
    inner: Iter<'a, Lazy<Field>>,
}

impl<'a> Iterator for FieldIter<'a> {
    type Item = &'a Lazy<Field>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a> IntoIterator for &'a FieldCollection {
    type Item = &'a Lazy<Field>;
    type IntoIter = FieldIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        FieldIter { inner: self.fields.iter() }
    }
}

impl<'a> FromIterator<&'a Uuid> for FieldCollection {
    fn from_iter<T>(iter: T) -> Self
        where T: IntoIterator<Item = &'a Uuid>
    {
        FieldCollection { fields: iter.into_iter().map(|id| id.into()).collect() }
    }
}
