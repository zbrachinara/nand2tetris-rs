use super::error::ModelConstructionError;
use super::native::Router;
use super::{builtin, Chip, Id};
use crate::channel_range::ChannelRange;
use crate::model::parser::{Channel, Connection, Interface};
use crate::model::parser::{Chip as ChipRepr, Form};
use bitvec::prelude::*;
use std::collections::HashMap;

pub struct ChipBuilder {
    registered: HashMap<String, ChipInfo>,
}

pub struct ChipInfo {
    pub interface: Interface,
    pub chip: Box<dyn Chip>,
}

impl Clone for ChipInfo {
    fn clone(&self) -> Self {
        Self {
            interface: self.interface.clone(),
            chip: self.chip.boxed_clone(),
        }
    }
}

struct IncompleteBarrier {
    chip: Box<dyn Chip>,
    interface: Interface,
    router: Router,
    default: BitVec,
}

enum Hook {
    Output(Id),
    Input(Id),
}

impl IncompleteBarrier {
    fn new(info: ChipInfo) -> Self {
        // generates a buffer and sets it to the proper size
        let storage_size = info.interface.size_in();
        let mut storage = BitVec::with_capacity(storage_size);
        unsafe {
            storage.set_len(storage_size);
        }
        storage.set_uninitialized(false);

        Self {
            chip: info.chip,
            interface: info.interface,
            router: Router { map: Vec::new() },
            default: storage,
        }
    }
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

        if let Form::Native(conns) = logic {
            if self
                .registered
                .insert(
                    name.to_string(),
                    self.build_native(*name, in_pins, out_pins, conns)?,
                )
                .is_some()
            {
                Err(ModelConstructionError::Rebuilt(name.to_string()))
            } else {
                Ok(())
            }
        } else {
            panic!("dynamic loading of native chips is not yet supported")
        }
    }

    fn build_native(
        &self,
        name: &str,
        in_pins: Vec<Channel>,
        out_pins: Vec<Channel>,
        connections: Vec<Connection>,
    ) -> Result<ChipInfo, ModelConstructionError> {
        let mut id_provider = Id(0);
        let out_id = id_provider.next();

        let edge_map: HashMap<Id, (Vec<Hook>, ChannelRange)> = HashMap::new();

        let connection = connections
            .into_iter()
            .map(|conn| {
                Result::<_, ModelConstructionError>::Ok((
                    id_provider.next(),
                    IncompleteBarrier::new(
                        self.registered
                            .get(*(conn.chip_name))
                            .ok_or(ModelConstructionError::Needs(conn.chip_name.to_string()))?
                            .clone(),
                    ),
                ))
            })
            .try_collect::<HashMap<_, _>>()?;

        todo!()
    }
}
