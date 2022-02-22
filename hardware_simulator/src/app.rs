use eframe::egui::{self, Align, CtxRef, Layout, TextEdit, Ui, Vec2};
use eframe::epi::Frame;

pub struct App {
    size: Vec2,
}

impl App {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            size: Vec2::new(width as f32, height as f32),
        }
    }
}

impl eframe::epi::App for App {
    fn update(&mut self, ctx: &CtxRef, _: &Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let size = ui.available_size();
            let width = size.x / 2f32;
            let height = size.y;
            let elem_size = Vec2::new(width, height);
            ui.columns(2, |uis| {
                uis[0].add_sized(elem_size, TextEdit::multiline(&mut "when the"));
                uis[1].allocate_ui_with_layout(
                    elem_size,
                    Layout::top_down_justified(Align::Center),
                    |ui| {
                        ui.heading("Pins");
                        self.pin_table(ui);
                    },
                );
            });
        });
    }

    fn name(&self) -> &str {
        "Nand2Tetris"
    }

    fn max_size_points(&self) -> Vec2 {
        self.size.clone()
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
