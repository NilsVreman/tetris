pub struct TetrisApp {
    // Score of the tetris game
    score: usize,

    // State of the board
    //state: Option<Vec<u16>>,
}

impl TetrisApp {
    //pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_font(&cc.egui_ctx);

        Self {
            score: 0,
            //state: None,
        }
    }
}

fn setup_font(ctx: &egui::Context) {
    // Change font sizes
    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (egui::TextStyle::Heading, egui::FontId::new(30.0, egui::FontFamily::Proportional)),
        (egui::TextStyle::Body, egui::FontId::new(18.0, egui::FontFamily::Proportional)),
        (egui::TextStyle::Monospace, egui::FontId::new(14.0, egui::FontFamily::Proportional)),
        (egui::TextStyle::Button, egui::FontId::new(14.0, egui::FontFamily::Proportional)),
        (egui::TextStyle::Small, egui::FontId::new(10.0, egui::FontFamily::Proportional)),
    ].into();
    ctx.set_style(style);
}

impl eframe::App for TetrisApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Change central grid
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("grid").show(ui, |ui| {
                ui.label("first row, first column");
                ui.label("first row, second column");
                ui.end_row();

                ui.label("second row, first column");
                ui.label("second row, second column");
                ui.label("second row, third column");
                ui.end_row();

                ui.horizontal(|ui| { ui.label("Same"); ui.label("cell"); });
                ui.label("third row, second column");
                ui.end_row();
            })
        });

        egui::SidePanel::right("side_panel").resizable(false).show(ctx, |ui| {
            egui::Grid::new("side_grid").show(ui, |ui| {
                ui.label("Score:");
                ui.end_row();

                ui.label(format!("{} p", self.score));
                ui.end_row();
            });
        });
    }
}
