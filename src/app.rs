use super::util::Coord;
use super::tetris::Tetris;
use super::block::Block;
use super::enums::{ShiftCmd, RotateCmd, BlockID};

use eframe;
use egui::*;

const STROKE_WIDTH: f32 = 2.0;
const STROKE: Stroke = Stroke { width: STROKE_WIDTH, color: Color32::BLACK };
const ROUNDING_PX: f32 = 2.0;
const ROUNDING: Rounding = Rounding { nw: ROUNDING_PX, ne: ROUNDING_PX, sw: ROUNDING_PX, se: ROUNDING_PX, };
pub const CELL_SIZE: f32 = 30.0;
const CELL: Rect = Rect { min: pos2(0.0, 0.0), max: pos2(CELL_SIZE, CELL_SIZE) };

const COLOR_WALL: Color32 = Color32::WHITE;
const COLOR_I: Color32 = Color32::from_rgb(250, 200, 200);
const COLOR_J: Color32 = Color32::from_rgb(200, 250, 200);
const COLOR_L: Color32 = Color32::from_rgb(200, 200, 250);
const COLOR_O: Color32 = Color32::from_rgb(250, 250, 200);
const COLOR_S: Color32 = Color32::from_rgb(250, 200, 250);
const COLOR_T: Color32 = Color32::from_rgb(200, 250, 250);
const COLOR_Z: Color32 = Color32::from_rgb(250, 250, 250);

pub struct TetrisApp {
    // Score of the tetris game
    score: usize,

    // State of the board
    game: Tetris,
}

impl TetrisApp {
    //pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_context(&cc.egui_ctx);

        Self {
            score: 0,
            game: Tetris::new(10, 20),
        }
    }

    fn paint_state(&self, painter: &egui::Painter) {
        self.game.state_config().for_each(|block| Self::paint_block(&painter, block));
    }

    fn paint_boundary(&self, painter: &egui::Painter) {
        self.game.boundary_config().for_each(|&coord| Self::paint_coord(&painter, CELL_SIZE * coord, &COLOR_WALL));
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

fn setup_context(ctx: &Context) {
    // Change font sizes
    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading, FontId::new(30.0, FontFamily::Proportional)),
        (TextStyle::Body, FontId::new(18.0, FontFamily::Proportional)),
        (TextStyle::Monospace, FontId::new(14.0, FontFamily::Proportional)),
        (TextStyle::Button, FontId::new(14.0, FontFamily::Proportional)),
        (TextStyle::Small, FontId::new(10.0, FontFamily::Proportional)),
    ].into();
    ctx.set_style(style);
}

impl eframe::App for TetrisApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Scoreboard
        SidePanel::right("side_panel")
            .exact_width(250.0)
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label("");
                    ui.label("Score:");
                    ui.label(format!("{} p", self.score));
                    ui.label("");
                    ui.separator();
                    ui.label("");
                    TetrisApp::paint_next_block(ui.painter(), &self.game.peek_next_block(), &ui.next_widget_position());
                });
            });

        // Close
        if ctx.input(|i| i.key_pressed(Key::Escape) || i.key_pressed(Key::Q)) {
            _frame.close()
        }

        // Commands
        if ctx.input(|i| i.key_pressed(Key::H) || i.key_pressed(Key::ArrowLeft))    { self.game.shift_block_if_feasible(&ShiftCmd::Left); }
        if ctx.input(|i| i.key_pressed(Key::L) || i.key_pressed(Key::ArrowRight))   { self.game.shift_block_if_feasible(&ShiftCmd::Right); }
        if ctx.input(|i| i.key_pressed(Key::K) || i.key_pressed(Key::ArrowUp))      { self.game.rotate_block_if_feasible(&RotateCmd::Left); }
        if ctx.input(|i| i.key_pressed(Key::J) || i.key_pressed(Key::ArrowDown))    { self.game.rotate_block_if_feasible(&RotateCmd::Right); }

        // Tetris field
        CentralPanel::default()
            .show(ctx, |ui| {
                self.paint_boundary(ui.painter());
                self.paint_state(ui.painter());
                TetrisApp::paint_block(ui.painter(), self.game.current_block());
            });
    }
}
