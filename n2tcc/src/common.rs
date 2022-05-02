use std::fs::{File, OpenOptions};
use std::io;
use std::path::Path;

pub fn open_file(path: impl AsRef<Path>, overwrite: bool) -> Result<File, io::Error> {
    if overwrite {
        OpenOptions::new().write(true).create(true).open(path)
    } else {
        OpenOptions::new().write(true).create_new(true).open(path)
    }
}
