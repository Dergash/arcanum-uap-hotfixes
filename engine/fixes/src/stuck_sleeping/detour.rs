use std::ffi::c_void;
use windows_sys::Win32::System::Diagnostics::Debug::FlushInstructionCache;
use windows_sys::Win32::System::Memory::{PAGE_EXECUTE_READWRITE, VirtualProtect};
use windows_sys::Win32::System::Threading::GetCurrentProcess;

use super::constants::ASM_TEST_NPC_BED_WP_FLAG;
use super::relative_displacement;

const JMP: u8 = 0xE9;

/// Replaces `test ebp,200` at `injection_address` with a
/// ```
///     jmp patch_address
///     nop
/// ```
/// to run patch for stuck sleeping instead of going into wandering/distance standpoint logic that vanilla game performs
///
/// It first prepares detour byte sequence, then:
/// - changes memory page protection to `PAGE_EXECUTE_READWRITE`
/// - writes detour into game memory
/// - restores original page protection
/// - flushes CPU instruction cache
pub fn detour(injection_address: usize, patch_address: usize) -> Result<(), &'static str> {
    let injection_pointer = injection_address as *mut u8;
    let mut detour = [0x90; ASM_TEST_NPC_BED_WP_FLAG.len()];
    detour[0] = JMP;
    let detour_displacement = relative_displacement(injection_address + 5, patch_address)
        .ok_or("patch branch is out of range")?;
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
        return Err("VirtualProtect failed at the patch site");
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
        return Err("unable to restore patch-site memory protection");
    }

    let flushed = unsafe {
        FlushInstructionCache(
            GetCurrentProcess(),
            injection_pointer.cast::<c_void>(),
            detour.len(),
        )
    };
    if flushed == 0 {
        return Err("FlushInstructionCache failed at the patch site");
    }

    Ok(())
}
