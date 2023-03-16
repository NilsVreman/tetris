use std::fmt;

use crate::board_util::u16_to_string;

#[derive(Copy, Clone)]
enum BlockType {
    I, J, L, O, S, T, Z,
}

#[derive(Copy, Clone)]
pub struct Block {
    lines: [u16; 4],  // Definitely possible to do this with u64, but a bit more convoluted
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
                BlockType::I => [0x03C0, 0x0000, 0x0000, 0x0000],
                BlockType::J => [0x0380, 0x0200, 0x0000, 0x0000],
                BlockType::L => [0x0380, 0x0080, 0x0000, 0x0000],
                BlockType::O => [0x0180, 0x0180, 0x0000, 0x0000],
                BlockType::S => [0x0300, 0x0180, 0x0000, 0x0000],
                BlockType::T => [0x0380, 0x0100, 0x0000, 0x0000],
                BlockType::Z => [0x0180, 0x0300, 0x0000, 0x0000],
            },
            blocktype: *bt
        }
    }

    pub fn config(&self) -> &[u16] {
        &self.lines
    }
}

pub struct BlockGenerator<'a> {
    blocktypes: &'a [BlockType],
    nextidx: usize,
    thisidx: usize,
}

impl<'a> BlockGenerator<'a> {
    pub fn new() -> Self {
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
            nextidx: 1,
            thisidx: 0,
        }
    }
}

impl<'a> Iterator for BlockGenerator<'a> {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        if self.blocktypes.len() <= self.thisidx {
            return None;
        }
        let ele = Block::new(&self.blocktypes[self.thisidx]);
        self.thisidx = self.nextidx;
        self.nextidx += 1;
        return Some(ele);
    }
}
