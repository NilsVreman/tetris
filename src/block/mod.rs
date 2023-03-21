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

////////////////
// Main Trait //
////////////////

pub trait Rotate {
    fn rotate(&mut self, cmd: &RotateCmd);
}

pub trait Shift {
    fn shift(&mut self, cmd: &ShiftCmd);
}

pub trait Config {
    fn config(&self) -> Vec<u16>;
}

pub trait Block: Rotate + Shift + Config + fmt::Display {}

///////////////
// BlockData //
///////////////

#[derive(Copy, Clone)]
struct BlockData {
    lines: [u16; 4],  // Definitely possible to do this with u64, but a bit more convoluted
    shifts: i8,
    rotation: BlockRotation,
}

impl BlockData {
    pub fn new(lines: [u16; 4]) -> Self {
        Self { lines, shifts: 0, rotation: BlockRotation::R1 }
    }

    pub fn config(&self) -> Vec<u16> {
        self.lines.iter()
            .map(|line| match self.shifts {
                x if x >= 0 => line << self.shifts,
                _ => line >> self.shifts.abs(),
            }).collect()
    }

    pub fn shift(&mut self, cmd: &ShiftCmd) {
        self.shifts += match cmd {
            ShiftCmd::Left => 1,
            ShiftCmd::Right => -1,
        };
    }
}

impl fmt::Display for BlockData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}\n{}\n{}",
               u16_to_string(self.lines[3]),
               u16_to_string(self.lines[2]),
               u16_to_string(self.lines[1]),
               u16_to_string(self.lines[0]))
    }
}

////////////
// BlockI //
////////////

#[derive(Copy, Clone)]
struct BlockI {
    data: BlockData,
}

impl BlockI {
    pub fn new() -> Self {
        Self { data: BlockData::new([0x0000, 0x0000, 0x03C0, 0x0000]) }
    }
}

impl Block for BlockI {}

impl Shift for BlockI {
    fn shift(&mut self, cmd: &ShiftCmd) { self.data.shift(&cmd); }
}

impl Config for BlockI {
    fn config(&self) -> Vec<u16> { self.data.config() }
}

impl Rotate for BlockI {
    fn rotate(&mut self, cmd: &RotateCmd) {
        (self.data.lines, self.data.rotation) = match (self.data.rotation, cmd) {
            (BlockRotation::R2, RotateCmd::Left) | (BlockRotation::R4, RotateCmd::Right) => {
                ([0x0000, 0x0000, 0x03C0, 0x0000],
                 BlockRotation::R1)
            },
            (BlockRotation::R3, RotateCmd::Left) | (BlockRotation::R1, RotateCmd::Right) => {
                ([0x0080, 0x0080, 0x0080, 0x0080],
                 BlockRotation::R2)
            },
            (BlockRotation::R4, RotateCmd::Left) | (BlockRotation::R2, RotateCmd::Right) => {
                ([0x0000, 0x03C0, 0x0000, 0x0000],
                 BlockRotation::R3)
            },
            (BlockRotation::R1, RotateCmd::Left) | (BlockRotation::R3, RotateCmd::Right) => {
                ([0x0100, 0x0100, 0x0100, 0x0100],
                 BlockRotation::R4)
            },
        };
    }
}

impl fmt::Display for BlockI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.data) }
}

////////////
// BlockJ //
////////////

#[derive(Copy, Clone)]
struct BlockJ {
    data: BlockData,
}

impl BlockJ {
    pub fn new() -> Self {
        Self { data: BlockData::new([0x0000, 0x0380, 0x0200, 0x0000]) }
    }
}

impl Block for BlockJ {}

impl Shift for BlockJ {
    fn shift(&mut self, cmd: &ShiftCmd) { self.data.shift(&cmd); }
}

impl Config for BlockJ {
    fn config(&self) -> Vec<u16> { self.data.config() }
}

