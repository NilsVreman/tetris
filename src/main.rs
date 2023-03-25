// Runs if target architecture is *NOT* wasm32
#[cfg(not(target_arch = "wasm32"))]
pub fn main() {
    //let mut tetris = tetris::TetrisGame::new();
    //tetris.run();

    let options = native_options(tetris::CELL_SIZE);
    eframe::run_native(
        "Tetris",
        options,
        Box::new(|cc| Box::new(tetris::TetrisApp::new(cc))),
    );
}

// Runs if target architecture is wasm32
#[cfg(target_arch = "wasm32")]
pub fn main() {
    //let mut tetris = tetris::TetrisGame::new();
    //tetris.run();
    let app = tetris::TetrisApp::new();
    let native_options = eframe::NativeOptions::default();
    eframe
}

fn native_options(cell_size: f32) -> eframe::NativeOptions {
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(egui::Vec2::new(cell_size * 12.0 + 250.0, cell_size * 22.0));
    native_options.resizable = false;
    native_options.mouse_passthrough = true;
    native_options.always_on_top = true;
    native_options.centered = true;
    native_options
}
