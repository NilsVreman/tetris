use std::mem;
use std::collections::HashSet;

use super::block::{Block, BlockGenerator};
use super::enums::{ShiftCmd, RotateCmd, GameStatus};
use super::util::Coord;

pub struct Tetris {
    width: i32,
    height: i32,
    state: Vec<Block>,
    boundary: HashSet<Coord>,
    current_block: Block,
    block_generator: BlockGenerator,
}

impl Tetris {
    pub fn new(width: i32, height: i32) -> Self {
        assert!(width >= 4 && height >= 4);

        // Walls on the outside of the tetris court
        let mut boundary: HashSet<Coord> = HashSet::with_capacity((height*2 + width + 2) as usize);
        for i in 0..=height {
            boundary.insert(Coord(0, i));
            boundary.insert(Coord(width+1, i));
        }
        for i in 1..width+1 {
            boundary.insert(Coord(i, height));
        }

        // Generate first block and center it
        let mut gen = BlockGenerator::new();
        let first_block = gen.next().unwrap();
        let first_block = &first_block + Coord(1 + ((width - first_block.width()) as f32 / 2.0) as i32, 0);

        Self {
            width,
            height,
            boundary,
            state: vec![],
            current_block: first_block,
            block_generator: gen,
        }
    }

    fn block_outside_bounds(&self, block: &Block) -> bool {
        block.config().any(|x| self.boundary.contains(x))
    }

    fn block_collision(&self, block: &Block) -> bool {
        self.state.iter().any(|b| b.is_collision(&block))
    }

    /// todo!()
    pub fn shift_block_if_feasible(&mut self, cmd: &ShiftCmd) {
        let shifted_block = self.current_block.shifted_version(&cmd);
        if !self.block_outside_bounds(&shifted_block)
            && !self.block_collision(&shifted_block)
        {
            self.current_block = shifted_block;
        }
    }

    /// todo!()
    pub fn rotate_block_if_feasible(&mut self, cmd: &RotateCmd) {
        let rotated_block = self.current_block.rotated_version(&cmd);
        if !self.block_outside_bounds(&rotated_block)
            && !self.block_collision(&rotated_block)
        {
            self.current_block = rotated_block;
        }
    }

    /// todo!()
    fn is_line_full(&self, line: usize) -> bool {
        self.state.iter()
            .flat_map(|block| block.config())
            .fold(0, |acc, coord| acc + if coord.1 == line as i32 { 1 } else { 0 })
            == self.width
    }

    // Clears line
    fn clear_line(&mut self, line: usize) {
        self.state.iter_mut().for_each(|block| block.clear_line(line));
    }

    // Clears all filled lines and removes the blocks that are now empty from our state
    fn clear_filled_lines(&mut self) -> usize {
        let mut num_cleared = 0;

        for line in 0..(self.height as usize) {
            if self.is_line_full(line) {
                num_cleared += 1;
                self.clear_line(line);
            }
        }

        // Removes all blocks that were fully cleared
        if num_cleared > 0 {
            self.state.retain(|block| !block.is_fully_cleared());
        }

        num_cleared
    }

    /// todo!()
    pub fn current_block(&self) -> &Block {
        &self.current_block
    }

    /// todo!()
    pub fn peek_next_block(&self) -> Option<Block> {
        self.block_generator.peek_next()
    }

    /// todo!()
    pub fn state_config(&self) -> impl Iterator<Item=&Block> {
        self.state.iter()
    }

    /// todo!()
    pub fn boundary_config(&self) -> impl Iterator<Item=&Coord> {
        self.boundary.iter()
    }

    /// todo!()
    pub fn status(&self) -> GameStatus {
        if self.block_collision(&self.current_block) {
            GameStatus::GameOver
        } else {
            GameStatus::Okay
        }
    }

    /// todo!()
    /// returns whether the game is lost or not
    pub fn tick(&mut self) -> Option<usize> {
        self.drop_n(1)
    }

    /// todo!()
    pub fn hard_drop(&mut self) -> Option<usize> {
        self.drop_n(self.height+2)
    }

    // aux function for tick and hard_drop (n = how many blocks we should drop maximally)
    fn drop_n(&mut self, n: i32) -> Option<usize> {
        assert!(n > 0);
        for _ in 0..n { // how many drops we should perform

            let dropped_block = self.current_block.drop_one();

            if self.block_outside_bounds(&dropped_block)
                || self.block_collision(&dropped_block)
            {
                // If dropped block is infeasible,
                // add the current block to the tetris state and change current_block
                if let Some(next_block) = self.block_generator.next() {
                    let next_block = self.center_block(&next_block);
                    let block_to_add = mem::replace(&mut self.current_block, next_block);
                    self.state.push(block_to_add);
                    let num_cleared = self.clear_filled_lines();

                    return Some(num_cleared)
                }
            } else {
                self.current_block = dropped_block;
            }
        }
        None
    }

    fn center_block(&self, block: &Block) -> Block {
        let half_block_width = block.width() as f32 / 2.0;
        let half_width = self.width as f32 / 2.0;
        block + Coord(1 + (half_width - half_block_width) as i32, 0)
    }
}
