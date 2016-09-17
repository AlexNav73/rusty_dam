

mod service;
mod log;
mod installer;

pub mod reg;

pub use self::service::{ Service, launch };

mod helpers {

    use std::slice;
    use std::sync::{ Once, ONCE_INIT };
    use std::ffi::{OsStr, OsString};
    use std::os::windows::ffi::{ OsStringExt, OsStrExt };
    use std::env;
    use std::ptr;

    #[allow(dead_code)]
    pub fn to_wchar<S: AsRef<OsStr>>(s: &S) -> *const u16 {
        s.as_ref().encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>().as_ptr()
    }

    pub unsafe fn from_wchar(ptr: *const u16) -> Option<String> {
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

    static INIT: Once = ONCE_INIT;
    static mut CRATE_NAME_UTF16: *const u16 = ptr::null();
    static mut CRATE_NAME_UTF8: Option<String> = None;

    pub unsafe fn get_crate_name_utf16() -> *const u16 {
        INIT.call_once(__init);
        CRATE_NAME_UTF16
    }

    pub fn get_crate_name_utf8<'a>() -> &'a str {
        INIT.call_once(__init);
        unsafe {
            if let Some(ref crate_name) = CRATE_NAME_UTF8 {
                return crate_name;
            }
        }
        panic!("");
    }

    fn __init() {
        unsafe {
            let os_str_crate = env::current_exe().unwrap();
            let file_name = os_str_crate.file_stem().unwrap().to_os_string();
            let crate_name = file_name.into_string().unwrap();
            CRATE_NAME_UTF16 = to_wchar(&crate_name.chars().chain(Some('\0').into_iter()).collect::<String>());
            CRATE_NAME_UTF8 = Some(crate_name);
        }
    }
}
