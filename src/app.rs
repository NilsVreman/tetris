use super::util::Coord;
use super::tetris::Tetris;
use super::block::Block;
use super::scoreboard::Scoreboard;
use super::timer::TickTimer;
use super::enums::{ShiftCmd, RotateCmd, BlockID};

use eframe;
use egui::*;

#[cfg(not(target_arch = "wasm32"))]
use super::enums::GameStatus;

// ------------------------------------------------------------------------------------------------
// Game constants

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
const LVL_UP: i64 = 1000;

// ------------------------------------------------------------------------------------------------
// Main application

pub struct TetrisApp {
    // Score of the tetris game
    scoreboard: Scoreboard,

    // How often the game should tick (in milliseconds, ms)
    timer: TickTimer,

    // State of the board
    game: Tetris,
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

        Self { scoreboard, timer, game }
    }

    /// todo!()
    pub fn handle_user_input(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
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
    }

    pub fn tick(&mut self) {
        if self.timer.get_time_until_tick() <= 0 {
            self.game.tick();
            self.timer.reset_tick();
        }
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
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {

        // Alter tetris state based on user input
        self.handle_user_input(&ctx, _frame);

        // Update tick
        self.tick();

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
                    paint_next_block(ui.painter(), &self.game.peek_next_block(), &ui.next_widget_position());
                });
            });

        // Paint tetris field
        CentralPanel::default()
            .show(ctx, |ui| {
                self.paint_boundary(ui.painter());
                self.paint_state(ui.painter());
                paint_block(ui.painter(), &self.game.current_block());
            });

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
fn paint_next_block(painter: &egui::Painter, block: &Option<Block>, at_pos: &Pos2) {
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

// Setup context styles
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
