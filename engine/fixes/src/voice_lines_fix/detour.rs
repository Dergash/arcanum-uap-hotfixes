use std::ffi::c_void;

use windows_sys::Win32::System::Diagnostics::Debug::FlushInstructionCache;
use windows_sys::Win32::System::Memory::{PAGE_EXECUTE_READWRITE, VirtualProtect};
use windows_sys::Win32::System::Threading::GetCurrentProcess;

use crate::relative_displacement;

use super::constants::PATCH_INJECTION_BYTES;

const JMP: u8 = 0xE9;
const NOP: u8 = 0x90;

/// Replaces
/// ```
///     push edi
///     push 1
///     lea edx,dword ptr ss:[esp+14]
/// ```
/// at `injection_address` with a
/// ```
///     jmp patch_address
///     nop
/// ```
/// to run patch for silent voice lines
///
/// It first prepares detour byte sequence, then:
/// - changes memory page protection to `PAGE_EXECUTE_READWRITE`
/// - writes detour into game memory
/// - restores original page protection
/// - flushes CPU instruction cache
pub fn detour(injection_address: usize, patch_address: usize) -> Result<(), &'static str> {
    let injection_pointer = injection_address as *mut u8;
    let mut detour = [NOP; PATCH_INJECTION_BYTES.len()];
    detour[0] = JMP;
    let detour_displacement = relative_displacement(injection_address + 5, patch_address)
        .ok_or("voice-line patch branch is out of range")?;
    detour[1..5].copy_from_slice(&detour_displacement.to_le_bytes());

    let mut previous_protection = 0;
    let changed = unsafe {
        VirtualProtect(
            injection_pointer.cast::<c_void>(),
            detour.len(),
            PAGE_EXECUTE_READWRITE,
            &mut previous_protection,
        )
    };
    if changed == 0 {
        return Err("VirtualProtect failed at the voice-line patch site");
    }

    unsafe {
        std::ptr::copy_nonoverlapping(detour.as_ptr(), injection_pointer, detour.len());
    }

    let mut ignored_protection = 0;
    let restored = unsafe {
        VirtualProtect(
            injection_pointer.cast::<c_void>(),
            detour.len(),
            previous_protection,
            &mut ignored_protection,
        )
    };
    if restored == 0 {
        return Err("unable to restore voice-line patch-site memory protection");
    }

    let flushed = unsafe {
        FlushInstructionCache(
            GetCurrentProcess(),
            injection_pointer.cast::<c_void>(),
            detour.len(),
        )
    };
    if flushed == 0 {
        return Err("FlushInstructionCache failed at the voice-line patch site");
    }

    Ok(())
}
