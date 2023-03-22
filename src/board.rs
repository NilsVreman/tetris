use std::fmt;

use super::consts::{
    WALLS,
    FLOOR,
    BOARD_FILLED_LINE,
    BOARD_LINE_FAIL,
    BOARD_LINE_HEIGHT,
    BOARD_LINE_FLOOR,
    BLOCK_HEIGHT,
};
use super::util::{
    u16_to_string,
    TetrisError,
};
use super::enums::BoardStatus;

pub struct Board {
    state: Vec<u16>,  // NOTE: We assume index [0] is bottom of board
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print_lines_on_board(|x| 0))
    }
}

impl Board {
    pub fn new() -> Self {
        let mut state = Vec::from([WALLS; BOARD_LINE_HEIGHT]);
        for i in 0..BOARD_LINE_FLOOR { state[i] = FLOOR; }
        Self { state }
    }

    /// todo!()
    pub fn config(&self, idx: usize) -> Result<Vec<u16>, TetrisError> {
        if idx+BLOCK_HEIGHT >= self.state.len() {
            return Err(TetrisError(format!("Can't access board at indices {:?}", idx..idx+BLOCK_HEIGHT)))
        }
        Ok(self.state[idx..idx+BLOCK_HEIGHT].to_vec())
    }

    /// Adds a block to the board and returns how many lines were cleared
    ///
    /// idx: Lowest line to add from
    /// block: The block to add
    ///
    /// returns an Option: Some(the number of lines to clear), or None if we have failed
    pub fn add_lines(&mut self, start_idx: usize, lines: &Vec<u16>) -> BoardStatus {
        let mut n = 0;
        for i in 0..lines.len() {
            self.state[start_idx+i] |= lines[i];
            n += if self.line_is_full(start_idx+i) { 1 } else { 0 };
        }
        return if self.state[BOARD_LINE_FAIL] == WALLS { BoardStatus::Okay(n) } else { BoardStatus::Overflow(n) } 
    }

    /// Prints the block with the corresponding state of the board
    pub fn print_lines_on_board<F>(&self, line_fn: F) -> String
    where
        F: Fn(usize) -> u16,
    {
        let mut s = String::new();
        for (i, &line) in self.state.iter()
                .enumerate()
                .skip(BOARD_LINE_FLOOR - 1)
                .rev()
                .skip(BOARD_LINE_HEIGHT - BOARD_LINE_FAIL - 1) {
            s.push_str( &u16_to_string( line | line_fn(i) )[..] );
            s.push_str("\n");
        }
        s
    }

    // Checks whether line is full or not
    fn line_is_full(&self, idx: usize) -> bool {
        self.state[idx] & BOARD_FILLED_LINE == self.state[idx]
    }

    // Clears line at idx and moves all other lines downwards
    pub fn clear_line(&mut self, idx: usize) {
        for i in idx..BOARD_LINE_FAIL-1 {
            self.state[i] = self.state[i+1];
        }
        self.state[BOARD_LINE_FAIL - 1] = WALLS;
    }
}
