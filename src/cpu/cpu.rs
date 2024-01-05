use anyhow::{bail, Ok};
use super::errors::{ReadError, WriteError};
use super::register::*;

/// Address type
// check out the "newtype" pattern in Rust to see more examples
// Doing this rather than just using a u16 provides a few benefits
// 1) Type saftey: instead of some vague u16 floating around in
//     the the type will make it clear what the value represents
// 2) Readability: This makes the code very readable and at a glance
//     it becomes clear based on the type alone what the variable does
// 3) Flexibilty: We can define methods on this type and even change its
//     representation and behavior if we need to in the future
#[derive(Clone, Copy)]
pub struct Address(u16);

impl std::fmt::Debug for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Address(0x{:04X})", self.0)
    }
}

/// Memory struct
// the memory is certainly going to be updated and more complex than an array
// of bytes, so for now we will interact through this struct and change behavior
// and implemenation later as needed
pub struct Memory([u8; 0xFFFF]);

impl Memory {
    /// read len bytes from memory starting at the passed address
    pub fn read(&self, src: Address, len: usize) -> anyhow::Result<&[u8]> {
        let start = src.0 as usize;
        let end = start + len;

        if end <= self.0.len() {
            Ok(&self.0[start..end])
        } else {
            bail!(ReadError::MemoryOverflow)
        }
    }

    /// read a single byte from memory at the passed address
    pub fn read_byte(&self, src: Address) -> Result<u8, ReadError> {
        self.0
            .get(src.0 as usize)
            .ok_or(ReadError::MemoryOverflow)
            .map(|val| val.clone())
    }

    /// write the contents of the passed buffer into memory starting at the passed address
    pub fn write(&mut self, dest: Address, value: &[u8]) -> anyhow::Result<()> {
        let len = value.len();
        let start = dest.0 as usize;
        let end = start + len;

        if end <= self.0.len() {
            let mem_slice = &mut self.0[start..end];
            mem_slice.copy_from_slice(value);
            Ok(())
        } else {
            bail!(WriteError::MemoryOverflow)
        }
    }

    /// write the contents of the passed buffer into memory starting at the passed address
    pub fn write_byte(&mut self, dest: Address, value: u8) -> Result<(), WriteError> {
        self.0
            .get_mut(dest.0 as usize)
            .ok_or(WriteError::MemoryOverflow)
            .map(|val| *val = value)
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self([0; 0xFFFF])
    }
}

/// struct to represent the CPU of a Gameboy
#[derive(Default)]
pub struct Cpu {
    /// Accumulator & Flag Register
    // 7  bit  0
    // ---- ----
    // ZNHC ----
    // |||| ||||
    // |||| |||+- These
    // |||| ||+-- bits
    // |||| |+--- aren't
    // |||| +---- used
    // ||||
    // |||+------ Carry Flag
    // ||+------- Half Carry
    // |+-------- Add/Sub Flag
    // +--------- Zero Flag
    registers: Registers,

    /// 64kb of Memory
    mem: Memory,
}

impl Cpu {
    /// maps the value of a 3 bit number to a register
    /// This *SHOULD* be consistent accross all opcodes
    fn from_bit_triple_helper(&mut self, trip: u8) -> anyhow::Result<Register> {
        match trip {
            0 => Ok(Register::U8(Register8::B)),
            1 => Ok(Register::U8(Register8::C)),
            2 => Ok(Register::U8(Register8::D)),
            3 => Ok(Register::U8(Register8::E)),
            4 => Ok(Register::U8(Register8::H)),
            5 => Ok(Register::U8(Register8::L)),
            6 => Ok(Register::U16(Register16::HL)),
            7 => Ok(Register::U8(Register8::A)),
            _ => bail!("Unknown Register ID: {}", trip),
        }
    }

