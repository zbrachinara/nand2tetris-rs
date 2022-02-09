use crate::model::parser::{chip, Builtin, Chip as ChipRepr, Implementation};
use crate::model::chip::builtin::get_builtin;
use crate::model::chip::native::build::native_chip;
use crate::model::chip::Chip;
use crate::Span;
use cached::proc_macro::cached;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

pub struct FileContext {
    root: PathBuf,
}

fn resolve_hdl_file(target: &str, path: impl AsRef<Path>) -> Option<String> {
    #[cached(key = "String", convert = "{target.to_string()}", option = true)] // TODO: Clear cache on directory change
    fn inner(target: &str, path: &Path) -> Option<String> {
        if path.is_dir() {
            path.read_dir()
                .ok()?
                .filter_map(|res| res.ok())
                .filter_map(|dir_entry| resolve_hdl_file(target, &dir_entry.path()))
                .next()
        } else if path.is_file() {
            if path.extension() == Some(OsStr::new("hdl"))
                && path.file_stem() == Some(OsStr::new(target))
            {
                Some(fs::read_to_string(path).unwrap())
            } else {
                None
            }
        } else {
            None
        }
    }

    inner(target, path.as_ref())
}

impl FileContext {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            root: path.as_ref().to_path_buf(),
        }
    }

    pub fn resolve_chip_maybe_builtin(&self, target: &str) -> Option<Box<dyn Chip>> {
        get_builtin(target).or_else(|| self.resolve_chip(target))
    }

    pub fn resolve_chip(&self, target: &str) -> Option<Box<dyn Chip>> {
        let str = resolve_hdl_file(target, &self.root)?;
        let buf = Span::from(str.as_str());
        Some(self.make_hdl(chip(buf).ok()?.1).ok()?)
    }

    pub fn make_hdl(&self, chip_repr: ChipRepr) -> Result<Box<dyn Chip>, ()> {
        let interface = chip_repr.interface();
        match chip_repr.logic {
            Implementation::Native(connections) => native_chip(&self, interface, connections),
            Implementation::Builtin(Builtin { name, .. }) => get_builtin(*name).ok_or(()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn general() {
        let mut dir = std::env::current_dir().unwrap();
        println!("{dir:?}");
        dir.push("../test_files");

        let ctx = FileContext::new(dir);
        assert!(matches!(ctx.resolve_chip("DMux8Way"), Some(_)));
    }
}
