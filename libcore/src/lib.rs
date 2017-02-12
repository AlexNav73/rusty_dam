#![allow(dead_code)]
#![allow(mutable_transmutes)]

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate lazy_static;
extern crate uuid;
extern crate rs_es;
extern crate chrono;

mod record;
mod file;
mod field;
mod classification;
mod es;
mod connection;

use uuid::Uuid;

use file::File;
use connection::Connection;

pub enum Lazy<T>
    where T: Entity
{
    Guid(Uuid),
    Value(T),
}

pub trait Entity {
    ///
    /// Unique identifier of entity
    ///
    fn id(&self) -> Uuid;
}

impl<T: Entity> From<Uuid> for Lazy<T> {
    #[inline]
    fn from(id: Uuid) -> Lazy<T> {
        Lazy::Guid(id)
    }
}

impl<'a, T: Entity> From<&'a Uuid> for Lazy<T> {
    #[inline]
    fn from(id: &'a Uuid) -> Lazy<T> {
        Lazy::Guid(id.clone())
    }
}

impl<T: Entity> From<T> for Lazy<T> {
    #[inline]
    fn from(value: T) -> Lazy<T> {
        Lazy::Value(value)
    }
}

impl<T: Entity> From<Lazy<T>> for Uuid {
    fn from(value: Lazy<T>) -> Uuid {
        match value {
            Lazy::Guid(id) => id,
            Lazy::Value(o) => o.id(),
        }
    }
}

impl<'a, T: Entity> From<&'a Lazy<T>> for Uuid {
    fn from(value: &'a Lazy<T>) -> Uuid {
        match value {
            &Lazy::Guid(id) => id,
            &Lazy::Value(ref o) => o.id(),
        }
    }
}

impl<'a, T: Entity> Entity for &'a T {
    fn id(&self) -> Uuid {
        (**self).id()
    }
}

// TODO: Proper impl
impl<'a> From<&'a Lazy<File>> for &'a File {
    fn from(value: &'a Lazy<File>) -> &'a File {
        match value {
            &Lazy::Guid(id) => load_object::<File>(id, value),
            &Lazy::Value(ref o) => o,
        }
    }
}

fn load_object<T: Entity>(id: Uuid, value: &Lazy<T>) -> &T {
    let connection = Connection::get();
    let mut guard = connection.lock().unwrap();
    let mut new_value: Lazy<T> = guard.load(id);
    unsafe {
        // FIXME: Remove mem::transmute to safer option
        ::std::mem::swap(::std::mem::transmute(value), &mut new_value);
    }
    match value {
        &Lazy::Value(ref o) => o,
        _ => unreachable!(),
    }
}

