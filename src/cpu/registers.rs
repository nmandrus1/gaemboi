use super::{errors::DecodeError, Address};
use anyhow::bail;

/// Simplify writing out the entire Register enum
///  
/// ```
/// # #[macro_use] extern crate foo;
/// # fn main() {
///       let x = register!(B);
///       assert_eq!(x, Register8::B)
///
///
///       let y = register!(SP);
///       assert_eq!(x, Register16::SP)
/// # }
/// ```
///
/// ```compile_fail
/// # #[macro_use] extern crate foo;
/// # fn main() {
///       // this won't compile!
///       let x = register!(Z)
/// # }
/// ```
#[macro_export]
macro_rules! register {
    // For 8-bit registers
    (A) => {
        Register8::A
    };
    (F) => {
        Register8::F
    };
    (B) => {
        Register8::B
    };
    (C) => {
        Register8::C
    };
    (D) => {
        Register8::D
    };
    (E) => {
        Register8::E
    };
    (H) => {
        Register8::H
    };
    (L) => {
        Register8::L
    };

    // For 16-bit registers
    (AF) => {
        Register16::AF
    };
    (BC) => {
        Register16::BC
    };
    (DE) => {
        Register16::DE
    };
    (HL) => {
        Register16::HL
    };
    (SP) => {
        Register16::SP
    };
    (PC) => {
        Register16::PC
    };
}

/// Register module
///
/// My goal for the Registers is to create a system where
/// All the different register types are able to work together smoothly
/// and can all operate together.

/// Decode a bit triple for load operations
///
/// Many load operations will encode the register operands in the bytes of the opcode
///
/// # Example
///```
/// use anyhow::Result;

/// # fn main() -> Result<()> {
///     // 0b000 is the encoding for Register B
///     let reg = decode_bit_triple(0b000)?;
///     assert_eq!(reg, Register::U8(Register8::B));
/// # }
///```

/// Struct containing the raw registers
#[derive(Default)]
pub struct Registers {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
}

/// All 8 bit registers
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Register8 {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
}

impl RegisterTrait for Register8 {
    type Size = u8;

    fn fetch(&self, registers: &Registers) -> Self::Size {
        match self {
            Register8::A => A.fetch(registers),
            Register8::F => F.fetch(registers),
            Register8::B => B.fetch(registers),
            Register8::C => C.fetch(registers),
            Register8::D => D.fetch(registers),
            Register8::E => E.fetch(registers),
            Register8::H => H.fetch(registers),
            Register8::L => L.fetch(registers),
        }
    }

    fn write(&self, registers: &mut Registers, value: Self::Size) {
        match self {
            Register8::A => A.write(registers, value),
            Register8::F => F.write(registers, value),
            Register8::B => B.write(registers, value),
            Register8::C => C.write(registers, value),
            Register8::D => D.write(registers, value),
            Register8::E => E.write(registers, value),
            Register8::H => H.write(registers, value),
            Register8::L => L.write(registers, value),
        }
    }

    fn inc(&self, registers: &mut Registers) {
        match self {
            Register8::A => A.inc(registers),
            Register8::F => F.inc(registers),
            Register8::B => B.inc(registers),
            Register8::C => C.inc(registers),
            Register8::D => D.inc(registers),
            Register8::E => E.inc(registers),
            Register8::H => H.inc(registers),
            Register8::L => L.inc(registers),
        }
    }

    fn dec(&self, registers: &mut Registers) {
        match self {
            Register8::A => A.dec(registers),
            Register8::F => F.dec(registers),
            Register8::B => B.dec(registers),
            Register8::C => C.dec(registers),
            Register8::D => D.dec(registers),
            Register8::E => E.dec(registers),
            Register8::H => H.dec(registers),
            Register8::L => L.dec(registers),
        }
    }
}

/// All 16 bit registers
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Register16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

impl RegisterTrait for Register16 {
    type Size = u16;

    fn fetch(&self, registers: &Registers) -> Self::Size {
        match self {
            Register16::AF => AF.fetch(registers),
            Register16::BC => BC.fetch(registers),
            Register16::DE => DE.fetch(registers),
            Register16::HL => HL.fetch(registers),
            Register16::SP => SP.fetch(registers),
            Register16::PC => PC.fetch(registers),
        }
    }

    fn write(&self, registers: &mut Registers, value: Self::Size) {
        match self {
            Register16::AF => AF.write(registers, value),
            Register16::BC => BC.write(registers, value),
            Register16::DE => DE.write(registers, value),
            Register16::HL => HL.write(registers, value),
            Register16::SP => SP.write(registers, value),
            Register16::PC => PC.write(registers, value),
        }
    }

