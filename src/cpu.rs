use crate::{instructions::Instruction, table::build_instruction_table};
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

    pub instruction_table: [Instruction; 256],
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

    pub fn rel(&mut self) -> u8 {
        let raw = self.memory[self.pc as usize];
        let offset = raw as i8;
        self.addr_rel = offset as i16;

        self.pc = self.pc.wrapping_add(1);
        0
    }

    pub fn ind(&mut self) -> u8 {
        // pc points at the low byte of the pointer
        let ptr_lo = self.memory[self.pc as usize] as u16;
        let ptr_hi = self.memory[self.pc.wrapping_add(1) as usize] as u16;

        let ptr = (ptr_hi << 8) | ptr_lo;

        let addr_lo = self.memory[ptr as usize] as u16;

        // 6502 bug: if low byte is $FF, wrap around to beginning of page
        let next_byte = if ptr_lo == 0x00FF {
            self.memory[(ptr & 0xFF00) as usize] as u16
        } else {
            self.memory[ptr.wrapping_add(1) as usize] as u16
        };

        self.addr_abs = (next_byte << 8) | addr_lo;

        // Consume the two-byte operand
        self.pc = self.pc.wrapping_add(2);
        0
    }

    /// Indexed Indirect (X)
    pub fn indx(&mut self) -> u8 {
        let base = self.memory[self.pc as usize].wrapping_add(self.x); // operand + X (with wrap)
        let ptr_lo = self.memory[base as usize] as u16;
        let ptr_hi = self.memory[base.wrapping_add(1) as usize] as u16;

        self.addr_abs = (ptr_hi << 8) | ptr_lo;
        self.pc = self.pc.wrapping_add(1); // advance PC past operand

        0
    }

    /// Indirect Indexed (Y)
    pub fn indy(&mut self) -> u8 {
        let base = self.memory[self.pc as usize];
        let ptr_lo = self.memory[base as usize] as u16;
        let ptr_hi = self.memory[base.wrapping_add(1) as usize] as u16;

        let base_addr = (ptr_hi << 8) | ptr_lo;
        self.addr_abs = base_addr.wrapping_add(self.y as u16);
        self.pc = self.pc.wrapping_add(1); // advance PC past operand

        if (base_addr & 0xFF00) != (self.addr_abs & 0xFF00) {
            1 // crossed page, extra cycle
        } else {
            0
        }
    }

    pub fn abs(&mut self) -> u8 {
        let lo = self.memory[self.pc as usize] as u16;
        self.pc = self.pc.wrapping_add(1);

        let hi = self.memory[self.pc as usize] as u16;
        self.pc = self.pc.wrapping_add(1);

        self.addr_abs = (hi << 8) | lo;
        0
    }

    pub fn absx(&mut self) -> u8 {
        let lo = self.memory[self.pc as usize] as u16;
        self.pc = self.pc.wrapping_add(1);
        let hi = self.memory[self.pc as usize] as u16;
        self.pc = self.pc.wrapping_add(1);

        let base = (hi << 8) | lo;
        self.addr_abs = base.wrapping_add(self.x as u16);

        if (base & 0xFF00) != (self.addr_abs & 0xFF00) {
            1
        } else {
            0
        }
    }

    pub fn absy(&mut self) -> u8 {
        let lo = self.memory[self.pc as usize] as u16;
        self.pc = self.pc.wrapping_add(1);
        let hi = self.memory[self.pc as usize] as u16;
        self.pc = self.pc.wrapping_add(1);

        let base = (hi << 8) | lo;
        self.addr_abs = base.wrapping_add(self.y as u16);

        // Check if page was crossed
        if (base & 0xFF00) != (self.addr_abs & 0xFF00) {
            1 // extra cycle needed
        } else {
            0
        }
    }

    pub fn imm(&mut self) -> u8 {
        self.addr_abs = self.pc;
        self.pc = self.pc.wrapping_add(1);
        0
    }

    pub fn zp0(&mut self) -> u8 {
        self.addr_abs = self.memory[self.pc as usize] as u16;
        self.pc = self.pc.wrapping_add(1);
        0
    }

    pub fn zpx(&mut self) -> u8 {
        let base = self.memory[self.pc as usize];
        self.addr_abs = base.wrapping_add(self.x) as u16 & 0x00FF;
        self.pc = self.pc.wrapping_add(1);
        0
    }

    pub fn zpy(&mut self) -> u8 {
        let base = self.memory[self.pc as usize];
        self.addr_abs = base.wrapping_add(self.y) as u16 & 0x00FF; // wrap around zero page
        self.pc = self.pc.wrapping_add(1);

        0
    }

    /// fetches the value from memory at the absolute address (`addr_abs`) and stores it in `fetched`
    pub fn fetch(&mut self) -> u8 {
        self.fetched = self.memory[self.addr_abs as usize];
        self.fetched
    }

    pub fn clock(&mut self) {
        if self.cycles == 0 {
            let opcode = self.memory[self.pc as usize];

            self.pc = self.pc.wrapping_add(1);
            let addr_cycles = (self.instruction_table[opcode as usize].addr_mode)(self);

            (self.instruction_table[opcode as usize].op)(self);
            self.cycles = self.instruction_table[opcode as usize].cycles + addr_cycles;
        }

        self.cycles = self.cycles.saturating_sub(1);
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
            instruction_table: build_instruction_table(),
        }
    }
}
