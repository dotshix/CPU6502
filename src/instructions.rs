use crate::cpu::{Cpu, Flag};

pub struct Instruction {
    pub name: &'static str,
    pub op: fn(&mut Cpu),              // Might be an issue later
    pub addr_mode: fn(&mut Cpu) -> u8, // Might be an issue later
    pub cycles: u8,
}

impl Default for Instruction {
    fn default() -> Self {
        Self {
            name: "???",
            op: |_cpu| {},
            addr_mode: |_cpu| 0,
            cycles: 0,
        }
    }
}

impl Cpu {
    /// BRK - Break (software IRQ)
    pub fn brk(&mut self) {
        let return_addr = self.pc;

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
        let return_addr = self.pc.wrapping_sub(1);
        self.push((return_addr >> 8) as u8); // Push high byte
        self.push((return_addr & 0xFF) as u8); // Push low byte

        self.pc = self.addr_abs;
    }

    /// RTI - Return from Interrupt
    pub fn rti(&mut self) {
        // 1. Pull Flags
        self.status = self.pull();
        // Bit 5 unused → always 1; Bit 4 (Break) always ignored on RTI
        self.status |= 1 << Flag::Unused as u8;
        self.status &= !(1 << Flag::Break as u8);
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
    pub fn bit(&mut self) {
        self.fetch();

        let result = self.a & self.fetched;

        self.set_flag(Flag::Zero, result == 0); // A & M == 0?
        self.set_flag(Flag::Overflow, self.fetched & 0x40 != 0); // Bit 6 of M
        self.set_flag(Flag::Negative, self.fetched & 0x80 != 0); // Bit 7 of M
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

    /// PLP - Pull Processor Status
    pub fn plp(&mut self) {
        self.status = self.pull();

        // Bit 5 is unused and should always be set to 1.
        self.status |= 1 << Flag::Unused as u8;

        // Bit 4 (Break) is ignored
        self.status &= !(1 << Flag::Break as u8);
    }

    /// PHA - Push A
    pub fn pha(&mut self) {
        self.push(self.a);
    }

    /// PLA - Pull A
    pub fn pla(&mut self) {
        self.a = self.pull();
        self.set_flag(Flag::Zero, self.a == 0);
        self.set_flag(Flag::Negative, self.a & 0x80 != 0);
    }

    /// DEY - Decrement Y
    pub fn dey(&mut self) {
        self.y = self.y.wrapping_sub(1);
        self.set_flag(Flag::Zero, self.y == 0);
        self.set_flag(Flag::Negative, self.y & 0x80 != 0);
    }

    /// TAY - Transfer A to Y
    pub fn tay(&mut self) {
        self.y = self.a;
        self.set_flag(Flag::Zero, self.y == 0);
        self.set_flag(Flag::Negative, self.y & 0x80 != 0);
    }

    /// INY - Increment Y
    pub fn iny(&mut self) {
        self.y = self.y.wrapping_add(1);
        self.set_flag(Flag::Zero, self.y == 0);
        self.set_flag(Flag::Negative, self.y & 0x80 != 0);
    }

    /// INX - Increment X
    pub fn inx(&mut self) {
        self.x = self.x.wrapping_add(1);
        self.set_flag(Flag::Zero, self.x == 0);
        self.set_flag(Flag::Negative, self.x & 0x80 != 0);
    }

    /// JMP - Jump
    pub fn jmp(&mut self) {
        self.pc = self.addr_abs;
    }

    /// BPL - Branch if Plus
    pub fn bpl(&mut self) {
        if !self.get_flag(Flag::Negative) {
            self.cycles += 1;

            self.addr_abs = self.pc.wrapping_add(self.addr_rel as u16);

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles += 1;
            }

            self.pc = self.addr_abs;
        }
    }

    /// BMI - Branch if Minus
    pub fn bmi(&mut self) {
        if self.get_flag(Flag::Negative) {
            self.cycles += 1;

            self.addr_abs = self.pc.wrapping_add(self.addr_rel as u16);

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles += 1;
            }

            self.pc = self.addr_abs;
        }
    }

