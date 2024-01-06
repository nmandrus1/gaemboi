use thiserror::Error;

use super::Instruction;

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
    #[error("Invalid RegisterID recieved, unable to decode bit triple : 0b{0:08b}")]
    RegisterDecodeError(u8),

    #[error("Invalid ALU table index recieved: {0}")]
    AluDecodeError(u8),
}

#[derive(Error, Debug)]
pub enum CpuError {
    #[error("Instruction encounted that is not yet supported: {0:#?}")]
    UnsupportedInstruction(Instruction),

    #[error("Failed to fetch data")]
    FetchError,
}