impl Rotate for BlockJ {
    fn rotate(&mut self, cmd: &RotateCmd) { 
        (self.data.lines, self.data.rotation) = match (self.data.rotation, cmd) {
            (BlockRotation::R2, RotateCmd::Left) | (BlockRotation::R4, RotateCmd::Right) => {
                ([0x0000, 0x0380, 0x0200, 0x0000],
                 BlockRotation::R1)
            },
            (BlockRotation::R3, RotateCmd::Left) | (BlockRotation::R1, RotateCmd::Right) => {
                ([0x0100, 0x0100, 0x0180, 0x0000],
                 BlockRotation::R2)
            },
            (BlockRotation::R4, RotateCmd::Left) | (BlockRotation::R2, RotateCmd::Right) => {
                ([0x0080, 0x0380, 0x0000, 0x0000],
                 BlockRotation::R3)
            },
            (BlockRotation::R1, RotateCmd::Left) | (BlockRotation::R3, RotateCmd::Right) => {
                ([0x0300, 0x0100, 0x0100, 0x0000],
                 BlockRotation::R4)
            },
        };
    }
}

impl fmt::Display for BlockJ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.data) }
}

////////////
// BlockL //
////////////

#[derive(Copy, Clone)]
struct BlockL {
    data: BlockData,
}

impl BlockL {
    pub fn new() -> Self {
        Self { data: BlockData::new([0x0000, 0x0380, 0x0080, 0x0000]) }
    }
}

impl Block for BlockL {}

impl Shift for BlockL {
    fn shift(&mut self, cmd: &ShiftCmd) { self.data.shift(&cmd); }
}

impl Config for BlockL {
    fn config(&self) -> Vec<u16> { self.data.config() }
}

impl Rotate for BlockL {
    fn rotate(&mut self, cmd: &RotateCmd) { 
        (self.data.lines, self.data.rotation) = match (self.data.rotation, cmd) {
            (BlockRotation::R2, RotateCmd::Left) | (BlockRotation::R4, RotateCmd::Right) => {
                ([0x0000, 0x0380, 0x0080, 0x0000],
                 BlockRotation::R1)
            },
            (BlockRotation::R3, RotateCmd::Left) | (BlockRotation::R1, RotateCmd::Right) => {
                ([0x0180, 0x0100, 0x0100, 0x0000],
                 BlockRotation::R2)
            },
            (BlockRotation::R4, RotateCmd::Left) | (BlockRotation::R2, RotateCmd::Right) => {
                ([0x0200, 0x0380, 0x0000, 0x0000],
                 BlockRotation::R3)
            },
            (BlockRotation::R1, RotateCmd::Left) | (BlockRotation::R3, RotateCmd::Right) => {
                ([0x0100, 0x0100, 0x0300, 0x0000],
                 BlockRotation::R4)
            },
        };
    }
}

impl fmt::Display for BlockL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.data) }
}

////////////
// BlockO //
////////////

#[derive(Copy, Clone)]
struct BlockO {
    data: BlockData,
}

impl BlockO {
    pub fn new() -> Self {
        Self { data: BlockData::new([0x0000, 0x0180, 0x0180, 0x0000]) }
    }
}

impl Block for BlockO {}

impl Shift for BlockO {
    fn shift(&mut self, cmd: &ShiftCmd) { self.data.shift(&cmd); }
}

impl Config for BlockO {
    fn config(&self) -> Vec<u16> { self.data.config() }
}

impl Rotate for BlockO {
    fn rotate(&mut self, cmd: &RotateCmd) {}
}

impl fmt::Display for BlockO {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.data) }
}

////////////
// BlockS //
////////////

#[derive(Copy, Clone)]
struct BlockS {
    data: BlockData,
}

impl BlockS {
    pub fn new() -> Self {
        Self { data: BlockData::new([0x0000, 0x0300, 0x0180, 0x0000]) }
    }
}

impl Block for BlockS {}

impl Shift for BlockS {
    fn shift(&mut self, cmd: &ShiftCmd) { self.data.shift(&cmd); }
}

impl Config for BlockS {
    fn config(&self) -> Vec<u16> { self.data.config() }
}

