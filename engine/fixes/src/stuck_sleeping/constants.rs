// Injection point is in `ai_standpoints_process`, after entering `if (dist <= wandering_range) branch`
// See https://github.com/alexbatalov/arcanum-ce/blob/a7ff41b300ef712f0e7d088183a3d08957110cdd/src/game/ai.c#L2728
pub const PATCH_INJECTION_RVA: usize = 0x000ACEC8;
// test ebp,200
// jne arcanum.4ACF07
pub const PATCH_INJECTION_BYTES: [u8; 8] = [0xF7, 0xC5, 0x00, 0x02, 0x00, 0x00, 0x75, 0x37];
// cmp dword ptr ss:[esp+10],0
pub const ASM_IS_SLEEPING: [u8; 5] = [0x83, 0x7C, 0x24, 0x10, 0x00];
// jne arcanum.4ACF6D
pub const ASM_JUMP_TO_WAKEUP: [u8; 6] = [0x0F, 0x85, 0xFF, 0xFF, 0xFF, 0xFF];
// test ebp,200
pub const ASM_TEST_NPC_BED_WP_FLAG: [u8; 6] = [0xF7, 0xC5, 0x00, 0x02, 0x00, 0x00];
// jmp arcanum.4ACECE
pub const ASM_JUMP_TO_ORIGINAL: [u8; 5] = [0xE9, 0xFF, 0xFF, 0xFF, 0xFF];

pub const PATCH_SIZE: usize = ASM_IS_SLEEPING.len()
    + ASM_JUMP_TO_WAKEUP.len()
    + ASM_TEST_NPC_BED_WP_FLAG.len()
    + ASM_JUMP_TO_ORIGINAL.len();

// Start of the original `is_sleeping` check and ai_wake_up(obj) call
// See https://github.com/alexbatalov/arcanum-ce/blob/a7ff41b300ef712f0e7d088183a3d08957110cdd/src/game/ai.c#L2755
pub const WAKE_UP_RVA: usize = 0x000ACF6D;
