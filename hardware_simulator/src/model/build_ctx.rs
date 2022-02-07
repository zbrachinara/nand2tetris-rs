use super::parser::{chip, Builtin, Chip as ChipRepr, Connection, Implementation, Interface};
use crate::model::builtin::get_builtin;
use crate::model::native::{
    build::edges_from_connections, vchip::VirtualBus, ConnEdge, NativeChip,
};
use crate::model::Chip;
use crate::Span;
use cached::proc_macro::cached;
use itertools::Itertools;
use petgraph::data::{Element, FromElements};
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use std::ffi::OsStr;
use std::fs;
use std::iter::once;
use std::path::{Path, PathBuf};

pub struct Context {
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

                let Interface {
                    com_in, com_out, ..
                } = chip_repr.interface();
                let (input, output) = (
                    VirtualBus::new_in(com_in.clone()),
                    VirtualBus::new_out(com_out.clone()),
                );
                println!("External interface: \n{input:?}\n{output:?}");

                // instantiate all chips this chip depends on
                let mut dependents = connections
                    .iter()
                    .filter_map(|Connection { chip_name, .. }| {
                        self.resolve_chip_maybe_builtin(**chip_name)
                    })
                    .chain(once(Box::new(input) as Box<dyn Chip>))
                    .chain(once(Box::new(output) as Box<dyn Chip>))
                    .collect_vec();

                // let mut graph = Graph::<_, ConnEdge>::from_elements(
                //     dependents.map(|chip| Element::Node { weight: chip }),
                // );

                // get list of all pins and their connections
                // This is done by checking in which `Connection` the name of the pin appears
                let pins = edges_from_connections(connections, &mut dependents);

                println!("{pins:#?}");

                // starting from the output pins, build a graph of all connections between chips
                // should work recursively, but also be aware of chips which were already found

                let mut graph = Graph::<_, ConnEdge>::from_elements(
                    dependents.into_iter().map(|x| Element::Node { weight: x }),
                );

                // check for contradictions (one pin with many sources, incompatible channel sizes, etc)
                // while changing edge sets to pairs
                for (name, edge_set) in pins {
                    let input = edge_set.input.ok_or(())?;
                    if edge_set.outputs.len() == 0 {
                        println!("No output!");
                        return Err(());
                    }
                    for output in edge_set.outputs {
                        if input.range.size() == output.range.size() {
                            // TODO: Function to determine whether combinatorial or sequential
                            graph.add_edge(
                                NodeIndex::new(input.index),
                                NodeIndex::new(output.index),
                                ConnEdge::Combinatorial {
                                    range: output.range,
                                    buf: Vec::with_capacity(input.range.size() as usize),
                                },
                            );
                        } else {
                            println!("No input!");
                            return Err(());
                        }
                    }
                }

                Ok(Box::new(NativeChip {
                    conn_graph: graph,
                    interface: chip_repr.interface(),
                }))
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
