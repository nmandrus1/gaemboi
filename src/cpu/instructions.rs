use super::*;
use anyhow::bail;
use registers::RegisterTrait;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InstructionType {
    // Some Load Instruction
    Load {
        src: Operand,
        dest: Operand,

        // Some Load Instructions will do more than just load, they will
        // decrement the value located at the previously written to address,
        // in cases like these we can represent that as an Optional followup
        // function to be run after the Operation
        //
        // fuck that noise ^^^
        // maybe just an enum for followup type???
        // TBD
        followup: Option<FollowUp>,
    },

    Arith8,
    Arith16,
    Nop,
    Halt,
}

/// Followup to instruction
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FollowUp {
    /// increment
    Inc,
    /// decrement
    Dec,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Instruction {
    instruction: InstructionType,
    cycles: u8,
}

impl Instruction {
    /// Create a load function from operands
    pub fn load(src: Operand, dest: Operand, followup: Option<FollowUp>) -> Self {
        Self {
            instruction: InstructionType::Load {
                src,
                dest,
                followup,
            },
            cycles: 1,
        }
    }

    /// create a HALT instruction
    pub fn halt() -> Self {
        Self {
            instruction: InstructionType::Halt,
            cycles: 1,
        }
    }

    /// create a NOP instruction
    pub fn nop() -> Self {
        Self {
            instruction: InstructionType::Nop,
            cycles: 1,
        }
    }

    /// return the InstructionType
    pub fn itype(&self) -> InstructionType {
        self.instruction
    }
}

/// Enum to represent the different possible operands for a Load instruction
// Things to keep in mind:
// 1) Errors: Not every combination of registers may be allowed,
//     maybe creating a special LoadRegister that only contains valid
//     registers?

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Operand {
    // Read or Write to/from a register
    Reg8(Register8),
    Reg16(Register16),

    /// If a register is to be read as an address, it should ALWAYS
    /// be converted into an Address, to do otherwise is a mistake
    // Address(Address),

    // 8 bit Immediate Data
    Immediate8,
    // 16 bit Immediate Data
    Immediate16,
}

impl Operand {
    /// decode the 8bit register table
    /// details: https://gb-archive.github.io/salvage/decoding_gbz80_opcodes/Decoding%20Gamboy%20Z80%20Opcodes.html#upfx
    pub fn from_8bit_table(trip: u8) -> anyhow::Result<Self> {
        match trip {
            0 => Ok(Self::Reg8(Register8::B)),
            1 => Ok(Self::Reg8(Register8::C)),
            2 => Ok(Self::Reg8(Register8::D)),
            3 => Ok(Self::Reg8(Register8::E)),
            4 => Ok(Self::Reg8(Register8::H)),
            5 => Ok(Self::Reg8(Register8::L)),
            6 => Ok(Self::Reg16(Register16::HL)),
            7 => Ok(Self::Reg8(Register8::A)),
            _ => bail!(DecodeError::RegisterDecodeError(trip)),
        }
    }
}

// #[rustfmt::skip]
// const OPCODES: [Instruction; 256] = [
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x00
//    Instruction {instruction: InstructionType::Load { src: LoadOperand::Im16, dest: LoadOperand::Reg(RegisterID::BC), followup: None}, cycles: 3}, // 0x01
//    Instruction {instruction: InstructionType::Load { src: LoadOperand::Reg(RegisterID::A), dest: LoadOperand::Reg(RegisterID::BC), followup:  None}, cycles: 2}, // 0x02

//    // TODO: These need to be filled out
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x03
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x04
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x05

//    Instruction {instruction: InstructionType::Load { src: LoadOperand::Im8, dest: LoadOperand::Reg(RegisterID::B), followup: None }, cycles: 2}, // 0x06

//    // TODO: This needs to be filled out
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x07

//    Instruction {instruction: InstructionType::Load { src: LoadOperand::Reg(RegisterID::SP), dest: LoadOperand::Im16, followup: None }, cycles: 5}, // 0x08

//    // TODO: This needs to be filled out
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x09

//    Instruction {instruction: InstructionType::Load { src: LoadOperand::Reg(RegisterID::BC), dest: LoadOperand::Reg(RegisterID::A), followup: None}, cycles: 2}, // 0x0A

//    // TODO: This needs to be filled out
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x0B
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x0C
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x0D

//    Instruction {instruction: InstructionType::Load { src: LoadOperand::Im8, dest: LoadOperand::Reg(RegisterID::C), followup: None }, cycles: 2}, // 0x0E

//    // TODO: This needs to be filled out
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x0F
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x10

//    Instruction {instruction: InstructionType::Load { src: LoadOperand::Im16, dest: LoadOperand::Reg(RegisterID::DE), followup: None}, cycles: 3}, // 0x11
//    Instruction {instruction: InstructionType::Load { src: LoadOperand::Reg(RegisterID::A), dest: LoadOperand::Reg(RegisterID::DE), followup:  None}, cycles: 2}, // 0x12

//    // TODO: This needs to be filled out
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x13
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x14
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x15

//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x16
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x17
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x18
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x19
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x1A
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x1B
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x1C
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x1D
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x1E
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x1F
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x20
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x21
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x22
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x23
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x24
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x25
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x26
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x27
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x28
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x29
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x2A
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x2B
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x2C
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x2D
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x2E
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x2F
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x30
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x31
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x32
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x33
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x34
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x35
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x36
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x37
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x38
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x39
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x3A
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x3B
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x3C
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x3D
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x3E
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x3F
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x40
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x41
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x42
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x43
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x44
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x45
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x46
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x47
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x48
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x49
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x4A
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x4B
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x4C
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x4D
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x5E
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x5F
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x50
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x51
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x52
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x53
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x54
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x55
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x56
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x57
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x58
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x59
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x5A
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x5B
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x5C
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x5D
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x5E
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x5F
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x60
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x61
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x62
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x63
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x64
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x65
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x66
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x67
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x68
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x69
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x6A
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x6B
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x6C
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x6D
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x6E
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x6F
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x70
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x71
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x72
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x73
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x74
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x75
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x76
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x77
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x78
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x79
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x7A
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x7B
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x7C
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x7D
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x7E
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x7F
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x80
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x81
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x82
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x83
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x84
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x85
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x86
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x87
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x88
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x89
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x8A
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x8B
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x8C
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x8D
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x8E
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x8F
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x90
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x91
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x92
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x93
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x94
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x95
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x96
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x97
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x98
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x99
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x9A
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x9B
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x9C
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x9D
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x9E
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0x9F
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xA0
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xA1
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xA2
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xA3
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xA4
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xA5
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xA6
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xA7
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xA8
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xA9
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xAA
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xAB
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xAC
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xAD
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xAE
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xAF
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xB0
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xB1
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xB2
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xB3
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xB4
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xB5
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xB6
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xB7
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xB8
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xB9
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xBA
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xBB
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xBC
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xBD
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xBE
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xBF
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xC0
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xC1
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xC2
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xC3
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xC4
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xC5
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xC6
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xC7
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xC8
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xC9
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xCA
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xCB
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xCC
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xCD
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xCE
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xCF
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xD0
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xD1
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xD2
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xD3
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xD4
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xD5
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xD6
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xD7
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xD8
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xD9
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xDA
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xDB
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xDC
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xDD
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xDE
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xDF
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xE0
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xE1
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xE2
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xE3
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xE4
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xE5
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xE6
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xE7
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xE8
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xE9
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xEA
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xEB
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xEC
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xED
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xEE
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xEF
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xF0
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xF1
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xF2
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xF3
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xF4
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xF5
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xF6
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xF7
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xF8
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xF9
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xFA
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xFB
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xFC
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xFD
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xFE
//    Instruction {instruction: InstructionType::Nop, cycles: 1}, // 0xFF
// ];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_bit_triple() -> anyhow::Result<()> {
        let operand = Operand::from_8bit_table(0b000)?;
        assert_eq!(operand, Operand::Reg8(register!(B)));

        // invalid register id
        let reg = Operand::from_8bit_table(69);
        assert!(reg.is_err());

        Ok(())
    }
}
