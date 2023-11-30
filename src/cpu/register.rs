use super::traits::{Readable, Writable};

pub trait Register: Writable {
    type Size;
}

/// Tuple Struct of two 8 bit registers
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Reg16(pub u8, pub u8);

impl From<u16> for Reg16 {
    fn from(value: u16) -> Self {
        // Little endian is sus af
        // I NEVER know whether the byte order is right
        let bytes = value.to_le_bytes();
        Self(bytes[0], bytes[1])
    }
}

impl From<Reg16> for u16 {
    fn from(value: Reg16) -> Self {
        u16::from_le_bytes([value.0, value.1])
    }
}

impl Writable for Reg16 {}
impl Readable for Reg16 {}

/// All the different registers that can be used by any instructions
// The contained data is a palceholder at the moment for the reg_write() function
// My plan was to reuse as much code as possible, and writing to a register is very
// common, so when you call reg_write() you pass in this enum and wrap it around
// the value you want to write to the register like: reg_write(Register::B(69))
// this approach garuantees type saftey since some registers have different sizes
pub enum Register8 {
    B,
    C,
    D,
    E,
    H,
    L,
    A,
}

impl Register for Register8 {
    type Size = u8;
}

pub enum Register16 {
    HL,
    BC,
    DE,
}

impl Register for Register16 {
    type Size = u16;
}

impl Writable for Register8 {}
impl Readable for Register8 {}
impl Writable for Register16 {}
impl Readable for Register16 {}

// /// maps the value of a 3 bit number to a register
// /// This *SHOULD* be consistent accross all opcodes
// pub fn from_bit_triple(trip: u8) -> impl Register {
//     match trip {
//         0 => Register8::B,
//         1 => Register8::C,
//         2 => Register8::D,
//         3 => Register8::E,
//         4 => Register8::H,
//         5 => Register8::L,
//         6 => Register16::HL,
//         7 => Register8::A,
//     }
// }
