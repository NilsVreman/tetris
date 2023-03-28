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

    // How often the game should tick (in milliseconds, ms)
    period: Arc<Mutex<UpdatePeriod>>,

    // State of the board
    game: Arc<Mutex<Tetris>>,

    // Handles to threads to drop when game is finished
    handles: Vec<Option<thread::JoinHandle<()>>>,
}

impl TetrisApp {
    //pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_context(&cc.egui_ctx);

        // Creates channels
        let (tx_tetris_score, rx_tetris_score)   = mpsc::channel();  // Between Tetris and Scoreboard
        let (tx_period_tetris, rx_period_tetris) = mpsc::channel();  // Between Period and Tetris
        let (tx_score_period, rx_score_period)   = mpsc::channel();  // Between Scoreboard and Period

        // Creates Shared resources
        let period      = Arc::new(Mutex::new(UpdatePeriod::new(START_PERIOD, MIN_PERIOD)));
        let scoreboard  = Arc::new(Mutex::new(Scoreboard::new()));
        let game        = Arc::new(Mutex::new(Tetris::new(10, 20)));

        // Creates Game thread
        let game_arc = Arc::clone(&game);
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

                match game.tick() {                         // Tick game
                    GameStatus::GameOver => tx_tetris_score.send(0),
                    GameStatus::Okay     => tx_tetris_score.send(1),
                };

                // If we have received a new period.
                if let Ok(period) = rx_period_tetris.try_recv() {
                    sleep_ms = period;
                }
            }
        });

        // Creates Scoreboard thread
        let scoreboard_arc = Arc::clone(&scoreboard);
        let scoreboard_handle = thread::spawn(move || loop {
            if let Ok(n) = rx_tetris_score.try_recv() {
                let mut score = scoreboard_arc.lock().unwrap();
                score.update_score(n);
                tx_score_period.send(score.get_score());
            }
        });

        // Creates Period thread
        let period_arc = Arc::clone(&period);
        let period_handle = thread::spawn(move || loop {
            if let Ok(score) = rx_score_period.try_recv() {
                let mut period = period_arc.lock().unwrap();
                period.decrease_period_if_score_reached(score);
                tx_period_tetris.send(period.get_period());
            }
        });

        Self {
            scoreboard,
            period,
            game,
            handles: vec![game_handle, scoreboard_handle, period_handle],
        }
    }

    fn paint_state<'a>(painter: &egui::Painter, state_config: impl Iterator<Item=&'a Block>) {
        state_config.for_each(|block| Self::paint_block(&painter, block));
    }

    fn paint_boundary<'a>(painter: &egui::Painter, boundary_config: impl Iterator<Item=&'a Coord>) {
        boundary_config.for_each(|&coord| Self::paint_coord(&painter, CELL_SIZE * coord, &COLOR_WALL));
    }

    fn paint_block(painter: &egui::Painter, block: &Block) {
        block.config().for_each(|&coord| {
            Self::paint_coord(&painter, CELL_SIZE * coord, Self::color_from_id(block.id()))
        });
    }

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

    fn paint_coord(painter: &egui::Painter, coord: Coord, color: &Color32) {
        painter.rect(
            CELL.translate(Vec2::new(coord.0 as f32, coord.1 as f32)),
            ROUNDING,
            *color,
            STROKE,
        );
    }

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
}

impl eframe::App for TetrisApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Close
        if ctx.input(|i| i.key_pressed(Key::Escape) || i.key_pressed(Key::Q)) {
            _frame.close()
        }

        let mut game = self.game.lock().unwrap();
        let mut num_cleared_lines = 0;

        // Commands
        if ctx.input(|i| i.key_pressed(Key::Space)) {
            if let Some(n) = game.hard_drop() {
                num_cleared_lines = n;
            }
        }
        if ctx.input(|i| i.key_pressed(Key::H) || i.key_pressed(Key::ArrowLeft))    { game.shift_block_if_feasible(&ShiftCmd::Left); }
        if ctx.input(|i| i.key_pressed(Key::L) || i.key_pressed(Key::ArrowRight))   { game.shift_block_if_feasible(&ShiftCmd::Right); }
        if ctx.input(|i| i.key_pressed(Key::K) || i.key_pressed(Key::ArrowUp))      { game.rotate_block_if_feasible(&RotateCmd::Left); }
        if ctx.input(|i| i.key_pressed(Key::J) || i.key_pressed(Key::ArrowDown))    { game.rotate_block_if_feasible(&RotateCmd::Right); }

        let mut scoreboard = self.scoreboard.lock().unwrap();
        scoreboard.update_score(num_cleared_lines); // If we cleared lines based on hard_drop, update score

        // Scoreboard
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

        // Tetris field
        CentralPanel::default()
            .show(ctx, |ui| {
                Self::paint_boundary(ui.painter(), game.boundary_config());
                Self::paint_state(ui.painter(), game.state_config());
                Self::paint_block(ui.painter(), &(*game).current_block());
            });

        // If game is over
        if let GameStatus::GameOver = game.status() {
            _frame.close();
        }

        // Sleep until request repaint or repaint at once if there exists other repaint requests
        let period = (*self.period.lock().unwrap()).get_period();
        ctx.request_repaint_after(Duration::from_millis(period));
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
