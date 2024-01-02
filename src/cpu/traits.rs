use thiserror::Error;

// TRAITS!!!!!!!!!!!!!!! We love traits

/// for the CPU to accept a Target and a Value
// pub trait TargetedWrite<T: Writable, V> {
//     fn write(&mut self, target: T, value: V);
// }

// /// for the CPU to accept a read target, and a buffer to read that data into
// pub trait TargetedRead<T: Readable, B> {
//     fn read(&self, target: T, B);
// }

pub trait Writable {}
pub trait Readable {}

/// trait for the CPU to execute instructions
pub trait Execute<I> {
    fn execute(&mut self, instruction: I);
}

// /// Trait for all structs representing different Instructions
// pub trait Instruction {
//     fn cycles(&self) -> u8;

// perhaps some debugging functions????
// }

pub trait TargetedRead {
    type Output; // Associated type for the output (read) data
    type Source; // Associated type for the source of the data

    fn read(&self, source: &Self::Source) -> Result<Self::Output, ReadError>;
}

pub trait TargetedWrite {
    type Source; // Associated type for the input (write) data
    type Destination; // Associated type for the destination of the data

    fn write(
        &mut self,
        destination: &Self::Destination,
        value: Self::Input,
    ) -> Result<(), WriteError>;
}
