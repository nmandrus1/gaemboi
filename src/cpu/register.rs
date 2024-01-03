/// Register module
///
/// My goal for the Registers is to create a system where
/// All the different register types are able to work together smoothly
/// and can all operate together. Registers can be transformed, split, joined,
/// etc and my goal is something like this
///
/// cpu.af.split.w

pub trait Register16 {
    /// write to the 8 highest bits in the 16 bit register
    fn write_hi(&mut self, src: u8);

    /// write to the 8 lowest bits in the 16 bit register
    fn write_lo(&mut self, src: u8);
}

/// Generically sized Register with size specified by T
pub struct Register<S>
where
    S: Copy + Clone + Eq + PartialEq,
{
    pub value: S,
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

    /// return the highest 8 bits of the register
    pub fn hi(&self) -> u8 {
        (self.value >> 8) as u8
    }

    /// write to the 8 highest bits in the 16 bit register
    pub fn write_hi(&mut self, src: u8) {
        self.value = (self.value & 0x00FF) | ((src as u16) << 8)
    }

    /// return the lowest 8 bits of the register
    pub fn lo(&self) -> u8 {
        self.value as u8
    }

    /// write to the 8 lowest bits in the 16 bit register
    pub fn write_lo(&mut self, src: u8) {
        self.value = (self.value & 0xFF00) | src;
    }
}

// impl From<Register<(u8, u8)>> for Register<u16> {
//     fn from(value: Register<(u8, u8)>) -> Self {
//         Register::from(((value.0 as u16) << 8) | value.1)
//     }
// }

// impl Register16 for Register<u16> {
/// write to the 8 highest bits in the 16 bit register
//     fn write_hi(&mut self, src: u8) {
//         self.value = (self.value & 0x00FF) | ((src as u16) << 8)
//     }

//     /// write to the 8 lowest bits in the 16 bit register
//     fn write_lo(&mut self, src: u8) {
//         self.value = (self.value & 0xFF00) | src;
//     }
// }

/// 16-bit Register as two bytes
// impl Register<(u8, u8)> {
//     /// join the two bytes into a single u16 Register
//     pub fn join(self) -> Register<u16> {
//         Register::from(self)
//     }
// }

// impl Register16 for Register<(u8, u8)> {
//     fn write_hi(&mut self, src: u8) {
//         self.value.0 = src;
//     }

//     fn write_lo(&mut self, src: u8) {
//         self.value.1 = src;
//     }
// }

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
    use crate::cpu::Cpu;

    use super::*;

    #[test]
    fn test_register16_write_high() {
        let mut reg = Register::from(0);
        reg.write_hi(1);

        let (hi, lo) = reg.split();

        assert_eq!(hi, 1);
        assert_eq!(reg.hi(), 1);
    }

    #[test]
    fn test_register16_write_low() {
        let mut reg = Register::from(0);
        reg.write_lo(1);

        let (hi, lo) = reg.split();

        assert_eq!(lo, 1);
        assert_eq!(reg.lo(), 1);
    }
}