    fn inc(&self, registers: &mut Registers) {
        match self {
            Register16::AF => AF.inc(registers),
            Register16::BC => BC.inc(registers),
            Register16::DE => DE.inc(registers),
            Register16::HL => HL.inc(registers),
            Register16::SP => SP.inc(registers),
            Register16::PC => PC.inc(registers),
        }
    }

    fn dec(&self, registers: &mut Registers) {
        match self {
            Register16::AF => AF.dec(registers),
            Register16::BC => BC.dec(registers),
            Register16::DE => DE.dec(registers),
            Register16::HL => HL.dec(registers),
            Register16::SP => SP.dec(registers),
            Register16::PC => PC.dec(registers),
        }
    }
}

impl Registers {
    /// fetch the value contained in a register, the return type depends
    /// on the size of the register passed, i.e. any U16 variant will return a u16
    /// and U8 variants will return a u8
    ///
    /// # Example
    /// ```
    /// let mut registers = Registers::default();
    /// registers.write(Register::U8(Register8::B), 420);
    ///
    /// let 8bit_b = registers.fetch(Register::U8(Register8::B));
    /// assert_eq!(8bit_b, 420);
    ///
    /// ```
    pub fn fetch<R: RegisterTrait>(&self, fetcher: R) -> R::Size {
        fetcher.fetch(self)
    }

    /// Write the value to the desired register, the type of value written depends
    /// on the size of the register passed, i.e. any U16 variant will require a u16
    /// and U8 variants will require a u8
    ///
    /// # Example
    /// ```
    /// let mut registers = Registers::default();
    /// registers.write(Register::U16(Register16::AF), 420);
    ///
    /// let 16bit_af = registers.fetch(Register::U16(Register16::AF));
    /// assert_eq!(16bit_af, 420);
    ///
    /// ```
    pub fn write<R: RegisterTrait>(&mut self, dest: R, value: R::Size) {
        dest.write(self, value)
    }

    /// Increment target register
    pub fn inc<R: RegisterTrait>(&mut self, dest: R) {
        dest.inc(self);
    }

    /// decrement target register
    pub fn dec<R: RegisterTrait>(&mut self, dest: R) {
        dest.dec(self);
    }

    /// return the desired 16 bit register as an address
    pub fn as_addr(&self, reg: Register16) -> Address {
        Address::from(reg.fetch(self))
    }
}

#[derive(Clone, Copy)]
struct A;
#[derive(Clone, Copy)]
struct F;
#[derive(Clone, Copy)]
struct B;
#[derive(Clone, Copy)]
struct C;
#[derive(Clone, Copy)]
struct D;
#[derive(Clone, Copy)]
struct E;
#[derive(Clone, Copy)]
struct H;
#[derive(Clone, Copy)]
struct L;

#[derive(Clone, Copy)]
struct AF;
#[derive(Clone, Copy)]
struct BC;
#[derive(Clone, Copy)]
struct DE;
#[derive(Clone, Copy)]
struct HL;
#[derive(Clone, Copy)]
struct SP;
#[derive(Clone, Copy)]
struct PC;

pub trait RegisterTrait: Copy {
    type Size: Copy;

    /// fetch the desired register
    fn fetch(&self, registers: &Registers) -> Self::Size;

    /// write to the desired register
    fn write(&self, registers: &mut Registers, value: Self::Size);

    /// increment register
    fn inc(&self, registers: &mut Registers);

    /// decrement register
    fn dec(&self, registers: &mut Registers);
}

#[macro_export]
macro_rules! impl_register_trait {
    // Implement for 8-bit registers
    ($regid:ident, $reg:ident, $type:ident) => {
        impl RegisterTrait for $regid {
            type Size = $type;

            fn fetch(&self, registers: &Registers) -> Self::Size {
                registers.$reg
            }

            fn write(&self, registers: &mut Registers, value: Self::Size) {
                registers.$reg = value;
            }

            fn inc(&self, registers: &mut Registers) {
                registers.$reg += 1;
            }

            fn dec(&self, registers: &mut Registers) {
                registers.$reg -= 1;
            }
        }
    };
}

#[macro_export]
macro_rules! impl_register_trait16 {
    // Implement for 16-bit register pairs
    ($regid:ident, $high_reg:ident, $low_reg:ident) => {
        impl RegisterTrait for $regid {
            type Size = u16;

            fn fetch(&self, registers: &Registers) -> Self::Size {
                ((registers.$high_reg as u16) << 8) | (registers.$low_reg as u16)
            }

            fn write(&self, registers: &mut Registers, value: Self::Size) {
                registers.$high_reg = (value >> 8) as u8;
                registers.$low_reg = value as u8;
            }

            fn inc(&self, registers: &mut Registers) {
                let value = self.fetch(registers);
                self.write(registers, value + 1);
            }

            fn dec(&self, registers: &mut Registers) {
                let value = self.fetch(registers);
                self.write(registers, value - 1);
            }
        }
    };
}

