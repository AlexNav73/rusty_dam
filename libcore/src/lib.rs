#![allow(dead_code)]

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

extern crate uuid;
extern crate rs_es;
extern crate chrono;

mod record;
mod file;
mod field;
mod classification;
mod es;

use uuid::Uuid;

pub enum Lazy<T> where T: Entity {
    Guid(Uuid),
    Value(T)
}

impl<T: Entity> From<Uuid> for Lazy<T> {
    #[inline]
    fn from(id: Uuid) -> Lazy<T> { Lazy::Guid(id) }
}

impl<T: Entity> From<T> for Lazy<T> {
    #[inline]
    fn from(value: T) -> Lazy<T> { Lazy::Value(value) }
}

impl<T: Entity> From<Lazy<T>> for Uuid {
    fn from(value: Lazy<T>) -> Uuid {
        match value {
            Lazy::Guid(id) => id,
            Lazy::Value(o) => o.id()
        }
    }
}

impl<'a, T: Entity> From<&'a Lazy<T>> for Uuid {
    fn from(value: &'a Lazy<T>) -> Uuid {
        match value {
            &Lazy::Guid(id) => id,
            &Lazy::Value(ref o) => o.id()
        }
    }
}

// TODO: Avoid pub access
pub trait Entity {
    ///
    /// Unique identifier of entity
    ///
    fn id(&self) -> Uuid;
}

