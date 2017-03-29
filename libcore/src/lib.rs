#![allow(dead_code)]
#![allow(mutable_transmutes)]

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

extern crate uuid;
extern crate rs_es;
extern crate chrono;

mod configuration;
mod connection;
mod es;
mod models;

use serde::{Serialize, Deserialize};

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

pub use uuid::Uuid;
pub use connection::{App, Connection};
pub use models::record::Record;
pub use configuration::Configuration;

pub trait Entity: Sized {
    type Dto: Document<Self>;
    ///
    /// Unique identifier of entity
    ///
    fn id(&self) -> Uuid;

    ///
    /// Maps to DTO for working with database
    ///
    fn map(&self) -> Self::Dto;

    ///
    /// Creates instance of Self initialized with connection
    ///
    fn create(app: &App) -> Self;
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
    fn map(self, conn: Rc<RefCell<Connection>>) -> T;
}

pub enum Lazy<T: Entity> {
    Guid(Uuid),
    Object(Box<T>),
}

impl<T: Entity> Lazy<T> {
    pub fn unwrap(&mut self, conn: Rc<RefCell<Connection>>) -> Result<&T, LoadError> {
        match self {
            &mut Lazy::Guid(id) => {
                *self = Lazy::Object(Box::new(Connection::by_id::<T>(conn, id)
                    .map_err(|_| LoadError::NotFound)?));

                if let &mut Lazy::Object(ref o) = self {
                    Ok(o)
                } else {
                    unreachable!()
                }
            }
            &mut Lazy::Object(ref o) => Ok(o),
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
