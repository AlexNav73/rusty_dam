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

use serde::{Serialize, Deserialize};
use uuid::Uuid;

use file::File;
use connection::Connection;

use std::fmt;

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

///
/// All documents which needs to be stored in elasticsearch must
/// implement this trait.
///
pub trait Document<T: Entity>: Serialize + Deserialize {
    ///
    /// Document type used by elasticsearch to distinguish documents
    ///
    fn doc_type() -> &'static str;

    ///
    /// Map DTO to actual object
    ///
    fn map(self) -> T;
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

pub struct LazyRef<'a, T: Entity + 'a>(&'a Lazy<T>);

impl<'a, T: Entity> From<&'a Lazy<T>> for LazyRef<'a, T> {
    fn from(value: &'a Lazy<T>) -> LazyRef<'a, T> {
        LazyRef(value)
    }
}

impl<'a, T: Entity + Document<T>> LazyRef<'a, T> {
    pub fn into_inner(&'a self) -> Result<&'a T, LoadError> {
        match self.0 {
            &Lazy::Guid(id) => load_object::<T>(id, self.0),
            &Lazy::Value(ref o) => Ok(o),
        }
    }
}

//// TODO: Proper impl
//impl<'a> From<&'a Lazy<File>> for &'a File {
    //fn from(value: &'a Lazy<File>) -> &'a File {
        //match value {
            //&Lazy::Guid(id) => load_object::<File>(id, value),
            //&Lazy::Value(ref o) => o,
        //}
    //}
//}

fn load_object<T: Entity + Document<T>>(id: Uuid, value: &Lazy<T>) -> Result<&T, LoadError> {
    let connection = Connection::get();
    let mut guard = connection.lock().unwrap();
    let mut new_value: Lazy<T> = guard.load::<T, T>(id).map_err(|_| LoadError::NotFound)?;
    unsafe {
        // FIXME: Remove mem::transmute to safer option
        ::std::mem::swap(::std::mem::transmute(value), &mut new_value);
    }
    match value {
        &Lazy::Value(ref o) => Ok(o),
        _ => unreachable!(),
    }
}

enum LoadError {
    NotFound
}

impl fmt::Debug for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &LoadError::NotFound => write!(f, "File not found"),
        }
    }
}

