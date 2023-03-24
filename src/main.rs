// Runs if target architecture is *NOT* wasm32
#[cfg(not(target_arch = "wasm32"))]
pub fn main() {
    //let mut tetris = tetris::TetrisGame::new();
    //tetris.run();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Tetris",
        native_options,
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
