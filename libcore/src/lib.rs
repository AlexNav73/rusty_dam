#![allow(dead_code)]
#![allow(mutable_transmutes)]

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
mod connection;

use serde::{Serialize, Deserialize};

use file::File;

use std::fmt;

pub use uuid::Uuid;
pub use connection::{Connection, ConnectionError};
pub use record::Record;

pub trait Entity
    where Self: Sized
{
    type Dto: Document<Self>;
    ///
    /// Unique identifier of entity
    ///
    fn id(&self) -> Uuid;

    ///
    /// Maps to DTO for working with database
    ///
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

    ///
    /// Maps DTO to parent type
    ///
    fn map(self) -> T;
}

pub enum Lazy<T: Entity> {
    Guid(Uuid),
    Object(Box<T>),
}

impl<T: Entity> Lazy<T> {
    pub fn unwrap(self, conn: &mut Connection) -> Result<Box<T>, LoadError> {
        match self {
            Lazy::Guid(id) => Ok(Box::new(conn.by_id::<T>(id).map_err(|_| LoadError::NotFound)?)),
            Lazy::Object(o) => Ok(o),
        }
    }
}

pub enum LoadError {
    NotFound,
}

impl fmt::Debug for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &LoadError::NotFound => write!(f, "File not found"),
        }
    }
}
