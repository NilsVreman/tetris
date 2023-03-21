mod block_generator;
pub use crate::block::block_generator::BlockGenerator;

use std::fmt;

use crate::{
    game_util::{
        u16_to_string,
        ShiftCmd,
        RotateCmd,
    },
};

#[derive(Copy, Clone)]
enum BlockRotation {
    R1, R2, R3, R4,
}

#[derive(Copy, Clone)]
enum BlockType {
    I, J, L, O, S, T, Z,
}

///////////
// Block //
///////////

#[derive(Copy, Clone)]
pub struct Block {
    lines: [u16; 4],  // Definitely possible to do this with u64, but a bit more convoluted
    shifts: i8,
    rotation: BlockRotation,
    blocktype: BlockType,
}

impl Block {
    fn new(blocktype: BlockType) -> Self {
        Self {
            lines: match blocktype {
                BlockType::I => [0x0000, 0x0000, 0x03C0, 0x0000],
                BlockType::J => [0x0000, 0x0380, 0x0200, 0x0000],
                BlockType::L => [0x0000, 0x0380, 0x0080, 0x0000],
                BlockType::O => [0x0000, 0x0180, 0x0180, 0x0000],
                BlockType::S => [0x0000, 0x0300, 0x0180, 0x0000],
                BlockType::T => [0x0000, 0x0380, 0x0100, 0x0000],
                BlockType::Z => [0x0000, 0x0180, 0x0300, 0x0000],
            },
            shifts: 0,
            rotation: BlockRotation::R1,
            blocktype,
        }
    }

    /// todo!()
    pub fn config(&self) -> Vec<u16> {
        self.lines.iter()
            .map(|line| match self.shifts {
                x if x >= 0 => line << self.shifts,
                _ => line >> self.shifts.abs(),
            }).collect()
    }

    /// todo!()
    pub fn shift(&mut self, cmd: &ShiftCmd) {
        self.shifts += match cmd {
            ShiftCmd::Left => 1,
            ShiftCmd::Right => -1,
        };
    }

    /// todo!()
    pub fn rotate(&mut self, cmd: &RotateCmd) {
        self.rotation = match (self.rotation, cmd) {
            (BlockRotation::R2, RotateCmd::Left) | (BlockRotation::R4, RotateCmd::Right) => BlockRotation::R1,
            (BlockRotation::R3, RotateCmd::Left) | (BlockRotation::R1, RotateCmd::Right) => BlockRotation::R2,
            (BlockRotation::R4, RotateCmd::Left) | (BlockRotation::R2, RotateCmd::Right) => BlockRotation::R3,
            (BlockRotation::R1, RotateCmd::Left) | (BlockRotation::R3, RotateCmd::Right) => BlockRotation::R4,
        };
        self.lines = match self.blocktype {
            BlockType::I => match self.rotation {
                BlockRotation::R1 => [0x0000, 0x0000, 0x03C0, 0x0000],
                BlockRotation::R2 => [0x0080, 0x0080, 0x0080, 0x0080],
                BlockRotation::R3 => [0x0000, 0x03C0, 0x0000, 0x0000],
                BlockRotation::R4 => [0x0100, 0x0100, 0x0100, 0x0100],
            },
            BlockType::J => match self.rotation {
                BlockRotation::R1 => [0x0000, 0x0380, 0x0200, 0x0000],
                BlockRotation::R2 => [0x0100, 0x0100, 0x0180, 0x0000],
                BlockRotation::R3 => [0x0080, 0x0380, 0x0000, 0x0000],
                BlockRotation::R4 => [0x0300, 0x0100, 0x0100, 0x0000],
            },
            BlockType::L => match self.rotation {
                BlockRotation::R1 => [0x0000, 0x0380, 0x0080, 0x0000],
                BlockRotation::R2 => [0x0180, 0x0100, 0x0100, 0x0000],
                BlockRotation::R3 => [0x0200, 0x0380, 0x0000, 0x0000],
                BlockRotation::R4 => [0x0100, 0x0100, 0x0300, 0x0000],
            },
            BlockType::O => self.lines,
            BlockType::S => match self.rotation {
                BlockRotation::R1 => [0x0000, 0x0300, 0x0180, 0x0000],
                BlockRotation::R2 => [0x0080, 0x0180, 0x0100, 0x0000],
                BlockRotation::R3 => [0x0300, 0x0180, 0x0000, 0x0000],
                BlockRotation::R4 => [0x0100, 0x0300, 0x0200, 0x0000],
            },
            BlockType::T => match self.rotation {
                BlockRotation::R1 => [0x0000, 0x0380, 0x0100, 0x0000],
                BlockRotation::R2 => [0x0100, 0x0180, 0x0100, 0x0000],
                BlockRotation::R3 => [0x0100, 0x0380, 0x0000, 0x0000],
                BlockRotation::R4 => [0x0100, 0x0300, 0x0100, 0x0000],
            },
            BlockType::Z => match self.rotation {
                BlockRotation::R1 => [0x0000, 0x0180, 0x0300, 0x0000],
                BlockRotation::R2 => [0x0100, 0x0180, 0x0080, 0x0000],
                BlockRotation::R3 => [0x0180, 0x0300, 0x0000, 0x0000],
                BlockRotation::R4 => [0x0200, 0x0300, 0x0100, 0x0000],
            },
        };
    }
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
