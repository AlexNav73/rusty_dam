
use ::winapi::minwindef::DWORD;
use ::advapi32::{OpenSCManagerW, CreateServiceW, CloseServiceHandle, DeleteService, OpenServiceW};
use ::winapi::winsvc::{SC_MANAGER_CREATE_SERVICE, SC_MANAGER_ALL_ACCESS, SERVICE_ALL_ACCESS, SERVICE_STOP};
use ::winapi::winnt::{SERVICE_WIN32_OWN_PROCESS, DELETE};

use super::helpers::to_wchar;

use std::ptr;
use std::io::Error;

const SERVICE_DEMAND_START: DWORD = 0x00000003;
const SERVICE_ERROR_NORMAL: DWORD = 0x00000001;

#[allow(non_snake_case)]
pub unsafe fn install_service() {
    let manager = OpenSCManagerW(ptr::null(), ptr::null(), SC_MANAGER_CREATE_SERVICE);
    if manager.is_null() {
        println!("OpenSCManagerW failed: {}", Error::last_os_error());
        return;
    }

    let hService = CreateServiceW(manager,
                                  to_wchar("Name"),
                                  to_wchar("Name"),
                                  SERVICE_ALL_ACCESS,
                                  SERVICE_WIN32_OWN_PROCESS,
                                  SERVICE_DEMAND_START,
                                  SERVICE_ERROR_NORMAL,
                                  to_wchar("D:\\Programms\\Rust\\rusty_dam\\target\\debug\\rusty_dam.exe"),
                                  ptr::null(),
                                  ptr::null_mut(),
                                  ptr::null(),
                                  ptr::null(),
                                  ptr::null());

    if hService.is_null() {
        println!("CreateServiceW failed");
        CloseServiceHandle(manager);
        return;
    }

    CloseServiceHandle(hService);
    println!("Service installed!");
    return;
}

pub unsafe fn delete_service() {
    let manager = OpenSCManagerW(ptr::null(), ptr::null(), SC_MANAGER_ALL_ACCESS);
    if manager.is_null() {
        println!("OpenSCManagerW failed: {}", Error::last_os_error());
        return;
    }

    let service = OpenServiceW(manager, to_wchar("Name"), SERVICE_STOP | DELETE);
    if service.is_null() {
        println!("OpenServiceW failed: {}", Error::last_os_error());
        CloseServiceHandle(manager);
        return;
    }

    DeleteService(service);
    CloseServiceHandle(service);
    CloseServiceHandle(manager);
    println!("Service deleted!");
    return;
}
