use anyhow::{anyhow, Result, bail, Ok, Context};
use super::errors::{ReadError, WriteError, CpuError};
use super::{registers::*, Instruction, InstructionType, LoadOperand};
use super::register;

/// Address type
// check out the "newtype" pattern in Rust to see more examples
// Doing this rather than just using a u16 provides a few benefits
// 1) Type saftey: instead of some vague u16 floating around in
//     the the type will make it clear what the value represents
// 2) Readability: This makes the code very readable and at a glance
//     it becomes clear based on the type alone what the variable does
// 3) Flexibilty: We can define methods on this type and even change its
//     representation and behavior if we need to in the future
#[derive(Clone, Copy, PartialEq, Eq)]
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
    pub fn read_byte(&self, src: Address) -> Result<u8> {
        self.0
            .get(src.0 as usize)
            .ok_or(anyhow!(ReadError::MemoryOverflow))
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
    /// fetch a word, logic determined by the LoadOperand
    /// i.e. a register16 operand will fetch a value from registers, 
    /// wheras an immediate16 operand will fetch a value from the PC
    /// NOTE: Only Reg16 and Immediate16 operands are allowed, all others will fail
    fn fetch_word_from_operand(&mut self, operand: LoadOperand) -> Result<u16> {
        match operand {
            LoadOperand::Reg16(reg) => Ok(self.registers.fetch(reg)),
            LoadOperand::Immediate16 => {
                // read lo and hi bytes from pc then read from the address
                let pc = self.registers.fetch(register!(PC));

                let lo = self.mem.read_byte(Address(pc))? as u16;
                let hi = self.mem.read_byte(Address(pc + 1))? as u16;
                self.registers.write(register!(PC), pc + 2);

                Ok((hi << 8) | lo)
            }

            _ => Err(anyhow!(CpuError::FetchError)
                .context(format!("Invalid operand for fetching u16 {:#?}", operand))),
        }
    }

    /// fetch a byte, logic determined by the LoadOperand
    /// i.e. an immediate16 operand will fetch a value from memory, 
    /// wheras an immediate8 operand will fetch a value from the PC
    fn fetch_byte_from_operand(&mut self, operand: LoadOperand) -> Result<u8> {
        match operand {
            LoadOperand::Reg8(reg) => Ok(self.registers.fetch(reg)),

            LoadOperand::Reg16(_) => Err(anyhow!(CpuError::FetchError)
                .context("16 bit registers should be converted to LoadOperand::Address")),

            LoadOperand::Address(addr) => self.mem.read_byte(addr),

            LoadOperand::Immediate8 => {
                // read the byte from the PC
                let pc = self.registers.fetch(register!(PC));
                let byte = self.mem.read_byte(Address(pc))?;
                self.registers.write(register!(PC), pc + 1);
                Ok(byte)
            }

            LoadOperand::Immediate16 => {
                // read lo and hi bytes from pc then read from the address
                let pc = self.registers.fetch(register!(PC));

                let lo = self.mem.read_byte(Address(pc))? as u16;
                let hi = self.mem.read_byte(Address(pc + 1))? as u16;
                self.registers.write(register!(PC), pc + 2);

                self.mem.read_byte(Address((hi << 8) | lo))
            }
        }
    }
    
    fn fetch_and_execute(&mut self, instr: Instruction) -> anyhow::Result<()> {
        match instr.instruction {
            // based on the destination, we should know exactly what kind of value it is expecting 
            // and then fetch that value
            InstructionType::Load { src, dest, .. } => {
                match (dest, src) {
                    (LoadOperand::Reg8(reg), _) => {
                        // fetch the byte from the source and write it to the register
                        let byte = self.fetch_byte_from_operand(src)?;
                        self.registers.write(reg, byte);
                    }

                    (LoadOperand::Address(addr), _) => {
                        let byte = self.fetch_byte_from_operand(src)?;
                        self.mem.write_byte(addr, byte)?;
                    }

                    (LoadOperand::Reg16(dest), LoadOperand::Reg8(_)) 
                    | (LoadOperand::Reg16(dest), LoadOperand::Immediate8) => {
                        
                        let word = self.fetch_word_from_operand(src)?;
                        self.registers.write(dest, word);
                    }

                    _ => bail!(CpuError::UnsupportedInstruction(instr))                  
                }
            }
            _ => unimplemented!()
        }

        Ok(())
    }

    /// Modular Decoder function, first we determine what kind of instruction
    /// and then we pass the opcode to a more specific decoder that generates the
    /// instruction
    fn decode(&mut self) -> anyhow::Result<Instruction> {
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
            | 0x08 | 0xF8 | 0xF9  => self.decode_load(opcode), // some decode function,
            _ => unimplemented!(),
        }
    }

     ///  Load instruction decoder
    fn decode_load(&self, opcode: u8) -> anyhow::Result<Instruction> {
        // isolate important opcode patterns
        let high_bits = opcode & 0xC0; // 0xC0 = 0b11000000
        let mid_bits = opcode & 0x38; // 0x38 = 0b00111000
        let last_bits = opcode & 0x07; // 0x07 = 0b00000111

        let (cycles, instruction) = match (high_bits, mid_bits, last_bits) {
            // LD r, (HL)
            // 0b01 xxx 110
            (0x40, _, 0x06) => {
                let cycles = 2;
                let dest = LoadOperand::Reg8(decode_bit_triple_reg8(mid_bits)?);               
                let src = LoadOperand::Address(Address(self.registers.fetch(register!(HL))));
                (cycles, InstructionType::Load { src, dest, followup: None })
            }

            // LD (HL), r
            // 0b01 110 xxx
            (0x40, 0x06, _) => {
                let cycles = 2;
                let dest = LoadOperand::Address(Address(self.registers.fetch(register!(HL))));
                let src = LoadOperand::Reg8(decode_bit_triple_reg8(last_bits)?);
                (cycles, InstructionType::Load { src , dest , followup: None })
            }

            // LD r, r
            // 0b01 xxx yyy
            (0x40, _, _) => {
                let cycles = 1;
                let dest = LoadOperand::Reg8(decode_bit_triple_reg8(mid_bits)?);               
                let src = LoadOperand::Reg8(decode_bit_triple_reg8(last_bits)?);
                (cycles, InstructionType::Load { src, dest, followup: None })
            }

            // LD, r, d8
            // 0b00 xxx 110
            (0x00, _, 0x06) => {
                let cycles = 2;
                let dest = LoadOperand::Reg8(decode_bit_triple_reg8(mid_bits)?);
                let src = LoadOperand::Immediate8;
                (cycles, InstructionType::Load { src, dest, followup: None })
            },


            _ => unimplemented!()
         };

        Ok(Instruction { instruction, cycles})
    }

    /// Move 1 step forward in execution
    // Read, Fetch, Execute
    fn step(&mut self) -> Result<()> {
        let instr = self.decode()?;
        self.fetch_and_execute(instr)?;
        Ok(())
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

    #[test]
    fn test_decode_ld_r_r() -> anyhow::Result<()> {
        let cpu = Cpu::default();

        let instr = cpu.decode_load(0x40)?;

        assert_eq!(
            instr,
            Instruction { 
                instruction: InstructionType::Load { 
                    src: LoadOperand::Reg8(register!(B)), 
                    dest: LoadOperand::Reg8(register!(B)), 
                    followup: None  }, 
                cycles: 1
            }
        );

        Ok(())
    }        

    
    #[test]
    fn test_decode_ld_r_hl() -> anyhow::Result<()> {
        let cpu = Cpu::default();

        let instr = cpu.decode_load(0x46)?;

        assert_eq!(
            instr,
            Instruction { 
                instruction: InstructionType::Load { 
                    src: LoadOperand::Address(Address(cpu.registers.fetch(register!(HL)))), 
                    dest: LoadOperand::Reg8(register!(B)), 
                    followup: None  }, 
                cycles: 2
            }
        );
        
        Ok(())
    }

    
    #[test]
    fn test_decode_ld_r_d8() -> anyhow::Result<()> {
        let mut cpu = Cpu::default();
        cpu.registers.write(register!(PC), 0x101);

        cpu.mem.write_byte(Address(0x101), 0x69)?;

        // LD B, d8
        let instr = cpu.decode_load(0x06)?;

        assert_eq!(
            instr,
            Instruction { 
                instruction: InstructionType::Load { 
                    src: LoadOperand::Immediate8, 
                    dest: LoadOperand::Reg8(register!(B)), 
                    followup: None  }, 
                cycles: 2
            }
        );

        assert_eq!(0x69, cpu.fetch_byte_from_operand(LoadOperand::Immediate8)?);
        assert_eq!(cpu.registers.fetch(register!(PC)), 0x102);

        Ok(())
    }        


    #[test]
    fn test_fetch_byte_loadoperand_reg8() -> anyhow::Result<()> {
        let mut cpu = Cpu::default();
        cpu.registers.write(register!(B), 0xAB);

        let byte = cpu.fetch_byte_from_operand(LoadOperand::Reg8(Register8::B))?;

        assert_eq!(byte, 0xAB);
        Ok(())
    }

    
    #[test]
    fn test_fetch_byte_loadoperand_reg16() -> anyhow::Result<()> {
        let mut cpu = Cpu::default();
        cpu.registers.write(register!(BC), 0xABCD);
        cpu.mem.write_byte(Address(0xABCD), 0xFE)?;

        let byte = cpu.fetch_byte_from_operand(LoadOperand::Address(Address(cpu.registers.fetch(register!(BC)))))?;
        assert_eq!(byte, 0xFE);

        Ok(())
    }

    
    #[test]
    fn test_fetch_byte_loadoperand_im8() -> anyhow::Result<()> {
        let mut cpu = Cpu::default();
        cpu.registers.write(register!(PC), 0x100);

        cpu.mem.write_byte(Address(0x100), 0x69)?;

        let byte = cpu.fetch_byte_from_operand(LoadOperand::Immediate8)?;

        assert_eq!(byte, 0x69);
        assert_eq!(cpu.registers.fetch(register!(PC)), 0x101);

        Ok(())
    }

    
    #[test]
    fn test_fetch_byte_loadoperand_im16() -> anyhow::Result<()> {
        let mut cpu = Cpu::default();
        cpu.registers.write(register!(PC), 0x100);

        cpu.mem.write_byte(Address(0x100), 0x69)?;
        cpu.mem.write_byte(Address(0x101), 0x42)?;
        cpu.mem.write_byte(Address(0x4269), 0xBC)?;

        let byte = cpu.fetch_byte_from_operand(LoadOperand::Immediate16)?;

        assert_eq!(byte, 0xBC);
        assert_eq!(cpu.registers.fetch(register!(PC)), 0x102);


        Ok(())
    }

    
    #[test]
    fn test_fetch_word_loadoperand_reg16() -> anyhow::Result<()> {
        let mut cpu = Cpu::default();
        cpu.registers.write(register!(BC), 0x1000);

        let word = cpu.fetch_word_from_operand(LoadOperand::Reg16(register!(BC)))?;

        assert_eq!(word, 0x1000);

        Ok(())
    }

    
    #[test]
    fn test_fetch_word_loadoperand_im16() -> anyhow::Result<()> {
        let mut cpu = Cpu::default();
        cpu.registers.write(register!(PC), 0x100);

        cpu.mem.write_byte(Address(0x100), 0x69)?;
        cpu.mem.write_byte(Address(0x101), 0x42)?;

        let word = cpu.fetch_word_from_operand(LoadOperand::Immediate16)?;

        assert_eq!(word, 0x4269);
        assert_eq!(cpu.registers.fetch(register!(PC)), 0x102);


        Ok(())
    }
}
