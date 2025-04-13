use crate::cpu::{Cpu, Flag};

impl Cpu {
    /// BRK - Break (software IRQ)
    pub fn brk(&mut self) {
        // [TODO] Might increment PC before calling this, use `.wrapping_add(1)` instead
        let return_addr = self.pc.wrapping_add(2);

        self.push((return_addr >> 8) as u8); // Push high byte
        self.push((return_addr & 0xFF) as u8); // push low byte

        let mut flags = self.status;
        flags |= 1 << Flag::Break as u8;
        flags |= 1 << Flag::Unused as u8;
        self.push(flags);

        self.set_flag(Flag::InterruptDisable, true);

        let lo = self.memory[0xFFFE] as u16;
        let hi = self.memory[0xFFFF] as u16;
        self.pc = (hi << 8) | lo;
    }

    /// JSR - Jump to Subroutine
    pub fn jsr(&mut self) {
        // Fetch target address from instruction stream
        let lo = self.memory[(self.pc + 1) as usize] as u16;
        let hi = self.memory[(self.pc + 2) as usize] as u16;
        let target = (hi << 8) | lo;

        // [TODO] Might increment PC before calling this, use `.wrapping_add(1)` instead
        let return_addr = self.pc.wrapping_add(2);
        self.push((return_addr >> 8) as u8); // Push high byte
        self.push((return_addr & 0xFF) as u8); // Push low byte

        self.pc = target;
    }

    /// RTI - Return from Interrupt
    pub fn rti(&mut self) {
        // 1. Pull Flags
        self.status = self.pull();
        self.status |= 1 << Flag::Unused as u8; // Always set bit 5

        // 2. Pull PC low byte
        let pcl = self.pull() as u16;

        // 3. Pull high byte
        let pch = self.pull() as u16;

        self.pc = (pch << 8) | pcl;
    }

    /// RTS - Return from Subroutine
    pub fn rts(&mut self) {
        let pcl = self.pull() as u16;
        let pch = self.pull() as u16;

        self.pc = (pch << 8) | pcl;
        self.pc += 1;
    }
}
