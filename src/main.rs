const GAME_WIDTH: i32 = 10;
const GAME_HEIGHT: i32 = 20;

#[cfg(not(target_arch = "wasm32"))]
pub fn main() {

    // Options
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(
        egui::Vec2::new(
            tetris::CELL_SIZE * (GAME_WIDTH as f32 + 2.0) + 250.0,
            tetris::CELL_SIZE * (GAME_HEIGHT as f32 + 0.5)
        )
    );
    native_options.resizable = false;
    native_options.mouse_passthrough = true;
    native_options.always_on_top = true;
    native_options.centered = true;

    // Start gui in native
    eframe::run_native(
        "Tetris",
        native_options,
        Box::new(|cc| Box::new(tetris::TetrisApp::new(cc, GAME_WIDTH, GAME_HEIGHT))),
    );
}

// Runs if target architecture is wasm32
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
pub fn main() {

    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();
    eframe::start_web(
        "the_canvas_id", // hardcode it
        web_options,
        Box::new(|cc| Box::new(tetris::TetrisApp::new(cc, GAME_WIDTH, GAME_HEIGHT))),
    );
}
