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

////////////////////////
// Struct definitions //
////////////////////////

#[derive(Copy, Clone)]
pub struct BlockData {
    lines: [u16; 4],  // Definitely possible to do this with u64, but a bit more convoluted
    shifts: i8,
    rotation: BlockRotation,
}

#[derive(Copy, Clone)]
pub struct BlockI {
    data: BlockData,
}

#[derive(Copy, Clone)]
pub struct BlockJ {
    data: BlockData,
}

#[derive(Copy, Clone)]
pub struct BlockL {
    data: BlockData,
}

#[derive(Copy, Clone)]
pub struct BlockO {
    data: BlockData,
}

#[derive(Copy, Clone)]
pub struct BlockS {
    data: BlockData,
}

#[derive(Copy, Clone)]
pub struct BlockT {
    data: BlockData,
}

#[derive(Copy, Clone)]
pub struct BlockZ {
    data: BlockData,
}

/////////////////////
// Implementations //
/////////////////////

pub trait Rotate {
    fn rotate(&mut self, cmd: &RotateCmd);
}

pub trait Shift {
    fn shift(&mut self, cmd: &ShiftCmd);
}

pub trait Config {
    fn config(&self) -> Vec<u16>;
}

pub trait Block: Rotate + Shift + Config {}

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

impl BlockI {
    pub fn new() -> Self {
        Self { data: BlockData::new([0x03C0, 0x0000, 0x0000, 0x0000]) }
    }
}

impl BlockJ {
    pub fn new() -> Self {
        Self { data: BlockData::new([0x0380, 0x0200, 0x0000, 0x0000]) }
    }
}

impl BlockL {
    pub fn new() -> Self {
        Self { data: BlockData::new([0x0380, 0x0080, 0x0000, 0x0000]) }
    }
}

impl BlockO {
    pub fn new() -> Self {
        Self { data: BlockData::new([0x0180, 0x0180, 0x0000, 0x0000]) }
    }
}

impl BlockS {
    pub fn new() -> Self {
        Self { data: BlockData::new([0x0300, 0x0180, 0x0000, 0x0000]) }
    }
}

impl BlockT {
    pub fn new() -> Self {
        Self { data: BlockData::new([0x0380, 0x0100, 0x0000, 0x0000]) }
    }
}

impl BlockZ {
    pub fn new() -> Self {
        Self { data: BlockData::new([0x0180, 0x0300, 0x0000, 0x0000]) }
    }
}

impl Block for BlockI {}
impl Shift for BlockI {
    fn shift(&mut self, cmd: &ShiftCmd) { self.data.shift(&cmd); }
}
impl Rotate for BlockI {
    fn rotate(&mut self, cmd: &RotateCmd) {}
}
impl Config for BlockI {
    fn config(&self) -> Vec<u16> { self.data.config() }
}

impl Block for BlockJ {}
impl Shift for BlockJ {
    fn shift(&mut self, cmd: &ShiftCmd) { self.data.shift(&cmd); }
}
impl Rotate for BlockJ {
    fn rotate(&mut self, cmd: &RotateCmd) {}
}
impl Config for BlockJ {
    fn config(&self) -> Vec<u16> { self.data.config() }
}

impl Block for BlockL {}
impl Shift for BlockL {
    fn shift(&mut self, cmd: &ShiftCmd) { self.data.shift(&cmd); }
}
impl Rotate for BlockL {
    fn rotate(&mut self, cmd: &RotateCmd) {}
}
impl Config for BlockL {
    fn config(&self) -> Vec<u16> { self.data.config() }
}

impl Block for BlockO {}
impl Shift for BlockO {
    fn shift(&mut self, cmd: &ShiftCmd) { self.data.shift(&cmd); }
}
impl Rotate for BlockO {
    fn rotate(&mut self, cmd: &RotateCmd) {}
}
impl Config for BlockO {
    fn config(&self) -> Vec<u16> { self.data.config() }
}

impl Block for BlockS {}
impl Shift for BlockS {
    fn shift(&mut self, cmd: &ShiftCmd) { self.data.shift(&cmd); }
}
impl Rotate for BlockS {
    fn rotate(&mut self, cmd: &RotateCmd) {}
}
impl Config for BlockS {
    fn config(&self) -> Vec<u16> { self.data.config() }
}

impl Block for BlockT {}
impl Shift for BlockT {
    fn shift(&mut self, cmd: &ShiftCmd) { self.data.shift(&cmd); }
}
impl Rotate for BlockT {
    fn rotate(&mut self, cmd: &RotateCmd) {}
}
impl Config for BlockT {
    fn config(&self) -> Vec<u16> { self.data.config() }
}

impl Block for BlockZ {}
impl Shift for BlockZ {
    fn shift(&mut self, cmd: &ShiftCmd) { self.data.shift(&cmd); }
}
impl Rotate for BlockZ {
    fn rotate(&mut self, cmd: &RotateCmd) {}
}
impl Config for BlockZ {
    fn config(&self) -> Vec<u16> { self.data.config() }
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

/// BlockGenerator which generates blocks
pub struct BlockGenerator {
    fn_next: Box<dyn Fn(&usize) -> usize>,
    nextidx: usize,
    thisidx: usize,
}

impl BlockGenerator {
    /// return a blockgenerator which generates the next block based on rule from closure f
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(&usize) -> usize + 'static,
    {
        Self {
            thisidx: 0,
            nextidx: f(&0),
            fn_next: Box::new(f),
        }
    }
}

impl Iterator for BlockGenerator {
    type Item = Box<dyn Block>;

    fn next(&mut self) -> Option<Self::Item> {
        let ele: Option<Self::Item> = match self.thisidx {
            0 => Some(Box::new(BlockI::new())),
            1 => Some(Box::new(BlockJ::new())),
            2 => Some(Box::new(BlockL::new())),
            3 => Some(Box::new(BlockO::new())),
            4 => Some(Box::new(BlockS::new())),
            5 => Some(Box::new(BlockT::new())),
            6 => Some(Box::new(BlockZ::new())),
            _ => None,
        };
        self.thisidx = self.nextidx;
        self.nextidx = (self.fn_next)(&self.thisidx);
        return ele;
    }
}
