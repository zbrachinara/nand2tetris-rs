#[cfg(feature = "native")]
use eframe::NativeOptions;

mod app;
mod lib;
mod utils;

fn main() {
    #[cfg(feature = "native")]
    eframe::run_native(Box::new(app::App::new()), NativeOptions::default())
}
