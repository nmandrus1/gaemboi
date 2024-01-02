use super::traits::{Readable, Writable};

/// Generically sized Register with size specified by T
pub struct Register<S>
where
    S: Copy + Clone + Eq + PartialEq,
{
    value: S,
}

impl<S> Register<S>
where
    S: Copy + Clone + Eq + PartialEq,
{
    /// read the value contained in the Register
    pub fn read(&self) -> S {
        self.value
    }

    /// write a value to this register
    pub fn write(&mut self, src: S) {
        self.value = src;
    }
}

impl<S> From<S> for Register<S> {
    fn from(value: S) -> Self {
        Self { value }
    }
}

impl Register<u16> {
    /// split the 16 bit register into two bytes
    pub fn split(&self) -> (u8, u8) {
        let high = (self.value >> 8) as u8;
        let low = self.value as u8;
        (high, low)
    }

    /// write to the 8 highest bits in the 16 bit register
    pub fn write_high(&mut self, src: u8) {
        let high: u16 = (src as u16) << 8;
        self.value = high & (self.value as u8);
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

pub enum Register16 {
    HL,
    BC,
    DE,
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_register16_write_high() {
        let mut reg = Register::from(0u16);

        reg.write_high(1);

        let (hi, lo) = reg.split();

        assert_eq!(hi, 1);
    }
}
