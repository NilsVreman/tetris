use std::fmt;

use crate::board_utils::u16_to_string;

pub struct Board<'a> {
    state: &'a [u16],
    walls: u16,
}

impl<'a> fmt::Display for Board<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for (i, &line) in self.state.iter().enumerate().rev() {
            s.push_str(&u16_to_string(line)[..]);
            if i != 0 { s.push_str("\n"); }
        }
        write!(f, "{}", s)
    }
}

impl<'a> Board<'a> {
    pub fn new() -> Self {
        Self {
            state: &[0x2004; 20],
            walls: 0x2004,
        }
    }
}
