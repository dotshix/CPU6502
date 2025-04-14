// Information grabbed from: https://www.nesdev.org/wiki/CPU

/// Represents the 6502 CPU core used in the NES.
pub struct Cpu {
    /// Program Counter (16-bit)
    pub pc: u16,
    /// Stack Pointer (8-bit, offset from 0x0100)
    pub sp: u8,
    /// Status Register (8-bit)
    pub status: u8,
    /// Accumulator or A Register (8-bit)
    pub a: u8,
    /// Y register (8-bit)
    pub y: u8,
    /// X register (8-bit)
    pub x: u8,
    /// Counts how many cycles the instruction has remaining
    pub cycles: u8,
    pub addr_abs: u16,
    pub addr_rel: i16,
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

    // Addressing modes below
    // [NOTE] In abs/absx we are not modifying pc. I am however modifying it in the rest. This is a mistake
    // A cleaner model should be:
    //     - Let the *main instruction decode* logic handle `pc` incrementing after reading operands
    //     - Addressing modes only compute the effective address (`addr_abs`) using operands already fetched
    //
    //     WILL need to refactor later on
    //

    pub fn rel(&mut self) -> u8 {
        let offset = self.memory[self.pc as usize] as i8;
        self.addr_rel = offset as i16; // signed offset for branch logic
        self.pc = self.pc.wrapping_add(1);
        0
    }

    pub fn ind(&mut self) {
        let ptr_lo = self.memory[(self.pc + 1) as usize] as u16;
        let ptr_hi = self.memory[(self.pc + 2) as usize] as u16;

        let ptr = (ptr_hi << 8) | ptr_lo;

        let addr_lo = self.memory[ptr as usize] as u16;

        // 6502 bug: if low byte is $FF, wrap around to beginning of page
        let next_byte = if ptr_lo == 0x00FF {
            self.memory[(ptr & 0xFF00) as usize] as u16 // Wrap to $xx00
        } else {
            self.memory[(ptr + 1) as usize] as u16
        };

        self.addr_abs = (next_byte << 8) | addr_lo;
    }

    // [NOTE] Currently assumes PC is pointing at the *opcode*
    // Might need to modify this later to match other modes,
    // where PC is incremented before addressing mode executes.
    pub fn abs(&mut self) -> u8 {
        let lo = self.memory[(self.pc + 1) as usize] as u16;
        let hi = self.memory[(self.pc + 2) as usize] as u16;
        self.addr_abs = (hi << 8) | lo;
        0 // no extra cycles
    }

    pub fn absx(&mut self) -> u8 {
        let lo = self.memory[self.pc.wrapping_add(1) as usize] as u16;
        let hi = self.memory[self.pc.wrapping_add(2) as usize] as u16;

        let base = (hi << 8) | lo;
        self.addr_abs = base.wrapping_add(self.x as u16);

        // Check if page was crossed
        if (base & 0xFF00) != (self.addr_abs & 0xFF00) {
            1 // extra cycle needed
        } else {
            0
        }
    }

    pub fn imm(&mut self) -> u8 {
        self.addr_abs = self.pc + 1;
        self.pc = self.pc.wrapping_add(1);
        0
    }

    pub fn zp0(&mut self) -> u8 {
        self.addr_abs = self.memory[self.pc as usize] as u16;
        self.pc = self.pc.wrapping_add(1);
        0
    }

    pub fn zpx(&mut self) -> u8 {
        self.addr_abs = self.memory[self.pc as usize].wrapping_add(self.x) as u16;
        self.pc = self.pc.wrapping_add(1);
        0
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
            a: 0,
            y: 0,
            x: 0,
            cycles: 0,
            addr_abs: 0,
            addr_rel: 0,
            fetched: 0,
            memory: [0; 0x10000],
        }
    }
}
