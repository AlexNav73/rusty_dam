
use std::panic;
use std::ptr;
use std::mem;
use std::sync::{ Arc, Mutex, Once, ONCE_INIT };
use std::cell::UnsafeCell;
use std::ffi::OsStr;
use std::borrow::{Cow, Borrow};

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

use super::{Service, to_wchar, from_wchar};

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

pub enum ServiceError {
    CouldNotStartService
}

pub struct ServiceBuilder {
    name: Option<String>
}

impl ServiceBuilder {

    pub fn new() -> ServiceBuilder {
        ServiceBuilder { name: None }
    }

    pub fn name<T: AsRef<str>>(&mut self, value: T) -> &mut ServiceBuilder {
        self.name = Some(value.as_ref().to_owned());
        self
    }

    pub fn run<S>(self, instance: S) -> Result<(), ServiceError>  
        where S: Service + 'static {
        match SERVICE.lock() {
            Ok(mut g) => {
                *g = Some(ServiceHolder {
                    service: Box::new(instance),
                    handler: None
                });
            },
            Err(_) => return Err(ServiceError::CouldNotStartService)
        }

        let unicode_service_name = 
            if let Some(ref n) = self.name { 
                to_wchar(n) 
            } else { 
                let os_str_crate = ::std::env::current_exe().unwrap();
                let file_name = os_str_crate.file_stem().unwrap();
                let crate_name = file_name.to_os_string().into_string().unwrap();

                to_wchar(&crate_name) 
            };
        
        let service_table_entry = unsafe { SERVICE_TABLE_ENTRYW {
            lpServiceName: unicode_service_name.as_ptr(),
            lpServiceProc: Some(start_service_proc),
        }};

        match SERVICE_NAME.lock() {
            Ok(mut g) => *g = Some(unicode_service_name),
            Err(_) => return Err(ServiceError::CouldNotStartService)
        }

        unsafe { StartServiceCtrlDispatcherW(&service_table_entry); } 
        Ok(())
    }

}

fn invoke<F: FnOnce(&mut ServiceHolder)>(func: F) {
    match SERVICE.lock() {
        Ok(ref mut s) => func(s.as_mut().unwrap()),
        Err(_) => {}
    }
}

#[allow(non_snake_case)]
unsafe extern "system" fn start_service_proc(dwNumServicesArgs: DWORD, lpServiceArgVectors: *mut LPWSTR) {
        
    let status_handler;
    {
        let mut guard = SERVICE_NAME.lock().unwrap();
        let name = guard.as_mut().unwrap();

        status_handler = unsafe { RegisterServiceCtrlHandlerW(name.as_ptr(), Some(service_dispatcher)) };
    }

    if status_handler.is_null() { return; }
    invoke(|serv| serv.handler = Some(status_handler));

    SetServiceStatus(status_handler, &mut service_status(SERVICE_RUNNING));

    write(&format!("{:?}", dwNumServicesArgs));

    let args = ::std::slice::from_raw_parts(lpServiceArgVectors, dwNumServicesArgs as usize).iter()
        .map(|x| unsafe { from_wchar2(*x) })
        .inspect(|x| write(&format!("{:?}", x)))
        .filter(|x| (*x).is_some())
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

    write(&format!("{:?}", args));
    
    invoke(|serv| {
        let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| { serv.service.start(args.as_slice()); }));
    });
    SetServiceStatus(status_handler, &mut service_status(SERVICE_STOPPED));
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

fn write(s: &str) {
    use std::io::Write;
    use std::fs::OpenOptions;

    let mut file = OpenOptions::new().append(true).open("D:\\Programms\\rusty_dam\\target\\debug\\out.txt").unwrap();
    file.write(s.as_bytes());
    file.write(b"\n");
}

unsafe fn from_wchar2(ptr: *const u16) -> Option<String> {
    use std::ffi::{OsStr, OsString};
    use std::os::windows::ffi::{ OsStringExt, OsStrExt };

    match ptr.is_null() {
        true => {
            let len = (0..::std::isize::MAX).position(|i| *ptr.offset(i) == 0).unwrap();
            write(&format!("len: {:?}", len));
            let slice = ::std::slice::from_raw_parts(ptr, len);
            write(&format!("slice: {:?}", slice));
            write(&format!("string: {:?}", OsString::from_wide(slice).to_string_lossy().into_owned()));
            Some(OsString::from_wide(slice).to_string_lossy().into_owned())
        }
        false => { None }
    }
}
