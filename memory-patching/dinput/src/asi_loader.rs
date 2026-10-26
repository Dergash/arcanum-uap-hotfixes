use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::PathBuf;

use windows_sys::Win32::Foundation::HMODULE;
use windows_sys::Win32::System::LibraryLoader::{GetModuleFileNameW, GetProcAddress, LoadLibraryW};

// ASI plugins to load, hardcoded instead of scanning as UAP comes with its own .asi files.
const ASI_FILE_NAMES: &[&str] = &["arcanum-extender.asi"];

pub fn load_asi_plugins(proxy_module: HMODULE) {
    if proxy_module.is_null() {
        return;
    }
    let mut path: Vec<u16> = vec![0u16; 32768];
    let length = unsafe {
        GetModuleFileNameW(
            proxy_module,
            path.as_mut_ptr(),
            path.len() as u32,
        )
    } as usize;
    if length == 0 || length >= path.len() {
        return;
    }

    let proxy_path = PathBuf::from(std::ffi::OsString::from_wide(&path[..length]));
    for file_name in ASI_FILE_NAMES {
        let asi_path = proxy_path.with_file_name(file_name);
        let wide_path: Vec<u16> = asi_path.as_os_str().encode_wide().chain(Some(0)).collect();
        let asi = unsafe { LoadLibraryW(wide_path.as_ptr()) };
        if asi.is_null() {
            continue;
        }

        if let Some(initialize) = unsafe { GetProcAddress(asi, c"InitializeASI".as_ptr().cast()) } {
            let initialize: unsafe extern "C" fn() = unsafe { std::mem::transmute(initialize) };
            unsafe { initialize() };
        }
    }
}
