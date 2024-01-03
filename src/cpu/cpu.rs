use super::*;
/// Nice macros for convience

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

impl From<Register<u16>> for Address {
    fn from(value: Register<u16>) -> Self {
        Self(value.value)
    }
}

/// Memory struct
// the memory is certainly going to be updated and more complex than an array
// of bytes, so for now we will interact through this struct and change behavior
// and implemenation later as needed
pub struct Memory([u8; 0xFFFF]);

impl Memory {
    /// read len bytes from memory starting at the passed address
    pub fn read(&self, src: Address, len: usize) -> Result<&[u8], ReadError> {
        let start = src.0 as usize;
        let end = start + len;

        if end <= self.0.len() {
            Ok(&self.0[start..end])
        } else {
            Err(ReadError::MemoryOverflow)
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
    pub fn write(&mut self, dest: Address, value: &[u8]) -> Result<(), WriteError> {
        let len = value.len();
        let start = dest.0 as usize;
        let end = start + len;

        if end <= self.0.len() {
            let mem_slice = &mut self.0[start..end];
            mem_slice.copy_from_slice(value);
            Ok(())
        } else {
            Err(WriteError::MemoryOverflow)
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
    af: Register<u16>,

    // other registers
    bc: Register<u16>,
    de: Register<u16>,
    hl: Register<u16>,

    /// Stack Pointer
    sp: Register<u16>,

    /// Program Counter
    pc: Register<u16>,

    /// 64kb of Memory
    mem: Memory,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            af: Register::from(0),
            bc: Register::from(0),
            de: Register::from(0),
            hl: Register::from(0),
            sp: Register::from(0),
            pc: Register::from(0),
            mem: Memory::default(),
        }
    }
}

impl Cpu {
    /// Modular Decoder function, first we determine what kind of instruction
    /// and then we pass the opcode to a more specific decoder that generates the
    /// instruction
    fn decode(&mut self) -> anyhow::Result<()> {
        let opcode = self.mem.read_byte(Address::from(self.pc))?;
        self.pc.value += 1;

        match opcode {
            // 8 Bit Load
            0x40..=0x75
            | 0x77..=0x7F
            | 0x02
            | 0x12
            | 0x22
            | 0x32
            | 0x06
            | 0x16
            | 0x26
            | 0x36
            | 0x0A
            | 0x1A
            | 0x2A
            | 0x3A
            | 0x0E
            | 0x1E
            | 0x2E
            | 0x3E => todo!(), // some decode function,
            _ => unimplemented!(),
        }
    }

    /// 8 Bit Load instruction decoder
    // fn decode_load8<R: RegisteR>(&self, _opcode: u8) {
    // // isolate important opcode patterns
    // let high_bits = opcode & 0xC0; // 0xC0 = 0b11000000
    // let mid_bits = opcode & 0x38; // 0x38 = 0b00111000
    // let last_bits = opcode & 0x07; // 0x07 = 0b00000111

    // match (high_bits, mid_bits, last_bits) {
    //     // 0b01xxx110
    //     (0x40, _, 0x06) => {
    //         // LD r, (HL)
    //     }

    //     // 0b00xxx110
    //     (0x00, _, 0x06) => {
    //         // LD, r, d8
    //     }
    // }

    // // start with loading between registers
    // if opcode >> 6 == 1 {
    //     // LD r, r
    //     // opcode = 0b01xxxyyy -> xxx = dest; yyy = src
    //     let src = from_bit_triple(opcode & 0b00000111);
    //     let dest = from_bit_triple((opcode >> 3) & 0b00000111);

    //     Instruction {
    //         op: InstructionType::Load8 {
    //             src: LoadOperand<Register8>::Reg(src),
    //             dest: LoadOperand<Register8>::Reg(dest),
    //             followup: None,
    //         },
    //         cycles: 1,
    //     }
    // } else if opcode & 0b00000110 == 0b00000110 {
    //     // LD r, d8
    //     // opcode = 0b00xxx110 -> xxx = dest

    //     // read immediate data and increment program counter
    //     let immediate = self.read(Address(self.pc));
    //     self.pc += 1;

    //     let dest = from_bit_triple((opcode >> 3) & 0b00000111);

    //     Instruction {
    //         op: InstructionType::Load8 {
    //             src: LoadOperand<Register8>::Im8(immediate),
    //             dest: LoadOperand<Register8>::Reg(dest),
    //             followup: None,
    //         },
    //         cycles: 2,
    //     }
    // } else {
    // todo!()
    // }
    // }

    /// Move 1 step forward in execution
    // Read, Fetch, Execute
    fn step(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod test {
    // all unit tests belong here
    use super::*;

    #[test]
    fn test_reg_write() {
        // To whom it may concern...
        //
        // if you have a problem with the byte order in these unit tests
        // seek comfort in the fact that I do as well
        // figuring out consistency in the byte order throughout the CPU
        // code has taken me an EMBARASSING amount of time and it's probably
        // STILL WRONG

        let mut cpu = Cpu::default();

        // Test setting the A register
        cpu.af.write_hi(0xAB);
        assert_eq!(cpu.af.hi(), 0xAB, "Failed to set register A");

        cpu.af.write_lo(0xCD);
        assert_eq!(cpu.af.lo(), 0xCD, "Failed to set register F");

        cpu.bc.write(0xBEEFu16);
    }

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
    fn test_read_register8() {
        let mut cpu = Cpu::default();
        // first byte is the LOW BYTE
        cpu.af = Register::from(0xABCD); // A = 0xCD, F = 0xAB
        cpu.bc = Register::from(0xCDEF);
        cpu.de = Register::from(0xBEEF);

        let mut buf = cpu.af.hi();
        println!("hi: 0x{:02X} lo: 0x{:02X}", cpu.af.hi(), cpu.af.lo());
        assert_eq!(buf, 0xCD);

        buf = cpu.bc.hi();
        assert_eq!(buf, 0xEF);

        buf = cpu.bc.lo();
        assert_eq!(buf, 0xCD);

        buf = cpu.de.hi();
        assert_eq!(buf, 0xEF);

        buf = cpu.de.lo();
        assert_eq!(buf, 0xBE);
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
    fn test_write_register8_to_register8() {
        let mut cpu = Cpu::default();
        cpu.af = 0xAA00.into(); // Initialize A with 0xAA
        cpu.bc = 0xBBCC.into(); // Initialize B with 0xBB and C with 0xCC
        cpu.de = 0xDDEE.into(); // Initialize D with 0xDD and E with 0xEE
        cpu.hl = 0x1122.into(); // Initialize H with 0x11 and L with 0x22

        // Test copying from Register A to Register B
        cpu.af.write_hi(cpu.bc.hi());
        assert_eq!(cpu.af.hi(), 0xCC, "Failed to copy from A to B");

        // Test copying from Register C to Register D
        cpu.de.write_hi(cpu.bc.lo());
        assert_eq!(cpu.de.hi(), 0xBB, "Failed to copy from C to D");
    }

    #[test]
    fn test_write_register8_from_address() -> Result<(), anyhow::Error> {
        let mut cpu = Cpu::default();
        cpu.mem.write_byte(Address(0x100), 0x69)?;

        cpu.bc.write_hi(cpu.mem.read_byte(Address(0x100))?);
        assert_eq!(cpu.bc.hi(), 0x69);

        Ok(())
    }

    #[test]
    fn test_write_register16_to_register16() {
        let mut cpu = Cpu::default();
        cpu.af = 0xAA00.into(); // Initialize A with 0xAA
        cpu.bc = 0xBBCC.into(); // Initialize B with 0xBB and C with 0xCC
        cpu.de = 0xDDEE.into(); // Initialize D with 0xDD and E with 0xEE
        cpu.hl = 0x1122.into(); // Initialize H with 0x11 and L with 0x22

        // Test copying from Register AF to Register BC
        cpu.bc.write(cpu.af);
        assert_eq!(cpu.bc.value, 0xAA00, "Failed to copy from AF to BC");

        // Test copying from Register C to Register D
        cpu.de.write(cpu.hl);
        assert_eq!(cpu.de.value, 0x1122, "Failed to copy from HL to DE");
    }
}
