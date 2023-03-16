use std::fmt;

use crate::board_util::{
    u16_to_string,
};
use crate::block::Block;

pub struct Board {
    state: [u16; 20],
    walls: u16,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for (i, &line) in self.state.iter().enumerate().rev() {
            s.push_str(&u16_to_string(line)[..]);
            if i != 0 { s.push_str("\n"); }
        }
        write!(f, "{}", s)
    }
}

impl Board {
    pub fn new() -> Self {
        Self {
            state: [0x2004; 20],
            walls: 0x2004,
        }
    }

    pub fn is_feasible(&self, idx: usize, block: Block) -> bool {
        let lines = block.config();
        if idx < 0 || idx >= self.state.len() { return false }
        return self.state[idx] & lines[0] != 0 ||
            self.state[idx+1] & lines[1] != 0||
            self.state[idx+2] & lines[2] != 0||
            self.state[idx+3] & lines[3] != 0
    }

    pub fn add_block(&mut self, idx: usize, block: Block) {
        let lines = block.config();
        self.state[idx] |= lines[0];
        self.state[idx+1] |= lines[1];
        self.state[idx+2] |= lines[2];
        self.state[idx+3] |= lines[3];
    }
}
