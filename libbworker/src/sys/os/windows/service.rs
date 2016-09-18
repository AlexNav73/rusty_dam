
use std::panic;    
use std::slice;
use std::sync::{Once, ONCE_INIT};
use std::io::Write;

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

use super::helpers::{ from_wchar, get_crate_name_utf16 };
use super::log::{ EventLog, LogType };

pub trait Service {
    fn start(&mut self, _args: &[String]) {}
    fn stop(&mut self) {}
}

static mut STATUS: ServiceHandler = ServiceHandler::new();

struct ServiceHandler {
    status: SERVICE_STATUS,
    handler: Option<SERVICE_STATUS_HANDLE>,
}

impl ServiceHandler {

    #[inline]
    const fn new() -> ServiceHandler {
        ServiceHandler {
            status: SERVICE_STATUS {
                dwServiceType: SERVICE_WIN32_OWN_PROCESS,
                dwCurrentState: SERVICE_START_PENDING,
                dwControlsAccepted: SERVICE_ACCEPT_STOP | SERVICE_ACCEPT_SHUTDOWN,
                dwWin32ExitCode: 0,
                dwServiceSpecificExitCode: 0,
                dwCheckPoint: 0,
                dwWaitHint: 0,
            },
            handler: None
        }
    }

    #[inline]
    fn as_ptr_mut(&mut self) -> *mut SERVICE_STATUS {
        &mut self.status as *mut _
    }

    #[inline]
    fn get_current_state(&self) -> DWORD {
        self.status.dwCurrentState
    }

    #[inline]
    fn handler(&mut self, handle: SERVICE_STATUS_HANDLE) {
        self.handler = Some(handle);
    }

    #[inline]
    fn status(&mut self, state: DWORD) {
        if let Some(handle) = self.handler {
            self.status.dwWin32ExitCode = 0;
            self.status.dwCurrentState = state;
            unsafe { SetServiceStatus(handle, self.as_ptr_mut()); }
        }
    }
}

static INIT: Once = ONCE_INIT;
static mut SERVICE: Option<Box<Service + 'static>> = None; // FIXME: Is there way to get rid from statics?

pub fn launch<T: Service + 'static>(service: Box<T>) {
    INIT.call_once(move || {
        unsafe { 
            let service_table_entry = Box::new(SERVICE_TABLE_ENTRYW {
                lpServiceName: get_crate_name_utf16(),
                lpServiceProc: Some(start_service_proc),
            });
        
            SERVICE = Some(service);
            StartServiceCtrlDispatcherW(service_table_entry.as_ref()); 
        }
    });
}

#[allow(non_snake_case)]
unsafe extern "system" fn start_service_proc(dwNumServicesArgs: DWORD, lpServiceArgVectors: *mut LPWSTR) {
    write("Service -1!");
    //let mut logger = EventLog::new().unwrap();
    //logger.message_type(LogType::AUDIT_FAILURE);

    write("Service 0!");
    let status_handler = RegisterServiceCtrlHandlerW(get_crate_name_utf16(), Some(service_dispatcher));

    if status_handler.is_null() { 
        //let _ = write!(logger, "Start service: call RegisterServiceCtrlHandlerW failed.");
        return; 
    }

    STATUS.handler(status_handler);
    STATUS.status(SERVICE_RUNNING);

    write("Service 1!");

    if STATUS.get_current_state() == SERVICE_RUNNING {
        write("Service 2!");
        if let Some(ref mut serv) = SERVICE {
            // let args = slice::from_raw_parts(lpServiceArgVectors, dwNumServicesArgs as usize).iter()
            //     .map(|x| from_wchar(*x))
            //     .filter(|x| (*x).is_some())
            //     .map(|x| x.unwrap())
            //     .collect::<Vec<_>>();

            write("Service 3!");

            let result = panic::catch_unwind(panic::AssertUnwindSafe(|| { serv.start(&[]); })); // args.as_slice()
            if !result.is_ok() { 
                //let _ = write!(logger, "Service start function panicked. Error: {:?}", result);
                write("Service 3-1!");
            }
            write("Service 4!");
        }
    }

    STATUS.status(SERVICE_STOPPED);
    write("Service 5!");
}

#[allow(non_snake_case)]
unsafe extern "system" fn service_dispatcher(dwControl: DWORD) {
    //let mut logger = EventLog::new().unwrap();
    //logger.message_type(LogType::AUDIT_FAILURE);

    match dwControl {
        SERVICE_CONTROL_STOP => {
            if let Some(ref mut d) = SERVICE {
                let result = panic::catch_unwind(panic::AssertUnwindSafe(|| { d.stop(); }));
                if !result.is_ok() {
                    //let _ = write!(logger, "Service stop function panicked. Error: {:?}", result);
                }
            }

            STATUS.status(SERVICE_STOPPED);
        }
        SERVICE_CONTROL_SHUTDOWN => {
            if let Some(ref mut d) = SERVICE {
                let result = panic::catch_unwind(panic::AssertUnwindSafe(|| { d.stop(); }));
                if !result.is_ok() {
                    //let _ = write!(logger, "Service stop function panicked. Error: {:?}", result);
                }
            }

            STATUS.status(SERVICE_STOPPED);
        }
        _ => { }
    }
}

fn write(s: &str) {
    use std::io::Write;
    use std::fs::OpenOptions;

    let mut file = OpenOptions::new().append(true).open("D:\\Programms\\rusty_dam\\target\\debug\\out.txt").unwrap();
    file.write(s.as_bytes());
    file.write(b"\n");
}

