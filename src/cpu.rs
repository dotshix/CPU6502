// Information grabbed from: https://www.nesdev.org/wiki/CPU

/// Represents the 6502 CPU core used in the NES.
pub struct Cpu {
    /// Program Counter (16-bit)
    pub pc: u16,
    /// Stack Pointer (8-bit, offset from 0x0100)
    pub sp: u8,
    /// Status Register (8-bit)
    pub status: u8,
    /// Y register (8-bit)
    pub y: u8,
    pub addr_abs: u16,
    pub fetched: u8,
    /// 64KB of addressable memory
    pub memory: [u8; 0x10000],
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

    pub fn get_flag(&self, flag: Flag) -> bool {
        self.status & (1 << flag as u8) != 0
    }

    /// set: bool, true means "set the flag"
    /// false means "clear the flag"
    pub fn set_flag(&mut self, flag: Flag, set: bool) {
        if set {
            self.status |= 1 << flag as u8;
        } else {
            self.status &= !(1 << flag as u8);
        }
    }

    pub fn push(&mut self, value: u8) {
        let addr = 0x0100 | (self.sp as u16);
        self.memory[addr as usize] = value;
        self.sp = self.sp.wrapping_sub(1);
    }

    pub fn pull(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        let addr = 0x0100 | self.sp as u16;
        self.memory[addr as usize]
    }

    pub fn abs(&mut self) -> u8 {
        let lo = self.memory[(self.pc + 1) as usize] as u16;
        let hi = self.memory[(self.pc + 2) as usize] as u16;
        self.addr_abs = (hi << 8) | lo;
        0 // no extra cycles
    }

    /// fetches the value from memory at the absolute address (`addr_abs`) and stores it in `fetched`
    pub fn fetch(&mut self) -> u8 {
        self.fetched = self.memory[self.addr_abs as usize];
        self.fetched
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu {
            pc: 0x0000,
            sp: 0xFD, // Stack starts here on power-up
            status: 0,
            y: 0,
            addr_abs: 0,
            fetched: 0,
            memory: [0; 0x10000],
        }
    }
}
