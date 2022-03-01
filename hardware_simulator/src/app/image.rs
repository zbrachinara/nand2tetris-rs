use eframe::egui::ColorImage;
use include_dir::{include_dir, Dir};
use once_cell::sync::Lazy;
use std::borrow::Cow;
use itertools::Itertools;

pub static IMAGE_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/images");

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
