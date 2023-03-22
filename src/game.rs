use std::{
    thread,
    time::{Instant, Duration},
};

use crate::{
    board,
    block,
    scoreboard,
    worker,
    util,
    enums,
    consts,
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
            blockgenerator: block::BlockGenerator::new(util::rule_nextblock()),
            score: scoreboard::Scoreboard::new(util::rule_score()),
            workers: Vec::new(),
            gravity: 0.15625,  // NOTE: Crucial that this is a divisor of consts::UPDATE_FREQ.
                               // TODO: Make it nicer by maybe taking the gravity as input and
                               // adapting UPDATE_FREQ thereafter.
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
        self.increase_gravity();

        // Maybe break out this function and explain what time_idx does
        let time_idx_fn = |x| (1.0 / x * 1_000.0) as u64;

        let mut executing = true;
        let mut cur_block = self.blockgenerator.next().unwrap();
        let mut time_idx = time_idx_fn(self.gravity);
        let mut line_idx = consts::BOARD_LINE_FAIL;

        while executing {
            let start = Instant::now();

            if time_idx == 0 {
                // TODO: DOC that time_idx_fn resets time counter
                time_idx = time_idx_fn(self.gravity);
                if !self.block_is_feasible(line_idx - 1, &cur_block) {

                    let status = self.board.add_lines(line_idx, &cur_block.config());

                    // TODO: In here we should fix behaviour for line removal and score updates //
                    if let enums::BoardStatus::Overflow(n) = status {
                        executing = false; // TODO: Fix
                    }

                    // TODO: Maybe clarify this slightly
                    if let Some(x) = self.blockgenerator.next() { cur_block = x; }
                    else { executing = false; }

                    // TODO: Clarify what line_idx means
                    line_idx = consts::BOARD_LINE_FAIL;

                } else {
                    line_idx -= 1;
                }

                // TODO: vvvvvvvvvvvv
                if line_idx % 2 == 0 {
                    self.shift_block_if_feasible(line_idx, &mut cur_block, &enums::ShiftCmd::Right);
                    self.rotate_block_if_feasible(line_idx, &mut cur_block, &enums::RotateCmd::Right);
                } else if line_idx % 1 == 0 {
                    self.shift_block_if_feasible(line_idx, &mut cur_block, &enums::ShiftCmd::Left);
                    self.rotate_block_if_feasible(line_idx, &mut cur_block, &enums::RotateCmd::Left);
                }
                // TODO: ^^^^^^^^^^^^
            }

            // TODO: vvvvvvvvvvvv
            println!("{}",
                 self.board.print_lines_on_board(
                     util::rule_line_at_index(line_idx, &cur_block.config())
                 )
            );
            println!("{} {} {}", self.gravity, time_idx_fn(self.gravity), time_idx);
            // TODO: ^^^^^^^^^^^^
            time_idx = time_idx - consts::UPDATE_FREQ;

            thread::sleep(
                Duration::from_millis(consts::UPDATE_FREQ)
                .checked_sub(start.elapsed())
                .unwrap_or_else(|| Duration::from_millis(0))
            );
        }
    }

    /// Increases speed of falling blocks
    pub fn increase_gravity(&mut self) {
        if self.gravity < consts::MAX_GRAVITY {
            self.gravity *= 2.0;
        }
    }

    // Checks whether the block can fit on the board
    fn block_is_feasible(&self, idx: usize, block: &block::Block) -> bool {
        if let Ok(board_lines) = self.board.config(idx) {
            let block_lines = block.config();
            return board_lines[0] & block_lines[0] == 0 &&
                board_lines[1] & block_lines[1] == 0 &&
                board_lines[2] & block_lines[2] == 0 &&
                board_lines[3] & block_lines[3] == 0
        }
        return false
    }

    fn rotate_block_if_feasible(&self, idx: usize, block: &mut block::Block, cmd: &enums::RotateCmd) {
        let mut block_clone = block.clone();
        block_clone.rotate(&cmd);
        if self.block_is_feasible(idx, &block_clone) {
            block.rotate(&cmd);
        }
    }

    fn shift_block_if_feasible(&self, idx: usize, block: &mut block::Block, cmd: &enums::ShiftCmd) {
        let mut block_clone = block.clone();
        block_clone.shift(&cmd);
        if self.block_is_feasible(idx, &block_clone) {
            block.shift(&cmd);
        }
    }
}