    /// Modular Decoder function, first we determine what kind of instruction
    /// and then we pass the opcode to a more specific decoder that generates the
    /// instruction
    fn decode(&mut self) -> anyhow::Result<()> {
        let pc = self.registers.fetch(Register16::PC);
        let opcode = self.mem.read_byte(Address(pc))?;
        self.registers.write(Register16::SP, pc + 1);

        #[rustfmt::skip]
        match opcode {
            // 8 bit Load
            0x40..=0x75 | 0x77..=0x7F
            | 0x02 | 0x12 | 0x22 | 0x32
            | 0x06 | 0x16 | 0x26 | 0x36
            | 0x0A | 0x1A | 0x2A | 0x3A
            | 0x0E | 0x1E | 0x2E | 0x3E 
            // 16 bit load
            | 0x01 | 0x11 | 0x21 | 0x31 
            | 0x08 | 0xF8 | 0xF9  => todo!(), // some decode function,
            _ => unimplemented!(),
        }
    }

     ///  Load instruction decoder
    fn decode_load(&self, opcode: u8) {
        // isolate important opcode patterns
        let high_bits = opcode & 0xC0; // 0xC0 = 0b11000000
        let mid_bits = opcode & 0x38; // 0x38 = 0b00111000
        let last_bits = opcode & 0x07; // 0x07 = 0b00000111

         match (high_bits, mid_bits, last_bits) {
            // 0b01xxxyyy
            // LD r, r
            (0x40, _, _) => {
                
            }

            // 0b01xxx110
            // LD r, (HL)
            (0x40, _, 0x06) => {},

            // 0b00xxx110
            // LD, r, d8
            (0x00, _, 0x06) => {},

            _ => unimplemented!()
         }
    }

    /// Move 1 step forward in execution
    // Read, Fetch, Execute
    fn step(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    // all unit tests belong here
    use super::*;

    // To whom it may concern...
    //
    // if you have a problem with the byte order in these unit tests
    // seek comfort in the fact that I do as well
    // figuring out consistency in the byte order throughout the CPU
    // code has taken me an EMBARASSING amount of time and it's probably
    // STILL WRONG

    #[test]
    fn test_write_slice_to_memory() -> Result<(), anyhow::Error> {
        let mut cpu = Cpu::default();

        // Prepare data to write
        let data = [0xAB, 0xCD, 0xEF];

        // Choose an address to write to
        let target_address = Address(0x100); // Example address

        // Perform the write operation
        cpu.mem.write(target_address, data.as_slice())?;

        // Verify that memory has been updated correctly
        assert_eq!(cpu.mem.read_byte(Address(0x100)).unwrap(), 0xAB);
        assert_eq!(cpu.mem.read_byte(Address(0x101)).unwrap(), 0xCD);
        assert_eq!(cpu.mem.read_byte(Address(0x102)).unwrap(), 0xEF);

        // Optionally, you can also check that surrounding memory hasn't been altered
        assert_eq!(cpu.mem.read_byte(Address(0x0FF)).unwrap(), 0x00); // Memory before the target address
        assert_eq!(cpu.mem.read_byte(Address(0x103)).unwrap(), 0x00); // Memory after the written data

        Ok(())
    }

    #[test]
    fn test_read_memory() -> anyhow::Result<()> {
        let mut cpu = Cpu::default();

        // Setup memory with some test data
        cpu.mem.write_byte(Address(0x100), 0xAA)?;
        cpu.mem.write_byte(Address(0x101), 0xBB)?;

        let buf = cpu.mem.read(Address(0x100), 2)?;
        assert_eq!(buf, [0xAA, 0xBB]);

        Ok(())
    }

    #[test]
    fn test_write_register8_from_address() -> Result<(), anyhow::Error> {
        let mut cpu = Cpu::default();
        cpu.mem.write_byte(Address(0x100), 0x69)?;

        cpu.registers.write(Register8::B, cpu.mem.read_byte(Address(0x100))?);
        assert_eq!(cpu.registers.fetch(Register8::B), 0x69);

        Ok(())
    }
}
