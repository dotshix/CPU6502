// Information grabbed from: https://www.nesdev.org/wiki/CPU

/// Represents the 6502 CPU core used in the NES.
pub struct Cpu {
    /// Program Counter (16-bit)
    pc: u16,
    /// Stack Pointer (8-bit, offset from 0x0100)
    sp: u8,
    /// Status Register (8-bit)
    status: u8,
    /// 64KB of addressable memory
    memory: [u8; 0x10000],
}

pub enum Flag {
    Carry = 0,            // C
    Zero = 1,             // Z
    InterruptDisable = 2, // I
    Decimal = 3,          // D
    Break = 4,            // B (only on stack)
    Unused = 5,           //
    Overflow = 6,         // V
    Negative = 7,         // N
}

impl Cpu {
    /// Create a new CPU instance with default state
    pub fn new() -> Self {
        Self::default()
    }

    fn get_flag(&self, flag: Flag) -> bool {
        self.status & (1 << flag as u8) != 0
    }

    /// set: bool, true means "set the flag"
    /// false means "clear the flag"
    fn set_flag(&mut self, flag: Flag, set: bool) {
        if set {
            self.status |= 1 << flag as u8;
        } else {
            self.status &= !(1 << flag as u8);
        }
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu {
            pc: 0x0000,
            sp: 0xFD, // Stack starts here on power-up
            status: 0,
            memory: [0; 0x10000],
        }
    }
}