impl Rotate for BlockS {
    fn rotate(&mut self, cmd: &RotateCmd) { 
        (self.data.lines, self.data.rotation) = match (self.data.rotation, cmd) {
            (BlockRotation::R2, RotateCmd::Left) | (BlockRotation::R4, RotateCmd::Right) => {
                ([0x0000, 0x0300, 0x0180, 0x0000],
                 BlockRotation::R1)
            },
            (BlockRotation::R3, RotateCmd::Left) | (BlockRotation::R1, RotateCmd::Right) => {
                ([0x0080, 0x0180, 0x0100, 0x0000],
                 BlockRotation::R2)
            },
            (BlockRotation::R4, RotateCmd::Left) | (BlockRotation::R2, RotateCmd::Right) => {
                ([0x0300, 0x0180, 0x0000, 0x0000],
                 BlockRotation::R3)
            },
            (BlockRotation::R1, RotateCmd::Left) | (BlockRotation::R3, RotateCmd::Right) => {
                ([0x0100, 0x0300, 0x0200, 0x0000],
                 BlockRotation::R4)
            },
        };
    }
}

impl fmt::Display for BlockS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.data) }
}

////////////
// BlockT //
////////////

#[derive(Copy, Clone)]
struct BlockT {
    data: BlockData,
}

impl BlockT {
    pub fn new() -> Self {
        Self { data: BlockData::new([0x0000, 0x0380, 0x0100, 0x0000]) }
    }
}

impl Block for BlockT {}

impl Shift for BlockT {
    fn shift(&mut self, cmd: &ShiftCmd) { self.data.shift(&cmd); }
}

impl Config for BlockT {
    fn config(&self) -> Vec<u16> { self.data.config() }
}

impl Rotate for BlockT {
    fn rotate(&mut self, cmd: &RotateCmd) { 
        (self.data.lines, self.data.rotation) = match (self.data.rotation, cmd) {
            (BlockRotation::R2, RotateCmd::Left) | (BlockRotation::R4, RotateCmd::Right) => {
                ([0x0000, 0x0380, 0x0100, 0x0000],
                 BlockRotation::R1)
            },
            (BlockRotation::R3, RotateCmd::Left) | (BlockRotation::R1, RotateCmd::Right) => {
                ([0x0100, 0x0180, 0x0100, 0x0000],
                 BlockRotation::R2)
            },
            (BlockRotation::R4, RotateCmd::Left) | (BlockRotation::R2, RotateCmd::Right) => {
                ([0x0100, 0x0380, 0x0000, 0x0000],
                 BlockRotation::R3)
            },
            (BlockRotation::R1, RotateCmd::Left) | (BlockRotation::R3, RotateCmd::Right) => {
                ([0x0100, 0x0300, 0x0100, 0x0000],
                 BlockRotation::R4)
            },
        };
    }
}

impl fmt::Display for BlockT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.data) }
}

////////////
// BlockZ //
////////////

#[derive(Copy, Clone)]
struct BlockZ {
    data: BlockData,
}

impl BlockZ {
    pub fn new() -> Self {
        Self { data: BlockData::new([0x0000, 0x0180, 0x0300, 0x0000]) }
    }
}

impl Block for BlockZ {}

impl Shift for BlockZ {
    fn shift(&mut self, cmd: &ShiftCmd) { self.data.shift(&cmd); }
}

impl Config for BlockZ {
    fn config(&self) -> Vec<u16> { self.data.config() }
}

impl Rotate for BlockZ {
    fn rotate(&mut self, cmd: &RotateCmd) { 
        (self.data.lines, self.data.rotation) = match (self.data.rotation, cmd) {
            (BlockRotation::R2, RotateCmd::Left) | (BlockRotation::R4, RotateCmd::Right) => {
                ([0x0000, 0x0180, 0x0300, 0x0000],
                 BlockRotation::R1)
            },
            (BlockRotation::R3, RotateCmd::Left) | (BlockRotation::R1, RotateCmd::Right) => {
                ([0x0100, 0x0180, 0x0080, 0x0000],
                 BlockRotation::R2)
            },
            (BlockRotation::R4, RotateCmd::Left) | (BlockRotation::R2, RotateCmd::Right) => {
                ([0x0180, 0x0300, 0x0000, 0x0000],
                 BlockRotation::R3)
            },
            (BlockRotation::R1, RotateCmd::Left) | (BlockRotation::R3, RotateCmd::Right) => {
                ([0x0200, 0x0300, 0x0100, 0x0000],
                 BlockRotation::R4)
            },
        };
    }
}

impl fmt::Display for BlockZ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.data) }
}
