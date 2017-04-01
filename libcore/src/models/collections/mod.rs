
use uuid::Uuid;

use std::collections::hash_map::{Keys, ValuesMut};
use std::rc::Rc;
use std::cell::RefCell;

use {Lazy, LoadError, Load};
use connection::Connection;

pub mod fields;
pub mod files;
pub mod classifications;

pub trait EntityCollection<T>
    where T: Load
{
    fn ids(&self) -> Ids<T>;
    fn iter_mut(&mut self) -> IterMut<T>;
}

pub struct Ids<'a, T>
    where T: Load + 'a
{
    inner: Keys<'a, Uuid, Lazy<T>>,
}

impl<'a, T: Load + 'a> Ids<'a, T> {
    pub fn new(ids: Keys<Uuid, Lazy<T>>) -> Ids<T> {
        Ids { inner: ids }
    }
}

impl<'a, T: Load + 'a> Iterator for Ids<'a, T> {
    type Item = Uuid;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|&id| id)
    }
}

pub struct IterMut<'a, T>
    where T: Load + 'a
{
    inner: ValuesMut<'a, Uuid, Lazy<T>>,
    connection: Rc<RefCell<Connection>>,
}

impl<'a, T: Load + 'a> IterMut<'a, T> {
    pub fn new(conn: Rc<RefCell<Connection>>, iter: ValuesMut<Uuid, Lazy<T>>) -> IterMut<T> {
        IterMut {
            inner: iter,
            connection: conn,
        }
    }
}

impl<'a, T: Load + 'a> Iterator for IterMut<'a, T> {
    type Item = Result<&'a T, LoadError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|x| x.unwrap(self.connection.clone()))
    }
}
