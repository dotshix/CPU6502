use crate::cpu::{Cpu, Flag};

impl Cpu {
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
}
