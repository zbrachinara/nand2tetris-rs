mod image;

use derive_more::*;
use eframe::egui::{self, Button, Context, Layout, TextEdit, TextureHandle, Ui, Vec2};
use eframe::epi::{Frame, Storage};
use std::collections::HashMap;
use std::sync::mpsc;

#[derive(From)]
struct SendRecvPair<T> {
    tx: mpsc::SyncSender<T>,
    rx: mpsc::Receiver<T>,
}

pub struct App {
    code: String,
    code_channel: SendRecvPair<String>,
    textures: HashMap<String, TextureHandle>,
}

impl App {
    pub fn new() -> Self {
        Self {
            code: "When the".to_string(),
            code_channel: mpsc::sync_channel(1).into(),
            textures: HashMap::with_capacity(64),
        }
    }
}

impl eframe::epi::App for App {
    fn update(&mut self, ctx: &Context, _: &Frame) {
        // check for an input file, and if received, display it
        if let Ok(str) = self.code_channel.rx.try_recv() {
            self.code = str;
        }

        const BUTTON_SIZE: f32 = 30f32;

        // repaint ui
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.columns(2, |uis| {
                uis[0].vertical_centered_justified(|ui| {
                    ui.allocate_ui_with_layout(
                        Vec2::new(ui.available_width(), BUTTON_SIZE),
                        Layout::right_to_left().with_cross_justify(true),
                        // .with_main_justify(true),
                        |ui| {
                            ui.add(Button::image_and_text(
                                self.textures.get("vcrrewind").unwrap().into(),
                                Vec2::splat(BUTTON_SIZE),
                                "",
                            ));
                            ui.add(Button::image_and_text(
                                self.textures.get("vcrstop").unwrap().into(),
                                Vec2::splat(BUTTON_SIZE),
                                "",
                            ));
                            ui.add(Button::image_and_text(
                                self.textures.get("vcrforward").unwrap().into(),
                                Vec2::splat(BUTTON_SIZE),
                                "",
                            ));
                            ui.add(Button::image_and_text(
                                self.textures.get("vcrfastforward").unwrap().into(),
                                Vec2::splat(BUTTON_SIZE),
                                "",
                            ));
                            if ui.button("Load local file").clicked() {
                                self.load_file();
                            }
                        },
                    );
                    ui.add_sized(
                        ui.available_size(),
                        TextEdit::multiline(&mut self.code).code_editor(),
                    );
                });
                uis[1].vertical_centered_justified(|ui| {
                    ui.heading("Pins");
                    self.pin_table(ui);
                });
            });
        });
    }

    fn setup(&mut self, ctx: &Context, _: &Frame, _: Option<&dyn Storage>) {
        for (name, data) in image::IMAGE_DATA.iter() {
            self.textures.insert(
                name.to_string(),
                ctx.load_texture(name.to_string(), data.clone()),
            );
        }
    }

    fn name(&self) -> &str {
        "Nand2Tetris"
    }

    fn max_size_points(&self) -> Vec2 {
        Vec2::new(f32::INFINITY, f32::INFINITY)
    }
}

impl App {
    fn load_file(&mut self) {
        let tx = self.code_channel.tx.clone();
        #[cfg(feature = "web")]
        wasm_bindgen_futures::spawn_local(async move {
            let f = rfd::AsyncFileDialog::new().pick_file().await;
            if let Some(f) = f {
                let buf = f.read().await;
                if let Ok(str) = String::from_utf8(buf) {
                    // crate::log(&str);
                    tx.send(str).unwrap(); //TODO: Find a better way to send info
                } else {
                    // crate::log(&format!("Could not decode given file"));
                }
            }
        });
        #[cfg(feature = "native")]
        futures_lite::future::block_on(async move {
            let f = rfd::AsyncFileDialog::new().pick_file().await;
            if let Some(f) = f {
                let buf = f.read().await;
                if let Ok(str) = String::from_utf8(buf) {
                    tx.send(str).unwrap();
                }
            }
        })
    }

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
