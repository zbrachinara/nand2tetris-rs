use crate::model::builtin::get_builtin;
use crate::model::Chip;
use crate::parser::{Chip as ChipRepr, chip};
use crate::Span;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

struct Context {
    root: PathBuf,
    cache: HashMap<String, PathBuf>,
}

impl Context {
    fn resolve_hdl_file(target: &str, path: &Path) -> Option<PathBuf> {
        if path.is_dir() {
            path.read_dir()
                .ok()?
                .filter_map(|res| res.ok())
                .filter_map(|dir_entry| Self::resolve_hdl_file(target, &dir_entry.path()))
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

    pub fn resolve_chip(&self, target: &str) -> Option<Box<dyn Chip>> {
        if let Some(chip) = get_builtin(target) {
            Some(chip)
        } else {
            let path = Self::resolve_hdl_file(target, &self.root)?;
            let str = fs::read_to_string(path).ok()?;
            let buf = Span::from(str.as_str());
            Some(self.add_hdl(chip(buf).ok()?.1))
        }
    }

    pub fn add_hdl(&self, chip_repr: ChipRepr) -> Box<dyn Chip> {
        todo!()
    }
}
