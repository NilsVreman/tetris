// Function to convert u16 to bitstring
pub fn u16_to_string(u: u16) -> String {
    (0..16).rev()
        .map(|i| if u & (1 << i) != 0 { '#' } else { ' ' })
        .collect()
}

// Constants for game size and function
pub const WALLS: u16 = 0x2004;          // 0010 0000 0000 0100
pub const FULL_LINE: u16 = 0x3FFC;      // 0011 1111 1111 1100
pub const BOARD_HEIGHT: usize = 24;
pub const FAIL_HEIGHT: usize = 20;
pub const UPDATE_FREQ: u64 = 5;
pub const MAX_GRAVITY: f32 = 8.0;

// Job struct
pub type Job = Box<dyn Fn() + Send + 'static>;
