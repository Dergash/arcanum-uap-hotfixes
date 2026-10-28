use std::sync::OnceLock;

use windows_sys::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryW};
use windows_sys::Win32::System::SystemInformation::GetSystemDirectoryW;

// Address for DirectInputCreateA imported from real dinput.dll
// This is the only symbol imported by Arcanum.exe from there so we keep it to forward calls necessary by the game itself
static DIRECT_INPUT_CREATE: OnceLock<usize> = OnceLock::new();

// See `uSize`:
// https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemdirectoryw
const PATH_BUFFER_SIZE: usize = 32768;

/// Loads system dinput.dll and obtains and caches address for its DirectInputCreateA func for use by game
pub fn direct_input_create() -> usize {
    *DIRECT_INPUT_CREATE.get_or_init(load_system_direct_input)
}

fn load_system_direct_input() -> usize {
    let Some(path) = system_dinput_path() else {
        return 0;
    };

    let module = unsafe { LoadLibraryW(path.as_ptr()) };
    if module.is_null() {
        return 0;
    }

    unsafe { GetProcAddress(module, c"DirectInputCreateA".as_ptr().cast()) }
        .map_or(0, |function| function as usize)
}

// Obtain path to the real dinput.dll in the system directory, i.e. C:\Windows\System32\dinput.dll
fn system_dinput_path() -> Option<Vec<u16>> {
    let mut path: Vec<u16> = vec![0; PATH_BUFFER_SIZE];
    let length = unsafe { GetSystemDirectoryW(path.as_mut_ptr(), path.len() as u32) } as usize;
    if length == 0 || length >= path.len() {
        return None;
    }

    path.truncate(length);
    path.extend("\\dinput.dll".encode_utf16());
    path.push(0);

    Some(path)
}
