/// Register module
///
/// My goal for the Registers is to create a system where
/// All the different register types are able to work together smoothly
/// and can all operate together.

/// Marker trait so that only specific primitive types can be used for Registers
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

pub enum Register8 {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
    // A(A),
    // F(F),
    // B(B),
    // C(C),
    // D(D),
    // E(E),
    // H(H),
    // L(L),
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
}

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
}

pub enum Register {
    U8(Register8),
    U16(Register16),
}

impl Registers {
    pub fn fetch<R: RegisterTrait>(&self, fetcher: R) -> R::Size {
        fetcher.fetch(self)
    }

    pub fn write<R: RegisterTrait>(&mut self, dest: R, value: R::Size) {
        dest.write(self, value)
    }
}

struct A;
struct F;
struct B;
struct C;
struct D;
struct E;
struct H;
struct L;

struct AF;
struct BC;
struct DE;
struct HL;
struct SP;
struct PC;

pub trait RegisterTrait {
    type Size;

    fn fetch(&self, registers: &Registers) -> Self::Size;
    fn write(&self, registers: &mut Registers, value: Self::Size);
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
mod test {
    use super::{Register16, Register8, Registers};

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
