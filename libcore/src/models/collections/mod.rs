
use uuid::Uuid;

use std::collections::hash_map::{Keys, ValuesMut};

use {ToDto, FromDto};
use connection::App;

pub mod fields;
pub mod files;
pub mod classifications;

pub trait EntityCollection<T>
    where T: ToDto + FromDto
{
    fn ids(&self) -> Ids<T>;
    fn iter_mut(&mut self) -> IterMut<T>;
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

pub struct IterMut<'a, T>
    where T: ToDto + FromDto + 'a
{
    inner: ValuesMut<'a, Uuid, T>,
    application: App,
}

impl<'a, T: ToDto + FromDto + 'a> IterMut<'a, T> {
    pub fn new(app: App, iter: ValuesMut<Uuid, T>) -> IterMut<T> {
        IterMut {
            inner: iter,
            application: app,
        }
    }
}

impl<'a, T: ToDto + FromDto + 'a> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
