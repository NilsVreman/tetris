// Runs if target architecture is *NOT* wasm32
#[cfg(not(target_arch = "wasm32"))]
pub fn main() {
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
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "Tetris", // hardcode it
            web_options,
            Box::new(|cc| Box::new(tetris::TetrisApp::new(cc))),
        )
        .await
        .expect("failed to start eframe");
    });
}

fn native_options(cell_size: f32) -> eframe::NativeOptions {
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(egui::Vec2::new(cell_size * 12.0 + 250.0, cell_size * 20.5));
    native_options.resizable = false;
    native_options.mouse_passthrough = true;
    native_options.always_on_top = true;
    native_options.centered = true;
    native_options
}
