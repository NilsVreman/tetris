use std::ops::Add;
use std::collections::HashSet;

use super::util::Coord;
use super::enums::{
    BlockID,
    ShiftCmd,
    RotateCmd,
};

#[derive(Clone)]
pub struct Block {
    coords: HashSet<Coord>,
    id: BlockID,
    center: Coord,
}

impl Add<Coord> for &Block {
    type Output = Block;

    fn add(self, rhs: Coord) -> Self::Output {
        Self::Output {
            id: self.id,
            center: self.center+rhs,
            coords: self.coords.iter().map(|&c| c + rhs).collect(),
        }
    }
}

impl Block {
    fn new(id: BlockID) -> Self {
        match id {
            BlockID::I => Self {
                coords: HashSet::from([Coord(0, 0), Coord(1, 0), Coord(2, 0), Coord(3, 0)]),
                center: Coord(1, 0),
                id,
            },
            BlockID::J => Self {
                coords: HashSet::from([Coord(0, 0), Coord(0, 1), Coord(1, 0), Coord(2, 0)]),
                center: Coord(1, 0),
                id,
            },
            BlockID::L => Self {
                coords: HashSet::from([Coord(0, 0), Coord(2, 1), Coord(1, 0), Coord(2, 0)]),
                center: Coord(1, 0),
                id,
            },
            BlockID::O => Self {
                coords: HashSet::from([Coord(1, 0), Coord(1, 1), Coord(2, 0), Coord(2, 1)]),
                center: Coord(0, 0),
                id,
            },
            BlockID::S => Self {
                coords: HashSet::from([Coord(0, 0), Coord(1, 0), Coord(1, 1), Coord(2, 1)]),
                center: Coord(1, 0),
                id,
            },
            BlockID::T => Self {
                coords: HashSet::from([Coord(0, 0), Coord(1, 0), Coord(1, 1), Coord(2, 0)]),
                center: Coord(1, 0),
                id,
            },
            BlockID::Z => Self {
                coords: HashSet::from([Coord(0, 1), Coord(1, 1), Coord(1, 0), Coord(2, 0)]),
                center: Coord(1, 0),
                id,
            },
        }
    }

    /// todo!()
    pub fn is_collision(&self, other: &Block) -> bool {
        !self.coords.is_disjoint(&other.coords)
    }

    /// todo!()
    pub fn config(&self) -> impl Iterator<Item=&Coord> {
        self.coords.iter()
    }

    /// todo!()
    pub fn drop_one(&self) -> Self {
        self + Coord(0, 1)
    }

    /// todo!()
    pub fn shifted_version(&self, cmd: &ShiftCmd) -> Self {
        match cmd {
            ShiftCmd::Left => self + Coord(-1, 0),
            ShiftCmd::Right => self + Coord(1, 0),
        }
    }

    /// todo!()
    pub fn rotated_version(&self, cmd: &RotateCmd) -> Self {
        if let BlockID::O = self.id {
            return self.clone()
        }

        let Coord(cx, cy) = self.center;
        Self {
            coords: match cmd {
                RotateCmd::Right => self.coords.iter().map(|c| Coord(cx + (c.1 - cy), cy - (c.0 - cx))).collect(),
                RotateCmd::Left  => self.coords.iter().map(|c| Coord(cx - (c.1 - cy), cy + (c.0 - cx))).collect(),
            },
            center: self.center,
            id: self.id,
        }
    }

    /// todo!()
    pub fn clear_line(&mut self, line: usize) {
        let line = line as i32;
        self.coords = self.coords.iter().filter_map(|&c| if c.1 == line {
                None
            } else if c.1 < line {
                Some( Coord(c.0, c.1 + 1) )
            } else {
                Some(c)
            }).collect();
    }
}

impl std::fmt::Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Block")
            .field("id", &self.id)
            .field("coords", &self.coords)
            .finish()
    }
}

////////////////////
// BlockGenerator //
////////////////////

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
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        let ele: Option<Self::Item> = match self.thisidx {
            0 => Some(Block::new(BlockID::I)),
            1 => Some(Block::new(BlockID::J)),
            2 => Some(Block::new(BlockID::L)),
            3 => Some(Block::new(BlockID::O)),
            4 => Some(Block::new(BlockID::S)),
            5 => Some(Block::new(BlockID::T)),
            6 => Some(Block::new(BlockID::Z)),
            _ => None,
        };
        self.thisidx = self.nextidx;
        self.nextidx = (self.fn_next)(&self.thisidx);
        return ele;
    }
}
