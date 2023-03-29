#![warn(clippy::all, rust_2018_idioms)]

mod util;
mod enums;
mod block;
mod tetris;
mod scoreboard;
mod app;

pub use tetris::Tetris;
pub use app::TetrisApp;

////////////
// Native //
////////////

#[cfg(not(target_arch = "wasm32"))]
pub fn start_native(canvas_id: &str, width: i32, height: i32) -> Result<(), eframe::Error> {
    // Options
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(
            egui::Vec2::new(
                app::CELL_SIZE * (width as f32 + 2.0) + app::SIDEPANEL_WIDTH,
                app::CELL_SIZE * (height as f32 + 0.5)
            )
        ),
        resizable: false,
        mouse_passthrough: true,
        always_on_top: true,
        centered: true,

        ..Default::default()
    };

    // Start gui in native
    eframe::run_native(
        canvas_id,
        native_options,
        Box::new(move |cc| Box::new(app::TetrisApp::new(cc, width, height))),
    )
}

//////////
// WASM //
//////////

/// This is the entry-point for all the web-assembly.
/// This is called once from the HTML.
/// It loads the app, installs some callbacks, then returns.
/// You can add more callbacks like this if you want to call in to your code.
#[cfg(target_arch = "wasm32")]
pub async fn start_web(canvas_id: &str, width: i32, height: i32) -> Result<eframe::web::AppRunnerRef, eframe::wasm_bindgen::JsValue> {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    // Start web assembly thread
    let web_options = eframe::WebOptions::default();
    eframe::start_web(
        canvas_id,
        web_options,
        Box::new(move |cc| Box::new(app::TetrisApp::new(cc, width, height))),
    )
    .await
}
