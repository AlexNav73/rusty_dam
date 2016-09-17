
use std::io;
use std::path::Path;
use std::env;

use ::winreg::RegKey;
use ::winreg::enums::*;

use super::log::LogType;

pub fn register_event_reg_key(app_name: &str) -> io::Result<()> {

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    let mut reg_key_path = String::new();
    reg_key_path.push_str("System\\CurrentControlSet\\Services\\EventLog\\Application\\");
    reg_key_path.push_str(app_name);

    if let Err(_) = hklm.open_subkey_with_flags(Path::new(&reg_key_path), KEY_READ) {

        let key = try!(hklm.create_subkey(Path::new(&reg_key_path)));

        let crate_name = try!(env::current_exe());
        key.set_value("EventMessageFile", &crate_name.as_os_str());

        let mut types_supported = LogType::AUDIT_FAILURE | 
                                LogType::AUDIT_SUCCESS | 
                                LogType::ERROR_TYPE | 
                                LogType::INFORMATION_TYPE | 
                                LogType::SUCCESS | 
                                LogType::WARNING_TYPE;

        key.set_value("TypeSupported", &(types_supported as u32));
    }
    Ok(())
}

pub fn unregister_event_reg_key(app_name: &str) -> io::Result<()> {
    let mut reg_key_path = "SYSTEM\\CurrentControlSet\\Services\\EventLog\\Application\\".to_owned();
    reg_key_path.push_str(app_name);

    try!(RegKey::predef(HKEY_LOCAL_MACHINE).delete_subkey(reg_key_path));
    Ok(())
}
