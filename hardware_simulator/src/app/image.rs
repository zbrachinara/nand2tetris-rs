use eframe::egui::ColorImage;
#[cfg(feature = "compile_resources")]
use include_dir::{include_dir, Dir};
use itertools::Itertools;
use once_cell::sync::Lazy;
use std::borrow::Cow;
use std::fs;

#[cfg(feature = "compile_resources")]
static IMAGE_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/images");

#[cfg(feature = "compile_resources")]
pub static IMAGE_DATA: Lazy<Vec<(Cow<str>, ColorImage)>> = Lazy::new(|| {
    IMAGE_DIR
        .files()
        .map(|file| {
            let name = file.path().file_stem().unwrap().to_string_lossy();
            let image = image::load_from_memory(file.contents()).unwrap();
            let color_image = ColorImage::from_rgba_unmultiplied(
                [image.width() as _, image.height() as _],
                image.to_rgba8().as_flat_samples().as_slice(),
            );
            (name, color_image)
        })
        .collect_vec()
});

#[cfg(not(feature = "compile_resources"))]
pub static IMAGE_DATA: Lazy<Vec<(Cow<str>, ColorImage)>> = Lazy::new(|| {
    //
    let path = std::env::current_dir()
        .unwrap()
        .join("hardware_simulator/images");
    fs::read_dir(path)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type().unwrap().is_file() {
                Some(entry.path())
            } else {
                None
            }
        })
        .map(|path| {
            let name = path.file_stem().unwrap().to_string_lossy().to_string();
            let image =
                image::load_from_memory(fs::read(path.clone()).unwrap().as_slice()).unwrap();
            let color_image = ColorImage::from_rgba_unmultiplied(
                [image.width() as _, image.height() as _],
                image.to_rgba8().as_flat_samples().as_slice(),
            );
            (Cow::Owned(name), color_image)
        })
        .collect_vec()
});
