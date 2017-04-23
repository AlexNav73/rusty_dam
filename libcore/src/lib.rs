#![allow(dead_code)]

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;

extern crate uuid;
extern crate rs_es;
extern crate chrono;

mod connection;
mod configuration;
mod es;
#[macro_use]
mod pg;
mod models;

use serde::{Serialize, Deserialize};

use std::fmt;

pub use uuid::Uuid;
pub use connection::App;
pub use models::record::Record;
pub use models::classification::Classification;
pub use configuration::Configuration;

pub trait Entity {
    ///
    /// Unique identifier of entity
    ///
    fn id(&self) -> Uuid;
}

pub trait ToDto {
    type Dto: Serialize + Deserialize;

    fn to_dto(&self) -> Self::Dto;
}

pub trait FromDto {
    type Dto: Serialize + Deserialize;

    fn from_dto(dto: Self::Dto, app: App) -> Self;
}

pub trait Load: Sized {
    fn load(c: App, id: Uuid) -> Result<Self, LoadError>;
}

pub trait Create: FromDto + ToDto {
    fn create(app: App) -> Self;
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
