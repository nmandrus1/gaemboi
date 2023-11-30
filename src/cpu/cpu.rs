use super::*;

/// Nice macros for convience

#[macro_export]
macro_rules! read_byte {
    ($cpu:expr, $target:expr) => {{
        let mut buf = 0;
        $cpu.read($target, &mut buf);
        buf
    }};
}

/// Address type
// check out the "newtype" pattern in Rust to see more examples
// Doing this rather than just using a u16 provides a few benefits
// 1) Type saftey: instead of some vague u16 floating around in
//     the the type will make it clear what the value represents
// 2) Readability: This makes the code very readable and at a glance
//     it becomes clear based on the type alone what the variable does
// 3) Flexibilty: We can define methods on this type and even change its
//     representation and behavior if we need to in the future
pub struct Address(u16);
impl Writable for Address {}
impl Readable for Address {}

/// struct to represent the CPU of a Gameboy
pub struct CPU {
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
    af: Reg16,

    // other registers
    bc: Reg16,
    de: Reg16,
    hl: Reg16,

    /// Stack Pointer
    sp: u16,

    /// Program Counter
    pc: u16,

    /// 64kb of Memory
    mem: [u8; 0xFFFF],
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            af: Reg16(0, 0),
            bc: Reg16(0, 0),
            de: Reg16(0, 0),
            hl: Reg16(0, 0),
            sp: 0,
            pc: 0,
            mem: [0; 0xFFFF],
        }
    }
}

impl CPU {
    /// Modular Decoder function, first we determine what kind of instruction
    /// and then we pass the opcode to a more specific decoder that generates the
    /// instruction
    fn decode(&mut self) {
        let opcode = read_byte!(self, Address(self.pc));
        self.pc += 1;

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
            | 0x3E => self.decode_load8::<Register8>(opcode),
            _ => unimplemented!(),
        }
    }

    /// 8 Bit Load instruction decoder
    fn decode_load8<R: Register>(&self, opcode: u8) {
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
        todo!()
        // }
    }

    /// Move 1 step forward in execution
    // Read, Fetch, Execute
    fn step(&mut self) {
        todo!()
    }

    /// returns a 16 bit number from the b and c registers
    fn bc(&self) -> u16 {
        self.bc.into()
    }

    /// returns a 16 bit number from the d and e registers
    fn de(&self) -> u16 {
        self.de.into()
    }

    /// returns a 16 bit number from the h and l registers
    fn hl(&self) -> u16 {
        self.hl.into()
    }

    /// Load Function "Entry Point", this determines which loading pattern needs to be executed
    // Ideally, we wouldn't have such separation of logic, but I think we can either fix this in the future
    // or come to realize that it isn't too bad considering there are like a billion load operations
    fn load8(&mut self, src: LoadOperand<Register8>, dest: LoadOperand<Register8>) {
        todo!()
        //  match (src, dest) {
        //      (LoadOperand::Reg(src), LoadOperand::Reg(dest)) => {}
        //      (LoadOperand::Reg(src), LoadOperand::Mem(dest)) => {}
        //      (LoadOperand::Mem(src), LoadOperand::Mem(dest)) => {}
        //      (LoadOperand::Mem(src), LoadOperand::Reg(dest)) => {}
        //      (LoadOperand::Im8(data), LoadOperand::Reg(dest) => {}
        // }
    }

    // /// Load data from one register to another
    // fn load_r_r(&mut self, src: Register, dest: Register) {}

    // /// Load data from one register to memory
    // fn load_r_m(&mut self, src: Register, dest: Address) {
    //     todo!()
    // }

    // /// Load data from memory to another memory location
    // fn load_m_m(&mut self, src: Address, dest: Address) {
    //     todo!()
    // }

    // /// Load data from memory to a register
    // fn load_m_r(&mut self, src: Address, dest: Register) {
    //     todo!()
    // }
}

impl TargetedWrite<Register8, u8> for CPU {
    fn write(&mut self, target: Register8, value: u8) {
        match target {
            Register8::A => self.af.0 = value,
            Register8::B => self.bc.0 = value,
            Register8::C => self.bc.1 = value,
            Register8::D => self.de.0 = value,
            Register8::E => self.de.1 = value,
            Register8::H => self.hl.0 = value,
            Register8::L => self.hl.1 = value,
        }
    }
}

impl TargetedWrite<Register16, u16> for CPU {
    fn write(&mut self, target: Register16, value: u16) {
        match target {
            Register16::HL => self.hl = value.into(),
            Register16::BC => self.bc = value.into(),
            Register16::DE => self.de = value.into(),
        }
    }
}

impl TargetedWrite<Address, u8> for CPU {
    fn write(&mut self, target: Address, value: u8) {
        self.mem[target.0 as usize] = value;
    }
}

impl TargetedWrite<Address, &[u8]> for CPU {
    fn write(&mut self, target: Address, value: &[u8]) {
        let len = value.len();
        let start = target.0 as usize;
        let end = start + len;

        if end <= self.mem.len() {
            let mem_slice = &mut self.mem[start..end];
            mem_slice.copy_from_slice(value);
        } else {
            panic!("Memory Overflow!!!")
        }
    }
}

