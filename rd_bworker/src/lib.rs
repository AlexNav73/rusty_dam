
#[macro_use]
extern crate lazy_static;

use std::fmt;
use std::error::Error;
use std::io;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use self::windows::Builder;

///
/// Service trait which all custom services must implement.
/// 
pub trait Service : Sync + Send {
    fn name(&self) -> &str;
    fn start(&self, args: &[String]);

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
            &ServiceError::IOError(_) => "IO error occured. See more details in inner error."
        }
    }
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &ServiceError::CantAcquireMutexLock => write!(f, "{}", self.description()),
            &ServiceError::RegisterServiceHandlerError => write!(f, "{}", self.description()),
            &ServiceError::IOError(ref e) => write!(f, "{}", e)
        }
    }
}

