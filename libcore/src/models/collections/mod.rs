
use uuid::Uuid;

use std::collections::hash_map::{Keys, ValuesMut};
use std::rc::Rc;
use std::cell::RefCell;

use {Lazy, Entity, LoadError};
use connection::Connection;

pub mod fields;
pub mod files;
pub mod classifications;

pub trait EntityCollection<T>
    where T: Entity
{
    fn ids(&self) -> Ids<T>;
    fn iter_mut(&mut self) -> IterMut<T>;
}

pub struct Ids<'a, T>
    where T: Entity + 'a
{
    inner: Keys<'a, Uuid, Lazy<T>>,
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

pub struct IterMut<'a, T>
    where T: Entity + 'a
{
    inner: ValuesMut<'a, Uuid, Lazy<T>>,
    connection: Rc<RefCell<Connection>>,
}

impl<'a, T: Entity + 'a> IterMut<'a, T> {
    pub fn new(conn: Rc<RefCell<Connection>>, iter: ValuesMut<Uuid, Lazy<T>>) -> IterMut<T> {
        IterMut {
            inner: iter,
            connection: conn,
        }
    }
}

impl<'a, T: Entity + 'a> Iterator for IterMut<'a, T> {
    type Item = Result<&'a T, LoadError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|x| x.unwrap(self.connection.clone()))
    }
}
