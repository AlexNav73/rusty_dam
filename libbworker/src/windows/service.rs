
use std::panic;
use std::mem;

use windows::advapi32::{ StartServiceCtrlDispatcherW, RegisterServiceCtrlHandlerW, SetServiceStatus };
use windows::winapi::winnt::{ LPWSTR, SERVICE_WIN32_OWN_PROCESS };
use windows::winapi::minwindef::DWORD;
use windows::winapi::winsvc::{
    SERVICE_STATUS, 
    SERVICE_STATUS_HANDLE, 
    SERVICE_CONTROL_SHUTDOWN, 
    SERVICE_ACCEPT_STOP,
    SERVICE_ACCEPT_SHUTDOWN, 
    SERVICE_RUNNING, 
    SERVICE_TABLE_ENTRYW, 
    SERVICE_CONTROL_STOP, 
    SERVICE_STOPPED
};

use ::{ Service, ServiceError };
use windows::{to_wchar, from_wchar, current_exe_name};

static mut SERVICE: Option<*const ServiceHolder> = None;
static mut SERVICE_NAME: Option<*const u16>      = None;

//
// Struct which contains pointer to user defined Server struct and
// service status handle.
//
struct ServiceHolder {
    service: *const Service,
    handler: Option<SERVICE_STATUS_HANDLE>
}

impl ServiceHolder {
    fn service(&self) -> &Service {
        unsafe { mem::transmute(self.service) }
    }
}

//
// This is safe, because ServiceHolder used synchronously.
// Instance of this struct dropped when signal SERVICE_CONTROL_STOP or SERVICE_CONTROL_SHUTDOWN
// occures.
//
unsafe impl Sync for ServiceHolder {}
unsafe impl Send for ServiceHolder {}

fn lock<F: FnOnce(&ServiceHolder)>(func: F) {
    unsafe {
        if let Some(ptr) = SERVICE {
            func(mem::transmute(ptr));
        }
    }
}

fn lock_mut<F: FnOnce(&mut ServiceHolder)>(func: F) {
    unsafe {
        if let Some(ptr) = SERVICE {
            func(mem::transmute(ptr));
        }
    }
}

pub struct ServiceBuilder {
    name: Option<String>
}

impl ServiceBuilder {

    #[inline]
    pub fn new() -> ServiceBuilder {
        ServiceBuilder { name: None }
    }

    #[inline]
    pub fn name<T: AsRef<str>>(&mut self, value: T) -> &mut ServiceBuilder {
        self.name = Some(value.as_ref().to_owned());
        self
    }

    pub fn run<S: Service + 'static>(self, inst: S) -> Result<(), ServiceError> {

        unsafe {
            match SERVICE {
                None => {
                    let holder = ServiceHolder {
                        service: &inst as *const _,
                        handler: None
                    };
                    SERVICE = Some(&holder as *const _);
                },
                Some(_) => return Err(ServiceError::MultInst)
            }
        }

        let unicode_service_name = match self.name {
            Some(ref n) => to_wchar(n),
            None => to_wchar(&current_exe_name())
        };
        
        let service_table_entry = SERVICE_TABLE_ENTRYW {
            lpServiceName: unicode_service_name.as_ptr(),
            lpServiceProc: Some(start_service_proc),
        };

        unsafe { 
            SERVICE_NAME = Some(unicode_service_name.as_ptr());

            //
            // Register callback, which Service Control Manager will trigger after
            // service will be launched
            //
            StartServiceCtrlDispatcherW(&service_table_entry); 

            SERVICE.take();
            SERVICE_NAME.take();
        } 

        Ok(())
    }
}

#[allow(non_snake_case)]
unsafe extern "system" fn start_service_proc(dwNumServicesArgs: DWORD, lpServiceArgVectors: *mut LPWSTR) {
    let status_handler = RegisterServiceCtrlHandlerW(SERVICE_NAME.unwrap(), Some(service_dispatcher));

    if status_handler.is_null() { return; }
    lock_mut(|serv| serv.handler = Some(status_handler));

    SetServiceStatus(status_handler, &mut service_status(SERVICE_RUNNING));

    let args = ::std::slice::from_raw_parts(lpServiceArgVectors, dwNumServicesArgs as usize).iter()
        .map(|x| from_wchar(*x))
        .filter(|x| (*x).is_some())
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

    lock(|serv| {
        ::crossbeam::scope(|scope| {
            scope.spawn(|| {
                let _ = panic::catch_unwind(
                    panic::AssertUnwindSafe(|| { serv.service().start(args.as_slice()); })
                );
            });
        });
    });

    SetServiceStatus(status_handler, &mut service_status(SERVICE_STOPPED));
}

#[allow(non_snake_case)]
unsafe extern "system" fn service_dispatcher(dwControl: DWORD) {
    match dwControl {
        SERVICE_CONTROL_STOP | SERVICE_CONTROL_SHUTDOWN => {
            lock(|serv| {
                ::crossbeam::scope(|scope| {
                    scope.spawn(|| {
                        let _ = panic::catch_unwind(
                            panic::AssertUnwindSafe(|| { serv.service().stop(); }));
                    });
                });

                SetServiceStatus(serv.handler.unwrap(), &mut service_status(SERVICE_STOPPED));
            });
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