impl TargetedRead<Register8, &mut u8> for CPU {
    fn read(&self, target: Register8, buf: &mut u8) {
        match target {
            Register8::A => *buf = self.af.0,
            Register8::B => *buf = self.bc.0,
            Register8::C => *buf = self.bc.1,
            Register8::D => *buf = self.de.0,
            Register8::E => *buf = self.de.1,
            Register8::H => *buf = self.hl.0,
            Register8::L => *buf = self.hl.1,
        }
    }
}

impl TargetedRead<Register16, &mut u16> for CPU {
    fn read(&self, target: Register16, buf: &mut u16) {
        match target {
            Register16::HL => *buf = self.hl.into(),
            Register16::BC => *buf = self.bc.into(),
            Register16::DE => *buf = self.de.into(),
        }
    }
}

impl TargetedRead<Address, &mut u8> for CPU {
    fn read(&self, target: Address, value: &mut u8) {
        *value = self.mem[target.0 as usize];
    }
}

impl TargetedRead<Address, &mut [u8]> for CPU {
    fn read(&self, target: Address, buf: &mut [u8]) {
        let len = buf.len();
        let start = target.0 as usize;
        let end = start + len;

        if end <= self.mem.len() {
            buf.copy_from_slice(&self.mem[start..end]);
        } else {
            panic!("Memory Overflow!!!")
        }
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

        let mut cpu = CPU::default();

        // Test setting the A register
        cpu.write(Register8::A, 0xAB);
        assert_eq!(cpu.af.0, 0xAB, "Failed to set register A");

        // Test setting the B register
        cpu.write(Register8::B, 0xBC);
        assert_eq!(cpu.bc.0, 0xBC, "Failed to set register B");

        // Test setting the C register
        cpu.write(Register8::C, 0xCD);
        assert_eq!(cpu.bc.1, 0xCD, "Failed to set register C");

        // Test setting the D register
        cpu.write(Register8::D, 0xDE);
        assert_eq!(cpu.de.0, 0xDE, "Failed to set register D");

        // Test setting the E register
        cpu.write(Register8::E, 0xEF);
        assert_eq!(cpu.de.1, 0xEF, "Failed to set register E");

        // Test setting the H and L registers using HL
        cpu.write(Register16::HL, 0x0102);
        assert_eq!(cpu.hl.0, 0x02, "Failed to set register H");
        assert_eq!(cpu.hl.1, 0x01, "Failed to set register L");

        // Test setting the B and C registers using BC
        cpu.write(Register16::BC, 0xBC0D);
        assert_eq!(cpu.bc.0, 0x0D, "Failed to set register B in BC");
        assert_eq!(cpu.bc.1, 0xBC, "Failed to set register C in BC");

        // Test setting the D and E registers using DE
        cpu.write(Register16::DE, 0xDEAD);
        assert_eq!(cpu.de.0, 0xAD, "Failed to set register D in DE");
        assert_eq!(cpu.de.1, 0xDE, "Failed to set register E in DE");
    }

    #[test]
    fn test_write_slice_to_memory() {
        let mut cpu = CPU::default();

        // Prepare data to write
        let data = [0xAB, 0xCD, 0xEF];

        // Choose an address to write to
        let target_address = Address(0x100); // Example address

        // Perform the write operation
        cpu.write(target_address, data.as_slice());

        // Verify that memory has been updated correctly
        assert_eq!(cpu.mem[0x100], 0xAB);
        assert_eq!(cpu.mem[0x101], 0xCD);
        assert_eq!(cpu.mem[0x102], 0xEF);

        // Optionally, you can also check that surrounding memory hasn't been altered
        assert_eq!(cpu.mem[0x0FF], 0x00); // Memory before the target address
        assert_eq!(cpu.mem[0x103], 0x00); // Memory after the written data
    }

    #[test]
    fn test_read_register8() {
        let mut cpu = CPU::default();
        // first byte is the LOW BYTE
        cpu.af = 0xABCD.into(); // A = 0xCD, F = 0xAB
        cpu.bc = 0xCDEF.into();
        cpu.de = 0xBEEF.into();

        let mut buf = read_byte!(cpu, Register8::A);
        assert_eq!(buf, 0xCD);

        buf = read_byte!(cpu, Register8::B);
        assert_eq!(buf, 0xEF);

        buf = read_byte!(cpu, Register8::C);
        assert_eq!(buf, 0xCD);

        buf = read_byte!(cpu, Register8::D);
        assert_eq!(buf, 0xEF);

        buf = read_byte!(cpu, Register8::E);
        assert_eq!(buf, 0xBE);
    }

    #[test]
    fn test_read_register16() {
        let mut cpu = CPU::default();
        cpu.bc = 0xCDEF.into();
        cpu.de = 0xBEEF.into();
        cpu.hl = 0x0102.into();

        let mut buf = 0;

        cpu.read(Register16::BC, &mut buf);
        assert_eq!(buf, 0xCDEF);

        cpu.read(Register16::DE, &mut buf);
        assert_eq!(buf, 0xBEEF);

        cpu.read(Register16::HL, &mut buf);
        assert_eq!(buf, 0x0102);
    }

    #[test]
    fn test_read_memory() {
        let mut cpu = CPU::default();

        // Setup memory with some test data
        cpu.mem[0x100] = 0xAA;
        cpu.mem[0x101] = 0xBB;

        let mut buf: [u8; 2] = [0; 2];
        cpu.read(Address(0x100), buf.as_mut_slice());
        assert_eq!(buf, [0xAA, 0xBB]);
    }
}
