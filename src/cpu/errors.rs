use thiserror::Error;

use super::{Instruction, Operand};

#[derive(Error, Debug)]
pub enum WriteError {
    #[error("Memory Overflow while writing to buffer")]
    MemoryOverflow,
}

#[derive(Error, Debug)]
pub enum ReadError {
    #[error("Memory Overflow while reading from buffer")]
    MemoryOverflow,
}

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("Invalid RP Table index : {0}")]
    RPTableLookupError(u8),

    #[error("Invalid RP1 Table index : {0}")]
    RP1TableLookupError(u8),

    #[error("Invalid RP2 Table index : {0}")]
    RP2TableLookupError(u8),

    #[error("Invalid ALU table index recieved: {0}")]
    AluDecodeError(u8),
}

#[derive(Error, Debug)]
pub enum CpuError {
    #[error("Instruction encounted that is not yet supported: {0:#?}")]
    UnsupportedInstruction(Instruction),

    #[error("Pair of invalid load operands src: {0:?} \t dest: {1:?}")]
    InvalidLoadOperands(Operand, Operand),

    #[error("Failed to fetch data")]
    FetchError,
}
