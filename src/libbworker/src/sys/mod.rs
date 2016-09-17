
mod os;

#[cfg(target_os="windows")]
pub use self::os::windows as service;
