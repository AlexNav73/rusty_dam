
use std::panic;
use std::mem;
use std::sync::Mutex;
use std::io::Error;
use std::collections::VecDeque;
use std::ptr;

use windows::advapi32::{ StartServiceCtrlDispatcherW, RegisterServiceCtrlHandlerExW, SetServiceStatus };
use windows::winapi::winerror::NO_ERROR;
use windows::winapi::winnt::{ LPWSTR, SERVICE_WIN32_SHARE_PROCESS }; // SERVICE_WIN32_OWN_PROCESS, 
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
    static ref SERVICE_POOL: Mutex<VecDeque<ServiceHolder>> = Mutex::new(VecDeque::new());
    static ref SERVICE_NAME: Vec<u16> = {
        let os_str_crate = ::std::env::current_exe().unwrap();
        let file_name = os_str_crate.file_stem().unwrap();
        to_wchar(&file_name.to_os_string().into_string().unwrap())
    };
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
    fn new() -> ServiceHolder {
        let mut guard = SERVICE_POOL.lock().unwrap();
        // Safe, because vec length never be less than number of registered services
        guard.pop_front().unwrap() 
    }

    fn service(&self) -> &Service {
        unsafe { &*self.service }
    }
}

pub fn spawn<S: Service + 'static>(services: &[S]) -> Result<(), ServiceError> {
    {
        let mut guard = SERVICE_POOL.lock().unwrap();
        guard.append(&mut services.iter()
                     .map(|s| ServiceHolder { service: s as *const _, handler: ptr::null_mut() })
                     .collect());
    }

    // Need one more extra space for null struct
    let mut tasks = Vec::with_capacity(services.len() + 1);

    for _ in 0..services.len() {
        tasks.push(SERVICE_TABLE_ENTRYW {
            lpServiceName: SERVICE_NAME.as_ptr(),
            lpServiceProc: Some(service_main),
        });
    }

    // Array of SERVICE_TABLE_ENTRYW always must ends with null struct.
    // For more information look at msdn StartServiceCtrlDispatcherW description
    tasks.push(SERVICE_TABLE_ENTRYW {
        lpServiceName: 0 as *const _,
        lpServiceProc: None
    });

    //unsafe { service_main(0, 0 as *mut _); }
    //return Ok(());

    unsafe { 
        if StartServiceCtrlDispatcherW(tasks.as_slice().as_ptr()) == 0 {
            Err(ServiceError::IOError(Error::last_os_error()))
        } else {
            Ok(())
        }
    }
}


#[allow(non_snake_case)]
unsafe extern "system" fn service_main(argc: DWORD, argv: *mut LPWSTR) {
    let mut holder = ServiceHolder::new();

    let status_handler = RegisterServiceCtrlHandlerExW(SERVICE_NAME.as_ptr(), Some(service_handler), mem::transmute(&mut holder));

    assert!(!status_handler.is_null());
    holder.handler = status_handler;

    SetServiceStatus(status_handler, &mut service_status(SERVICE_RUNNING));

    let args = ::std::slice::from_raw_parts(argv, argc as usize).iter()
        .map(|x| from_wchar(*x))
        .filter(|x| (*x).is_some())
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

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
        //dwServiceType: SERVICE_WIN32_OWN_PROCESS,
        dwCurrentState: state,
        dwControlsAccepted: SERVICE_ACCEPT_STOP | SERVICE_ACCEPT_SHUTDOWN,
        dwWin32ExitCode: 0,
        dwServiceSpecificExitCode: 0,
        dwCheckPoint: 0,
        dwWaitHint: 0,
    }
}

fn _write<S: AsRef<str>>(s: S) {
    use std::io::Write;
    use std::fs::OpenOptions;

    let mut file = OpenOptions::new().append(true).open("D:\\Programms\\rusty_dam\\target\\debug\\out.txt").unwrap();
    let _ = file.write(s.as_ref().as_bytes());
}
