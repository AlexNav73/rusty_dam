
use std::panic;
use std::mem;
use std::sync::{Mutex, Arc};
use std::io::Error;
use std::collections::VecDeque;
use std::ptr;

use windows::advapi32::{ StartServiceCtrlDispatcherW, RegisterServiceCtrlHandlerExW, SetServiceStatus };
use windows::winapi::winerror::NO_ERROR;
use windows::winapi::winnt::{ LPWSTR, SERVICE_WIN32_SHARE_PROCESS };
use windows::winapi::minwindef::{ DWORD, LPVOID };
use windows::winapi::winsvc::{
    SERVICE_STATUS, 
    SERVICE_STATUS_HANDLE, 
    SERVICE_RUNNING, 
    SERVICE_TABLE_ENTRYW, 
    SERVICE_STOPPED,

    SERVICE_ACCEPT_STOP,
    SERVICE_ACCEPT_SHUTDOWN, 

    SERVICE_CONTROL_SHUTDOWN,
    SERVICE_CONTROL_STOP
};

use ::{ Service, ServiceError };
use windows::{to_wchar, from_wchar};

lazy_static! {
    static ref SERVICE_POOL: Arc<Mutex<ServicePool>> = Arc::new(Mutex::new(ServicePool::new()));
}

struct ServicePool {
    services: VecDeque<ServiceHolder>
}

impl ServicePool {
    fn new() -> ServicePool {
        ServicePool {
            services: VecDeque::new()
        }
    }

    fn enq(&mut self, s: *const Service) {
        self.services.push_back(ServiceHolder::new(s));
    }

    fn deq<S: AsRef<str>>(&mut self, name: S) -> ServiceHolder {
        // Safe, because vec length never be less than number of registered services
        self.services.retain(|&x| x.service().name() == name.as_ref());
        self.services[0]
    }
}

//
// Struct which contains pointer to user defined Server struct and
// service status handle.
//
// Life times doesn't allowed in statics, because of that i have to use raw pointers
//
#[derive(Copy, Clone)]
struct ServiceHolder {
    service: *const Service,
    handler: SERVICE_STATUS_HANDLE,
}

// Safe, because instances of ServiceHolder will never shared among multiple threads.
// Thread will always consume ownership of instance
unsafe impl Send for ServiceHolder {}
unsafe impl Sync for ServiceHolder {}

impl ServiceHolder {
    fn new(s: *const Service) -> ServiceHolder {
        ServiceHolder {
            service: s,
            handler: ptr::null_mut()
        }
    }

    fn service(&self) -> &Service {
        unsafe { &*self.service }
    }
}

///
/// Using Builder struct you can create chain of services
/// which will run after appropriate service will be launched using
/// Service Control Manager.
///
/// usage:
/// ```rust
///    use bworker::Builder;
///
///    let mut b = Builder::new()
///        .service(&s1)
///        .service(&s2)
///        .spawn();
/// ```
///
pub struct Builder<'a>(Vec<&'a Service>);

impl<'a> Builder<'a> {

    ///
    /// Construct new instance of Builder struct
    ///
    pub fn new() -> Builder<'a> {
        Builder(Vec::new())
    }

    ///
    /// Use this method to register service. Service lifetime must match
    /// Builders lifetime.
    ///
    pub fn service(&mut self, s: &'a Service) -> &'a mut Builder {
        self.0.push(s);
        self
    }

    /// 
    /// Register all services in Service Control Manager database and then
    /// blocks until all running services will finish their jobs.
    ///
    pub fn spawn(&self) -> Result<(), ServiceError> {
        {
            let mutex = SERVICE_POOL.clone();
            let mut guard = mutex.lock().unwrap();
            for s in &self.0 {
                guard.enq(unsafe { mem::transmute(*s) });
            }
        }

        // Need one more extra space for null struct
        let mut tasks = Vec::with_capacity(self.0.len() + 1);

        for s in &self.0 {
            tasks.push(SERVICE_TABLE_ENTRYW {
                lpServiceName: to_wchar(&s.name()).as_ptr(),
                lpServiceProc: Some(service_main),
            });
        }

        // Array of SERVICE_TABLE_ENTRYW always must ends with null struct.
        // For more information look at msdn StartServiceCtrlDispatcherW description
        tasks.push(SERVICE_TABLE_ENTRYW {
            lpServiceName: 0 as *const _,
            lpServiceProc: None
        });

        unsafe { 
            if StartServiceCtrlDispatcherW(tasks.as_slice().as_ptr()) == 0 {
                Err(ServiceError::IOError(Error::last_os_error()))
            } else {
                Ok(())
            }
        }
    }

}

fn get_service_holder<S: AsRef<str>>(name: S) -> ServiceHolder {
    let mutex = SERVICE_POOL.clone();
    let mut guard = mutex.lock().unwrap();

    guard.deq(name)
}

// 
// Service main function handles services startup logic. Through it's args
// function recieve name of service, which must be launched. First argument
// points to service name, not executable name.
//
#[allow(non_snake_case)]
unsafe extern "system" fn service_main(argc: DWORD, argv: *mut LPWSTR) {

    let args = ::std::slice::from_raw_parts(argv, argc as usize).iter()
        .map(|x| from_wchar(*x))
        .filter(|x| (*x).is_some())
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

    let mut holder = get_service_holder(&args[0]);

    let status_handler = RegisterServiceCtrlHandlerExW(to_wchar(&args[0]).as_ptr(), Some(service_handler), mem::transmute(&mut holder));

    assert!(!status_handler.is_null());
    holder.handler = status_handler;

    SetServiceStatus(status_handler, &mut service_status(SERVICE_RUNNING));

    let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| { 
        holder.service().start(args.as_slice()); 
    }));

    SetServiceStatus(status_handler, &mut service_status(SERVICE_STOPPED));
}

#[allow(non_snake_case)]
unsafe extern "system" fn service_handler(dwControl: DWORD, _: DWORD, _: LPVOID, lpContext: LPVOID) -> DWORD {
    let holder: &ServiceHolder = mem::transmute(lpContext);

    match dwControl {
        SERVICE_CONTROL_STOP | SERVICE_CONTROL_SHUTDOWN => {
            let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| { holder.service().stop(); }));

            SetServiceStatus(holder.handler, &mut service_status(SERVICE_STOPPED));
        }
        _ => { }
    }

    NO_ERROR
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

