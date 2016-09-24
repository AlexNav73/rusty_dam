
#[macro_use]
extern crate lazy_static;
extern crate winapi;
extern crate advapi32;
extern crate crossbeam;

use std::slice;
use std::ffi::{OsStr, OsString};
use std::os::windows::ffi::{ OsStringExt, OsStrExt };

mod service;

pub use self::service::ServiceBuilder;

pub trait Service : Sync + Send {
    fn start(&self, _args: &[String]) {}
    fn stop(&self) {}
}

fn to_wchar<S: AsRef<OsStr>>(s: &S) -> Vec<u16> {
    s.as_ref().encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>()
}

unsafe fn from_wchar(ptr: *const u16) -> Option<String> {
    match ptr.is_null() {
        true => {
            let len = (0..::std::isize::MAX).position(|i| *ptr.offset(i) == 0).unwrap();
            let slice = slice::from_raw_parts(ptr, len);
            Some(OsString::from_wide(slice).to_string_lossy().into_owned())
        }
        false => { None }
    }
}
