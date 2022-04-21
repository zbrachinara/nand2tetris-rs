//! The chips here do not correspond with any user-defined chip, but rather come from that this
//! program represents chips using a graph. Therefore, all connections must be a directed edge
//! between two nodes. This is not possible for, for example, numerical constants, so they must be
//! defined here.

use crate::channel_range::ChannelRange;
use crate::model::chip::{Chip, ChipObject};
use crate::model::parser::Interface;
use bitvec::prelude::*;
use std::collections::HashMap;

/// Represents a bus. For edges which connect to IN or OUT pins, connect to these instead
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct VirtualBus {
    size: u16,
    interface: Interface,
}

impl VirtualBus {
    pub fn new_in(h: HashMap<String, ChannelRange>) -> Chip {
        Chip::Builtin(Box::new(Self {
            size: h.iter().map(|(_, x)| x.size() as u16).sum(),
            interface: Interface {
                name: "_Input".to_string(),
                com_out: h,
                ..Default::default()
            },
        }))
    }

    pub fn new_out(h: HashMap<String, ChannelRange>) -> Chip {
        Chip::Builtin(Box::new(Self {
            size: h.iter().map(|(_, x)| x.size() as u16).sum(),
            interface: Interface {
                name: "_Output".to_string(),
                com_in: h,
                ..Default::default()
            },
        }))
    }
}

impl ChipObject for VirtualBus {
    fn interface(&self) -> Interface {
        self.interface.clone()
    }

    fn clock(&mut self) {
        // empty
    }

    fn eval(&mut self, x: &BitSlice) -> BitVec {
        x.to_bitvec()
    }

    fn chip_clone(&self) -> Box<dyn ChipObject> {
        Box::new(self.clone())
    }
}

// #[derive(Debug, Clone)]
// struct VirtualConst {
//     value: Vec<bool>,
//     interface: Interface,
// }
//
// #[allow(dead_code)]
// impl VirtualConst {
//     fn from_number(mut n: usize, channel_size: u16, name: String) -> Self {
//         // TODO: assert that n fits within the channel
//         let value = {
//             let mut bits = Vec::new();
//             while n > 0 {
//                 bits.push(n & 1 == 1);
//                 n >>= 1;
//             }
//             bits
//         };
//         VirtualConst {
//             value,
//             interface: all_out(channel_size, name),
//         }
//     }
//     fn from_bool(b: bool, channel_size: u16, name: String) -> Self {
//         VirtualConst {
//             value: vec![b; channel_size as usize],
//             interface: all_out(channel_size, name),
//         }
//     }
// }
//
// impl ChipObject for VirtualConst {
//     fn interface(&self) -> Interface {
//         self.interface.clone()
//     }
//
//     fn clock(&mut self) {
//         // empty
//     }
//
//     fn eval(&mut self, _: &[bool]) -> Vec<bool> {
//         self.value.clone()
//     }
//
//     fn chip_clone(&self) -> Box<dyn ChipObject> {
//         Box::new(self.clone())
//     }
// }
