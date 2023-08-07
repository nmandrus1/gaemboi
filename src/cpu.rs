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

// impl Default for CPU {
//     fn default() -> Self {}
// }

/// Enum to represent the different possible operands for a Load instruction
enum LoadTarget {
    Reg(Register),
    Mem(u16),
}
