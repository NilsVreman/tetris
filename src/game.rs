use std::{
    thread,
    time::{Instant, Duration},
};

use crate::{
    game_util::{UPDATE_FREQ, MAX_GRAVITY, FAIL_HEIGHT},
    board,
    block,
    scoreboard,
    rules,
    worker,
};


pub struct TetrisGame<'a> {
    board: board::Board,
    blockgenerator: block::BlockGenerator<'a>,
    score: scoreboard::Scoreboard,
    workers: Vec<worker::Worker>,
    gravity: f32,
}

impl<'a> TetrisGame<'a> {
    pub fn new() -> Self {
        Self { 
            board: board::Board::new(),
            blockgenerator: block::BlockGenerator::new(rules::rule_nextblock()),
            score: scoreboard::Scoreboard::new(rules::rule_score()),
            workers: Vec::new(),
            gravity: 0.25_f32,
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

        let mut executing = true;
        let mut cur_block = self.blockgenerator.next();
        let mut time_idx = (1.0 / self.gravity * 1_000.0) as u64;
        let mut line_idx = FAIL_HEIGHT;

        while executing && cur_block.is_some() {
            let start = Instant::now();

            if time_idx == 0 {
                time_idx = (1.0 / self.gravity * 1_000.0) as u64;
                if line_idx == 0 || !self.board.is_feasible(line_idx.checked_sub(1).unwrap(), &cur_block.unwrap()) {
                    let status = self.board.add_block(line_idx, &cur_block.unwrap());
                    if let board::BoardStatus::Overflow(n) = status {
                        executing = false; // TODO: Fix
                    }
                    cur_block = self.blockgenerator.next();
                    line_idx = FAIL_HEIGHT;
                    println!("{}\n\n", self.board);
                } else {
                    line_idx = line_idx.checked_sub(1).unwrap();
                }
            }

            time_idx = time_idx.checked_sub(UPDATE_FREQ).unwrap();

            thread::sleep(
                Duration::from_millis(UPDATE_FREQ)
                .checked_sub(start.elapsed())
                .unwrap()
            )
        }
    }

    /// Increases speed of falling blocks
    pub fn increase_gravity(&mut self) {
        if self.gravity < MAX_GRAVITY {
            println!("Inside: {}", self.gravity);
            self.gravity *= 2.0;
        }
    }
}

impl<'a> Drop for TetrisGame<'a> {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker");
            worker.join();
        }
    }
}
