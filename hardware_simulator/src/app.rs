use std::time::Duration;
use eframe::egui::{CtxRef, Rgba, Vec2};
use eframe::epi::{Frame, Storage};

pub struct App {
    size: Vec2,
}

impl App {
    pub fn new(width: u32, height: u32) -> Self {
        crate::log("Starting to make the app");
        Self {
            size: Vec2::new(width as f32, height as f32)
        }
    }
}

impl eframe::epi::App for App {
    fn update(&mut self, ctx: &CtxRef, frame: &Frame) {
        crate::log("Updating correctly");
    }

    fn name(&self) -> &str {
        "Nand2Tetris"
    }

    fn max_size_points(&self) -> Vec2 {
        self.size.clone()
    }
}
