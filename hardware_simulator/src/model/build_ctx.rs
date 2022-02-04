use crate::model::builtin::get_builtin;
use crate::model::native::{BusVChip, connections_by_pin};
use crate::model::Chip;
use crate::parser::{chip, Builtin, Chip as ChipRepr, Connection, Implementation, Interface};
use crate::Span;
use cached::proc_macro::cached;
use itertools::Itertools;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Context {
    root: PathBuf,
}

fn resolve_hdl_file(target: &str, path: impl AsRef<Path>) -> Option<String> {
    #[cached(key = "String", convert = "{target.to_string()}", option = true)]
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

impl Context {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            root: path.as_ref().to_path_buf(),
        }
    }

    fn resolve_chip_maybe_builtin(&self, target: &str) -> Option<Box<dyn Chip>> {
        get_builtin(target).or_else(|| self.resolve_chip(target))
    }

    pub fn resolve_chip(&self, target: &str) -> Option<Box<dyn Chip>> {
        let str = resolve_hdl_file(target, &self.root)?;
        let buf = Span::from(str.as_str());
        Some(self.make_hdl(chip(buf).ok()?.1).ok()?)
    }

    pub fn make_hdl(&self, chip_repr: ChipRepr) -> Result<Box<dyn Chip>, ()> {
        match &chip_repr.logic {
            Implementation::Native(connections) => {

                println!("Evaluating {}", chip_repr.name);

                let Interface { com_in, com_out, .. } = chip_repr.interface();
                let (input, output) = (BusVChip::new_in(com_in), BusVChip::new_out(com_out));
                println!("External interface: \n{input:?}\n{output:?}");

                // instantiate all chips this chip depends on
                let dependents = connections
                    .iter()
                    .filter_map(|Connection { chip_name, .. }| {
                        self.resolve_chip_maybe_builtin(**chip_name)
                    })
                    .collect_vec();

                // get list of all pins and their connections
                // This is done by checking in which `Connection` the name of the pin appears
                let pins = connections_by_pin(connections, &dependents);

                println!("{pins:?}");

                // check for contradictions (one pin with many sources, incompatible channel sizes, etc)

                // starting from the output pins, build a graph of all connections between chips
                // should work recursively, but also be aware of chips which were already found

                todo!()
            }
            Implementation::Builtin(Builtin { name, .. }) => get_builtin(**name).ok_or(()),
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

        let ctx = Context::new(dir);
        assert!(matches!(ctx.resolve_chip("DMux8Way"), Some(_)));
    }
}
