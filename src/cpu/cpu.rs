use anyhow::{anyhow, Result, bail, Ok, Context};
use super::errors::{ReadError, WriteError, CpuError};
use super::{registers::*, Instruction, InstructionType, Operand, FollowUp};
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

impl From<u16> for Address {
    fn from(value: u16) -> Self {
        Self (value)
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
    fn fetch_word_from_operand(&mut self, operand: Operand) -> Result<u16> {
        match operand {
            Operand::Reg16(reg) => Ok(self.registers.fetch(reg)),
            Operand::Immediate16 => {
                // read lo and hi bytes from pc then read from the address
                let pc = self.registers.fetch(register!(PC));

                let lo = self.mem.read_byte(Address(pc))? as u16;
                let hi = self.mem.read_byte(Address(pc + 1))? as u16;

                self.registers.inc(register!(PC));
                self.registers.inc(register!(PC));

                Ok((hi << 8) | lo)
            }

            _ => Err(anyhow!(CpuError::FetchError)
                .context(format!("Invalid operand for fetching u16 {:#?}", operand))),
        }
    }

    /// fetch a byte, logic determined by the LoadOperand
    /// i.e. an immediate16 operand will fetch a value from memory, 
    /// wheras an immediate8 operand will fetch a value from the PC
    fn fetch_byte_from_operand(&mut self, operand: Operand) -> Result<u8> {
        match operand {
            Operand::Reg8(reg) => Ok(self.registers.fetch(reg)),

            Operand::Reg16(reg) => Ok(self.mem.read_byte(self.registers.as_addr(reg))?),

            // operand::address is depcrecated for now
            // Operand::Address(addr) => self.mem.read_byte(addr),

            Operand::Immediate8 => {
                // read the byte from the PC
                let pc = self.registers.fetch(register!(PC));
                let byte = self.mem.read_byte(Address(pc))?;
                self.registers.write(register!(PC), pc + 1);
                Ok(byte)
            }

            Operand::Immediate16 => {
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
        match instr.itype() {
            // based on the destination, we should know exactly what kind of value it is expecting 
            // and then fetch that value
            InstructionType::Load { src, dest, .. } => {
                match (dest, src) {
                    (Operand::Reg8(reg), _) => {
                        // fetch the byte from the source and write it to the register
                        let byte = self.fetch_byte_from_operand(src)?;
                        self.registers.write(reg, byte);
                    }

                    // deprecated for now
                    // (Operand::Address(addr), _) => {
                    //     let byte = self.fetch_byte_from_operand(src)?;
                    //     self.mem.write_byte(addr, byte)?;
                    // }

                    (Operand::Reg16(dest), Operand::Reg8(_)) 
                    | (Operand::Reg16(dest), Operand::Immediate8) => {
                        
                        let addr = self.registers.as_addr(dest);
                        let byte = self.fetch_byte_from_operand(src)?;
                        self.mem.write_byte(addr, byte)?;
                    }

                    _ => bail!(CpuError::UnsupportedInstruction(instr))                  
                }
            }

            InstructionType::Nop => {},

            _ => bail!(anyhow!(CpuError::UnsupportedInstruction(instr))
                .context("Stopping Execution at preprogrammed HALT"))
        }

        Ok(())
    }

    ///  Instruction decoder
    fn decode(&self, opcode: u8) -> anyhow::Result<Instruction> {
        // isolate important opcode patterns
        // based on this: https://gb-archive.github.io/salvage/decoding_gbz80_opcodes/Decoding%20Gamboy%20Z80%20Opcodes.html#upfx
        // very useful table
        let x = (opcode & 0xC0) >> 6; // 0xC0 = 0b11000000
        let y = (opcode & 0x38) >> 3; // 0x38 = 0b00111000
        let q = y & 1;
        let p = y >> 1;
        let z = opcode & 0x07; // 0x07 = 0b00000111

        match (x, y, z, p, q) {
            // LD (BC), A
            (0, _, 2, 0, 0) => {
                let dest = Operand::Reg16(register!(BC));
                let src = Operand::Reg8(Register8::A);
                Ok(Instruction::load(src, dest, None))
            }

            // LD (DE), A
            (0, _, 2, 1, 0) => {
                let dest = Operand::Reg16(register!(DE));
                let src = Operand::Reg8(Register8::A);
                Ok(Instruction::load(src, dest, None))
            }
            
            // LD (HL+), A
            (0, _, 2, 2, 0) => {
                let dest = Operand::Reg16(register!(HL));
                let src = Operand::Reg8(Register8::A);
                Ok(Instruction::load(src, dest, Some(FollowUp::Inc)))
            }

            // LD (HL-), A
            (0, _, 2, 3, 0) => {
                let dest = Operand::Reg16(register!(HL));
                let src = Operand::Reg8(Register8::A);
                Ok(Instruction::load(src, dest, Some(FollowUp::Dec)))
            }
            
            // LD A, (BC)
            (0, _, 2, 0, 1) => {
                let dest = Operand::Reg8(Register8::A);
                let src = Operand::Reg16(register!(BC));
                Ok(Instruction::load(src, dest, None))
            }

            // LD A, (DE)
            (0, _, 2, 1, 1) => {
                let dest = Operand::Reg8(Register8::A);
                let src = Operand::Reg16(register!(DE));
                Ok(Instruction::load(src, dest, None))
            }

            // LD A, (HL+)
            (0, _, 2, 2, 1) => {
                let dest = Operand::Reg8(Register8::A);
                let src = Operand::Reg16(register!(HL));
                Ok(Instruction::load(src, dest, Some(FollowUp::Inc)))
            }

            // LD A, (HL-)
            (0, _, 2, 3, 1) => {
                let dest = Operand::Reg8(Register8::A);
                let src = Operand::Reg16(register!(HL));
                Ok(Instruction::load(src, dest, Some(FollowUp::Dec)))
            }

            // 16 bit INC
            (0, _, 3, _, 0) => Ok(Instruction::inc(Operand::from_rp_table(p)?)),
            // 16 bit DEC
            (0, _, 3, _, 1) => Ok(Instruction::dec(Operand::from_rp_table(p)?)),

            // special instruction
            (1, 6, 6, _, _ ) => Ok(Instruction::halt()),

            // LD r, r
            (1, _, _, _, _) => {
                let dest = Operand::from_r_table(y)?;               
                let src = Operand::from_r_table(z)?;
                // nop optimization
                if src == dest { return Ok(Instruction::nop())}
                Ok(Instruction::load(src, dest, None))
            }

            // LD, r, d8
            (0, _, 6, _, _) => {
                let dest = Operand::from_r_table(y)?;
                let src = Operand::Immediate8;
                Ok(Instruction::load(src, dest, None))
            },

            (0, 0, 0, _, _) => Ok(Instruction::nop()),

            _ => bail!("Failed to match opcode: {:08b}\tx: {:02b}\t y: {:03b}\t z: {:03b}\t p: {:02b}\t q: {:01b}", opcode,  x, y, z, p, q)
         }
    }

    /// Move 1 step forward in execution
    // Read, Fetch, Execute
    fn step(&mut self) -> Result<()> {
        // fetch PC and increment
        let pc = self.registers.fetch(register!(PC));
        self.registers.inc(register!(PC));

        // read the opcode from memory
        let opcode = self.mem.read_byte(Address(pc))?;

        // decode
        let instr = self.decode(opcode)?;

        self.fetch_and_execute(instr)?;
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        // halt instruction to stop the CPU
        self.mem.write_byte(Address(0x100), 0x76)?;

        loop {
            self.step()?
        }
    }
}

#[cfg(test)]
mod tests {
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

        let instr = cpu.decode(0x41)?;

        assert_eq!(
            instr,
            Instruction::load(Operand::Reg8(register!(C)), Operand::Reg8(register!(B)), None)
        );

        
        let instr = cpu.decode(0x40)?;

        assert_eq!(
            instr,
            Instruction::nop(),
        );

        Ok(())
    }        

    #[test]
    fn test_decode_ld_hl_r() -> anyhow::Result<()> {
        let cpu = Cpu::default();

        let instr = cpu.decode(0x70)?;

        assert_eq!(
            instr,
            Instruction::load(Operand::Reg8(register!(B)), Operand::Reg16(register!(HL)), None)
        );
        
        Ok(())
    }    

    #[test]
    fn test_decode_ld_r_hl() -> anyhow::Result<()> {
        let cpu = Cpu::default();

        let instr = cpu.decode(0x46)?;

        assert_eq!(
            instr,
            Instruction::load(Operand::Reg16(register!(HL)), Operand::Reg8(register!(B)), None)
        );
        
        Ok(())
    }

    
    #[test]
    fn test_decode_ld_r_d8() -> anyhow::Result<()> {
        let mut cpu = Cpu::default();
        cpu.registers.write(register!(PC), 0x101);

        cpu.mem.write_byte(Address(0x101), 0x69)?;

        // LD B, d8
        let instr = cpu.decode(0x06)?;

        assert_eq!(
            instr,
            Instruction::load(Operand::Immediate8, Operand::Reg8(register!(B)), None)
        );

        assert_eq!(0x69, cpu.fetch_byte_from_operand(Operand::Immediate8)?);
        assert_eq!(cpu.registers.fetch(register!(PC)), 0x102);

        Ok(())
    }        

    
    #[test]
    fn test_indirect_load_decoding() -> anyhow::Result<()> {
        let cpu = Cpu::default();

        // Decode the LD (BC), A instruction
        let instr = cpu.decode(0x02)?;

        // Check if the decoded instruction matches the expected instruction
        assert_eq!(
            instr,
            Instruction::load(
                Operand::Reg8(Register8::A),
                Operand::Reg16(Register16::BC),
                None
            )
        );

        // Decode the LD (BC), A instruction
        let instr = cpu.decode(0x2A)?;

        // Check if the decoded instruction matches the expected instruction
        assert_eq!(
            instr,
            Instruction::load(
                Operand::Reg16(Register16::HL),
                Operand::Reg8(Register8::A),
                Some(FollowUp::Inc)
            )
        );

        Ok(())
    }

    #[test]
    fn test_decod_16bit_inc() -> anyhow::Result<()> {
        let cpu = Cpu::default();

        // Decode the INC BC instruction
        let instr = cpu.decode(0x03)?;

        assert_eq!(instr, Instruction::inc(Operand::Reg16(Register16::BC)));
        
        Ok(())
    }

    #[test]
    fn test_fetch_byte_loadoperand_reg8() -> anyhow::Result<()> {
        let mut cpu = Cpu::default();
        cpu.registers.write(register!(B), 0xAB);

        let byte = cpu.fetch_byte_from_operand(Operand::Reg8(Register8::B))?;

        assert_eq!(byte, 0xAB);
        Ok(())
    }

    
    #[test]
    fn test_fetch_byte_loadoperand_reg16() -> anyhow::Result<()> {
        let mut cpu = Cpu::default();
        cpu.registers.write(register!(BC), 0xABCD);
        cpu.mem.write_byte(Address(0xABCD), 0xFE)?;

        let byte = cpu.fetch_byte_from_operand(Operand::Reg16(register!(BC)))?;
        assert_eq!(byte, 0xFE);

        Ok(())
    }

    
    #[test]
    fn test_fetch_byte_loadoperand_im8() -> anyhow::Result<()> {
        let mut cpu = Cpu::default();
        cpu.registers.write(register!(PC), 0x100);

        cpu.mem.write_byte(Address(0x100), 0x69)?;

        let byte = cpu.fetch_byte_from_operand(Operand::Immediate8)?;

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

        let byte = cpu.fetch_byte_from_operand(Operand::Immediate16)?;

        assert_eq!(byte, 0xBC);
        assert_eq!(cpu.registers.fetch(register!(PC)), 0x102);


        Ok(())
    }

    
    #[test]
    fn test_fetch_word_loadoperand_reg16() -> anyhow::Result<()> {
        let mut cpu = Cpu::default();
        cpu.registers.write(register!(BC), 0x1000);

        let word = cpu.fetch_word_from_operand(Operand::Reg16(register!(BC)))?;

        assert_eq!(word, 0x1000);

        Ok(())
    }

    
    #[test]
    fn test_fetch_word_loadoperand_im16() -> anyhow::Result<()> {
        let mut cpu = Cpu::default();
        cpu.registers.write(register!(PC), 0x100);

        cpu.mem.write_byte(Address(0x100), 0x69)?;
        cpu.mem.write_byte(Address(0x101), 0x42)?;

        let word = cpu.fetch_word_from_operand(Operand::Immediate16)?;

        assert_eq!(word, 0x4269);
        assert_eq!(cpu.registers.fetch(register!(PC)), 0x102);


        Ok(())
    }

}
