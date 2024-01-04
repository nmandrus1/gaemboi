use super::{register::RegisterID, Address, Register, RegisterSize};

pub enum InstructionType {
    // Some Load Instruction
    Load {
        src: LoadOperand,
        dest: LoadOperand,

        // Some Load Instructions will do more than just load, they will
        // decrement the value located at the previously written to address,
        // in cases like these we can represent that as an Optional followup
        // function to be run after the Operation
        //
        // fuck that noise ^^^
        // maybe just an enum for followup type???
        // TBD
        followup: Option<fn(Address)>,
    },

    Arith8,
    Arith16,
    Nop,
}

pub struct Instruction {
    instruction: InstructionType,
    cycles: u8,
}

/// Enum to represent the different possible operands for a Load instruction
// Things to keep in mind:
// 1) Errors: Not every combination of registers may be allowed,
//     maybe creating a special LoadRegister that only contains valid
//     registers?

pub enum LoadOperand {
    // Read or Write to/from a register
    Reg(RegisterID),

    // 8 bit Immediate Data
    Im8,

    // 16 bit Immediate Data
    Im16,
}

let opcodes: [u8; 256] = [
    
]
