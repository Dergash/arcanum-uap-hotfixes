use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use windows_sys::Win32::Foundation::CloseHandle;
use windows_sys::Win32::System::Threading::CreateThread;

mod runtime;
mod stuck_sleeping;

const PROCESS_ATTACH: u32 = 1;

static PLUGIN_MODULE: AtomicUsize = AtomicUsize::new(0);
static INITIALIZATION_STARTED: AtomicBool = AtomicBool::new(false);

#[unsafe(no_mangle)]
pub unsafe extern "system" fn DllMain(
    module: *mut c_void,
    reason: u32,
    _reserved: *mut c_void,
) -> i32 {
    if reason == PROCESS_ATTACH {
        PLUGIN_MODULE.store(module as usize, Ordering::Release);
    }
    1
}

#[unsafe(no_mangle)]
pub extern "C" fn InitializeASI() {
    if INITIALIZATION_STARTED.swap(true, Ordering::AcqRel) {
        return;
    }

    let module = PLUGIN_MODULE.load(Ordering::Acquire) as *mut c_void;
    if module.is_null() {
        return;
    }

    let thread = unsafe {
        CreateThread(
            std::ptr::null(),
            0,
            Some(initialize_runtime),
            module,
            0,
            std::ptr::null_mut(),
        )
    };
    if !thread.is_null() {
        unsafe { CloseHandle(thread) };
    }
}

unsafe extern "system" fn initialize_runtime(module: *mut c_void) -> u32 {
    let Some(runtime) = runtime::Runtime::new(module) else {
        return 0;
    };
    runtime.log(&format!(
        "[DEBUG] Successfully injected into Arcanum.exe (PID {}).",
        std::process::id()
    ));
    match stuck_sleeping::install(&runtime) {
        Ok(()) => runtime.log("[DEBUG] Standpoint sleeping-condition patch installed."),
        Err(error) => runtime.log(&format!(
            "[ERROR] Unable to install standpoint sleeping-condition patch: {error}"
        )),
    }
    0
}
