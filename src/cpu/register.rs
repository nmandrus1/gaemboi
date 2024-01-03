/// Register module
///
/// My goal for the Registers is to create a system where
/// All the different register types are able to work together smoothly
/// and can all operate together.

/// Marker trait so that only specific primitive types can be used for Registers
pub trait RegisterSize {}
impl RegisterSize for u8 {}
impl RegisterSize for u16 {}

/// Generically sized Register with size specified by T
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Register<S>
where
    S: Copy + Clone + Eq + PartialEq + RegisterSize,
{
    pub value: S,
}

impl<S> Register<S>
where
    S: Copy + Clone + Eq + PartialEq + RegisterSize,
{
    pub fn write<I: Into<S>>(&mut self, src: I) {
        self.value = src.into();
    }
}

impl<S> From<S> for Register<S>
where
    S: Copy + Clone + Eq + PartialEq + RegisterSize,
{
    fn from(value: S) -> Self {
        Self { value }
    }
}

impl From<Register<u8>> for u8 {
    fn from(value: Register<u8>) -> Self {
        value.value
    }
}

impl From<Register<u16>> for u16 {
    fn from(value: Register<u16>) -> Self {
        value.value
    }
}

impl Register<u16> {
    /// return the highest 8 bits of the register
    pub fn hi(&self) -> u8 {
        self.value as u8
    }

    /// write to the 8 highest bits in the 16 bit register
    pub fn write_hi(&mut self, src: u8) {
        self.value = (self.value & 0xFF00) | src as u16;
    }

    /// return the lowest 8 bits of the register
    pub fn lo(&self) -> u8 {
        (self.value >> 8) as u8
    }

    /// write to the 8 lowest bits in the 16 bit register
    pub fn write_lo(&mut self, src: u8) {
        self.value = (self.value & 0x00FF) | ((src as u16) << 8)
    }
}

pub enum Register8 {
    B,
    C,
    D,
    E,
    H,
    L,
    A,
}

pub enum RegisteR16 {
    HL,
    BC,
    DE,
}

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_register16_write_high() {
        let mut reg = Register::from(0);
        reg.write_hi(1);

        let hi = reg.hi();

        assert_eq!(hi, 1);
        assert_eq!(reg.hi(), 1);
    }

    #[test]
    fn test_register16_write_low() {
        let mut reg = Register::from(0);
        reg.write_lo(1);

        let lo = reg.lo();

        assert_eq!(lo, 1);
        assert_eq!(reg.lo(), 1);
    }
}
