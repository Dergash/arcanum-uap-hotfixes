use super::constants::{ASM_TEST_NPC_BED_WP_FLAG, PATCH_SIZE, WAKE_UP_RVA};
use crate::runtime::Runtime;
use crate::stuck_sleeping::relative_displacement;

/// Prepares patch to fix the sleeping bug
///
/// The patch is:
/// - check `is_sleeping` flag on NPC
/// - if day `hour` (TODO), 
/// - jump to a branch where game supposed to wake them up (`ai_wake_up(obj)`)
///     - see https://github.com/alexbatalov/arcanum-ce/blob/a7ff41b300ef712f0e7d088183a3d08957110cdd/src/game/ai.c#L2755
/// - otherwise execute original instruction that we were hijacking (`test ebp,200`) and jump to original flow
pub fn prepare_patch_code(
    patch_address: usize,
    injection_address: usize,
    runtime: &Runtime,
) -> Result<Vec<u8>, &'static str> {
    let mut patch_code = Vec::with_capacity(PATCH_SIZE);

    // Compare the saved `is_sleeping` value with zero.
    patch_code.extend_from_slice(&[0x83, 0x7C, 0x24, 0x10, 0x00]);

    // `jne arcanum.4ACF6D`: take the original wake-up path when sleeping.
    let wake_instruction_end = patch_address + 11;
    let wake_displacement =
        relative_displacement(wake_instruction_end, runtime.game_address(WAKE_UP_RVA))
            .ok_or("wake-up branch is out of range")?;
    patch_code.extend_from_slice(&[0x0F, 0x85]);
    patch_code.extend_from_slice(&wake_displacement.to_le_bytes());

    // If not sleeping, replay the original instructions displaced at the patch site.
    patch_code.extend_from_slice(&ASM_TEST_NPC_BED_WP_FLAG);

    // Resume execution immediately after the displaced instructions.
    patch_code.push(0xE9);
    let return_instruction_end = patch_address + PATCH_SIZE;
    let return_displacement = relative_displacement(
        return_instruction_end,
        injection_address + ASM_TEST_NPC_BED_WP_FLAG.len(),
    )
    .ok_or("trampoline return branch is out of range")?;
    patch_code.extend_from_slice(&return_displacement.to_le_bytes());

    debug_assert_eq!(patch_code.len(), PATCH_SIZE);
    Ok(patch_code)
}
