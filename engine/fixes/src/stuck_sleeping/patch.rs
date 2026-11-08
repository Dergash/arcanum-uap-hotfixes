use super::constants::{ASM_TEST_NPC_BED_WP_FLAG, FIND_NEAREST_BED_RVA, PATCH_SIZE, WAKE_UP_RVA};
use crate::runtime::Runtime;
use crate::stuck_sleeping::relative_displacement;

// assembly instructions
const CALL: u8 = 0xE8;
// push EAX, push ECX, push EDX
const SAVE_REGISTERS: [u8; 3] = [0x50, 0x51, 0x52];
// pop EDX, pop ECX, pop EAX
const RESTORE_REGISTERS: [u8; 3] = [0x5A, 0x59, 0x58];
const JNE_SHORT: u8 = 0x75;
const JE_SHORT: u8 = 0x74;
const JMP: u8 = 0xE9;

/// Prepares patch to fix the sleeping bug
///
/// The patch is:
/// - if the NPC is sleeping, check for a bed at its current standpoint
/// - wake it if no bed exists at that standpoint
/// - jump to a branch where game supposed to wake them up (`ai_wake_up(obj)`)
///     - see https://github.com/alexbatalov/arcanum-ce/blob/a7ff41b300ef712f0e7d088183a3d08957110cdd/src/game/ai.c#L2755
/// - otherwise execute original instruction that we were hijacking (`test ebp,200`) and jump to original flow
pub fn prepare_patch_code(
    patch_address: usize,
    injection_address: usize,
    runtime: &Runtime,
) -> Result<Vec<u8>, &'static str> {
    let mut patch_code = Vec::with_capacity(PATCH_SIZE);

    patch_code.extend_from_slice(&SAVE_REGISTERS);
    // The original `is_sleeping` local is at [ESP+0x10].
    // Saving three registers moves it to [ESP+0x1C].
    patch_code.extend_from_slice(&[0x83, 0x7C, 0x24, 0x1C, 0x00]);
    // If not sleeping, skip to the original flow.
    patch_code.extend_from_slice(&[JE_SHORT, 0x1E]);

    // standpoint_loc is at original [ESP+0x1C]/[ESP+0x20].
    // After saving registers and pushing the high dword, both values are at [ESP+0x2C].
    patch_code.extend_from_slice(&[0xFF, 0x74, 0x24, 0x2C]);
    patch_code.extend_from_slice(&[0xFF, 0x74, 0x24, 0x2C]);
    patch_code.push(CALL);
    let bed_check_displacement = relative_displacement(
        patch_address + 23,
        runtime.game_address(FIND_NEAREST_BED_RVA),
    )
    .ok_or("bed lookup branch is out of range")?;
    patch_code.extend_from_slice(&bed_check_displacement.to_le_bytes());
    // Discard the int64 location argument.
    patch_code.extend_from_slice(&[0x83, 0xC4, 0x08]);
    // OBJ_HANDLE_NULL is 0; a nonzero 64-bit handle means a bed exists.
    patch_code.extend_from_slice(&[0x09, 0xD0, 0x85, 0xC0]);
    // If a bed exists at this standpoint, keep the NPC asleep.
    patch_code.extend_from_slice(&[JNE_SHORT, 0x08]);

    // No bed exists and the NPC is sleeping: take the original wake-up path.
    patch_code.extend_from_slice(&RESTORE_REGISTERS);
    patch_code.extend_from_slice(&[JMP]);
    let wake_displacement =
        relative_displacement(patch_address + 40, runtime.game_address(WAKE_UP_RVA))
            .ok_or("wake-up branch is out of range")?;
    patch_code.extend_from_slice(&wake_displacement.to_le_bytes());

    // Not sleeping or a bed exists: restore registers and resume original flow.
    patch_code.extend_from_slice(&RESTORE_REGISTERS);
    patch_code.extend_from_slice(&ASM_TEST_NPC_BED_WP_FLAG);
    patch_code.push(JMP);
    let return_instruction_end = patch_address + PATCH_SIZE;
    let return_displacement = relative_displacement(
        return_instruction_end,
        injection_address + ASM_TEST_NPC_BED_WP_FLAG.len(),
    )
    .ok_or("patch return branch is out of range")?;
    patch_code.extend_from_slice(&return_displacement.to_le_bytes());

    debug_assert_eq!(patch_code.len(), PATCH_SIZE);
    Ok(patch_code)
}
