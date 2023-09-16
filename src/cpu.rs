/// struct to represent the CPU of a Gameboy
struct CPU {
    /// Accumulator Register
    a: u8,

    /// Flag Register
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
    f: u8,

    /// Other Registers
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,

    /// Stack Pointer
    sp: u16,

    /// Program Counter
    pc: u16,

    /// 64kb of Memory
    mem: [u8; 0xFFFF],
}

/// Enum that maps a binary code to a register
enum Register {
    B = 0b000,
    C = 0b001,
    D = 0b010,
    E = 0b011,
    H = 0b100,
    L = 0b101,
    HL = 0b110,
    A = 0b111,
}

impl CPU {
    /// read a byte from memory from the passed adress
    fn read(&self, addr: u16) -> u8 {
        todo!()
    }

    /// write a byte to memory at the passed address
    fn write(&mut self, addr: u16, value: u8) {
        todo!()
    }

    /// Move 1 step forward in execution
    fn step(&mut self) {
        todo!()
    }

    /// returns a 16 bit number from the b and c registers
    fn bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    /// returns a 16 bit number from the d and e registers
    fn de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    /// returns a 16 bit number from the h and l registers
    fn hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }
}

/// Address type
// check out the "newtype" pattern in Rust to see more examples
// Doing this rather than just using a u16 provides a few benefits
// 1) Type saftey: instead of some vague u16 floating around in
//     the the type will make it clear what the value represents
// 2) Readability: This makes the code very readable and at a glance
//     it becomes clear based on the type alone what the variable does
// 3) Flexibilty: We can define methods on this type and even change its
//     representation and behavior if we need to in the future
struct Address(u16);

/// Enum to represent the different possible operands for a Load instruction
// Things to keep in mind:
// 1) Errors: Not every combination of registers may be allowed,
//     maybe creating a special LoadRegister that only contains valid
//     registers?
enum LoadOperand {
    // Read or Write to/from a register
    Reg(Register),

    // Read or Write to/from a memory address
    Mem(Address),
}

/// Struct to contain the source and destination of a Load Instruction
struct LoadOp {
    src: LoadOperand,
    dest: LoadOperand,

    // Some Load Instructions will do more than just load, they will
    // decrement the value located at the previously written to address,
    // in cases like these we can represent that as an Optional followup
    // function to be run after the Operation
    followup: Option<fn(Address)>,
}