// Using the macro to implement the trait for each register
impl_register_trait!(A, a, u8);
impl_register_trait!(F, f, u8);
impl_register_trait!(B, b, u8);
impl_register_trait!(C, c, u8);
impl_register_trait!(D, d, u8);
impl_register_trait!(E, e, u8);
impl_register_trait!(H, h, u8);
impl_register_trait!(L, l, u8);
impl_register_trait!(SP, sp, u16);
impl_register_trait!(PC, pc, u16);

impl_register_trait16!(AF, a, f);
impl_register_trait16!(BC, b, c);
impl_register_trait16!(DE, d, e);
impl_register_trait16!(HL, h, l);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register8_write() {
        let mut registers = Registers::default();
        registers.write(Register8::A, 8);

        assert_eq!(registers.a, 8, "Failed to write to register A");
    }

    #[test]
    fn test_register8_fetch() {
        let mut registers = Registers::default();
        registers.a = 8;

        let byte = registers.fetch(Register8::A);
        assert_eq!(byte, 8, "Failed to fetch the proper value from register A");
    }

    #[test]
    fn test_register8_inc() {
        let mut registers = Registers::default();
        registers.a = 7;
        registers.inc(Register8::A);

        assert_eq!(registers.a, 8, "Failed to write to register A");
    }

    #[test]
    fn test_register8_dec() {
        let mut registers = Registers::default();
        registers.a = 8;
        registers.dec(Register8::A);

        assert_eq!(registers.a, 7, "Failed to write to register A");
    }

    #[test]
    fn test_split_register_write() {
        let mut registers = Registers::default();
        registers.write(Register16::AF, 0xA445); // A = 0xA4 F = 0x45

        assert_eq!(
            registers.a, 0xA4,
            "Failed to write to register A, A = 0x{:02X}",
            registers.a
        );

        assert_eq!(
            registers.f, 0x45,
            "Failed to write to register F, F = 0x{:02X}",
            registers.f
        );
    }

    #[test]
    fn test_split_register_inc() {
        let mut registers = Registers::default();
        registers.write(Register16::AF, 0xA445); // A = 0xA4 F = 0x45
        registers.inc(Register16::AF);

        assert_eq!(
            registers.a, 0xA4,
            "Failed to write to register A, A = 0x{:02X}",
            registers.a
        );

        assert_eq!(
            registers.f, 0x46,
            "Failed to write to register F, F = 0x{:02X}",
            registers.f
        );
    }

    #[test]
    fn test_split_register_dec() {
        let mut registers = Registers::default();
        registers.write(Register16::AF, 0xA445); // A = 0xA4 F = 0x45
        registers.dec(Register16::AF);

        assert_eq!(
            registers.a, 0xA4,
            "Failed to write to register A, A = 0x{:02X}",
            registers.a
        );

        assert_eq!(
            registers.f, 0x44,
            "Failed to write to register F, F = 0x{:02X}",
            registers.f
        );
    }

    #[test]
    fn test_split_register_fetch() {
        let mut registers = Registers::default();
        registers.a = 0xA4;
        registers.f = 0x45;

        let wide = registers.fetch(Register16::AF);
        assert_eq!(
            wide, 0xA445,
            "Failed to fetch the proper value from register AF"
        );
    }

    #[test]
    fn test_register16_write() {
        let mut registers = Registers::default();
        registers.write(Register16::SP, 0xA445);
        assert_eq!(
            registers.sp, 0xA445,
            "Failed to fetch the proper value from register SP"
        );
    }

    #[test]
    fn test_register16_inc() {
        let mut registers = Registers::default();
        registers.write(Register16::SP, 0xA445);
        registers.inc(Register16::SP);

        assert_eq!(
            registers.sp, 0xA446,
            "Failed to fetch the proper value from register SP"
        );
    }

    #[test]
    fn test_register16_dec() {
        let mut registers = Registers::default();
        registers.write(Register16::SP, 0xA445);
        registers.dec(Register16::SP);

        assert_eq!(
            registers.sp, 0xA444,
            "Failed to fetch the proper value from register SP"
        );
    }

    #[test]
    fn test_register16_fetch() {
        let mut registers = Registers::default();
        registers.sp = 0xA445;

        let wide = registers.fetch(Register16::SP);
        assert_eq!(
            wide, 0xA445,
            "Failed to fetch the proper value from register SP"
        );
    }
}
