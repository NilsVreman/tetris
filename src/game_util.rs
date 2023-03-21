// Function to convert u16 to bitstring
pub fn u16_to_string(u: u16) -> String {
    (0..16).rev()
        .map(|i| if u & (1 << i) != 0 { '#' } else { ' ' })
        .collect()
}

// Constants for game size and function
pub const WALLS: u16 = 0xE007;                // 1110 0000 0000 0111
pub const FLOOR: u16 = 0x3FFC;                // 0011 1111 1111 1100
pub const BOARD_FILLED_LINE: u16 = 0xFFFF;    // 1111 1111 1111 1111
pub const BOARD_LINE_FLOOR: usize = 2;
pub const BOARD_LINE_HEIGHT: usize = 26;
pub const BOARD_LINE_FAIL: usize = 22;
pub const BLOCK_HEIGHT: usize = 4;
pub const UPDATE_FREQ: u64 = 5;
pub const MAX_GRAVITY: f32 = 20.0;

// Job struct
pub type Job = Box<dyn Fn() + Send + 'static>;

// Command enums
pub enum ShiftCmd {
    Left,
    Right,
}

pub enum RotateCmd {
    Left,
    Right,
}

// Error type
pub struct TetrisError(pub String);
