use crate::model::chip::builtin::get_builtin;
use crate::model::chip::error::ModelConstructionError;
use crate::model::chip::native::build::native_chip;
use crate::model::chip::Chip;
use crate::model::parser::{create_chip, Builtin, Chip as ChipRepr, Form};
use crate::Span;
use anyhow::anyhow;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

pub struct ChipBuilder {
    chips: HashMap<String, Chip>,
}

impl ChipBuilder {
    pub fn new() -> Self {
        Self {
            chips: HashMap::new(),
        }
    }

    pub fn add_hdl(&mut self, path: impl AsRef<Path>) -> Result<(), ModelConstructionError> {
        fn inner(ctx: &mut ChipBuilder, path: &Path) -> Result<Chip, ModelConstructionError> {
            let name = path
                .file_stem()
                .ok_or(ModelConstructionError::Unk(Some(anyhow!(
                    "Could not read the path: {path:?}"
                ))))?
                .to_string_lossy()
                .to_string();
            match path.extension() {
                Some(x) if x == OsStr::new("hdl") => {
                    let str = fs::read_to_string(path)
                        .map_err(|_| ModelConstructionError::ChipNotFound(name))?;
                    let buf = Span::from(str.as_str());
                    let chip =
                        create_chip(buf).map_err(|_| ModelConstructionError::HdlParseError)?;
                    ctx.make_hdl(chip)
                }
                Some(_) => Err(ModelConstructionError::ChipNotFound(name)),
                None => Err(ModelConstructionError::Unk(None)),
            }
        }

        let chip = inner(self, path.as_ref())?;
        self.chips.insert(chip.interface().name, chip);
        Ok(())
    }

    pub fn resolve_chip(&self, target: &str) -> Result<Chip, ModelConstructionError> {
        get_builtin(target)
            .map(|x| Chip::Builtin(x))
            .or_else(|| self.chips.get(&target.to_string()).cloned())
            .ok_or(ModelConstructionError::ChipNotFound(target.to_string()))
    }

    fn make_hdl(&mut self, chip_repr: ChipRepr) -> Result<Chip, ModelConstructionError> {
        let interface = chip_repr.interface();
        match chip_repr.logic {
            Form::Native(connections) => native_chip(self, interface, connections)
                .map(|x| Chip::Native(x))
                .map_err(|_| {
                    ModelConstructionError::Unk(Some(anyhow!(
                        "Error somewhere in construction of native chip"
                    )))
                }),
            Form::Builtin(Builtin { name, .. }) => get_builtin(*name)
                .map(|x| Chip::Builtin(x))
                .ok_or(ModelConstructionError::ChipNotFound(name.to_string())),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use petgraph::dot::Dot;

    #[test]
    fn general() {
        let mut dir = std::env::current_dir().unwrap();
        dir.push("../test_files/01");

        let mut ctx = ChipBuilder::new();
        assert!(matches!(ctx.add_hdl(dir.join("Not.hdl")), Ok(_)));
        assert!(matches!(ctx.add_hdl(dir.join("And.hdl")), Ok(_)));
        assert!(matches!(ctx.add_hdl(dir.join("DMux.hdl")), Ok(_)));
        assert!(matches!(ctx.add_hdl(dir.join("DMux4Way.hdl")), Ok(_)));
        assert!(matches!(ctx.add_hdl(dir.join("DMux8Way.hdl")), Ok(_)));
        {
            let chip = ctx.resolve_chip("DMux8Way").unwrap();
            assert!(!chip.is_clocked());
            if let Chip::Native(chip) = chip {
                println!("{}", Dot::new(&chip.conn_graph))
            }
        }

        {
            let chip = ctx.resolve_chip("Not").unwrap();
            assert!(!chip.is_clocked());
        }
    }
}
