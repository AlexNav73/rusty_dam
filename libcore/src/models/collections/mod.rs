
use uuid::Uuid;

use std::collections::hash_map::{Keys, Values};
use std::marker::PhantomData;

use {ToDto, FromDto};
use connection::App;

pub mod fields;
pub mod files;
pub mod classifications;

pub trait EntityCollection<'a, T>
    where T: ToDto + FromDto<'a>
{
    fn ids<'b>(&'b self) -> Ids<'b, 'a, T>;
    fn iter<'b>(&'b self) -> Iter<'b, 'a, T>;
}

pub struct Ids<'a, 'b: 'a, T>
    where T: ToDto + FromDto<'b> + 'b
{
    inner: Keys<'a, Uuid, T>,
    _marker: PhantomData<&'b ()>
}

impl<'a, 'b: 'a, T: ToDto + FromDto<'b> + 'b> Ids<'a, 'b, T> {
    pub fn new(ids: Keys<Uuid, T>) -> Self {
        Ids { inner: ids, _marker: PhantomData }
    }
}

impl<'a, 'b, T: ToDto + FromDto<'b> + 'b> Iterator for Ids<'a, 'b, T> {
    type Item = Uuid;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|&id| id)
    }
}

pub struct Iter<'a, 'b: 'a, T>
    where T: ToDto + FromDto<'b> + 'b
{
    inner: Values<'a, Uuid, T>,
    application: App,
    _marker: PhantomData<&'b ()>
}

impl<'a, 'b, T: ToDto + FromDto<'b> + 'b> Iter<'a, 'b, T> {
    pub fn new(app: App, iter: Values<Uuid, T>) -> Self {
        Iter {
            inner: iter,
            application: app,
            _marker: PhantomData
        }
    }
}

impl<'a, 'b, T: ToDto + FromDto<'b> + 'b> Iterator for Iter<'a, 'b, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
