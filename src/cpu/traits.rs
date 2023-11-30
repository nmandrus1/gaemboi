// TRAITS!!!!!!!!!!!!!!! We love traits

/// for the CPU to accept a Target and a Value
pub trait TargetedWrite<T: Writable, V> {
    fn write(&mut self, target: T, value: V);
}

/// for the CPU to accept a read target, and a buffer to read that data into
pub trait TargetedRead<T: Readable, B> {
    fn read(&self, target: T, buf: B);
}

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
