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

pub trait Entity where Self: Sized {
    type Dto: Document<Self>;
    ///
    /// Unique identifier of entity
    ///
    fn id(&self) -> Uuid;

    fn map(&self) -> Self::Dto;
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

    fn map(self) -> T;
}

pub enum Lazy1<T: Entity> {
    Guid(Uuid),
    Object(Box<T>)
}

impl<T: Entity> Lazy1<T> {
    pub fn unwrap(self, conn: &mut Connection) -> Result<Box<T>, LoadError> {
        match self {
            Lazy1::Guid(id) => Ok(Box::new(conn.by_id::<T>(id).map_err(|_| LoadError::NotFound)?)),
            Lazy1::Object(o) => Ok(o)
        }
    }
}

pub enum LoadError {
    NotFound
}

impl fmt::Debug for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &LoadError::NotFound => write!(f, "File not found"),
        }
    }
}

