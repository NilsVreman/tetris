use std::fmt;

use crate::game_util::{
    u16_to_string,
    WALLS,
    FULL_LINE,
    FAIL_HEIGHT,
    BOARD_HEIGHT,
};
use crate::block::Block;

pub enum BoardStatus {
    Overflow(usize),
    Okay(usize),
}

pub struct Board {
    state: [u16; BOARD_HEIGHT],  // NOTE: We assume index [0] is bottom of board
    walls: u16,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for &line in self.state.iter().rev().skip(BOARD_HEIGHT.checked_sub(FAIL_HEIGHT+1).unwrap()) {
            s.push_str(&u16_to_string(line)[..]);
            s.push_str("\n");
        }
        s.push_str(&format!("  {}  ", "#".repeat(12))[..]);
        write!(f, "{}", s)
    }
}

impl Board {
    pub fn new() -> Self {
        Self {
            state: [WALLS; BOARD_HEIGHT],
            walls: WALLS,
        }
    }

    /// Checks whether a block can be added to the board
    ///
    /// idx: Lowest line to check from
    /// block: The block to check
    ///
    /// returns feasibility certificate
    pub fn is_feasible(&self, idx: usize, block: &Block) -> bool {
        let lines = block.config();
        if idx+3 >= BOARD_HEIGHT { return false }
        return self.state[idx] & lines[0] == 0 &&
            self.state[idx+1] & lines[1] == 0 &&
            self.state[idx+2] & lines[2] == 0 &&
            self.state[idx+3] & lines[3] == 0
    }

    /// Adds a block to the board and returns how many lines were cleared
    ///
    /// idx: Lowest line to add from
    /// block: The block to add
    ///
    /// returns an Option: Some(the number of lines to clear), or None if we have failed
    pub fn add_block(&mut self, idx: usize, block: &Block) -> BoardStatus {
        let lines = block.config();
        let mut n = 0;
        for i in 0..4 {
            self.state[idx+i] |= lines[i];
            n += if self.line_is_full(idx+i) { 1 } else { 0 };
        }
        return if self.state[FAIL_HEIGHT] == WALLS { BoardStatus::Okay(n) } else { BoardStatus::Overflow(n) } 
    }

    // Checks whether line is full or not
    fn line_is_full(&self, idx: usize) -> bool {
        self.state[idx] & FULL_LINE == self.state[idx]
    }

    // Clears line at idx and moves all other lines downwards
    pub fn clear_line(&mut self, idx: usize) {
        for i in idx..FAIL_HEIGHT-1 {
            self.state[i] = self.state[i+1];
        }
        self.state[FAIL_HEIGHT - 1] = WALLS;
    }
}
