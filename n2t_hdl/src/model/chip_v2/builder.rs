use super::error::ModelConstructionError;
use super::{builtin, Chip, Id};
use crate::channel_range::ChannelRange;
use crate::model::parser::Interface;
use crate::model::parser::{Chip as ChipRepr, Form};
use std::collections::HashMap;

pub struct ChipBuilder {
    registered: HashMap<String, ChipInfo>,
}

pub struct ChipInfo {
    pub interface: Interface,
    pub chip: Box<dyn Chip>,
}

impl ChipBuilder {
    pub fn new() -> Self {
        Self {
            registered: HashMap::new(),
        }
    }

    pub fn with_builtins(&mut self) {
        self.registered
            .extend([("Nand".to_string(), builtin::nand())]);
    }

    pub fn register_hdl(&mut self, chip: ChipRepr) -> Result<(), ModelConstructionError> {
        let top_interface = chip.interface();
        let ChipRepr {
            name,
            in_pins,
            out_pins,
            logic,
        } = chip;
        let top_router: Vec<(ChannelRange, (Id, ChannelRange))> = Vec::new();

        if let Form::Native(_) = logic {
            todo!()
        } else {
            panic!("dynamic loading of native chips is not yet supported")
        }
    }
}
