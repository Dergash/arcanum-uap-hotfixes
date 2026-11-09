use windows_sys::Win32::System::Diagnostics::Debug::FlushInstructionCache;
use windows_sys::Win32::System::Memory::{
    MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_EXECUTE_READWRITE, VirtualAlloc, VirtualFree,
};

mod constants;
mod detour;
mod patch;

use crate::runtime::Runtime;
use constants::{PATCH_INJECTION_BYTES, PATCH_INJECTION_RVA, PATCH_SIZE};

/// Applies the patch for voice-line files whose size is an exact multiple of 32 KB.
///
/// It obtains the in-game address for the injection point, and then:
/// - verifies that the injection point is correct (the game executable is compatible)
/// - allocates memory for the patch code sequence
/// - prepares and copies the patch code into allocated memory
/// - flushes the CPU instruction cache
/// - injects a detour in game memory to run the patch
/// 
/// Original game `file.c#tig_file_copy_internal` has a bug where if the file read is
/// exact multiple of 32 KB (size of the reading buffer), then after reading through whole file game doesn't realize
/// the reading is done and tries to do one last read via `fread` with zero-byte `size`, treating the result as failure
/// 
/// That causes bugs like some voice over lines (Virgil, Gilbert Bates, etc.) to not play because 
/// they are of the 160kb size (163840/32768 = 5 exact buffers)
///
/// See https://github.com/alexbatalov/arcanum-ce/blob/a7ff41b300ef712f0e7d088183a3d08957110cdd/first_party/tig/src/file.c#L1932
pub fn install(runtime: &Runtime) -> Result<(), &'static str> {
    let injection_address = runtime.game_address(PATCH_INJECTION_RVA);
    let injection_pointer = injection_address as *const u8;
    let actual =
        unsafe { std::slice::from_raw_parts(injection_pointer, PATCH_INJECTION_BYTES.len()) };
    if actual != PATCH_INJECTION_BYTES {
        return Err("unexpected game bytes at the voice-line patch site");
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
        return Err("VirtualAlloc failed for the voice-line patch");
    }

    let patch_code = match patch::prepare_patch_code(patch as usize, injection_address) {
        Ok(code) => code,
        Err(error) => {
            if unsafe { VirtualFree(patch, 0, MEM_RELEASE) } == 0 {
                return Err("VirtualFree failed after voice-line patch preparation failed");
            }
            return Err(error);
        }
    };

    unsafe {
        std::ptr::copy_nonoverlapping(patch_code.as_ptr(), patch.cast::<u8>(), patch_code.len());
    }

    let flushed = unsafe {
        FlushInstructionCache(
            windows_sys::Win32::System::Threading::GetCurrentProcess(),
            patch.cast(),
            patch_code.len(),
        )
    };
    if flushed == 0 {
        if unsafe { VirtualFree(patch, 0, MEM_RELEASE) } == 0 {
            return Err("VirtualFree failed after voice-line patch cache flush failed");
        }
        return Err("FlushInstructionCache failed for the voice-line patch");
    }

    detour::detour(injection_address, patch as usize)
}
