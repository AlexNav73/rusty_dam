
use std::ptr;
use std::io;
use std::env;

use ::winapi::minwindef::DWORD;
use ::advapi32::{OpenSCManagerW, CreateServiceW, CloseServiceHandle, DeleteService, OpenServiceW};
use ::winapi::winsvc::{SC_MANAGER_CREATE_SERVICE, SC_MANAGER_ALL_ACCESS, SERVICE_ALL_ACCESS, SERVICE_STOP};
use ::winapi::winnt::{SERVICE_WIN32_OWN_PROCESS, DELETE};

use super::helpers::{ to_wchar, get_crate_name_utf16 };
use super::service::launch;
use super::Service;

const SERVICE_DEMAND_START: DWORD = 0x00000003;
const SERVICE_ERROR_NORMAL: DWORD = 0x00000001;

pub struct ServiceInstaller;

impl ServiceInstaller {
    pub fn run<T: Service + 'static>(service: T) -> io::Result<Guard> {
        if cfg!(feature = "install") {
            try!(install_service());
        } else {
            launch(Box::new(service));
        }
        Ok(Guard)
    }
}

pub struct Guard;

impl Drop for Guard {
    fn drop(&mut self) {
        clean_up().unwrap()
    }
}

#[cfg(feature = "install")]
fn install_service() -> io::Result<()> {
    let manager = unsafe { OpenSCManagerW(ptr::null(), ptr::null(), SC_MANAGER_CREATE_SERVICE) };
    if manager.is_null() {
        return Err(io::Error::last_os_error());
    }

    let handle = unsafe {
        CreateServiceW(manager,
                get_crate_name_utf16(),
                get_crate_name_utf16(),
                SERVICE_ALL_ACCESS,
                SERVICE_WIN32_OWN_PROCESS,
                SERVICE_DEMAND_START,
                SERVICE_ERROR_NORMAL,
                //to_wchar(&"\" D:\\Programms\\rusty_dam\\target\\debug\\rusty_dam.exe\"".to_owned()),
                to_wchar(&(env::current_exe().unwrap().as_os_str())).as_ptr(),
                ptr::null(),
                ptr::null_mut(),
                ptr::null(),
                ptr::null(),
                ptr::null())
    };

    if handle.is_null() {
        unsafe { CloseServiceHandle(manager); }
        return Err(io::Error::last_os_error());
    }

    unsafe { CloseServiceHandle(handle); }
}

#[cfg(not(feature = "install"))]
fn install_service() -> io::Result<()> { Ok(()) }

// #[cfg(not(install))]
// fn clean_up() -> io::Result<()> {
//     unsafe {
//         let manager = OpenSCManagerW(ptr::null(), ptr::null(), SC_MANAGER_ALL_ACCESS);
//         if manager.is_null() {
//             return Err(io::Error::last_os_error());
//         }

//         let service = OpenServiceW(manager, get_crate_name_utf16(), SERVICE_STOP | DELETE);
//         if service.is_null() {
//             CloseServiceHandle(manager);
//             return Err(io::Error::last_os_error());
//         }

//         DeleteService(service);
//         CloseServiceHandle(service);
//         CloseServiceHandle(manager);
//         Ok(())
//     }
// }

// #[cfg(install)]
fn clean_up() -> io::Result<()> {
    Ok(())
}

