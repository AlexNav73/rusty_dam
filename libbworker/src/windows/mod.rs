
extern crate winapi;
extern crate advapi32;

use std::slice;
use std::ffi::{OsStr, OsString};
use std::os::windows::ffi::{ OsStringExt, OsStrExt };

mod service;

pub use self::service::ServiceBuilder;

fn to_wchar<S: AsRef<OsStr>>(s: &S) -> Vec<u16> {
    s.as_ref().encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>()
}

unsafe fn from_wchar(ptr: *const u16) -> Option<String> {
    match ptr.is_null() {
        false => {
            let len = (0..::std::isize::MAX).position(|i| *ptr.offset(i) == 0).unwrap();
            let slice = slice::from_raw_parts(ptr, len);
            Some(OsString::from_wide(slice).to_string_lossy().into_owned())
        }
        true => { None }
    }
}

