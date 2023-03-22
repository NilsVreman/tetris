// Constants for game size and function
pub const WALLS: u16 = 0xE007;                // 1110 0000 0000 0111
pub const FLOOR: u16 = 0x3FFC;                // 0011 1111 1111 1100
pub const BOARD_FILLED_LINE: u16 = 0xFFFF;    // 1111 1111 1111 1111
pub const BOARD_LINE_FLOOR: usize = 3;
pub const BOARD_LINE_HEIGHT: usize = 27;
pub const BOARD_LINE_FAIL: usize = 23;
pub const BLOCK_HEIGHT: usize = 4;
pub const UPDATE_FREQ: u64 = 5;
pub const MAX_GRAVITY: f32 = 20.0;
