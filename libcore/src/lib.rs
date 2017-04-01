#![allow(dead_code)]

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

pub trait Entity {
    ///
    /// Unique identifier of entity
    ///
    fn id(&self) -> Uuid;

    ///
    /// Creates instance of Self initialized with connection
    ///
    fn create(app: &App) -> Self;
}

pub trait ToDto {
    type Dto: FromDto;

    fn to_dto(&self) -> Self::Dto;
}

pub trait FromDto: Serialize + Deserialize {
    type Item;

    fn from_dto(self, conn: Rc<RefCell<Connection>>) -> Self::Item;
}

pub trait Load: Sized + ToDto {
    fn load(c: Rc<RefCell<Connection>>, id: Uuid) -> Result<Self, LoadError>;
}

pub enum Lazy<T: Load> {
    Guid(Uuid),
    Object(Box<T>),
}

impl<T: Load> Lazy<T> {
    pub fn unwrap(&mut self, conn: Rc<RefCell<Connection>>) -> Result<&T, LoadError> {
        match self {
            &mut Lazy::Guid(id) => {
                *self = Lazy::Object(Box::new(T::load(conn, id).map_err(|_| LoadError::NotFound)?));

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

