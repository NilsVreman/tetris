use std::fmt;

use crate::{
    game_util::{
        u16_to_string,
        Shift,
        Rotate,
    },
};

#[derive(Copy, Clone)]
enum BlockType {
    I, J, L, O, S, T, Z,
}

#[derive(Copy, Clone)]
pub struct Block {
    lines: [u16; 4],  // Definitely possible to do this with u64, but a bit more convoluted
    shifts: i8,
    blocktype: BlockType,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}\n{}\n{}",
               u16_to_string(self.lines[3]),
               u16_to_string(self.lines[2]),
               u16_to_string(self.lines[1]),
               u16_to_string(self.lines[0]))
    }
}

impl Block {
    fn new(bt: &BlockType) -> Self {
        Self {
            lines: match bt {
                BlockType::I => [0x03C0, 0x0000, 0x0000, 0x0000],  // NOTE: index 0 is bottom row
                BlockType::J => [0x0380, 0x0200, 0x0000, 0x0000],  // NOTE: index 0 is bottom row
                BlockType::L => [0x0380, 0x0080, 0x0000, 0x0000],  // NOTE: index 0 is bottom row
                BlockType::O => [0x0180, 0x0180, 0x0000, 0x0000],  // NOTE: index 0 is bottom row
                BlockType::S => [0x0300, 0x0180, 0x0000, 0x0000],  // NOTE: index 0 is bottom row
                BlockType::T => [0x0380, 0x0100, 0x0000, 0x0000],  // NOTE: index 0 is bottom row
                BlockType::Z => [0x0180, 0x0300, 0x0000, 0x0000],  // NOTE: index 0 is bottom row
            },
            shifts: 0,
            blocktype: *bt,
        }
    }

    pub fn config(&self) -> Vec<u16> {
        match self.shifts {
            x if x < 0 => vec![
                self.lines[0] >> self.shifts as u8,
                self.lines[1] >> self.shifts as u8,
                self.lines[2] >> self.shifts as u8,
                self.lines[3] >> self.shifts as u8,
            ],
            x if x >= 0 => vec![
                self.lines[0] << self.shifts,
                self.lines[1] << self.shifts,
                self.lines[2] << self.shifts,
                self.lines[3] << self.shifts,
            ],
            _ => panic!("Shouldn't get here")
        }
    }

    pub fn shift(&mut self, cmd: &Shift) {
        self.shifts += match cmd {
            Shift::Left => 1,
            Shift::Right => -1,
        };
    }

    pub fn rotate(&mut self, cmd: &Rotate) {
    }
}

pub struct BlockGenerator<'a> {
    blocktypes: &'a [BlockType],
    fn_next: Box<dyn Fn(&usize) -> usize>,
    nextidx: usize,
    thisidx: usize,
}

impl<'a> BlockGenerator<'a> {
    /// return a blockgenerator which generates the next block based on rule from closure f
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(&usize) -> usize + 'static,
    {
        Self {
            blocktypes: &[
                BlockType::I,
                BlockType::J,
                BlockType::L,
                BlockType::O,
                BlockType::S,
                BlockType::T,
                BlockType::Z,
            ],
            thisidx: 0,
            nextidx: f(&0),
            fn_next: Box::new(f),
        }
    }

    pub fn peek_next(&self) -> Option<Block> {
        if self.nextidx >= self.blocktypes.len() {
            return None;
        }
        return Some(Block::new(&self.blocktypes[self.nextidx]));
    }
}

impl<'a> Iterator for BlockGenerator<'a> {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        if self.thisidx >= self.blocktypes.len() {
            return None;
        }
        let ele = Block::new(&self.blocktypes[self.thisidx]);
        self.thisidx = self.nextidx;
        self.nextidx = (self.fn_next)(&self.thisidx);
        return Some(ele);
    }
}
