use std::{
    thread,
    sync::{Arc, Mutex, mpsc},
    time::{Duration, Instant},
};

use super::util::Coord;
use super::tetris::Tetris;
use super::block::Block;
use super::scoreboard::Scoreboard;
use super::enums::{ShiftCmd, RotateCmd, BlockID, GameStatus};

use eframe;
use egui::*;

const SIDEPANEL_WIDTH: f32 = 250.0;
const STROKE_WIDTH: f32 = 2.0;
const STROKE: Stroke = Stroke { width: STROKE_WIDTH, color: Color32::BLACK };
const ROUNDING_PX: f32 = 2.0;
const ROUNDING: Rounding = Rounding { nw: ROUNDING_PX, ne: ROUNDING_PX, sw: ROUNDING_PX, se: ROUNDING_PX, };
pub const CELL_SIZE: f32 = 30.0;
const CELL: Rect = Rect { min: pos2(0.0, 0.0), max: pos2(CELL_SIZE, CELL_SIZE) };

const COLOR_WALL: Color32 = Color32::WHITE;
const COLOR_I: Color32 = Color32::from_rgb(200, 150, 150);
const COLOR_J: Color32 = Color32::from_rgb(150, 200, 150);
const COLOR_L: Color32 = Color32::from_rgb(150, 150, 200);
const COLOR_O: Color32 = Color32::from_rgb(200, 200, 150);
const COLOR_S: Color32 = Color32::from_rgb(200, 150, 200);
const COLOR_T: Color32 = Color32::from_rgb(150, 200, 200);
const COLOR_Z: Color32 = Color32::from_rgb(200, 200, 200);

const START_PERIOD: u64 = 1024;
const MIN_PERIOD: u64 = 32;
const SPEED_UP_LEVEL: usize = 10;

///////////////////////////////////////

struct UpdatePeriod {
    min_period: u64,
    period: u64,  // Update frequency in milliseconds (ms)
}

impl UpdatePeriod {
    pub fn new(period: u64, min_period: u64) -> Self {
        Self { period, min_period }
    }

    pub fn decrease_period(&mut self) {
        if self.period / 2  > self.min_period {
            self.period /= 2;
        }
    }

    pub fn get_period(&self) -> u64 {
        self.period
    }
}

///////////////////////////////////////

pub struct TetrisApp {
    // Score of the tetris game
    scoreboard: Arc<Mutex<Scoreboard>>,
    score_ch: Option<mpsc::Sender<usize>>,

    // How often the game should tick (in milliseconds, ms)
    period: Arc<Mutex<UpdatePeriod>>,

    // State of the board
    game: Arc<Mutex<Tetris>>,

    // Game Over
    status: Arc<Mutex<GameStatus>>,

