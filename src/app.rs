use super::util::Coord;
use super::tetris::Tetris;
use super::block::Block;
use super::scoreboard::Scoreboard;
use super::timer::TickTimer;
use super::enums::{ShiftCmd, RotateCmd, BlockID, GameStatus};

use eframe;
use egui::{self, Key, Color32};

// ------------------------------------------------------------------------------------------------
// Game constants

pub const CELL_SIZE: f32 = 30.0;
pub const SIDEPANEL_WIDTH: f32 = 250.0;

const STROKE_WIDTH: f32 = 2.0;
const ROUNDING_PX: f32 = 2.0;
const STROKE: egui::Stroke      = egui::Stroke { width: STROKE_WIDTH, color: Color32::BLACK };
const ROUNDING: egui::Rounding  = egui::Rounding { nw: ROUNDING_PX, ne: ROUNDING_PX, sw: ROUNDING_PX, se: ROUNDING_PX, };
const CELL: egui::Rect          = egui::Rect { min: egui::pos2(0.0, 0.0), max: egui::pos2(CELL_SIZE, CELL_SIZE) };

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
const LVL_UP: i64 = 500;

// ------------------------------------------------------------------------------------------------
// Main application

pub struct TetrisApp {
    // Score of the tetris game
    scoreboard: Scoreboard,

    // How often the game should tick (in milliseconds, ms)
    timer: TickTimer,

    // State of the board
    game: Tetris,

    // Game size
    width: i32,
    height: i32,
}

impl TetrisApp {

    /// todo!()
    pub fn new(cc: &eframe::CreationContext<'_>, width: i32, height: i32) -> Self {

        // Setup font of context
        setup_context(&cc.egui_ctx);

        // Creates resources
        let timer       = TickTimer::new(START_PERIOD, MIN_PERIOD, LVL_UP);
        let scoreboard  = Scoreboard::new();
        let game        = Tetris::new(width, height);

        Self { scoreboard, timer, game, width, height }
    }

    /// todo!()
    /// Handles user input that affect the tetris app state
    fn handle_user_input_app(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Close
        #[cfg(not(target_arch = "wasm32"))]
        if ctx.input(|i| i.key_pressed(Key::Escape) || i.key_pressed(Key::Q)) {
            _frame.close();
        }

        if ctx.input(|i| i.key_pressed(Key::R)) { self.reset(); }
    }

    /// todo!()
    /// Handles user input that affect the tetris game state
    fn handle_user_input_game(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // User Commands
        if ctx.input(|i| i.key_pressed(Key::Space)) {
            if let Some(num_lines_cleared) = self.game.hard_drop() {
                self.update_score_and_tickrate(num_lines_cleared);
            }
        }
        if ctx.input(|i| i.key_pressed(Key::H) || i.key_pressed(Key::ArrowLeft))    { self.game.shift_block_if_feasible(&ShiftCmd::Left); }
        if ctx.input(|i| i.key_pressed(Key::L) || i.key_pressed(Key::ArrowRight))   { self.game.shift_block_if_feasible(&ShiftCmd::Right); }
        if ctx.input(|i| i.key_pressed(Key::K) || i.key_pressed(Key::ArrowUp))      { self.game.rotate_block_if_feasible(&RotateCmd::Left); }
        if ctx.input(|i| i.key_pressed(Key::J) || i.key_pressed(Key::ArrowDown))    { self.game.rotate_block_if_feasible(&RotateCmd::Right); }
    }

    /// todo!()
    fn tick(&mut self) {
        if self.timer.get_time_until_tick() <= 0 {
            if let Some(num_lines_cleared) = self.game.tick() {
                self.update_score_and_tickrate(num_lines_cleared);
            }
            self.timer.reset_tick();
        }
    }

    /// todo!()
    fn reset(&mut self) {
        // Creates resources
        self.timer      = TickTimer::new(START_PERIOD, MIN_PERIOD, LVL_UP);
        self.scoreboard = Scoreboard::new();
        self.game       = Tetris::new(self.width, self.height);
    }

    // Send how many lines were cleared to the scoreboard
    fn update_score_and_tickrate(&mut self, num_lines_cleared: usize) {
        self.scoreboard.update_score(num_lines_cleared);
        self.timer.update_period_from_score(self.scoreboard.get_score());
    }

    // Paint the state config of the tetris game
    fn paint_state(&self, painter: &egui::Painter) {
        self.game.state_config()
            .for_each(|block| paint_block(&painter, block));
    }

    // Paint the walls (boundary) of the tetris game
    fn paint_boundary(&self, painter: &egui::Painter) {
        self.game.boundary_config()
            .for_each(|&coord| paint_coord(&painter, CELL_SIZE * coord, &COLOR_WALL));
    }
}

// ------------------------------------------------------------------------------------------------
// Gui loop

impl eframe::App for TetrisApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // Alter app state based on user input
        self.handle_user_input_app(&ctx, _frame);

        // If we are still able to play
        if let GameStatus::Okay = self.game.status() {
            // Alter tetris state based on user input
            self.handle_user_input_game(&ctx, _frame);

            // Update tick
            self.tick();
        }

        // Update right hand side of gui
        egui::SidePanel::right("side_panel")
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
                    if let GameStatus::Okay = self.game.status() {
                        paint_next_block(ui.painter(), &self.game.peek_next_block(), &ui.next_widget_position());
                    };
                });
            });

        // Paint tetris field
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                self.paint_boundary(ui.painter());
                self.paint_state(ui.painter());
                if let GameStatus::Okay = self.game.status() {
                    paint_block(ui.painter(), &self.game.current_block());
                };
            });

        // If game is over print popup
        if let GameStatus::GameOver = self.game.status() {
            egui::Window::new("Game Over!")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label(format!("Final score: {} p", self.scoreboard.get_score()));
                });
        };

        // Sleep until request repaint or repaint at once if there exists other repaint requests
        let time = self.timer.get_time_until_tick();
        ctx.request_repaint_after(
            if time <= 0 { std::time::Duration::ZERO }
            else { std::time::Duration::from_millis(time as u64) }
        );
    }
}


// ------------------------------------------------------------------------------------------------
// Aux functions for drawing


// Paint a block 
fn paint_block(painter: &egui::Painter, block: &Block) {
    block.config().for_each(|&coord| {
        paint_coord(&painter, CELL_SIZE * coord, color_from_id(block.id()))
    });
}

// paint the next block that is gonna appear
fn paint_next_block(painter: &egui::Painter, block: &Option<Block>, at_pos: &egui::Pos2) {
    if let Some(block) = block {
        let half_block_width = CELL_SIZE * (block.width() as f32) / 2.0;
        block.config().for_each(|&coord| {
            paint_coord(
                &painter,
                Coord(
                    (at_pos.x - half_block_width + CELL_SIZE * coord.0 as f32) as i32,
                    (at_pos.y + CELL_SIZE * (coord.1 + 1) as f32) as i32
                ),
                color_from_id(block.id()));
        });
    }
}

// Paint one cell
fn paint_coord(painter: &egui::Painter, coord: Coord, color: &Color32) {
    painter.rect(
        CELL.translate(egui::Vec2::new(coord.0 as f32, coord.1 as f32)),
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

// Setup context styles
fn setup_context(ctx: &egui::Context) {
    use egui::{TextStyle, FontId, FontFamily};

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
