#![allow(dead_code)]

#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate serde;

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;

extern crate uuid;
extern crate rs_es;
extern crate chrono;

mod configuration;
mod connection;
mod es;
mod pg;
mod models;
mod schema;

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
    type Dto: Serialize + Deserialize;

    fn to_dto(&self) -> Self::Dto;
}

pub trait FromDto {
    type Dto: Serialize + Deserialize;

    fn from_dto(dto: Self::Dto, conn: Rc<RefCell<Connection>>) -> Self;
}

pub trait Load: Sized + FromDto {
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