    // Handles to threads to drop when game is finished
    threads: Vec<Option< (&'static str, thread::JoinHandle<()>) >>,
}

impl TetrisApp {

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {

        // Setup font of context
        setup_context(&cc.egui_ctx);

        // Creates channels
        let (tx_tetris_score, rx_tetris_score)   = mpsc::channel();  // Between Tetris and Scoreboard
        let (tx_period_tetris, rx_period_tetris) = mpsc::channel();  // Between Period and Tetris
        let (tx_score_period, rx_score_period)   = mpsc::channel();  // Between Scoreboard and Period

        // Creates Shared resources
        let period      = Arc::new(Mutex::new(UpdatePeriod::new(START_PERIOD, MIN_PERIOD)));
        let scoreboard  = Arc::new(Mutex::new(Scoreboard::new()));
        let game        = Arc::new(Mutex::new(Tetris::new(10, 20)));
        let status      = Arc::new(Mutex::new(GameStatus::Okay));

        // Creates Game thread
        let tx_tetris_score_2 = tx_tetris_score.clone();
        let status_arc = Arc::clone(&status);
        let game_arc    = Arc::clone(&game);
        let game_handle = thread::spawn(move || {

            let mut sleep_ms = START_PERIOD;                // Current Period
            let mut start = Instant::now();                 // Start timer
            loop {

                thread::sleep(                              // Sleep until period
                    Duration::from_millis(sleep_ms)
                    .checked_sub(start.elapsed())
                    .unwrap()
                );

                // Order of mutex locks is important to avoid deadlocks.
                start = Instant::now();                     // Start timer
                let mut game = game_arc.lock().unwrap();    // Wait for lock

                // Send num_cleared_lines to score
                if let Some(num_cleared) = game.tick() {    // Tick game
                    tx_tetris_score_2.send(num_cleared).unwrap();
                }

                // If game is over, break
                if let GameStatus::GameOver = game.status() {
                    drop(tx_tetris_score_2);
                    return
                };
                let status = status_arc.lock().unwrap();
                if let GameStatus::GameOver = *status {
                    drop(tx_tetris_score_2);
                    return
                }

                // If we have received a new period.
                let msg = rx_period_tetris.try_recv();
                if let Ok(period) = msg { sleep_ms = period; }
            }
        });

        // Creates Scoreboard thread
        let scoreboard_arc    = Arc::clone(&scoreboard);
        let scoreboard_handle = thread::spawn(move || loop {
            // Wait for message
            let msg = rx_tetris_score.recv();
            match msg {
                Ok(n) => { // If all is okay, update scoreboard and send score over channel
                    let mut score = scoreboard_arc.lock().unwrap();
                    score.update_score(n);
                    tx_score_period.send(score.get_score()).unwrap();
                },
                Err(_) => { // If something is wrong, drop channel handle and return
                    drop(tx_score_period);
                    return;
                }
            }
        });

        // Creates Period thread
        let period_arc    = Arc::clone(&period);
        let period_handle = thread::spawn(move || {
            // The function determining speed based on the score
            let mut speed_up_level = (SPEED_UP_LEVEL..10*SPEED_UP_LEVEL).step_by(SPEED_UP_LEVEL);
            let mut next_level = speed_up_level.next().unwrap();
            loop {
                // Wait for msg
                let msg = rx_score_period.recv();
                match msg {
                    Ok(score) => if score > next_level { // If all is okay and score is above speedup level, speed up and transmit new period to Tetris thread
                        next_level = speed_up_level.next().unwrap();
                        let mut period = period_arc.lock().unwrap();
                        period.decrease_period();
                        tx_period_tetris.send(period.get_period()).unwrap();
                    },
                    Err(_) => { // If something is wrong, drop channel handle and return
                        drop(tx_period_tetris);
                        return;
                    }
                }
            }
        });

        Self {
            scoreboard,
            score_ch: Some(tx_tetris_score),
            period,
            game,
            status,
            threads: vec![
                Some(("game", game_handle)),
                Some(("scoreboard", scoreboard_handle)),
                Some(("period", period_handle))
            ],
        }
    }

    // Paint the state config of the tetris game
    fn paint_state<'a>(painter: &egui::Painter, state_config: impl Iterator<Item=&'a Block>) {
        state_config.for_each(|block| Self::paint_block(&painter, block));
    }

    // Paint the walls (boundary) of the tetris game
    fn paint_boundary<'a>(painter: &egui::Painter, boundary_config: impl Iterator<Item=&'a Coord>) {
        boundary_config.for_each(|&coord| Self::paint_coord(&painter, CELL_SIZE * coord, &COLOR_WALL));
    }

    // Paint a block 
    fn paint_block(painter: &egui::Painter, block: &Block) {
        block.config().for_each(|&coord| {
            Self::paint_coord(&painter, CELL_SIZE * coord, Self::color_from_id(block.id()))
        });
    }

    // paint the next block that is gonna appear
    fn paint_next_block(painter: &egui::Painter, block: &Option<Block>, at_pos: &Pos2) {
        if let Some(block) = block {
            let half_block_width = CELL_SIZE * (block.width() as f32) / 2.0;
            block.config().for_each(|&coord| {
                Self::paint_coord(
                    &painter,
                    Coord(
                        (at_pos.x - half_block_width + CELL_SIZE * coord.0 as f32) as i32,
                        (at_pos.y + CELL_SIZE * (coord.1 + 1) as f32) as i32
                    ),
                    Self::color_from_id(block.id()));
            });
        }
    }

    // Paint one cell
    fn paint_coord(painter: &egui::Painter, coord: Coord, color: &Color32) {
        painter.rect(
            CELL.translate(Vec2::new(coord.0 as f32, coord.1 as f32)),
            ROUNDING,
            *color,
            STROKE,
        );
    }

    // Choose color based on BlockID
    fn color_from_id(id: &BlockID) -> &Color32 {
        match id {
            BlockID::I => &COLOR_I,
            BlockID::J => &COLOR_J,
            BlockID::L => &COLOR_L,
            BlockID::O => &COLOR_O,
            BlockID::S => &COLOR_S,
            BlockID::T => &COLOR_T,
            BlockID::Z => &COLOR_Z,
        }
    }

    fn game_over(&mut self) {
        // Set gameover to true
        let mut status = self.status.lock().unwrap();
        *status = GameStatus::GameOver;

        // Drop score channel
        if let Some(ref mut ch) = self.score_ch.take() { drop(ch); }
    }
}

