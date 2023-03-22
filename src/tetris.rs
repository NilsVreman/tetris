use std::mem;
use super::block::{Block, BlockGenerator};
use super::enums::{ShiftCmd, RotateCmd, GameStatus};

pub struct Tetris {
    width: i32,
    height: i32,
    state: Vec<Block>,
    current_block: Block,
    block_generator: BlockGenerator,
}

impl Tetris {
    pub fn new(width: i32, height: i32) -> Self {
        assert!(width >= 4 && height >= 4);
        let mut gen = BlockGenerator::new(|x| (x + 1) % 7);
        Self {
            width,
            height,
            state: vec![],
            current_block: gen.next().unwrap(),
            block_generator: gen,
        }
    }

    fn block_outside_bounds(&self, block: &Block) -> bool {
        block.config().any(|x| {
            x.0 < 0 || x.0 >= self.width || x.1 < 0 || x.1 >= self.height
        })
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
    pub fn is_line_full(&self, line: usize) -> bool {
        self.state.iter()
            .flat_map(|block| block.config())
            .fold(0, |acc, coord| acc + if coord.1 == line as i32 { 1 } else { 0 })
            == self.width
    }

    fn clear_line(&mut self, line: usize) {
        self.state.iter_mut().for_each(|block| block.clear_line(line));
    }

    fn clear_filled_lines(&mut self) {
        for line in 0..(self.height as usize) {
            if self.is_line_full(line) {
                self.clear_line(line);
            }
        }
    }

    /// todo!()
    /// returns whether the game is lost or not
    pub fn tick(&mut self) -> GameStatus {
        let dropped_block = self.current_block.drop_one();
        if self.block_outside_bounds(&dropped_block)
            || self.block_collision(&dropped_block)
        {
            if let Some(next_block) = self.block_generator.next() {
                let block_to_add = mem::replace(&mut self.current_block, next_block);
                self.state.push(block_to_add);
                self.clear_filled_lines();
                
                if self.block_collision(&self.current_block) {
                    return GameStatus::GameOver
                }
            } 
        } else {
            self.current_block = dropped_block;
        }

        return GameStatus::Okay
    }

    pub fn print(&self) {
        println!("{} {} {:#?} {:?}\n", self.width, self.height, self.state, self.current_block);
    }

    pub fn run(&mut self) {
        
    }
}
