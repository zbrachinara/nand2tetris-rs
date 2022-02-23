use eframe::egui::{self, Align, Context, Layout, TextEdit, Ui, Vec2};
use eframe::epi::Frame;

pub struct App {
    code: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            code: "When the".to_string(),
        }
    }
}

impl eframe::epi::App for App {
    fn update(&mut self, ctx: &Context, _: &Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let size = ui.available_size();
            let width = size.x / 2f32;
            let height = size.y;
            let elem_size = Vec2::new(width, height);
            ui.columns(2, |uis| {
                uis[0].add_sized(
                    elem_size,
                    TextEdit::multiline(&mut self.code).code_editor(),
                );
                uis[1].allocate_ui_with_layout(
                    elem_size,
                    Layout::top_down_justified(Align::Center),
                    |ui| {
                        ui.heading("Pins");
                        self.pin_table(ui);
                    },
                );
            })
        });
    }

    fn name(&self) -> &str {
        "Nand2Tetris"
    }

    fn max_size_points(&self) -> Vec2 {
        Vec2::new(f32::INFINITY, f32::INFINITY)
    }
}

impl App {
    fn pin_table(&self, ui: &mut Ui) {
        ui.with_layout(
            egui::Layout::centered_and_justified(egui::Direction::TopDown),
            |ui| {
                egui::Grid::new("_external_pin_table")
                    .num_columns(2)
                    .show(ui, |ui| {
                        ui.label("Pin Name");
                        ui.label("Pin Value");
                        ui.end_row();
                    });
            },
        );
    }
}
