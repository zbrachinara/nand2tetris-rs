//! The chips here do not correspond with any user-defined chip, but rather come from that this
//! program represents chips using a graph. Therefore, all connections must be a directed edge
//! between two nodes. This is not possible for, for example, numerical constants, so they must be
//! defined here.

use crate::bus_range::BusRange;
use crate::model::Chip;
use crate::parser::Interface;
use std::iter::once;

const DEFAULT_NAME: &'static str = "_any";

fn all_in(size: u16) -> Interface {
    Interface {
        com_in: once((
            DEFAULT_NAME.to_string(),
            BusRange {
                start: 0,
                end: size - 1,
            },
        )).collect(),
        ..Default::default()
    }
}

fn all_out(size: u16) -> Interface {
    Interface {
        com_out: once((
            DEFAULT_NAME.to_string(),
            BusRange {
                start: 0,
                end: size - 1,
            },
        )).collect(),
        ..Default::default()
    }
}

/// Represents a bus. For edges which connect to IN or OUT pins, connect to these instead
pub struct BusVChip {
    size: u16,
    interface: Interface,
}

impl BusVChip {
    pub fn new_in(size: u16) -> Self {
        Self {
            size,
            interface: all_out(size),
        }
    }
    pub fn new_out(size: u16) -> Self {
        Self {
            size,
            interface: all_in(size),
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
    fn from_number(mut n: usize, channel_size: u16) -> Self {
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
            interface: all_out(channel_size),
        }
    }
    fn from_bool(b: bool, channel_size: u16) -> Self {
        ConstVChip {
            value: vec![b; channel_size as usize],
            interface: all_out(channel_size),
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
