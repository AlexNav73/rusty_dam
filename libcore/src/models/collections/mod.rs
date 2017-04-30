
use uuid::Uuid;

use std::collections::hash_map::{Keys, Values};

use {ToDto, FromDto};
use connection::App;

pub mod fields;
pub mod files;
pub mod classifications;

pub trait EntityCollection<T>
    where T: ToDto + FromDto
{
    fn ids(&self) -> Ids<T>;
    fn iter(&self) -> Iter<T>;
}

pub struct Ids<'a, T>
    where T: ToDto + FromDto + 'a
{
    inner: Keys<'a, Uuid, T>,
}

impl<'a, T: ToDto + FromDto + 'a> Ids<'a, T> {
    pub fn new(ids: Keys<Uuid, T>) -> Ids<T> {
        Ids { inner: ids }
    }
}

impl<'a, T: ToDto + FromDto + 'a> Iterator for Ids<'a, T> {
    type Item = Uuid;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|&id| id)
    }
}

pub struct Iter<'a, T>
    where T: ToDto + FromDto + 'a
{
    inner: Values<'a, Uuid, T>,
    application: App,
}

impl<'a, T: ToDto + FromDto + 'a> Iter<'a, T> {
    pub fn new(app: App, iter: Values<Uuid, T>) -> Iter<T> {
        Iter {
            inner: iter,
            application: app,
        }
    }
}

impl<'a, T: ToDto + FromDto + 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
