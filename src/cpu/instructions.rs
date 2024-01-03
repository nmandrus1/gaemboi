use super::Address;

pub enum InstructionType<R> {
    Load {
        src: LoadOperand<R>,
        dest: LoadOperand<R>,

        // Some Load Instructions will do more than just load, they will
        // decrement the value located at the previously written to address,
        // in cases like these we can represent that as an Optional followup
        // function to be run after the Operation
        followup: Option<fn(Address)>,
    },
    Arith8,
    Arith16,
    Nop,
}

pub struct Instruction<R> {
    instruction: InstructionType<R>,
    cycles: u8,
}

/// Enum to represent the different possible operands for a Load instruction
// Things to keep in mind:
// 1) Errors: Not every combination of registers may be allowed,
//     maybe creating a special LoadRegister that only contains valid
//     registers?

pub enum LoadOperand<R> {
    // Read or Write to/from a register
    Reg(R),

    // Read or Write to/from a memory address
    Mem(Address),

    // Immediate Data
    Im,
}
