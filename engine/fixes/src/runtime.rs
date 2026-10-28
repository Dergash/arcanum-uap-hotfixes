use std::ffi::{OsString, c_void};
use std::fs::OpenOptions;
use std::io::Write;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;

use windows_sys::Win32::System::LibraryLoader::{GetModuleFileNameW, GetModuleHandleW};

const TARGET_EXECUTABLE: &str = "Arcanum.exe";
const LOG_FILE_NAME: &str = "engine_fixes.log";

pub struct Runtime {
    log_path: PathBuf,
    game_base: usize,
}

impl Runtime {
    pub fn new(plugin_module: *mut c_void) -> Option<Self> {
        let mut path_buffer = vec![0u16; 32768];
        let path_len = unsafe {
            GetModuleFileNameW(
                plugin_module,
                path_buffer.as_mut_ptr(),
                path_buffer.len() as u32,
            )
        } as usize;
        if path_len == 0 || path_len >= path_buffer.len() {
            return None;
        }

        let plugin_path = PathBuf::from(OsString::from_wide(&path_buffer[..path_len]));
        let log_path = plugin_path.with_file_name(LOG_FILE_NAME);

        let mut executable_buffer = vec![0u16; 32768];
        let executable_len = unsafe {
            GetModuleFileNameW(
                std::ptr::null_mut(),
                executable_buffer.as_mut_ptr(),
                executable_buffer.len() as u32,
            )
        } as usize;
        if executable_len == 0 || executable_len >= executable_buffer.len() {
            return None;
        }

        let executable_path =
            PathBuf::from(OsString::from_wide(&executable_buffer[..executable_len]));
        if !executable_path.file_name().is_some_and(|name| {
            name.to_string_lossy()
                .eq_ignore_ascii_case(TARGET_EXECUTABLE)
        }) {
            return None;
        }

        let game_module = unsafe { GetModuleHandleW(std::ptr::null()) };
        if game_module.is_null() {
            return None;
        }

        Some(Self {
            log_path,
            game_base: game_module as usize,
        })
    }

    pub fn game_address(&self, rva: usize) -> usize {
        self.game_base + rva
    }

    pub fn log(&self, message: &str) {
        if let Ok(mut log) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)
        {
            let _ = writeln!(log, "{message}");
        }
    }
}
