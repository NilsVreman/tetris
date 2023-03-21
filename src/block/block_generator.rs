////////////////////
// BlockGenerator //
////////////////////

use super::*;

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
            0 => Some(Block::new(BlockType::I)),
            1 => Some(Block::new(BlockType::J)),
            2 => Some(Block::new(BlockType::L)),
            3 => Some(Block::new(BlockType::O)),
            4 => Some(Block::new(BlockType::S)),
            5 => Some(Block::new(BlockType::T)),
            6 => Some(Block::new(BlockType::Z)),
            _ => None,
        };
        self.thisidx = self.nextidx;
        self.nextidx = (self.fn_next)(&self.thisidx);
        return ele;
    }
}
