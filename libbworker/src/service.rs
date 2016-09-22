
use std::panic;
use std::ptr;
use std::mem;
use std::sync::{ Arc, Mutex, Once, ONCE_INIT };
use std::cell::UnsafeCell;
use std::ffi::OsStr;

use ::advapi32::{ StartServiceCtrlDispatcherW, RegisterServiceCtrlHandlerW, SetServiceStatus };
use ::winapi::winnt::{ LPWSTR, SERVICE_WIN32_OWN_PROCESS };
use ::winapi::minwindef::DWORD;
use ::winapi::winsvc::{
    SERVICE_STATUS, 
    SERVICE_STATUS_HANDLE, 
    SERVICE_START_PENDING, 
    SERVICE_CONTROL_SHUTDOWN, 
    SERVICE_ACCEPT_STOP,
    SERVICE_ACCEPT_SHUTDOWN, 
    SERVICE_RUNNING, 
    SERVICE_TABLE_ENTRYW, 
    SERVICE_CONTROL_STOP, 
    SERVICE_STOPPED
};

use super::{Service, to_wchar};

lazy_static! {
    static ref SERVICE: Arc<Mutex<Option<ServiceHolder>>> = Arc::new(Mutex::new(None));
    static ref SERVICE_NAME: Arc<Mutex<Option<Vec<u16>>>> = Arc::new(Mutex::new(None));
}

struct ServiceHolder {
    service: Box<Service + 'static>,
    handler: Option<SERVICE_STATUS_HANDLE>
}

unsafe impl Sync for ServiceHolder {}
unsafe impl Send for ServiceHolder {}

#[inline]
fn service_status(state: DWORD) -> SERVICE_STATUS {
    SERVICE_STATUS {
        dwServiceType: SERVICE_WIN32_OWN_PROCESS,
        dwCurrentState: state,
        dwControlsAccepted: SERVICE_ACCEPT_STOP | SERVICE_ACCEPT_SHUTDOWN,
        dwWin32ExitCode: 0,
        dwServiceSpecificExitCode: 0,
        dwCheckPoint: 0,
        dwWaitHint: 0,
    }
}

pub enum ServiceError {
    CouldNotStartService
}

pub fn run<T, N>(serv: T, name: N) -> Result<(), ServiceError> 
    where T: Service + 'static,
          N: AsRef<OsStr>
{
    match SERVICE.lock() {
        Ok(mut g) => {
            *g = Some(ServiceHolder {
                service: Box::new(serv),
                handler: None
            });

            let unicode_service_name = to_wchar(&name);

            unsafe {
                let service_table_entry = SERVICE_TABLE_ENTRYW {
                    lpServiceName: unicode_service_name.as_ptr(),
                    lpServiceProc: Some(start_service_proc),
                };

                StartServiceCtrlDispatcherW(&service_table_entry); 
            }

            *(SERVICE_NAME.lock().unwrap()) = Some(unicode_service_name);
            Ok(())
        },
        Err(_) => Err(ServiceError::CouldNotStartService)
    }
}

fn invoke<F: FnOnce(&mut ServiceHolder)>(func: F) {
    match SERVICE.lock() {
        Ok(ref mut s) => func(&mut s.unwrap()), // FIXME: s <- cannot move out of borrowed content
        Err(_) => {}
    }
}

#[allow(non_snake_case)]
unsafe extern "system" fn start_service_proc(dwNumServicesArgs: DWORD, lpServiceArgVectors: *mut LPWSTR) {
    invoke(|serv| {
        let name = (*(SERVICE_NAME.lock().unwrap()).unwrap()).as_ptr(); // FIXME: cannot move out of borrowed content
        let status_handler = unsafe { RegisterServiceCtrlHandlerW(name, Some(service_dispatcher)) };
        if status_handler.is_null() { return; }
        serv.handler = Some(status_handler);

        SetServiceStatus(status_handler, &mut service_status(SERVICE_RUNNING));

        // let args = slice::from_raw_parts(lpServiceArgVectors, dwNumServicesArgs as usize).iter()
        //     .map(|x| from_wchar(*x))
        //     .filter(|x| (*x).is_some())
        //     .map(|x| x.unwrap())
        //     .collect::<Vec<_>>();

        let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| { serv.service.start(&[]); })); // args.as_slice()
        SetServiceStatus(status_handler, &mut service_status(SERVICE_STOPPED));
    });
}

#[allow(non_snake_case)]
unsafe extern "system" fn service_dispatcher(dwControl: DWORD) {
    match dwControl {
        SERVICE_CONTROL_STOP | SERVICE_CONTROL_SHUTDOWN => {
            invoke(|serv| {
                let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| { serv.service.stop(); }));
                SetServiceStatus(serv.handler.unwrap(), &mut service_status(SERVICE_STOPPED));
            })
        }
        _ => { }
    }
}

// fn write(s: &str) {
//     use std::io::Write;
//     use std::fs::OpenOptions;

//     let mut file = OpenOptions::new().append(true).open("D:\\Programms\\rusty_dam\\target\\debug\\out.txt").unwrap();
//     file.write(s.as_bytes());
//     file.write(b"\n");
// }

