use std::ffi::c_void;

use windows_sys::Win32::System::Diagnostics::Debug::FlushInstructionCache;
use windows_sys::Win32::System::Memory::{
    MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_EXECUTE_READWRITE, VirtualAlloc, VirtualFree,
};
use windows_sys::Win32::System::Threading::GetCurrentProcess;

mod constants;
mod detour;
mod patch;

use crate::runtime::Runtime;
use constants::{PATCH_INJECTION_BYTES, PATCH_INJECTION_RVA, PATCH_SIZE};

/// Applies the patch for NPC stuck sleeping to the game
///
/// It obtains in-game memory address for the injection point, and then:
/// - verifies that injection point is correct (arcanum.exe is compatible)
/// - allocates memory for the patch code sequence
/// - calls function to prepare the patch
/// - copies patch into allocated memory
/// - flushes CPU instructions cache
/// - calls function to inject a detour in game memory which will call the patch
pub fn install(runtime: &Runtime) -> Result<(), &'static str> {
    let injection_address = runtime.game_address(PATCH_INJECTION_RVA);
    let injection_pointer = injection_address as *mut u8;
    let actual =
        unsafe { std::slice::from_raw_parts(injection_pointer, PATCH_INJECTION_BYTES.len()) };
    if actual != PATCH_INJECTION_BYTES {
        return Err("unexpected game bytes at the patch site");
    }

    let patch = unsafe {
        VirtualAlloc(
            std::ptr::null(),
            PATCH_SIZE,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_EXECUTE_READWRITE,
        )
    };
    if patch.is_null() {
        return Err("VirtualAlloc failed for the patch");
    }

    let patch_code = match patch::prepare_patch_code(patch as usize, injection_address, runtime) {
        Ok(code) => code,
        Err(error) => {
            let freed = unsafe { VirtualFree(patch, 0, MEM_RELEASE) };
            if freed == 0 {
                return Err("VirtualFree failed after patch preparation failed");
            }
            return Err(error);
        }
    };

    unsafe {
        std::ptr::copy_nonoverlapping(patch_code.as_ptr(), patch.cast::<u8>(), patch_code.len());
    }

    let flushed = unsafe {
        FlushInstructionCache(
            GetCurrentProcess(),
            patch.cast::<c_void>(),
            patch_code.len(),
        )
    };
    if flushed == 0 {
        return Err("FlushInstructionCache failed for the patch");
    }

    detour::detour(injection_address, patch as usize)
}

pub fn relative_displacement(instruction_end: usize, destination: usize) -> Option<i32> {
    i32::try_from(destination as i64 - instruction_end as i64).ok()
}
