#![warn(clippy::all, rust_2018_idioms)]

const GAME_WIDTH: i32 = 10;
const GAME_HEIGHT: i32 = 20;

#[cfg(not(target_arch = "wasm32"))]
pub fn main() {
    tetris::start_native("tetris_canvas", GAME_WIDTH, GAME_HEIGHT)
        .expect("Couldn't start eframe");
}

// Runs if target architecture is wasm32
#[cfg(target_arch = "wasm32")]
pub fn main() {
    wasm_bindgen_futures::spawn_local(async {
        tetris::start_web("tetris_canvas", GAME_WIDTH, GAME_HEIGHT)
            .await
            .expect("Couldn't start eframe");
    });
}
