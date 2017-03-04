
use uuid::Uuid;

use std::collections::hash_map::Keys;

use {Lazy, Entity};

pub mod fields;
pub mod files;
pub mod classifications;

pub trait EntityCollection<T> where T: Entity {
    fn ids(&self) -> Ids<T>;
}

pub struct Ids<'a, T> where T: Entity + 'a {
    inner: Keys<'a, Uuid, Lazy<T>>
}

impl<'a, T: Entity + 'a> Ids<'a, T> {
    pub fn new(ids: Keys<Uuid, Lazy<T>>) -> Ids<T> {
        Ids { inner: ids }
    }
}

impl<'a, T: Entity + 'a> Iterator for Ids<'a, T> {
    type Item = Uuid;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|&id| id)
    }
}

