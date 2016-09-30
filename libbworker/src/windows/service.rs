
use std::cell::{ Cell, UnsafeCell };
use std::panic;
use std::mem;
use std::sync::{ Arc, Mutex };

use windows::advapi32::{ StartServiceCtrlDispatcherW, RegisterServiceCtrlHandlerW, SetServiceStatus };
use windows::winapi::winnt::{ LPWSTR, SERVICE_WIN32_SHARE_PROCESS };
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
use windows::{to_wchar, from_wchar};

thread_local! {
    static SERVICE: Cell<Option<ServiceHolder>> = Cell::new(None);
}

lazy_static! {
    static ref SERVICE_POOL: Mutex<Vec<Task>> = Mutex::new(Vec::new());
}

unsafe impl Send for Task {}
unsafe impl Sync for Task {}

struct Task(*const Service);

//
// Struct which contains pointer to user defined Server struct and
// service status handle.
//
// Life times doesn't allowed in statics, because of that i have to use raw pointers
//
#[derive(Copy, Clone)]
struct ServiceHolder {
    service: *const Service,
    handler: Option<SERVICE_STATUS_HANDLE>,
}

impl ServiceHolder {
    fn new() -> ServiceHolder {
        use std::sync::{ Once, ONCE_INIT };

        static INIT: Once = ONCE_INIT;

        INIT.call_once(|| {
            let task = {
                let guard = SERVICE_POOL.lock().unwrap();
                guard.pop().unwrap()
            };

            SERVICE.with(|h| h.set(Some(
                ServiceHolder {
                    service: task.0,
                    handler: None
                }
            )));
        });

        SERVICE.with(|h| h.get().unwrap())
    }

    fn service(&self) -> &Service {
        unsafe { mem::transmute(self.service) }
    }
}

pub fn spawn<S: Service + 'static>(services: &[S]) -> Result<(), ServiceError> {

    {
        let guard = SERVICE_POOL.lock().unwrap();
        guard.append(&mut services.iter().map(|s| Task(s as *const _)).collect());
    }

    let os_str_crate = ::std::env::current_exe().unwrap();
    let file_name = os_str_crate.file_stem().unwrap();
    let service_name = to_wchar(&file_name.to_os_string().into_string().unwrap());

    let mut tasks = Vec::with_capacity(services.len());
    for task in &mut tasks {
        task = &mut SERVICE_TABLE_ENTRYW {
            lpServiceName: service_name.as_ptr(),
            lpServiceProc: Some(start_service_proc),
        };
    }

    unsafe { StartServiceCtrlDispatcherW(tasks.as_slice().as_ptr()); } 

    Ok(())
}


#[allow(non_snake_case)]
unsafe extern "system" fn start_service_proc(dwNumServicesArgs: DWORD, lpServiceArgVectors: *mut LPWSTR) {
    let mut holder = ServiceHolder::new();

    let status_handler = SERVICE.with(|s| RegisterServiceCtrlHandlerW(/*SERVICE NAME*/, Some(service_dispatcher)));

    if status_handler.is_null() { return; }
    holder.handler = Some(status_handler);

    SetServiceStatus(status_handler, &mut service_status(SERVICE_RUNNING));

    let args = ::std::slice::from_raw_parts(lpServiceArgVectors, dwNumServicesArgs as usize).iter()
        .map(|x| from_wchar(*x))
        .filter(|x| (*x).is_some())
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

    let service = holder.service();
    ::crossbeam::scope(|scope| {
        scope.spawn(|| {
            let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| { service.start(args.as_slice()); }));
        });
    });

    SetServiceStatus(status_handler, &mut service_status(SERVICE_STOPPED));
    //SERVICE.with(|s| s.set(None));
}

#[allow(non_snake_case)]
unsafe extern "system" fn service_dispatcher(dwControl: DWORD) {
    let holder = ServiceHolder::new();

    match dwControl {
        SERVICE_CONTROL_STOP | SERVICE_CONTROL_SHUTDOWN => {
            let service = holder.service();
            ::crossbeam::scope(|scope| {
                scope.spawn(|| {
                    let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| { service.stop(); }));
                });
            });

            SetServiceStatus(holder.handler.unwrap(), &mut service_status(SERVICE_STOPPED));
            /*SERVICE.with(|s| s.set(None));*/
        }
        _ => { }
    }
}

#[inline]
fn service_status(state: DWORD) -> SERVICE_STATUS {
    SERVICE_STATUS {
        dwServiceType: SERVICE_WIN32_SHARE_PROCESS,
        dwCurrentState: state,
        dwControlsAccepted: SERVICE_ACCEPT_STOP | SERVICE_ACCEPT_SHUTDOWN,
        dwWin32ExitCode: 0,
        dwServiceSpecificExitCode: 0,
        dwCheckPoint: 0,
        dwWaitHint: 0,
    }
}

