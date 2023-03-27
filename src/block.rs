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
    center: Coord,
    id: BlockID,
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
                coords: HashSet::from([Coord(0, 0), Coord(0, -1), Coord(1, 0), Coord(2, 0)]),
                center: Coord(1, 0),
                id,
            },
            BlockID::L => Self {
                coords: HashSet::from([Coord(0, 0), Coord(2, -1), Coord(1, 0), Coord(2, 0)]),
                center: Coord(1, 0),
                id,
            },
            BlockID::O => Self {
                coords: HashSet::from([Coord(0, 0), Coord(0, -1), Coord(1, 0), Coord(1, -1)]),
                center: Coord(0, 0),
                id,
            },
            BlockID::S => Self {
                coords: HashSet::from([Coord(0, 0), Coord(1, 0), Coord(1, -1), Coord(2, -1)]),
                center: Coord(1, 0),
                id,
            },
            BlockID::T => Self {
                coords: HashSet::from([Coord(0, 0), Coord(1, 0), Coord(1, -1), Coord(2, 0)]),
                center: Coord(1, 0),
                id,
            },
            BlockID::Z => Self {
                coords: HashSet::from([Coord(0, -1), Coord(1, -1), Coord(1, 0), Coord(2, 0)]),
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
    /// TODO: Maybe fix to be mutating and isntead Clone in Tetris game
    pub fn shifted_version(&self, cmd: &ShiftCmd) -> Self {
        match cmd {
            ShiftCmd::Left => self + Coord(-1, 0),
            ShiftCmd::Right => self + Coord(1, 0),
        }
    }

    /// todo!()
    /// TODO: Maybe fix to be mutating and instead Clone in Tetris game
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

    /// todo!()
    pub fn is_fully_cleared(&self) -> bool {
        self.coords.is_empty()
    }

    /// todo!()
    pub fn id(&self) -> &BlockID {
        &self.id
    }

    /// todo!()
    pub fn width(&self) -> i32 {
        let mut x_min = i32::MAX;
        let mut x_max = 0;
        for coord in &self.coords {
            if coord.0 < x_min { x_min = coord.0; }
            if coord.0 > x_max { x_max = coord.0; }
        }
        x_max - x_min + 1
    }
}

////////////////////
// BlockGenerator //
////////////////////

/// BlockGenerator which generates blocks
pub struct BlockGenerator {
    idx: usize,
}

impl BlockGenerator {
    /// return a blockgenerator which generates the next block based on rule from closure f
    pub fn new() -> Self {
        Self { idx: 0 }
    }

    pub fn peek_next(&self) -> Option<Block> {
        match self.idx {
            0 => Some(Block::new(BlockID::I)),
            1 => Some(Block::new(BlockID::J)),
            2 => Some(Block::new(BlockID::L)),
            3 => Some(Block::new(BlockID::O)),
            4 => Some(Block::new(BlockID::S)),
            5 => Some(Block::new(BlockID::T)),
            6 => Some(Block::new(BlockID::Z)),
            _ => None,
        }
    }
}

impl Iterator for BlockGenerator {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        let ele = match self.idx {
            0 => Some(Block::new(BlockID::I)),
            1 => Some(Block::new(BlockID::J)),
            2 => Some(Block::new(BlockID::L)),
            3 => Some(Block::new(BlockID::O)),
            4 => Some(Block::new(BlockID::S)),
            5 => Some(Block::new(BlockID::T)),
            6 => Some(Block::new(BlockID::Z)),
            _ => None,
        };
        self.idx = (self.idx + 1) % 7;
        ele
    }
}
