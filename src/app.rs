use super::util::Coord;
use super::tetris::Tetris;
use super::block::Block;
use super::scoreboard::Scoreboard;
use super::enums::{ShiftCmd, RotateCmd, BlockID};

use eframe;
use egui::*;

#[cfg(not(target_arch = "wasm32"))]
use super::enums::GameStatus;

#[cfg(target_arch = "wasm32")]
use chrono::Timelike;

// ------------------------------------------------------------------------------------------------

pub const CELL_SIZE: f32 = 30.0;
pub const SIDEPANEL_WIDTH: f32 = 250.0;

const STROKE_WIDTH: f32 = 2.0;
const STROKE: Stroke = Stroke { width: STROKE_WIDTH, color: Color32::BLACK };
const ROUNDING_PX: f32 = 2.0;
const ROUNDING: Rounding = Rounding { nw: ROUNDING_PX, ne: ROUNDING_PX, sw: ROUNDING_PX, se: ROUNDING_PX, };
const CELL: Rect = Rect { min: pos2(0.0, 0.0), max: pos2(CELL_SIZE, CELL_SIZE) };

const COLOR_WALL: Color32 = Color32::WHITE;
const COLOR_I: Color32 = Color32::from_rgb(200, 150, 150);
const COLOR_J: Color32 = Color32::from_rgb(150, 200, 150);
const COLOR_L: Color32 = Color32::from_rgb(150, 150, 200);
const COLOR_O: Color32 = Color32::from_rgb(200, 200, 150);
const COLOR_S: Color32 = Color32::from_rgb(200, 150, 200);
const COLOR_T: Color32 = Color32::from_rgb(150, 200, 200);
const COLOR_Z: Color32 = Color32::from_rgb(200, 200, 200);

const START_PERIOD: i64 = 1024;
const MIN_PERIOD: i64 = 32;
const SPEED_UP_LEVEL: i64 = 10;

// ------------------------------------------------------------------------------------------------

struct TickPeriod {
    min_period: i64,
    period: i64,  // Update frequency in milliseconds (ms)
    next_level: i64,
}

impl TickPeriod {
    fn new(period: i64, min_period: i64) -> Self {
        Self { period, min_period, next_level: SPEED_UP_LEVEL }
    }

    fn decrease_period(&mut self) {
        if self.period / 2  > self.min_period {
            self.period /= 2;
        }
    }

    fn update_period_from_score(&mut self, score: usize) {
        if score as i64 >= self.next_level {
            self.next_level += SPEED_UP_LEVEL;
            self.decrease_period();
        }
    }

    fn get_period(&self) -> i64 {
        self.period
    }
}

struct TickTimer {
    period: TickPeriod,
    time: i64,
}

impl TickTimer {
    fn new() -> Self {
        let time = chrono::Local::now().timestamp_millis();
        Self {
            period: TickPeriod::new(START_PERIOD, MIN_PERIOD),
            time
        }
    }

    fn update_period_from_score(&mut self, score: usize) {
        self.period.update_period_from_score(score);
    }

    fn get_time_until_tick(&self) -> i64 {
        (self.time + self.period.get_period()) - chrono::Local::now().timestamp_millis()
    }

    fn reset_tick(&mut self) {
        self.time = chrono::Local::now().timestamp_millis();
    }
}

// ------------------------------------------------------------------------------------------------

pub struct TetrisApp {
    // Score of the tetris game
    scoreboard: Scoreboard,

    // How often the game should tick (in milliseconds, ms)
    timer: TickTimer,

    // State of the board
    game: Tetris,
}

impl TetrisApp {

    pub fn new(cc: &eframe::CreationContext<'_>, width: i32, height: i32) -> Self {

        // Setup font of context
        setup_context(&cc.egui_ctx);

        // Creates resources
        let timer       = TickTimer::new();
        let scoreboard  = Scoreboard::new();
        let game        = Tetris::new(width, height);

        Self { scoreboard, timer, game }
    }

    // Paint the state config of the tetris game
    fn paint_state<'a>(&self, painter: &egui::Painter) {
        self.game.state_config()
            .for_each(|block| Self::paint_block(&painter, block));
    }

    // Paint the walls (boundary) of the tetris game
    fn paint_boundary<'a>(&self, painter: &egui::Painter) {
        self.game.boundary_config()
            .for_each(|&coord| Self::paint_coord(&painter, CELL_SIZE * coord, &COLOR_WALL));
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
}


impl eframe::App for TetrisApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {

        // Close
        #[cfg(not(target_arch = "wasm32"))]
        if ctx.input(|i| i.key_pressed(Key::Escape) || i.key_pressed(Key::Q))
            || self.game.status() == GameStatus::GameOver
        {
            _frame.close();
        }

        // User Commands
        if ctx.input(|i| i.key_pressed(Key::Space)) {
            if let Some(num_cleared) = self.game.hard_drop() {
                // Send how many lines were cleared to the scoreboard
                self.scoreboard.update_score(num_cleared);
                self.timer.update_period_from_score(self.scoreboard.get_score());
            }
        }
        if ctx.input(|i| i.key_pressed(Key::H) || i.key_pressed(Key::ArrowLeft))  { self.game.shift_block_if_feasible(&ShiftCmd::Left); }
        if ctx.input(|i| i.key_pressed(Key::L) || i.key_pressed(Key::ArrowRight)) { self.game.shift_block_if_feasible(&ShiftCmd::Right); }
        if ctx.input(|i| i.key_pressed(Key::K) || i.key_pressed(Key::ArrowUp))    { self.game.rotate_block_if_feasible(&RotateCmd::Left); }
        if ctx.input(|i| i.key_pressed(Key::J) || i.key_pressed(Key::ArrowDown))  { self.game.rotate_block_if_feasible(&RotateCmd::Right); }

        // Update tick
        if self.timer.get_time_until_tick() <= 0 {
            println!("########## {}", self.timer.get_time_until_tick());
            self.game.tick();
            self.timer.reset_tick();
        }

        // Update right hand side of gui
        SidePanel::right("side_panel")
            .exact_width(SIDEPANEL_WIDTH)
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label("");
                    ui.label("Score:");
                    ui.label(format!("{} p", self.scoreboard.get_score()));
                    ui.label("");
                    ui.separator();
                    ui.label("");
                    Self::paint_next_block(ui.painter(), &self.game.peek_next_block(), &ui.next_widget_position());
                });
            });

        // Paint tetris field
        CentralPanel::default()
            .show(ctx, |ui| {
                self.paint_boundary(ui.painter());
                self.paint_state(ui.painter());
                Self::paint_block(ui.painter(), &self.game.current_block());
            });

        // Sleep until request repaint or repaint at once if there exists other repaint requests
        ctx.request_repaint();
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
