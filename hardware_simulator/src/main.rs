
#[cfg(feature = "native")]
use eframe::NativeOptions;

mod lib;
mod app;
mod utils;

fn main() {

    #[cfg(feature = "native")]
    eframe::run_native(Box::new(app::App::new()), NativeOptions::default())

}