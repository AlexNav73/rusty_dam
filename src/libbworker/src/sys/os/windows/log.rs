
use std::ptr;
use std::io;
use std::io::Write;
use std::ops::BitOr;

use ::winapi::winnt::{LPCWSTR, HANDLE, PSID};
use ::winapi::minwindef::{BOOL, WORD, DWORD, LPVOID};

use super::helpers::{ get_crate_name_utf16, get_crate_name_utf8, to_wchar };
use super::reg;

pub struct EventLog {
    handler: HANDLE,
    message_type: LogType
}

impl EventLog {
    pub fn new() -> Result<EventLog, Box<io::Error>> {
        try!(reg::register_event_reg_key(&get_crate_name_utf8()).map_err(|e| Box::new(e)));

        let hEventSource = unsafe { RegisterEventSourceW(ptr::null(), get_crate_name_utf16()) };
        if hEventSource.is_null() {
            return Err(Box::new(io::Error::last_os_error()));
        }
        Ok(EventLog { handler: hEventSource, message_type: LogType::SUCCESS })
    }

    pub fn message_type(&mut self, m_type: LogType) {
        self.message_type = m_type;
    }
}

impl Drop for EventLog {
    fn drop(&mut self) {
        unsafe { DeregisterEventSource(self.handler); }
    }
}

impl Write for EventLog {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let message = String::from_utf8(buf.to_owned());
        let strings = [ to_wchar(&message.unwrap()) ]; 
 
        unsafe {
            ReportEventW(self.handler,             // Event log handle 
                self.message_type.clone() as u16,  // Event type 
                0,                                 // Event category 
                0,                                 // Event identifier 
                ptr::null_mut(),                   // No security identifier 
                2,                                 // Size of strings array 
                0,                                 // No binary data 
                strings.as_ptr(),                  // Array of strings 
                ptr::null_mut()                    // No binary data 
            ); 
        }

        Ok(0)
    }

    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum LogType {
    SUCCESS          = 0x0000,
    ERROR_TYPE       = 0x0001,
    WARNING_TYPE     = 0x0002,
    INFORMATION_TYPE = 0x0004,
    AUDIT_SUCCESS    = 0x0008,
    AUDIT_FAILURE    = 0x0010,
}

impl BitOr for LogType {
    type Output = LogType;
    fn bitor(self, rhs: LogType) -> Self::Output {
        use std::mem;

        unsafe { mem::transmute(self as u8 | rhs as u8) }
    }
}

#[link(name = "advapi32")]
extern "system" {
    fn RegisterEventSourceW(lpUNCServerName: LPCWSTR, lpSourceName: LPCWSTR) -> HANDLE;
    fn DeregisterEventSource(hEventLog: HANDLE) -> BOOL;

    fn ReportEventW(
        hEventLog: HANDLE,
        wType: WORD,
        wCategory: WORD,
        dwEventID: DWORD,
        lpUserSid: PSID,
        wNumStrings: WORD,
        dwDataSize: DWORD,
        lpStrings: *const LPCWSTR,
        lpRawData: LPVOID  
    ) -> BOOL;
}
