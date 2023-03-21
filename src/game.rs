use std::{
    thread,
    time::{Instant, Duration},
};

use crate::{
    board,
    block,
    scoreboard,
    rules,
    worker,
    game_util,
};

pub struct TetrisGame {
    board: board::Board,
    blockgenerator: block::BlockGenerator,
    score: scoreboard::Scoreboard,
    workers: Vec<worker::Worker>,
    gravity: f32,
}

impl Drop for TetrisGame {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker");
            worker.join();
        }
    }
}

impl TetrisGame {
    pub fn new() -> Self {
        Self { 
            board: board::Board::new(),
            blockgenerator: block::BlockGenerator::new(rules::rule_nextblock()),
            score: scoreboard::Scoreboard::new(rules::rule_score()),
            workers: Vec::new(),
            gravity: 0.15625,
        }
    }

    /// The main run function of the tetris game.
    ///
    /// Spawns threads to run the game indefinitely
    pub fn run(&mut self) {
        // ------------------------------
        //let worker = worker::Worker::new(Box::new(|| println!("Worker executing")), 1.0);
        //self.workers.push(worker);
        // ------------------------------
        self.increase_gravity();
        self.increase_gravity();
        self.increase_gravity();
        self.increase_gravity();
        self.increase_gravity();
        self.increase_gravity();
        //self.increase_gravity();

        let time_idx_fn = |x| (1.0 / self.gravity * 1_000.0) as u64;

        let mut executing = true;
        let mut cur_block = self.blockgenerator.next().unwrap();
        let mut time_idx = time_idx_fn(self.gravity);
        let mut line_idx = game_util::BOARD_LINE_FAIL;

        while executing {
            let start = Instant::now();

            if time_idx == 0 {
                time_idx = time_idx_fn(self.gravity);
                if line_idx == 0 || !self.block_is_feasible(line_idx - 1, &cur_block) {

                    let status = self.board.add_lines(line_idx, &cur_block.config());

                    if let board::BoardStatus::Overflow(n) = status {
                        executing = false; // TODO: Fix
                    }
                    if let Some(x) = self.blockgenerator.next() {
                        cur_block = x;
                    } else {
                        executing = false;
                    }
                    line_idx = game_util::BOARD_LINE_FAIL;
                } else {
                    line_idx -= 1;
                    // TODO: Remove //
                    if line_idx % 7 == 0 {
                        self.shift_block_if_feasible(&mut cur_block, &game_util::ShiftCmd::Right);
                        self.rotate_block_if_feasible(&mut cur_block, &game_util::RotateCmd::Right);
                    } else if line_idx % 11 == 0 {
                        self.shift_block_if_feasible(&mut cur_block, &game_util::ShiftCmd::Left);
                        self.rotate_block_if_feasible(&mut cur_block, &game_util::RotateCmd::Left);
                    }
                }
            }

            println!("{}\n",
                 self.board.print_lines_on_board(
                     rules::rule_line_at_index(line_idx, &cur_block.config())
                 )
            );
            println!("{} {} {}\n{}", self.gravity, time_idx_fn(self.gravity), time_idx, &cur_block);
            time_idx = time_idx - game_util::UPDATE_FREQ;

            thread::sleep(
                Duration::from_millis(game_util::UPDATE_FREQ)
                .checked_sub(start.elapsed())
                .unwrap_or_else(|| Duration::from_millis(0))
            );
        }
    }

    /// Increases speed of falling blocks
    pub fn increase_gravity(&mut self) {
        if self.gravity < game_util::MAX_GRAVITY {
            self.gravity *= 2.0;
        }
    }

    // Checks whether the block can fit on the board
    fn block_is_feasible(&self, idx: usize, block: &block::Block) -> bool {
        if let Ok(board_lines) = self.board.config(idx..idx+4) {
            let block_lines = block.config();
            return board_lines[0] & block_lines[0] == 0 &&
                board_lines[1] & block_lines[1] == 0 &&
                board_lines[2] & block_lines[2] == 0 &&
                board_lines[3] & block_lines[3] == 0
        }
        return false
    }

    fn rotate_block_if_feasible(&self, block: &mut block::Block, cmd: &game_util::RotateCmd) {
        
    }

    fn shift_block_if_feasible(&self, block: &mut block::Block, cmd: &game_util::ShiftCmd) {
        
    }
}

