use super::{Builtin, Channel, Chip, Form};
use crate::channel_range::ChannelRange;
use crate::clock_behavior::ClockBehavior;
use crate::Span;
use std::collections::HashMap;

// type PinMap = HashMap<String, ChannelRange>;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ChannelPin {
    ComIn(ChannelRange),
    ComOut(ChannelRange),
    SeqIn(ChannelRange),
    SeqOut(ChannelRange),
}

impl ChannelPin {
    fn unwrap(&self) -> &ChannelRange {
        match self {
            ChannelPin::ComIn(c) => c,
            ChannelPin::ComOut(c) => c,
            ChannelPin::SeqIn(c) => c,
            ChannelPin::SeqOut(c) => c,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Default)]
// pub struct Interface {
//     pub name: String,
//     pub com_in: PinMap,
//     pub com_out: PinMap,
//     pub seq_in: PinMap,
//     pub seq_out: PinMap,
// }
pub struct Interface {
    pub name: String,
    pub map: HashMap<String, ChannelPin>,
}
fn sort_pins(
    inputs: &[Channel],
    outputs: &[Channel],
    clocked: Option<&[Span]>,
) -> HashMap<String, ChannelPin> {
    enum PinIntermediate {
        ComIn(u16),
        ComOut(u16),
        SeqIn(u16),
        SeqOut(u16),
    }

    let clocked = clocked.unwrap_or(&[]);
    println!("{clocked:?}");

    let mut next_input = 0;
    let mut next_output = 0;

    inputs
        .into_iter()
        .map(|Channel { name, size }| {
            let size = size.unwrap_or(1);
            let pin = if clocked.iter().any(|x| **x == **name) {
                PinIntermediate::SeqIn
            } else {
                PinIntermediate::ComIn
            }(size);
            (name.to_string(), pin)
        })
        .chain(outputs.into_iter().map(|Channel { name, size }| {
            let size = size.unwrap_or(1);
            let pin = if clocked.iter().any(|x| **x == **name) {
                PinIntermediate::SeqOut
            } else {
                PinIntermediate::ComOut
            }(size);
            (name.to_string(), pin)
        }))
        .map(|(x, pin)| {
            (
                x,
                match pin {
                    PinIntermediate::ComIn(size) => {
                        next_input += size;
                        ChannelPin::ComIn(ChannelRange::new(next_input - size, next_input - 1))
                    }
                    PinIntermediate::ComOut(size) => {
                        next_output += size;
                        ChannelPin::ComOut(ChannelRange::new(next_output - size, next_output - 1))
                    }
                    PinIntermediate::SeqIn(size) => {
                        next_input += size;
                        ChannelPin::SeqIn(ChannelRange::new(next_input - size, next_input - 1))
                    }
                    PinIntermediate::SeqOut(size) => {
                        next_output += size;
                        ChannelPin::SeqOut(ChannelRange::new(next_output - size, next_output - 1))
                    }
                },
            )
        })
        .collect()
}

impl<'a> Chip<'a> {
    // defines the rules for interacting with the chip using Vec
    pub fn interface(&self) -> Interface {
        if let Form::Builtin(Builtin { ref clocked, .. }) = self.logic {
            Interface {
                name: self.name.to_string(),
                map: sort_pins(
                    &self.in_pins,
                    &self.out_pins,
                    clocked.as_ref().map(|x| x.as_slice()),
                ),
            }
        } else {
            Interface {
                name: self.name.to_string(),
                map: sort_pins(&self.in_pins, &self.out_pins, None),
            }
        }
    }
}

impl Interface {
    pub fn iter_inputs(&self) -> impl Iterator<Item = (&String, &ChannelRange)> {
        self.map.iter().filter_map(|pin| match pin {
            (name, ChannelPin::ComIn(range)) => Some((name, range)),
            (name, ChannelPin::SeqIn(range)) => Some((name, range)),
            _ => None,
        })
    }

    pub fn iter_outputs(&self) -> impl Iterator<Item = (&String, &ChannelRange)> {
        self.map.iter().filter_map(|pin| match pin {
            (name, ChannelPin::ComOut(range)) => Some((name, range)),
            (name, ChannelPin::SeqOut(range)) => Some((name, range)),
            _ => None,
        })
    }

    pub fn iter_all(&self) -> impl Iterator<Item = (&String, &ChannelRange)> {
        self.map.iter().map(|(x, pin)| (x, pin.unwrap()))
    }

    pub fn iter_combinatorial(&self) -> impl Iterator<Item = (&String, &ChannelRange)> {
        self.map.iter().filter_map(|pin| match pin {
            (name, ChannelPin::ComIn(range)) => Some((name, range)),
            (name, ChannelPin::ComOut(range)) => Some((name, range)),
            _ => None,
        })
    }

    pub fn iter_sequential(&self) -> impl Iterator<Item = (&String, &ChannelRange)> {
        self.map.iter().filter_map(|pin| match pin {
            (name, ChannelPin::SeqIn(range)) => Some((name, range)),
            (name, ChannelPin::SeqOut(range)) => Some((name, range)),
            _ => None,
        })
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
        let com_chip = chip(Span::from(COM_CHIP)).unwrap().1.interface();
        assert_eq!(com_chip.name, "And16");
        assert!(matches!(com_chip.map["a"], ChannelPin::ComIn(range) if range.size() == 16));
        assert!(matches!(com_chip.map["b"], ChannelPin::ComIn(range) if range.size() == 16));
        assert!(matches!(com_chip.map["out"], ChannelPin::ComOut(range) if range.size() == 16));

        let seq_chip = chip(Span::from(SEQ_CHIP)).unwrap().1.interface();
        assert_eq!(seq_chip.name, "DFF");
        assert!(matches!(seq_chip.map["in"], ChannelPin::SeqIn(range) if range.size() == 1));
        assert!(matches!(seq_chip.map["out"], ChannelPin::ComOut(range) if range.size() == 1));

        let example_chip = chip(Span::from(EXAMPLE_CHIP)).unwrap().1.interface();
        assert_eq!(example_chip.name, "test");
        assert!(matches!(example_chip.map["a"], ChannelPin::ComIn(range) if range.size() == 2));
        assert!(matches!(example_chip.map["b"], ChannelPin::SeqIn(range) if range.size() == 2));
        assert!(matches!(example_chip.map["c"], ChannelPin::SeqIn(range) if range.size() == 3));
        assert!(matches!(example_chip.map["d"], ChannelPin::ComOut(range) if range.size() == 1));
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
