//! The chips here do not correspond with any user-defined chip, but rather come from that this
//! program represents chips using a graph. Therefore, all connections must be a directed edge
//! between two nodes. This is not possible for, for example, numerical constants, so they must be
//! defined here.

use std::collections::HashMap;
use crate::bus_range::BusRange;
use crate::model::Chip;
use crate::parser::Interface;
use std::iter::once;

fn all_out(size: u16, name: String) -> Interface {
    Interface {
        com_out: once((
            name,
            BusRange {
                start: 0,
                end: size - 1,
            },
        )).collect(),
        ..Default::default()
    }
}

/// Represents a bus. For edges which connect to IN or OUT pins, connect to these instead
#[derive(Debug)]
pub struct BusVChip {
    size: u16,
    interface: Interface,
}

impl BusVChip {
    pub fn new_in(h: HashMap<String, BusRange>) -> Self {
        Self {
            size: h.iter().map(|(_, x)| x.size()).sum(),
            interface: Interface {
                com_out: h,
                ..Default::default()
            },
        }
    }
    pub fn new_out(h: HashMap<String, BusRange>) -> Self {
        Self {
            size: h.iter().map(|(_, x)| x.size()).sum(),
            interface: Interface {
                com_in: h,
                ..Default::default()
            },
        }
    }
}

impl Chip for BusVChip {
    fn interface(&self) -> Interface {
        self.interface.clone()
    }

    fn clock(&mut self) {
        // empty
    }

    fn eval(&mut self, x: &[bool]) -> Vec<bool> {
        x.to_vec()
    }
}

struct ConstVChip {
    value: Vec<bool>,
    interface: Interface,
}

impl ConstVChip {
    fn from_number(mut n: usize, channel_size: u16, name: String) -> Self {
        // TODO: assert that n fits within the channel
        let value = {
            let mut bits = Vec::new();
            while n > 0 {
                bits.push(n & 1 == 1);
                n >>= 1;
            }
            bits
        };
        ConstVChip {
            value,
            interface: all_out(channel_size, name),
        }
    }
    fn from_bool(b: bool, channel_size: u16, name: String) -> Self {
        ConstVChip {
            value: vec![b; channel_size as usize],
            interface: all_out(channel_size, name),
        }
    }
}

impl Chip for ConstVChip {
    fn interface(&self) -> Interface {
        self.interface.clone()
    }

    fn clock(&mut self) {
        // empty
    }

    fn eval(&mut self, _: &[bool]) -> Vec<bool> {
        self.value.clone()
    }
}
