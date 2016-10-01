
#[macro_use]
extern crate lazy_static;
extern crate crossbeam;

use std::fmt;
use std::error::Error;
use std::io;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use self::windows::spawn;

pub trait Service : Sync + Send {
    fn name(&self) -> &str;
    fn start(&self, _args: &[String]) {}
    fn stop(&self) {}
}

#[derive(Debug)]
pub enum ServiceError {
    CantAcquireMutexLock,
    RegisterServiceHandlerError,
    IOError(io::Error),
}

impl Error for ServiceError {
    fn description(&self) -> &'static str {
        match self {
            &ServiceError::CantAcquireMutexLock => "Can not lock service pool mutex.",
            &ServiceError::RegisterServiceHandlerError => "Can't register service handler.",
            _ => { "" } // TODO: Handle other errors
        }
    }
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.description())
    }
}

