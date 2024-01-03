use super::errors::*;

// TRAITS!!!!!!!!!!!!!!! We love traits

/// trait for the CPU to execute instructions
pub trait Execute<I> {
    fn execute(&mut self, instruction: I);
}

pub trait Readable {
    type Buffer; // Associated type for the buffer
    type Source; // Associated type for the source of the data

    fn read(&self, source: &Self::Source, buf: &mut Self::Buffer) -> Result<(), ReadError>;
}

pub trait Writable {
    type Source; // Associated type for the input (write) data
    type Destination; // Associated type for the destination of the data

    fn write(
        &mut self,
        destination: &Self::Destination,
        value: Self::Source,
    ) -> Result<(), WriteError>;
}
