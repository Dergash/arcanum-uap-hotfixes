// Injection point is in `ai_standpoints_process`, after entering `if (dist <= wandering_range) branch`
// See https://github.com/alexbatalov/arcanum-ce/blob/a7ff41b300ef712f0e7d088183a3d08957110cdd/src/game/ai.c#L2728
pub const PATCH_INJECTION_RVA: usize = 0x000ACEC8;

// test ebp,200
// jne arcanum.4ACF07
pub const PATCH_INJECTION_BYTES: [u8; 8] = [0xF7, 0xC5, 0x00, 0x02, 0x00, 0x00, 0x75, 0x37];

// test ebp,200
pub const ASM_TEST_NPC_BED_WP_FLAG: [u8; 6] = [0xF7, 0xC5, 0x00, 0x02, 0x00, 0x00];

// refer to patch.asm
pub const PATCH_SIZE: usize = 54;

// Start of the original `is_sleeping` check and ai_wake_up(obj) call
// See https://github.com/alexbatalov/arcanum-ce/blob/a7ff41b300ef712f0e7d088183a3d08957110cdd/src/game/ai.c#L2755
pub const WAKE_UP_RVA: usize = 0x000ACF6D;

// `ai_find_nearest_bed`
// See https://github.com/alexbatalov/arcanum-ce/blob/a7ff41b300ef712f0e7d088183a3d08957110cdd/src/game/ai.c#L2804
pub const FIND_NEAREST_BED_RVA: usize = 0x000AD140;
