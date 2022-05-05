use std::borrow::Borrow;
use std::ffi::OsString;
use std::fs::{File, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};

pub fn open_file(path: impl AsRef<Path>, overwrite: bool) -> Result<File, io::Error> {
    if overwrite {
        OpenOptions::new().write(true).create(true).open(path)
    } else {
        OpenOptions::new().write(true).create_new(true).open(path)
    }
}

pub fn calculate_destination<PF, P>(
    original_path: Option<impl AsRef<Path>>,
    default_path: PF,
    extension: &str,
) -> PathBuf
where
    PF: FnOnce() -> P,
    P: AsRef<Path>,
{
    let dest = if let Some(path) = original_path {
        path.as_ref().to_path_buf()
    } else {
        default_path().as_ref().to_path_buf()
    };
    if dest.extension() == Some(OsString::from(extension).borrow()) {
        dest
    } else {
        dest.with_extension(extension)
    }
}