    /// BVC - Branch if Overflow Clear
    pub fn bvc(&mut self) {
        if !self.get_flag(Flag::Overflow) {
            self.cycles += 1;

            self.addr_abs = self.pc.wrapping_add(self.addr_rel as u16);

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles += 1;
            }

            self.pc = self.addr_abs;
        }
    }

    /// BVS - Branch if Overflow Set
    pub fn bvs(&mut self) {
        if self.get_flag(Flag::Overflow) {
            self.cycles += 1;

            self.addr_abs = self.pc.wrapping_add(self.addr_rel as u16);

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles += 1;
            }

            self.pc = self.addr_abs;
        }
    }

    /// BCC - Branch if Carry Clear
    pub fn bcc(&mut self) {
        if !self.get_flag(Flag::Carry) {
            self.cycles += 1;

            self.addr_abs = self.pc.wrapping_add(self.addr_rel as u16);

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles += 1;
            }

            self.pc = self.addr_abs;
        }
    }

    /// BCS - Branch if Carry Set
    pub fn bcs(&mut self) {
        if self.get_flag(Flag::Carry) {
            self.cycles += 1;

            self.addr_abs = self.pc.wrapping_add(self.addr_rel as u16);

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles += 1;
            }

            self.pc = self.addr_abs;
        }
    }

    /// BNE - Branch if Not Equal
    pub fn bne(&mut self) {
        if !self.get_flag(Flag::Zero) {
            self.cycles += 1;

            self.addr_abs = self.pc.wrapping_add(self.addr_rel as u16);

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles += 1;
            }

            self.pc = self.addr_abs;
        }
    }

    /// BEQ - Branch if Equal
    pub fn beq(&mut self) {
        if self.get_flag(Flag::Zero) {
            self.cycles += 1;

            self.addr_abs = self.pc.wrapping_add(self.addr_rel as u16);

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles += 1;
            }

            self.pc = self.addr_abs;
        }
    }

    /// CLC - Clear Carry
    pub fn clc(&mut self) {
        self.set_flag(Flag::Carry, false);
    }

    /// SEC - Set Carry
    pub fn sec(&mut self) {
        self.set_flag(Flag::Carry, true);
    }

    /// CLI - Clear Interrupt Disable
    pub fn cli(&mut self) {
        self.set_flag(Flag::InterruptDisable, false);
    }

    /// SEI - Set Interrupt Disable
    pub fn sei(&mut self) {
        self.set_flag(Flag::InterruptDisable, true);
    }

    /// TYA - Transfer Y to A
    pub fn tya(&mut self) {
        self.a = self.y;
        self.set_flag(Flag::Zero, self.a == 0);
        self.set_flag(Flag::Negative, self.a & 0x80 != 0);
    }

    /// CLV - Clear Overflow
    pub fn clv(&mut self) {
        self.set_flag(Flag::Overflow, false);
    }

    /// CLD - Clear Decimal
    pub fn cld(&mut self) {
        self.set_flag(Flag::Decimal, false);
    }

    /// SED - Set Decimal
    pub fn sed(&mut self) {
        self.set_flag(Flag::Decimal, true);
    }

    /// ORA - Bitwise OR
    pub fn ora(&mut self) {
        self.fetch();
        self.a |= self.fetched;

        self.set_flag(Flag::Zero, self.a == 0);
        self.set_flag(Flag::Negative, self.a & 0x80 != 0);
    }

    /// AND - Bitwise AND
    pub fn and(&mut self) {
        self.fetch();
        self.a &= self.fetched;

        self.set_flag(Flag::Zero, self.a == 0);
        self.set_flag(Flag::Negative, self.a & 0x80 != 0);
    }

    /// EOR - Bitwise Exclusive OR
    pub fn eor(&mut self) {
        self.fetch();
        self.a ^= self.fetched;

        self.set_flag(Flag::Zero, self.a == 0);
        self.set_flag(Flag::Negative, self.a & 0x80 != 0);
    }
    /// ADC - Add with Carry
    pub fn adc(&mut self) {
        self.fetch();
        let carry_in = if self.get_flag(Flag::Carry) { 1 } else { 0 };

        let a = self.a;
        let m = self.fetched;

        let result = a as u16 + m as u16 + carry_in as u16;

        self.set_flag(Flag::Carry, result > 0xFF);
        self.set_flag(Flag::Zero, (result & 0xFF) == 0);
        self.set_flag(Flag::Overflow, (!(a ^ m) & (a ^ result as u8) & 0x80) != 0);
        self.set_flag(Flag::Negative, (result & 0x80) != 0);

        self.a = result as u8;
    }

    /// STA - Store A
    pub fn sta(&mut self) {
        self.memory[self.addr_abs as usize] = self.a;
    }

    /// LDA - Load A
    pub fn lda(&mut self) {
        self.fetch();
        self.a = self.fetched;

        self.set_flag(Flag::Zero, self.a == 0);
        self.set_flag(Flag::Negative, self.a & 0x80 != 0);
    }

    /// CMP - Compare A
    pub fn cmp(&mut self) {
        self.fetch();
        let res = self.a.wrapping_sub(self.fetched);

        self.set_flag(Flag::Carry, self.a >= self.fetched);
        self.set_flag(Flag::Zero, self.a == self.fetched);
        self.set_flag(Flag::Negative, res & 0x80 != 0);
    }

    /// SBC - Subtract with Carry
    pub fn sbc(&mut self) {
        self.fetch();

        let value = (self.fetched as u16) ^ 0x00FF; // bitwise NOT of fetched
        let carry_in = if self.get_flag(Flag::Carry) { 1 } else { 0 };

        let sum = self.a as u16 + value + carry_in;

        self.set_flag(Flag::Carry, sum > 0xFF);
        self.set_flag(Flag::Zero, (sum & 0xFF) == 0);
        self.set_flag(
            Flag::Overflow,
            ((sum ^ self.a as u16) & (sum ^ value) & 0x80) != 0,
        );
        self.set_flag(Flag::Negative, (sum & 0x80) != 0);

        self.a = (sum & 0xFF) as u8; // can leabe out 0xFF but its less clear
    }

    /// ASL - Arithmetic Shift Left (Accumulator)
    pub fn asl_acc(&mut self) {
        // get carry from bit 7
        self.set_flag(Flag::Carry, self.a & 0x80 != 0);

        // Shift A left by 1
        self.a <<= 1;

        self.set_flag(Flag::Zero, self.a == 0);
        self.set_flag(Flag::Negative, self.a & 0x80 != 0);
    }

    /// ASL - Arithmetic Shift Left (Memory)
    pub fn asl_mem(&mut self) {
        let value = self.memory[self.addr_abs as usize];

        // [Read-Modify-Write] Write original value back
        self.memory[self.addr_abs as usize] = value;

        // Step 2: Perform shift
        let res = value << 1;

        // Step 3: Set flags
        self.set_flag(Flag::Carry, value & 0x80 != 0);
        self.set_flag(Flag::Zero, res == 0);
        self.set_flag(Flag::Negative, res & 0x80 != 0);

        // Step 4: Write result
        self.memory[self.addr_abs as usize] = res;
    }

    /// ROL - Rotate Left (Memory)
    pub fn rol_mem(&mut self) {
        let value = self.memory[self.addr_abs as usize];

        // [Read-Modify-Write] Write original value back
        self.memory[self.addr_abs as usize] = value;

        let carry_flag = if self.get_flag(Flag::Carry) { 1 } else { 0 };

        // Step 2: Perform shift
        let res = value << 1 | carry_flag;

        // Step 3: Set flags
        self.set_flag(Flag::Carry, value & 0x80 != 0);
        self.set_flag(Flag::Zero, res == 0);
        self.set_flag(Flag::Negative, res & 0x80 != 0);

        // Step 4: Write result
        self.memory[self.addr_abs as usize] = res;
    }

    /// ROL - Rotate Left (Accumulator)
    pub fn rol_acc(&mut self) {
        let carry_in = if self.get_flag(Flag::Carry) { 1 } else { 0 };
        let old_a = self.a;
        let result = (old_a << 1) | carry_in;

        self.set_flag(Flag::Carry, old_a & 0x80 != 0);
        self.set_flag(Flag::Zero, result == 0);
        self.set_flag(Flag::Negative, result & 0x80 != 0);

        self.a = result;
    }

    /// LSR - Logical Shift Right
    pub fn lsr_mem(&mut self) {
        let value = self.memory[self.addr_abs as usize];

        // [Read-Modify-Write] Write original value back
        self.memory[self.addr_abs as usize] = value;

        // Step 2: Perform shift
        let res = value >> 1;

        // Step 3: Set flags
        self.set_flag(Flag::Carry, value & 0x1 != 0);
        self.set_flag(Flag::Zero, res == 0);
        self.set_flag(Flag::Negative, res & 0x80 != 0);

        // Step 4: Write result
        self.memory[self.addr_abs as usize] = res;
    }

    /// LSR - Logical Shift Right (Accumulator)
    pub fn lsr_acc(&mut self) {
        let old_a = self.a;
        let result = old_a >> 1;

        self.set_flag(Flag::Carry, old_a & 0x1 != 0);
        self.set_flag(Flag::Zero, result == 0);
        self.set_flag(Flag::Negative, result & 0x80 != 0);

        self.a = result;
    }

    /// ROR - Rotate Right (Memory)
    pub fn ror_mem(&mut self) {
        let value = self.memory[self.addr_abs as usize];

        // [Read-Modify-Write] Write original value back
        self.memory[self.addr_abs as usize] = value;

        let carry_flag = if self.get_flag(Flag::Carry) { 1 } else { 0 };

        // Step 2: Perform shift
        let res = (value >> 1) | (carry_flag << 7);

        // Step 3: Set flags
        self.set_flag(Flag::Carry, value & 0x01 != 0);
        self.set_flag(Flag::Zero, res == 0);
        self.set_flag(Flag::Negative, res & 0x80 != 0);

        // Step 4: Write result
        self.memory[self.addr_abs as usize] = res;
    }

    /// ROR - Rotate Right (Accumulator)
    pub fn ror_acc(&mut self) {
        let carry_in = if self.get_flag(Flag::Carry) { 1 } else { 0 };
        let old_a = self.a;
        let result = (old_a >> 1) | (carry_in << 7);

        self.set_flag(Flag::Carry, old_a & 0x01 != 0);
        self.set_flag(Flag::Zero, result == 0);
        self.set_flag(Flag::Negative, result & 0x80 != 0);

        self.a = result;
    }

    /// STX - Store X
    pub fn stx(&mut self) {
        self.memory[self.addr_abs as usize] = self.x;
    }

    /// TXA - Transfer X to A
    pub fn txa(&mut self) {
        self.a = self.x;

        self.set_flag(Flag::Zero, self.a == 0);
        self.set_flag(Flag::Negative, self.a & 0x80 != 0);
    }

    /// TAX - Transfer A to X
    pub fn tax(&mut self) {
        self.x = self.a;

        self.set_flag(Flag::Zero, self.a == 0);
        self.set_flag(Flag::Negative, self.a & 0x80 != 0);
    }

    /// LDX - Load X
    pub fn ldx(&mut self) {
        self.fetch();
        self.x = self.fetched;

        self.set_flag(Flag::Zero, self.x == 0);
        self.set_flag(Flag::Negative, self.x & 0x80 != 0);
    }

    /// TSX - Transfer Stack Pointer to X
    pub fn tsx(&mut self) {
        self.x = self.sp;

        self.set_flag(Flag::Zero, self.x == 0);
        self.set_flag(Flag::Negative, self.x & 0x80 != 0);
    }

    /// TXS - Transfer X to Stack Pointer
    pub fn txs(&mut self) {
        self.sp = self.x;
    }

    /// DEC - Decrement Memory
    pub fn dec(&mut self) {
        let value = self.memory[self.addr_abs as usize];

        // [Read-Modify-Write] Write original value back
        self.memory[self.addr_abs as usize] = value;

        // Step 2: Perform math
        let res = value.wrapping_sub(1);

        // Step 3: Set flags
        self.set_flag(Flag::Zero, res == 0);
        self.set_flag(Flag::Negative, res & 0x80 != 0);

        // Step 4: Write result
        self.memory[self.addr_abs as usize] = res;
    }

    /// DEX - Decrement X
    pub fn dex(&mut self) {
        self.x = self.x.wrapping_sub(1);

        self.set_flag(Flag::Zero, self.x == 0);
        self.set_flag(Flag::Negative, self.x & 0x80 != 0);
    }

    /// INC - Increment Memory
    pub fn inc(&mut self) {
        let value = self.memory[self.addr_abs as usize];

        // [Read-Modify-Write] Write original value back
        self.memory[self.addr_abs as usize] = value;

        // Step 2: Perform math
        let res = value.wrapping_add(1);

        // Step 3: Set flags
        self.set_flag(Flag::Zero, res == 0);
        self.set_flag(Flag::Negative, res & 0x80 != 0);

        // Step 4: Write result
        self.memory[self.addr_abs as usize] = res;
    }

    /// NOP - No Operation
    pub fn nop(&mut self) {}
}
