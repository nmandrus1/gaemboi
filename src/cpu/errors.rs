use thiserror::Error;

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
