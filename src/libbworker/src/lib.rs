
#![feature(drop_types_in_const)]
#![feature(const_fn)]

extern crate winapi;
extern crate advapi32;
extern crate winreg;

mod sys;

pub use sys::*;
