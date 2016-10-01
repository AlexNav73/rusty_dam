
use std::panic;
use std::mem;
use std::sync::Mutex;
use std::io::Error;
use std::collections::VecDeque;

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
    static ref SERVICE_POOL: Mutex<VecDeque<Task>> = Mutex::new(VecDeque::new());
    static ref SERVICE_NAME: Vec<u16> = {
        let os_str_crate = ::std::env::current_exe().unwrap();
        let file_name = os_str_crate.file_stem().unwrap();
        to_wchar(&file_name.to_os_string().into_string().unwrap())
    };
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
        let task = {
            let mut guard = SERVICE_POOL.lock().unwrap();
            guard.pop_front().unwrap()
        };

        ServiceHolder {
            service: task.0,
            handler: None,
        }
    }

    fn service(&self) -> &Service {
        unsafe { mem::transmute(self.service) }
    }
}

pub fn spawn<S: Service + 'static>(services: &[S]) -> Result<(), ServiceError> {
    {
        let mut guard = SERVICE_POOL.lock().unwrap();
        guard.append(&mut services.iter().map(|s| Task(s as *const _)).collect());
    }

    let mut tasks = Vec::with_capacity(services.len() + 1);

    for _ in 0..services.len() {
        tasks.push(SERVICE_TABLE_ENTRYW {
            lpServiceName: SERVICE_NAME.as_ptr(),
            lpServiceProc: Some(start_service_proc),
        });
    }
    tasks.push(SERVICE_TABLE_ENTRYW {
        lpServiceName: 0 as *const _,
        lpServiceProc: None
    });

    //unsafe { start_service_proc(0, 0 as *mut _); }
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
unsafe extern "system" fn start_service_proc(argc: DWORD, argv: *mut LPWSTR) {
    let mut holder = ServiceHolder::new();

    let status_handler = RegisterServiceCtrlHandlerExW(SERVICE_NAME.as_ptr(), Some(service_dispatcher), mem::transmute(&mut holder));

    assert!(!status_handler.is_null());
    holder.handler = Some(status_handler);

    SetServiceStatus(status_handler, &mut service_status(SERVICE_RUNNING));

    let args = ::std::slice::from_raw_parts(argv, argc as usize).iter()
        .map(|x| from_wchar(*x))
        .filter(|x| (*x).is_some())
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

    let service = holder.service();
    let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| { service.start(args.as_slice()); }));

    SetServiceStatus(status_handler, &mut service_status(SERVICE_STOPPED));
}

#[allow(non_snake_case)]
unsafe extern "system" fn service_dispatcher(dwControl: DWORD, _: DWORD, _: LPVOID, lpContext: LPVOID) -> DWORD {
    let holder: &ServiceHolder = mem::transmute(lpContext);

    match dwControl {
        SERVICE_CONTROL_STOP | SERVICE_CONTROL_SHUTDOWN => {
            let service = holder.service();
            let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| { service.stop(); }));

            SetServiceStatus(holder.handler.unwrap(), &mut service_status(SERVICE_STOPPED));
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
