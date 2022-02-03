use crate::bus_range::BusRange;
use crate::model::builtin::get_builtin;
use crate::model::Chip;
use crate::parser::{
    chip, Argument, Builtin, Chip as ChipRepr, Connection, Implementation, Interface, Symbol,
};
use crate::Span;
use cached::proc_macro::cached;
use itertools::Itertools;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Context {
    root: PathBuf,
}

fn resolve_hdl_file(target: &str, path: impl AsRef<Path>) -> Option<PathBuf> {

    #[cached(
        key = "(String, PathBuf)",
        convert = "{(target.to_string(), path.to_path_buf())}",
        option = true
    )]
    fn inner(target: &str, path: &Path) -> Option<PathBuf> {
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
                Some(path.to_path_buf())
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

    pub fn resolve_chip(&self, target: &str) -> Option<Box<dyn Chip>> {
        let path = resolve_hdl_file(target, &self.root)?;
        let str = fs::read_to_string(path).ok()?;
        let buf = Span::from(str.as_str());
        Some(self.make_hdl(chip(buf).ok()?.1).ok()?)
    }

    pub fn resolve_interface(&self, target: &str) -> Option<Interface> {
        if let Some(chip) = get_builtin(target) {
            Some(chip.interface())
        } else {
            let path = resolve_hdl_file(target, &self.root)?;
            let str = fs::read_to_string(path).ok()?;
            let buf = Span::from(str.as_str());
            Some(chip(buf).ok()?.1.interface())
        }
    }

    pub fn make_hdl(&self, chip_repr: ChipRepr) -> Result<Box<dyn Chip>, ()> {
        match &chip_repr.logic {
            Implementation::Native(connections) => {
                // instantiate all chips this chip depends on

                // get list of all pins and their connections
                // This is done by checking in which `Connection` the name of the pin appears
                let mut pins: HashMap<String, Vec<(usize, BusRange)>> = HashMap::new();
                connections.iter().enumerate().for_each(
                    |(index, Connection { inputs, chip_name })| {
                        let _ = inputs.iter().try_for_each::<_, Result<(), ()>>(
                            |Argument {
                                 internal,
                                 internal_bus,
                                 external,
                                 external_bus,
                             }| {
                                let interface = self.resolve_interface(chip_name).ok_or(())?;

                                let mut insert = |k: String, v: (usize, BusRange)| {
                                    pins.entry(k)
                                        .and_modify(|e| e.push(v.clone()))
                                        .or_insert(vec![v]);
                                };

                                insert(
                                    internal.to_string(),
                                    (index, interface.real_range(internal, internal_bus.clone())?),
                                );

                                if let Symbol::Name(external) = external {
                                    insert(
                                        external.to_string(),
                                        (
                                            index,
                                            interface.real_range(external, external_bus.clone())?,
                                        ),
                                    )
                                }

                                Ok(())
                            },
                        );
                    },
                );

                // check for contradictions (one pin with many sources, incompatible channel sizes, etc)

                // starting from the output pins, build a graph of all connections between chips
                // should work recursively, but also be aware of chips which were already found

                todo!()
            }
            Implementation::Builtin(Builtin { name, .. }) => get_builtin(**name).ok_or(()),
        }
    }
}
