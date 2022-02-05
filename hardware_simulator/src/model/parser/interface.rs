use crate::bus_range::BusRange;
use super::{Builtin, Chip, Implementation, Pin};
use crate::Span;
use std::collections::HashMap;

type PinMap = HashMap<String, BusRange>;

#[derive(PartialEq, Debug, Clone, Default)]
pub struct Interface {
    pub com_in: PinMap,
    pub com_out: PinMap,
    pub seq_in: PinMap,
    pub seq_out: PinMap,
}

fn to_map(pins: Vec<Pin>, mut next: u16) -> (PinMap, u16) {
    let map = pins
        .into_iter()
        .map(|Pin { name, size }| {
            let size = if let Some(n) = size { n } else { 1 };
            let range = BusRange {
                start: next,
                end: next + size - 1,
            };
            next += size;
            ((*name).to_string(), range)
        })
        .collect();

    (map, next)
}

fn split_seq_com(pins: &Vec<Pin>, seq_names: &Vec<Span>) -> (PinMap, PinMap) {
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
        if let Implementation::Builtin(Builtin { ref clocked, .. }) = self.logic {
            let empty = vec![];
            let clocked = if let Some(x) = clocked { x } else { &empty };
            let (seq_in, com_in) = split_seq_com(&self.in_pins, clocked);
            let (seq_out, com_out) = split_seq_com(&self.out_pins, clocked);

            Interface {
                seq_in,
                com_in,
                seq_out,
                com_out,
            }
        } else {
            Interface {
                com_in: to_map(self.in_pins.clone(), 0).0,
                com_out: to_map(self.out_pins.clone(), 0).0,
                seq_in: HashMap::with_capacity(0),
                seq_out: HashMap::with_capacity(0),
            }
        }
    }
}

impl Interface {
    pub fn real_range(&self, name: &str, relative: Option<BusRange>) -> Result<BusRange, ()> {
        let raw = self
            .com_in
            .iter()
            .chain(self.com_out.iter())
            .chain(self.seq_in.iter())
            .chain(self.seq_out.iter())
            .find(|(n, _)| n.as_str() == name)
            .map(|(_, range)| range)
            .ok_or(())?;
        if let Some(relative) = relative {
            if raw.size() < relative.size() {
                return Err(());
            }
            // offset the provided relative range
            Ok(BusRange {
                start: raw.start + relative.start,
                end: raw.start + relative.end,
            })
        } else {
            Ok(raw.clone())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::model::parser::chip;
    use std::iter::once;

    const COM_CHIP: &'static str = include_str!("../../../../test_files/And16.hdl");
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
                com_in: [
                    ("a".to_string(), BusRange { start: 0, end: 15 }),
                    ("b".to_string(), BusRange { start: 16, end: 31 })
                ]
                .into_iter()
                .collect(),
                com_out: [("out".to_string(), BusRange { start: 0, end: 15 })]
                    .into_iter()
                    .collect(),
                seq_in: Default::default(),
                seq_out: Default::default()
            }
        );

        let (_, seq_chip) = chip(Span::from(SEQ_CHIP)).unwrap();
        assert_eq!(
            seq_chip.interface(),
            Interface {
                com_in: Default::default(),
                com_out: once(("out".to_string(), BusRange { start: 0, end: 0 })).collect(),
                seq_in: once(("in".to_string(), BusRange { start: 0, end: 0 })).collect(),
                seq_out: Default::default()
            }
        );

        let (_, example_chip) = chip(Span::from(EXAMPLE_CHIP)).unwrap();
        assert_eq!(
            example_chip.interface(),
            Interface {
                com_in: once(("a".to_string(), BusRange { start: 5, end: 6 })).collect(),
                com_out: once(("d".to_string(), BusRange { start: 0, end: 0 })).collect(),
                seq_in: [
                    ("b".to_string(), BusRange { start: 0, end: 1 }),
                    ("c".to_string(), BusRange { start: 2, end: 4 }),
                ]
                .into_iter()
                .collect(),
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
                .real_range("b", Some(BusRange { start: 0, end: 7 })),
            Ok(BusRange { start: 16, end: 23 })
        );
        assert_eq!(
            com_chip.interface().real_range("b", None),
            Ok(BusRange { start: 16, end: 31 })
        )
    }
}