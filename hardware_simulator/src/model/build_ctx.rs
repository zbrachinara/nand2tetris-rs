use crate::model::builtin::get_builtin;
use crate::model::Chip;
use crate::parser::{chip, Chip as ChipRepr};
use crate::Span;
use cached::proc_macro::cached;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

struct Context {
    root: PathBuf,
}

#[cached(
    key = "(String, PathBuf)",
    convert = "{(target.to_string(), path.to_path_buf())}",
    option = true
)]
fn resolve_hdl_file(target: &str, path: &Path) -> Option<PathBuf> {
    if path.is_dir() {
        path.read_dir()
            .ok()?
            .filter_map(|res| res.ok())
            .filter_map(|dir_entry| resolve_hdl_file(target, &dir_entry.path()))
            .next()
    } else if path.is_file() {
        if path.extension() == Some(OsStr::new("hdl"))
            && path.file_name() == Some(OsStr::new(target))
        {
            Some(path.to_path_buf())
        } else {
            None
        }
    } else {
        None
    }
}

impl Context {
    pub fn resolve_chip(&self, target: &str) -> Option<Box<dyn Chip>> {
        if let Some(chip) = get_builtin(target) {
            Some(chip)
        } else {
            let path = resolve_hdl_file(target, &self.root)?;
            let str = fs::read_to_string(path).ok()?;
            let buf = Span::from(str.as_str());
            Some(self.make_hdl(chip(buf).ok()?.1).ok()?)
        }
    }

    pub fn make_hdl(&self, chip_repr: ChipRepr) -> Result<Box<dyn Chip>, ()> {
        todo!()
    }
}
