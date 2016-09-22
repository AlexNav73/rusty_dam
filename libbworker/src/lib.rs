
#[macro_use]
extern crate lazy_static;
extern crate winapi;
extern crate advapi32;

use std::slice;
use std::sync::{ Once, ONCE_INIT };
use std::ffi::{OsStr, OsString};
use std::os::windows::ffi::{ OsStringExt, OsStrExt };
use std::env;
use std::ptr;
use std::mem;

mod service;

pub use self::service::run;

pub trait Service : Sync + Send {
    fn name(&self) -> String {
        let os_str_crate = env::current_exe().unwrap();
        let file_name = os_str_crate.file_stem().unwrap();
        file_name.to_os_string().into_string().unwrap()
    }
    fn start(&mut self, _args: &[String]) {}
    fn stop(&mut self) {}
}

fn to_wchar<S: AsRef<OsStr>>(s: &S) -> Vec<u16> {
    s.as_ref().encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>()
}

unsafe fn from_wchar(ptr: *const u16) -> Option<String> {
    use std::isize::MAX;

    match ptr.is_null() {
        true => {
            let len = (0..MAX).position(|i| *ptr.offset(i) == 0).unwrap();
            let slice = slice::from_raw_parts(ptr, len);
            Some(OsString::from_wide(slice).to_string_lossy().into_owned())
        }
        false => { None }
    }
}
