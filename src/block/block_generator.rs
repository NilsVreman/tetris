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
