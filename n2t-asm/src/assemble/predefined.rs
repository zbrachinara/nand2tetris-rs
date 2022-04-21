use crate::assemble::symbol_table::Address;
use crate::parse::{CExpr, Dst, JumpCondition, Source};

pub const LAST_PHYSICAL_ADDRESS: u16 = 0x4000 - 1;

pub const SYMBOLS: &[(&str, Address)] = &[
    ("R0", Address::Ram(0)),
    ("R1", Address::Ram(1)),
    ("R2", Address::Ram(2)),
    ("R3", Address::Ram(3)),
    ("R4", Address::Ram(4)),
    ("R5", Address::Ram(5)),
    ("R6", Address::Ram(6)),
    ("R7", Address::Ram(7)),
    ("R8", Address::Ram(8)),
    ("R9", Address::Ram(9)),
    ("R10", Address::Ram(10)),
    ("R11", Address::Ram(11)),
    ("R12", Address::Ram(12)),
    ("R13", Address::Ram(13)),
    ("R14", Address::Ram(14)),
    ("R15", Address::Ram(15)),
    ("SP", Address::Ram(0)),
    ("LCL", Address::Ram(1)),
    ("ARG", Address::Ram(2)),
    ("THIS", Address::Ram(3)),
    ("THAT", Address::Ram(4)),
    ("SCREEN", Address::Ram(0x4000)),
    ("KBD", Address::Ram(0x6000)),
];

impl CExpr {
    #[allow(clippy::unusual_byte_groupings)]
    pub const fn as_bits(&self) -> u16 {
        let raw_bits = match self {
            CExpr::Zero => 0b0_101010,
            CExpr::One => 0b0_111111,
            CExpr::NegOne => 0b0_111010,
            CExpr::D => 0b0_001100,
            CExpr::X(Source::Register) => 0b0_110000,
            CExpr::X(Source::Memory) => 0b1_110000,
            CExpr::NotD => 0b0_001101,
            CExpr::NotX(Source::Register) => 0b0_110001,
            CExpr::NotX(Source::Memory) => 0b1_110001,
            CExpr::NegD => 0b0_001111,
            CExpr::NegX(Source::Register) => 0b0_110011,
            CExpr::NegX(Source::Memory) => 0b1_110011,
            CExpr::DPlusOne => 0b0_011111,
            CExpr::DMinusOne => 0b0_001110,
            CExpr::XPlusOne(Source::Register) => 0b0_110111,
            CExpr::XPlusOne(Source::Memory) => 0b1_110111,
            CExpr::XMinusOne(Source::Register) => 0b0_110010,
            CExpr::XMinusOne(Source::Memory) => 0b1_110010,
            CExpr::DPlusX(Source::Register) => 0b0_000010,
            CExpr::DPlusX(Source::Memory) => 0b1_000010,
            CExpr::DMinusX(Source::Register) => 0b0_010011,
            CExpr::DMinusX(Source::Memory) => 0b1_010011,
            CExpr::XMinusD(Source::Register) => 0b0_000111,
            CExpr::XMinusD(Source::Memory) => 0b1_000111,
            CExpr::DAndX(Source::Register) => 0b0_000000,
            CExpr::DAndX(Source::Memory) => 0b1_000000,
            CExpr::DOrX(Source::Register) => 0b0_010101,
            CExpr::DOrX(Source::Memory) => 0b1_010101,
        };

        raw_bits << 6
    }
}

impl Dst {
    pub const fn as_bits(self) -> u16 {
        (self.bits() as u16) << 3
    }
}

impl JumpCondition {
    pub const fn as_bits(&self) -> u16 {
        match self {
            JumpCondition::Never => 0b000,
            JumpCondition::Always => 0b111,
            JumpCondition::GreaterThan => 0b001,
            JumpCondition::LessThan => 0b100,
            JumpCondition::GreaterEqual => 0b011,
            JumpCondition::LessEqual => 0b110,
            JumpCondition::Equal => 0b010,
            JumpCondition::NEqual => 0b101,
        }
    }
}
