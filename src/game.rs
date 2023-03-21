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
};
use crate::game_util::{
    UPDATE_FREQ,
    MAX_GRAVITY,
    FAIL_HEIGHT,
    ShiftCmd,
    RotateCmd,
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
        let mut cur_block = self.blockgenerator.next();
        let mut time_idx = time_idx_fn(self.gravity);
        let mut line_idx = FAIL_HEIGHT;

        while executing && cur_block.is_some() {
            let start = Instant::now();

            if time_idx == 0 {
                time_idx = time_idx_fn(self.gravity);
                todo!("Problem here because line_idx is not representative of lower edge anymore");
                if line_idx == 0 || !self.board.is_feasible(line_idx - 1, cur_block.as_ref().unwrap()) {

                    let status = self.board.add_block(line_idx, cur_block.as_ref().unwrap());

                    if let board::BoardStatus::Overflow(n) = status {
                        executing = false; // TODO: Fix
                    }
                    cur_block = self.blockgenerator.next();
                    line_idx = FAIL_HEIGHT;
                } else {
                    line_idx -= 1;
                    // TODO: Remove //
                    if line_idx % 7 == 0 {
                        cur_block.as_mut().unwrap().shift(&ShiftCmd::Right);
                        cur_block.as_mut().unwrap().rotate(&RotateCmd::Right);
                    } else if line_idx % 11 == 0 {
                        cur_block.as_mut().unwrap().shift(&ShiftCmd::Left);
                        cur_block.as_mut().unwrap().rotate(&RotateCmd::Left);
                    }
                }
            }

            println!("{}\n",
                 self.board.print_block_on_board(
                     rules::rule_blockline_at_index(line_idx, cur_block.as_ref().unwrap())
                 )
            );
            println!("{} {} {}\n{}", self.gravity, time_idx_fn(self.gravity), time_idx, cur_block.as_ref().unwrap());
            time_idx = time_idx - UPDATE_FREQ;

            thread::sleep(
                Duration::from_millis(UPDATE_FREQ)
                .checked_sub(start.elapsed())
                .unwrap_or_else(|| Duration::from_millis(0))
            );
        }
    }

    /// Increases speed of falling blocks
    pub fn increase_gravity(&mut self) {
        if self.gravity < MAX_GRAVITY {
            self.gravity *= 2.0;
        }
    }
}

