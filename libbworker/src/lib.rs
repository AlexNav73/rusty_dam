
#[macro_use]
extern crate lazy_static;
extern crate crossbeam;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use self::windows::ServiceBuilder;

pub trait Service : Sync + Send {
    fn start(&self, _args: &[String]) {}
    fn stop(&self) {}
}

