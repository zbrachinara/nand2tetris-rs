use crate::model::chip::builtin::get_builtin;
use crate::model::chip::native::build::native_chip;
use crate::model::chip::Chip;
use crate::model::parser::{create_chip, Builtin, Chip as ChipRepr, Implementation};
use crate::Span;
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

    pub fn add_hdl(&mut self, path: impl AsRef<Path>) -> Result<(), ()> {
        fn inner(ctx: &mut ChipBuilder, path: &Path) -> Result<Chip, ()> {
            if path.extension() == Some(OsStr::new("hdl")) {
                let str = fs::read_to_string(path).map_err(|_| ())?;
                let buf = Span::from(str.as_str());
                ctx.make_hdl(create_chip(buf).map_err(|_| ())?)
            } else {
                Err(())
            }
        }

        if let Ok(chip) = inner(self, path.as_ref()) {
            self.chips.insert(chip.interface().name, chip);
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn resolve_chip(&mut self, target: &str) -> Result<Chip, ()> {
        get_builtin(target)
            .map(|x| Chip::Builtin(x))
            .or_else(|| self.chips.get(&target.to_string()).cloned())
            .ok_or(())
    }

    fn make_hdl(&mut self, chip_repr: ChipRepr) -> Result<Chip, ()> {
        let interface = chip_repr.interface();
        match chip_repr.logic {
            Implementation::Native(connections) => {
                native_chip(self, interface, connections).map(|x| Chip::Native(x))
            }
            Implementation::Builtin(Builtin { name, .. }) => {
                get_builtin(*name).map(|x| Chip::Builtin(x)).ok_or(())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn general() {
        let mut dir = std::env::current_dir().unwrap();
        dir.push("../test_files");

        let mut ctx = ChipBuilder::new();
        assert!(matches!(ctx.add_hdl(dir.join("Not.hdl")), Ok(_)));
        assert!(matches!(ctx.add_hdl(dir.join("And.hdl")), Ok(_)));
        assert!(matches!(ctx.add_hdl(dir.join("DMux.hdl")), Ok(_)));
        assert!(matches!(ctx.add_hdl(dir.join("DMux4Way.hdl")), Ok(_)));
        assert!(matches!(ctx.add_hdl(dir.join("DMux8Way.hdl")), Ok(_)));
        assert!(matches!(ctx.resolve_chip("DMux8Way"), Ok(_)));
    }
}
