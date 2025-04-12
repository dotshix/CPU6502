// Information grabbed from: https://www.nesdev.org/wiki/CPU

/// Represents the 6502 CPU core used in the NES.
pub struct Cpu {
    /// Program Counter (16-bit)
    pc: u16,
    /// Stack Pointer (8-bit, offset from 0x0100)
    sp: u8,
    /// 64KB of addressable memory
    memory: [u8; 0x10000],
}

impl Cpu {
    /// Create a new CPU instance with default state
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu {
            pc: 0x0000,
            sp: 0xFD, // Stack starts here on power-up
            memory: [0; 0x10000],
        }
    }
}