// Graceful shutdown of the tetris app
impl Drop for TetrisApp {
    fn drop(&mut self) {

        // Set status of tetris app to GameOver
        self.game_over();

        // Shut down each thread
        for thread in &mut self.threads {
            if let Some((id, handle)) = thread.take() {
                println!("Shutting down thread: {}", id);
                handle.join().unwrap();
            }
        }
    }
}

impl eframe::App for TetrisApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {

        // Take game mutex
        let mut game = self.game.lock().unwrap();

        // Close
        if ctx.input(|i| i.key_pressed(Key::Escape) || i.key_pressed(Key::Q))
            || game.status() == GameStatus::GameOver
        {
            _frame.close();
        }

        // User Commands
        if ctx.input(|i| i.key_pressed(Key::Space)) {
            if let Some(n) = game.hard_drop() {
                // Send how many lines were cleared to the scoreboard
                self.score_ch.as_ref().unwrap().send(n).unwrap();
            }
        }
        if ctx.input(|i| i.key_pressed(Key::H) || i.key_pressed(Key::ArrowLeft))    { game.shift_block_if_feasible(&ShiftCmd::Left); }
        if ctx.input(|i| i.key_pressed(Key::L) || i.key_pressed(Key::ArrowRight))   { game.shift_block_if_feasible(&ShiftCmd::Right); }
        if ctx.input(|i| i.key_pressed(Key::K) || i.key_pressed(Key::ArrowUp))      { game.rotate_block_if_feasible(&RotateCmd::Left); }
        if ctx.input(|i| i.key_pressed(Key::J) || i.key_pressed(Key::ArrowDown))    { game.rotate_block_if_feasible(&RotateCmd::Right); }

        // Take the scoreboard as mutex
        let scoreboard = self.scoreboard.lock().unwrap();

        // Update right hand side of gui
        SidePanel::right("side_panel")
            .exact_width(SIDEPANEL_WIDTH)
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label("");
                    ui.label("Score:");
                    ui.label(format!("{} p", scoreboard.get_score()));
                    ui.label("");
                    ui.separator();
                    ui.label("");
                    Self::paint_next_block(ui.painter(), &(*game).peek_next_block(), &ui.next_widget_position());
                });
            });

        // Paint tetris field
        CentralPanel::default()
            .show(ctx, |ui| {
                Self::paint_boundary(ui.painter(), game.boundary_config());
                Self::paint_state(ui.painter(), game.state_config());
                Self::paint_block(ui.painter(), &(*game).current_block());
            });

        // Sleep until request repaint or repaint at once if there exists other repaint requests
        let p = self.period.lock().unwrap();
        ctx.request_repaint_after(Duration::from_millis(p.get_period()));
    }
}

fn setup_context(ctx: &Context) {
    // Change font sizes
    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading,    FontId::new(30.0, FontFamily::Proportional)),
        (TextStyle::Body,       FontId::new(18.0, FontFamily::Proportional)),
        (TextStyle::Monospace,  FontId::new(14.0, FontFamily::Proportional)),
        (TextStyle::Button,     FontId::new(14.0, FontFamily::Proportional)),
        (TextStyle::Small,      FontId::new(10.0, FontFamily::Proportional)),
    ].into();
    ctx.set_style(style);
}

///////////////////////////////////////
