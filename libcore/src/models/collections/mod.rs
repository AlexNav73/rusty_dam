
use uuid::Uuid;

use std::collections::hash_map::{Keys, Values};
use std::marker::PhantomData;

use {ToDto, FromDto};
use connection::App;

pub mod fields;
pub mod files;
pub mod classifications;

pub trait EntityCollection<'a, 'b, T>
    where T: ToDto<'a> + FromDto<'b>
{
    fn ids(&self) -> Ids<'a, 'b, T>;
    fn iter(&self) -> Iter<'a, 'b, T>;
}

pub struct Ids<'a, 'b, T>
    where T: ToDto<'a> + FromDto<'b> + 'a
{
    inner: Keys<'a, Uuid, T>,
    _marker: PhantomData<&'b ()>
}

impl<'a, 'b, T: ToDto<'a> + FromDto<'b> + 'a> Ids<'a, 'b, T> {
    pub fn new(ids: Keys<Uuid, T>) -> Self {
        Ids { inner: ids, _marker: PhantomData }
    }
}

impl<'a, 'b, T: ToDto<'a> + FromDto<'b> + 'a> Iterator for Ids<'a, 'b, T> {
    type Item = Uuid;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|&id| id)
    }
}

pub struct Iter<'a, 'b, T>
    where T: ToDto<'a> + FromDto<'b> + 'a
{
    inner: Values<'a, Uuid, T>,
    application: App<'a>,
    _marker: PhantomData<&'b ()>
}

impl<'a, 'b, T: ToDto<'a> + FromDto<'b> + 'a> Iter<'a, 'b, T> {
    pub fn new(app: App, iter: Values<Uuid, T>) -> Iter<'a, 'b, T> {
        Iter {
            inner: iter,
            application: app,
            _marker: PhantomData
        }
    }
}

impl<'a, 'b, T: ToDto<'a> + FromDto<'b> + 'a> Iterator for Iter<'a, 'b, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
