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
        self.abs(); //  sets addr_abs to target

        // [TODO] Might increment PC before calling this, use `.wrapping_add(1)` instead
        let return_addr = self.pc.wrapping_add(2);
        self.push((return_addr >> 8) as u8); // Push high byte
        self.push((return_addr & 0xFF) as u8); // Push low byte

        self.pc = self.addr_abs;
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

    /// LDY - Load Y
    // LDY supports: imm, zp0, zpx, abs, absx
    pub fn ldy(&mut self) {
        self.fetch();
        self.y = self.fetched;

        self.set_flag(Flag::Zero, self.y == 0);
        self.set_flag(Flag::Negative, self.y & 0x80 != 0);
    }

    /// CPY - Compare Y
    pub fn cpy(&mut self) {
        self.fetch();

        let res = self.y.wrapping_sub(self.fetched);

        self.set_flag(Flag::Carry, self.y >= self.fetched);
        self.set_flag(Flag::Zero, self.y == self.fetched);
        self.set_flag(Flag::Negative, res & 0x80 != 0);
    }

    /// CPX - Compare X
    pub fn cpx(&mut self) {
        self.fetch();

        let res = self.x.wrapping_sub(self.fetched);

        self.set_flag(Flag::Carry, self.x >= self.fetched);
        self.set_flag(Flag::Zero, self.x == self.fetched);
        self.set_flag(Flag::Negative, res & 0x80 != 0);
    }

    /// BIT - Bit Test
    pub fn bit(&mut self) -> u8 {
        self.fetch();

        let result = self.a & self.fetched;

        self.set_flag(Flag::Zero, result == 0); // A & M == 0?
        self.set_flag(Flag::Overflow, self.fetched & 0x40 != 0); // Bit 6 of M
        self.set_flag(Flag::Negative, self.fetched & 0x80 != 0); // Bit 7 of M

        0
    }
    /// STY - Store Y
    pub fn sty(&mut self) {
        self.memory[self.addr_abs as usize] = self.y;
    }

    /// PHP - Push Processor Status
    pub fn php(&mut self) {
        let mut flags = self.status;
        flags |= 1 << Flag::Break as u8; // Set Break flag (bit 4)
        flags |= 1 << Flag::Unused as u8; // Set Unused flag (bit 5)

        self.push(flags);
    }
}
