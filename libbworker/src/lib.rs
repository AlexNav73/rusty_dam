
#[macro_use]
extern crate lazy_static;
extern crate crossbeam;

use std::fmt;
use std::error::Error;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use self::windows::ServiceBuilder;

pub trait Service : Sync + Send {
    fn start(&self, _args: &[String]) {}
    fn stop(&self) {}
}

#[derive(Debug)]
pub enum ServiceError {
    MultInst
}

impl Error for ServiceError {
    fn description(&self) -> &'static str {
        match self {
            &ServiceError::MultInst => "Can not start more than one service instance"
        }
    }
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.description())
    }
}

