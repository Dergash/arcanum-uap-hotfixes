/// Minimalistic ASI loader to circumvent conflicting for .asi files with UAP
use std::ffi::c_void;
use std::sync::atomic::{AtomicPtr, Ordering};

use windows_sys::Win32::Foundation::{HINSTANCE, HMODULE};

mod asi_loader;
mod system_dinput;

// See <https://learn.microsoft.com/en-us/windows/win32/dlls/dllmain#parameters>
const DLL_PROCESS_ATTACH: u32 = 1;

// Unspecified failure, see <https://learn.microsoft.com/en-us/windows/win32/seccrypto/common-hresult-values#listing-of-common-hresult-values>
const E_FAIL: i32 = 0x80004005_u32 as i32;

// Handle to our dinput.dll
static PROXY_MODULE: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());

//
static ASI_PLUGINS_LOADED: std::sync::OnceLock<()> = std::sync::OnceLock::new();

/// See <https://learn.microsoft.com/en-us/windows/win32/dlls/dllmain>
#[unsafe(no_mangle)]
pub unsafe extern "system" fn DllMain(
    module: HINSTANCE,
    reason: u32,
    _reserved: *mut c_void,
) -> i32 {
    if reason == DLL_PROCESS_ATTACH {
        PROXY_MODULE.store(module as *mut c_void, Ordering::Release);
    }
    1
}

/// Forwards DirectInputCreateA calls to system dinput.dll and loads ASI plugins on the first call
#[unsafe(no_mangle)]
pub unsafe extern "system" fn DirectInputCreateA(
    instance: HINSTANCE,
    version: u32,
    direct_input: *mut *mut c_void,
    outer: *mut c_void,
) -> i32 {
    let proxy_module = PROXY_MODULE.load(Ordering::Acquire) as HMODULE;
    ASI_PLUGINS_LOADED.get_or_init(|| asi_loader::load_asi_plugins(proxy_module));

    let create_fn = system_dinput::direct_input_create();
    if create_fn == 0 {
        return E_FAIL;
    }

    let create: unsafe extern "system" fn(HINSTANCE, u32, *mut *mut c_void, *mut c_void) -> i32 =
        unsafe { std::mem::transmute(create_fn) };
    unsafe { create(instance, version, direct_input, outer) }
}
