use super::{Builtin, Channel, Chip, Form};
use crate::channel_range::ChannelRange;
use crate::clock_behavior::ClockBehavior;
use crate::Span;
use std::collections::HashMap;

type PinMap = HashMap<String, ChannelRange>;

#[derive(PartialEq, Debug, Clone, Default)]
pub struct Interface {
    pub name: String,
    pub com_in: PinMap,
    pub com_out: PinMap,
    pub seq_in: PinMap,
    pub seq_out: PinMap,
}

fn to_map(pins: Vec<Channel>, mut next: u16) -> (PinMap, u16) {
    let map = pins
        .into_iter()
        .map(|Channel { name, size }| {
            let size = size.unwrap_or(1);
            let range = ChannelRange::new(next, next + size - 1);
            next += size;
            ((*name).to_string(), range)
        })
        .collect();

    (map, next)
}

fn split_seq_com(pins: &Vec<Channel>, seq_names: &Vec<Span>) -> (PinMap, PinMap) {
    let (in_seq, in_com) = pins.iter().cloned().partition(|pin| {
        seq_names
            .iter()
            .find(|name| ***name == *(pin.name))
            .is_some()
    });
    let (seq_in, next) = to_map(in_seq, 0);
    let (com_in, _) = to_map(in_com, next);

    (seq_in, com_in)
}

impl<'a> Chip<'a> {
    // defines the rules for interacting with the chip using Vec
    pub fn interface(&self) -> Interface {
        if let Form::Builtin(Builtin { ref clocked, .. }) = self.logic {
            let empty = vec![];
            let clocked = if let Some(x) = clocked { x } else { &empty };
            let (seq_in, com_in) = split_seq_com(&self.in_pins, clocked);
            let (seq_out, com_out) = split_seq_com(&self.out_pins, clocked);

            Interface {
                name: self.name.to_string(),
                seq_in,
                com_in,
                seq_out,
                com_out,
            }
        } else {
            Interface {
                name: self.name.to_string(),
                com_in: to_map(self.in_pins.clone(), 0).0,
                com_out: to_map(self.out_pins.clone(), 0).0,
                ..Default::default()
            }
        }
    }
}

impl Interface {
    fn iter_inputs(&self) -> impl Iterator<Item = (&String, &ChannelRange)> {
        self.com_in.iter().chain(self.seq_in.iter())
    }

    fn iter_outputs(&self) -> impl Iterator<Item = (&String, &ChannelRange)> {
        self.com_out.iter().chain(self.seq_out.iter())
    }

    fn iter_all(&self) -> impl Iterator<Item = (&String, &ChannelRange)> {
        self.iter_inputs().chain(self.iter_outputs())
    }

    fn iter_combinatorial(&self) -> impl Iterator<Item = (&String, &ChannelRange)> {
        self.com_in.iter().chain(self.com_out.iter())
    }

    fn iter_sequential(&self) -> impl Iterator<Item = (&String, &ChannelRange)> {
        self.seq_in.iter().chain(self.seq_out.iter())
    }

    pub fn real_range(
        &self,
        name: &str,
        relative: Option<&ChannelRange>,
    ) -> Result<ChannelRange, ()> {
        let raw = self
            .iter_all()
            .find(|(n, _)| n.as_str() == name)
            .map(|(_, range)| range)
            .ok_or(())?;
        if let Some(relative) = relative {
            if raw.size() < relative.size() {
                return Err(());
            }
            // offset the provided relative range
            Ok(ChannelRange::new(
                raw.start + relative.start,
                raw.start + relative.end,
            ))
        } else {
            Ok(raw.clone())
        }
    }

    pub fn is_input(&self, name: &str) -> bool {
        self.iter_inputs().find(|(s, _)| *s == name).is_some()
    }

    pub fn clocked(&self, name: &str) -> ClockBehavior {
        self.iter_combinatorial()
            .find(|(n, _)| *n == name)
            .map_or(ClockBehavior::Sequential, |_| ClockBehavior::Combinatorial)
    }

    pub fn has_clocked(&self) -> bool {
        self.iter_sequential().count() > 0
    }

    pub fn size_in(&self) -> usize {
        self.iter_inputs()
            .map(|(_, channel_range)| channel_range.end())
            .max()
            .map(|x| x as usize + 1)
            .unwrap_or(0)
    }

    pub fn size_out(&self) -> usize {
        self.iter_outputs()
            .map(|(_, channel_range)| channel_range.end())
            .max()
            .map(|x| x as usize + 1)
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::model::parser::chip;

    const COM_CHIP: &'static str = include_str!("../../../../test_files/01/And16.hdl");
    const SEQ_CHIP: &'static str = include_str!("../../../../test_files/DFF.hdl");
    const EXAMPLE_CHIP: &'static str = "\
CHIP test {
    IN a[2], b[2], c[3];
    OUT d;
    BUILTIN bruh;
    CLOCKED b, c;
}
        ";

    #[test]
    fn test_gen_interface() {
        let (_, com_chip) = chip(Span::from(COM_CHIP)).unwrap();
        assert_eq!(
            com_chip.interface(),
            Interface {
                name: "And16".to_string(),
                com_in: [
                    ("a".to_string(), ChannelRange::new(0, 15)),
                    ("b".to_string(), ChannelRange::new(16, 31)),
                ]
                .into(),
                com_out: [("out".to_string(), ChannelRange::new(0, 15))].into(),
                seq_in: Default::default(),
                seq_out: Default::default()
            }
        );

        let (_, seq_chip) = chip(Span::from(SEQ_CHIP)).unwrap();
        assert_eq!(
            seq_chip.interface(),
            Interface {
                name: String::from("DFF"),
                com_in: Default::default(),
                com_out: [("out".to_string(), ChannelRange::new(0, 0))].into(),
                seq_in: [("in".to_string(), ChannelRange::new(0, 0))].into(),
                seq_out: Default::default()
            }
        );

        let (_, example_chip) = chip(Span::from(EXAMPLE_CHIP)).unwrap();
        assert_eq!(
            example_chip.interface(),
            Interface {
                name: "test".to_string(),
                com_in: [("a".to_string(), ChannelRange::new(5, 6))].into(),
                com_out: [("d".to_string(), ChannelRange::new(0, 0))].into(),
                seq_in: [
                    ("b".to_string(), ChannelRange::new(0, 1)),
                    ("c".to_string(), ChannelRange::new(2, 4)),
                ]
                .into(),
                seq_out: Default::default()
            }
        )
    }

    #[test]
    fn test_real_range() {
        let (_, com_chip) = chip(Span::from(COM_CHIP)).unwrap();
        assert_eq!(
            com_chip
                .interface()
                .real_range("b", Some(&ChannelRange::new(0, 7))),
            Ok(ChannelRange::new(16, 23))
        );
        assert_eq!(
            com_chip.interface().real_range("b", None),
            Ok(ChannelRange::new(16, 31))
        )
    }
}
