use crate::relative_displacement;

use super::constants::{PATCH_INJECTION_BYTES, PATCH_SIZE};

// Game engine uses 32KB buffer for reading (1 << 15)
// See https://github.com/alexbatalov/arcanum-ce/blob/a7ff41b300ef712f0e7d088183a3d08957110cdd/first_party/tig/src/file.c#L1932
const BUFFER_SIZE: u32 = 0x8000;
// assembly instructions
const JMP: u8 = 0xE9;
const JNZ_SHORT: u8 = 0x75;
const RESTORE_REGISTERS: [u8; 3] = [0x5F, 0x5E, 0x5B];
const RET: u8 = 0xC3;

/// Prepares the patch for copying files whose size is an exact multiple of the buffer size.
///
/// The patch is:
/// - return true immediately when no bytes remain after the 32 KB copy loop
/// - otherwise replay the original instructions overwritten by the detour and resume the final read
pub fn prepare_patch_code(
    patch_address: usize,
    injection_address: usize,
) -> Result<Vec<u8>, &'static str> {
    let mut patch_code = Vec::with_capacity(PATCH_SIZE);

    // A zero remainder means the full file was copied by the 32 KB loop.
    patch_code.extend_from_slice(&[0x85, 0xF6, JNZ_SHORT]);
    let remainder_branch_offset = patch_code.len();
    patch_code.push(0);

    // Return true through the original function
    patch_code.extend_from_slice(&RESTORE_REGISTERS);
    // add esp, 8000
    patch_code.extend_from_slice(&[0x81, 0xC4]);
    patch_code.extend_from_slice(&BUFFER_SIZE.to_le_bytes());
    // mov eax, 1
    patch_code.extend_from_slice(&[0xB8, 0x01, 0x00, 0x00, 0x00]);
    patch_code.push(RET);

    let nonzero_remainder_offset = patch_code.len();
    let branch_end_offset = remainder_branch_offset + 1;
    let branch_displacement = nonzero_remainder_offset
        .checked_sub(branch_end_offset)
        .and_then(|offset| u8::try_from(offset).ok())
        .ok_or("voice-line remainder branch is out of range")?;
    patch_code[remainder_branch_offset] = branch_displacement;

    // Replay the instructions replaced at the patch site for nonzero remainders.
    patch_code.extend_from_slice(&PATCH_INJECTION_BYTES);
    patch_code.push(JMP);
    let return_instruction_end = patch_address + patch_code.len() + 4;
    let return_displacement = relative_displacement(
        return_instruction_end,
        injection_address + PATCH_INJECTION_BYTES.len(),
    )
    .ok_or("voice-line patch return branch is out of range")?;
    patch_code.extend_from_slice(&return_displacement.to_le_bytes());

    debug_assert_eq!(patch_code.len(), PATCH_SIZE);
    Ok(patch_code)
}
