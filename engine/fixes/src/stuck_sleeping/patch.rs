use super::constants::{ASM_TEST_NPC_BED_WP_FLAG, IS_DAY_ADDRESS, PATCH_SIZE, WAKE_UP_RVA};
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
const CMP_EAX: [u8; 2] = [0x83, 0xF8];

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

    // `ai_is_day` modifies EAX so we'll need to restore its original value later
    patch_code.extend_from_slice(&SAVE_REGISTERS);
    // call `ai_is_day` (arcanum.4ACD90)
    patch_code.extend_from_slice(&[CALL]);
    // 8 = 3 for pushing eax, ecx and edx and 5 for `call``
    let day_check_displacement =
        relative_displacement(patch_address + 8, runtime.game_address(IS_DAY_ADDRESS))
            .ok_or("test")?;
    patch_code.extend_from_slice(&day_check_displacement.to_le_bytes());
    // check is_day
    patch_code.extend_from_slice(&CMP_EAX);
    patch_code.extend_from_slice(&[0x01]);
    // if not, jump to exit flow without waking
    patch_code.extend_from_slice(&[JNE_SHORT, 0x0F]);
    // if day, check `is_sleeping` flag on NPC ([ESP+1C]) against 0
    patch_code.extend_from_slice(&[0x83, 0x7C, 0x24, 0x1C, 0x00]);
    // if not sleeping, jump to exit flow without waking
    patch_code.extend_from_slice(&[JE_SHORT, 0x08]);

    // exit flow with waking up:

    // if sleeping, then first restore edx,ecx,eax to how they were before day check...
    patch_code.extend_from_slice(&RESTORE_REGISTERS);
    // ...and then exit from patch to waking up procedure `arcanum.4ACF6D`
    patch_code.extend_from_slice(&[JMP]);
    // 28 = length of all the previous instructions
    let wake_displacement =
        relative_displacement(patch_address + 28, runtime.game_address(WAKE_UP_RVA))
            .ok_or("wake-up branch is out of range")?;
    patch_code.extend_from_slice(&wake_displacement.to_le_bytes());

    // exit flow without waking up:

    // restore original edx,ecx,eax
    patch_code.extend_from_slice(&RESTORE_REGISTERS);
    // replay the original instructions displaced by the detour to our patch
    patch_code.extend_from_slice(&ASM_TEST_NPC_BED_WP_FLAG);
    // Resume execution immediately after the displaced instructions.
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
