use cpu6502::cpu::{Cpu, Flag};

#[test]
fn test_brk_instruction() {
    let mut cpu = Cpu::default();

    // Simulate starting at $8000
    cpu.pc = 0x8000;

    // Set IRQ vector to point to $1234
    cpu.memory[0xFFFE] = 0x34;
    cpu.memory[0xFFFF] = 0x12;

    let pre_sp = cpu.sp;

    // Execute BRK
    cpu.brk();

    // PC should now be loaded from IRQ vector
    assert_eq!(cpu.pc, 0x1234);

    // Stack pointer should be down by 3
    assert_eq!(cpu.sp, pre_sp.wrapping_sub(3));

    // Read from stack: post-BRK SP points just below last pushed byte
    let stack_base = 0x0100;
    let flags_byte = cpu.memory[stack_base + cpu.sp.wrapping_add(1) as usize];
    let pcl = cpu.memory[stack_base + cpu.sp.wrapping_add(2) as usize];
    let pch = cpu.memory[stack_base + cpu.sp.wrapping_add(3) as usize];

    // Pushed PC should be 0x8002
    let pushed_pc = ((pch as u16) << 8) | pcl as u16;
    assert_eq!(pushed_pc, 0x8002);

    // Flags pushed should include Break and Unused
    let break_bit = 1 << Flag::Break as u8;
    let unused_bit = 1 << Flag::Unused as u8;
    assert_eq!(flags_byte & break_bit, break_bit);
    assert_eq!(flags_byte & unused_bit, unused_bit);

    // Interrupt Disable flag should be set
    assert!(cpu.get_flag(Flag::InterruptDisable));
}

#[test]
fn test_jsr_instruction() {
    let mut cpu = Cpu::default();

    // Place JSR at 0x8000 with target address 0x1234
    cpu.pc = 0x8000;
    cpu.memory[0x8000] = 0x20; // JSR opcode (for clarity)
    cpu.memory[0x8001] = 0x34; // target lo
    cpu.memory[0x8002] = 0x12; // target hi

    let pre_sp = cpu.sp;

    // Call JSR (PC still points to opcode)
    cpu.jsr();

    // PC should now be set to 0x1234
    assert_eq!(cpu.pc, 0x1234);

    // SP should be decremented by 2
    assert_eq!(cpu.sp, pre_sp.wrapping_sub(2));

    // Check that return address 0x8002 was pushed
    let stack_base = 0x0100;
    let pushed_pcl = cpu.memory[stack_base + cpu.sp.wrapping_add(1) as usize];
    let pushed_pch = cpu.memory[stack_base + cpu.sp.wrapping_add(2) as usize];
    let pushed_pc = ((pushed_pch as u16) << 8) | (pushed_pcl as u16);

    assert_eq!(pushed_pc, 0x8002);
}

#[test]
fn test_brk_then_rti() {
    let mut cpu = Cpu::default();

    // Initial CPU state before interrupt
    cpu.pc = 0x8000;
    cpu.status = 0b0011_0101;

    // Set BRK vector to jump to handler at 0x9000
    cpu.memory[0xFFFE] = 0x00;
    cpu.memory[0xFFFF] = 0x90;

    // Save stack pointer before BRK
    let sp_before = cpu.sp;

    // Execute BRK (simulate interrupt)
    cpu.brk();

    // CPU should now jump to 0x9000
    assert_eq!(cpu.pc, 0x9000);

    // Check that SP was decremented by 3
    assert_eq!(cpu.sp, sp_before.wrapping_sub(3));

    // Now simulate that handler modified status
    cpu.status = 0b1111_0000; // Random changed state

    // Execute RTI to restore original PC and flags
    cpu.rti();

    // PC should now be 0x8002 (skipped BRK + 1 byte)
    assert_eq!(cpu.pc, 0x8002);

    // Status should be restored (original + Unused bit set)
    let expected_flags = 0b0011_0101 | (1 << Flag::Unused as u8);
    assert_eq!(cpu.status, expected_flags);

    // SP should be back to original
    assert_eq!(cpu.sp, sp_before);
}

#[test]
fn test_jsr_then_rts() {
    let mut cpu = Cpu::default();

    // Initial CPU state
    cpu.pc = 0x8000;

    // Place a JSR $1234 at $8000
    cpu.memory[0x8000] = 0x20; // JSR opcode (not used by jsr() itself here)
    cpu.memory[0x8001] = 0x34; // target lo
    cpu.memory[0x8002] = 0x12; // target hi

    // Track SP before JSR
    let sp_before_jsr = cpu.sp;

    // Call JSR — should push 0x8002 and set PC to 0x1234
    cpu.jsr();

    assert_eq!(cpu.pc, 0x1234);
    assert_eq!(cpu.sp, sp_before_jsr.wrapping_sub(2));

    // Simulate that RTS is located at the subroutine target
    cpu.rts();

    // After RTS, we should return to the instruction *after* the original JSR
    assert_eq!(cpu.pc, 0x8003);

    // Stack pointer should be back to original
    assert_eq!(cpu.sp, sp_before_jsr);
}

#[test]
fn test_ldy_immediate() {
    let mut cpu = Cpu::default();

    // Simulate LDY $42 at 0x8000
    cpu.pc = 0x8000;
    cpu.memory[0x8001] = 0x42;

    cpu.imm(); // sets addr_abs to 0x8001
    cpu.ldy(); // fetches and loads Y

    assert_eq!(cpu.addr_abs, 0x8001);
    assert_eq!(cpu.y, 0x42);
    assert!(!cpu.get_flag(Flag::Zero));
    assert!(!cpu.get_flag(Flag::Negative));
}

#[test]
fn test_ldy_absolute() {
    let mut cpu = Cpu::default();

    cpu.pc = 0x8000;
    // Absolute target: 0x1234
    cpu.memory[0x8001] = 0x34; // lo
    cpu.memory[0x8002] = 0x12; // hi
    cpu.memory[0x1234] = 0xFF; // value to load into Y

    cpu.abs(); // sets addr_abs to 0x1234
    cpu.ldy(); // fetches and loads Y

    assert_eq!(cpu.addr_abs, 0x1234);
    assert_eq!(cpu.y, 0xFF);
    assert!(!cpu.get_flag(Flag::Zero));
    assert!(cpu.get_flag(Flag::Negative));
}

#[test]
fn test_zp0_addressing_mode() {
    let mut cpu = Cpu::default();

    // Simulate program at 0x8000
    cpu.pc = 0x8000;

    // Place operand for zero page at PC
    cpu.memory[0x8000] = 0x42; // Will be interpreted as address $0042
    cpu.memory[0x0042] = 0xAB; // Actual data to load via LDY

    cpu.zp0(); // sets addr_abs to 0x0042

    // Now fetch from addr_abs and load into Y
    cpu.ldy(); // fetches 0xAB from memory[0x0042]

    // Check internal state
    assert_eq!(cpu.addr_abs, 0x0042);
    assert_eq!(cpu.y, 0xAB);
    assert_eq!(cpu.pc, 0x8001);

    // Flag assertions
    assert!(!cpu.get_flag(Flag::Zero));
    assert!(cpu.get_flag(Flag::Negative)); // 0xAB has bit 7 set
}

#[test]
fn test_zpx_addressing_mode() {
    let mut cpu = Cpu::default();

    cpu.pc = 0x8000;
    cpu.x = 0x10;
    cpu.memory[0x8000] = 0x20; // base = 0x20
    cpu.memory[0x0030] = 0x55; // 0x20 + 0x10 = 0x30 (within zero page)

    cpu.zpx(); // should set addr_abs = 0x0030
    cpu.ldy(); // load Y from addr_abs

    assert_eq!(cpu.addr_abs, 0x0030);
    assert_eq!(cpu.y, 0x55);
    assert_eq!(cpu.pc, 0x8001);
    assert!(!cpu.get_flag(Flag::Zero));
    assert!(!cpu.get_flag(Flag::Negative));
}

#[test]
fn test_absx_no_page_cross() {
    let mut cpu = Cpu::default();

    // Base address: 0x1230, X = 0x0A → addr_abs = 0x123A
    cpu.pc = 0x8000;
    cpu.memory[0x8001] = 0x30; // low byte
    cpu.memory[0x8002] = 0x12; // high byte
    cpu.x = 0x0A;

    let extra_cycle = cpu.absx();

    assert_eq!(cpu.addr_abs, 0x123A); //  Correct address
    assert_eq!(extra_cycle, 0); // No page cross
}

#[test]
fn test_absx_with_page_cross() {
    let mut cpu = Cpu::default();

    // Base = 0x12F0, X = 0x20 → addr_abs = 0x1310 (crosses page)
    cpu.pc = 0x8000;
    cpu.memory[0x8001] = 0xF0; // low
    cpu.memory[0x8002] = 0x12; // high
    cpu.x = 0x20;

    let extra_cycle = cpu.absx();

    assert_eq!(cpu.addr_abs, 0x1310); //  Correct address
    assert_eq!(extra_cycle, 1); //  Page crossed
}

#[test]
fn test_cpy_equal_sets_zero_and_carry() {
    let mut cpu = Cpu::default();
    cpu.y = 0x42;

    cpu.pc = 0x8000;
    cpu.memory[0x8001] = 0x42; // Immediate operand

    cpu.imm(); // addr_abs = 0x8001
    cpu.cpy();

    assert!(cpu.get_flag(Flag::Zero)); // Equal
    assert!(cpu.get_flag(Flag::Carry)); // Y >= mem
    assert!(!cpu.get_flag(Flag::Negative)); // 0 - 0x42 = 0, bit 7 clear
}

#[test]
fn test_cpy_less_than_clears_carry_sets_negative() {
    let mut cpu = Cpu::default();
    cpu.y = 0x10;
    cpu.memory[0x0042] = 0x20; // Y < mem
    cpu.pc = 0x8000;
    cpu.memory[0x8000] = 0x42; // zp0 operand

    cpu.zp0(); // sets addr_abs = 0x0042
    cpu.cpy();

    assert!(!cpu.get_flag(Flag::Zero));
    assert!(!cpu.get_flag(Flag::Carry)); // Y < mem
    assert!(cpu.get_flag(Flag::Negative)); // 0x10 - 0x20 underflows to negative
}

#[test]
fn test_cpy_greater_than_sets_carry() {
    let mut cpu = Cpu::default();
    cpu.y = 0x90;
    cpu.memory[0x1234] = 0x20; // Y > mem
    cpu.pc = 0x8000;
    cpu.memory[0x8001] = 0x34; // lo
    cpu.memory[0x8002] = 0x12; // hi

    cpu.abs(); // sets addr_abs = 0x1234
    cpu.cpy();

    assert!(!cpu.get_flag(Flag::Zero));
    assert!(cpu.get_flag(Flag::Carry)); // Y > mem
    assert!(!cpu.get_flag(Flag::Negative)); // result not negative
}

#[test]
fn test_cpy_negative_result_sets_negative_flag() {
    let mut cpu = Cpu::default();
    cpu.y = 0x10;

    cpu.pc = 0x8000;
    cpu.memory[0x8001] = 0x80; // Y - mem = 0x10 - 0x80 = 0x90 (bit 7 set)

    cpu.imm();
    cpu.cpy();

    assert!(cpu.get_flag(Flag::Negative));
    assert!(!cpu.get_flag(Flag::Zero));
    assert!(!cpu.get_flag(Flag::Carry));
}

#[test]
fn test_cpx_equal_sets_zero_and_carry() {
    let mut cpu = Cpu::default();
    cpu.x = 0x42;

    cpu.pc = 0x8000;
    cpu.memory[0x8001] = 0x42; // Immediate operand

    cpu.imm(); // addr_abs = 0x8001
    cpu.cpx();

    assert!(cpu.get_flag(Flag::Zero)); // Equal
    assert!(cpu.get_flag(Flag::Carry)); // X >= mem
    assert!(!cpu.get_flag(Flag::Negative)); // 0 - 0x42 = 0, bit 7 clear
}

#[test]
fn test_cpx_less_than_clears_carry_sets_negative() {
    let mut cpu = Cpu::default();
    cpu.x = 0x10;
    cpu.memory[0x0042] = 0x20; // X < mem
    cpu.pc = 0x8000;
    cpu.memory[0x8000] = 0x42; // zp0 operand

    cpu.zp0(); // sets addr_abs = 0x0042
    cpu.cpx();

    assert!(!cpu.get_flag(Flag::Zero));
    assert!(!cpu.get_flag(Flag::Carry)); // x < mem
    assert!(cpu.get_flag(Flag::Negative)); // 0x10 - 0x20 underflows to negative
}

#[test]
fn test_cpx_greater_than_sets_carry() {
    let mut cpu = Cpu::default();
    cpu.x = 0x90;
    cpu.memory[0x1234] = 0x20; // X > mem
    cpu.pc = 0x8000;
    cpu.memory[0x8001] = 0x34; // lo
    cpu.memory[0x8002] = 0x12; // hi

    cpu.abs(); // sets addr_abs = 0x1234
    cpu.cpx();

    assert!(!cpu.get_flag(Flag::Zero));
    assert!(cpu.get_flag(Flag::Carry)); // x > mem
    assert!(!cpu.get_flag(Flag::Negative)); // result not negative
}

#[test]
fn test_cpx_negative_result_sets_negative_flag() {
    let mut cpu = Cpu::default();
    cpu.x = 0x10;

    cpu.pc = 0x8000;
    cpu.memory[0x8001] = 0x80;

    cpu.imm();
    cpu.cpx();

    assert!(cpu.get_flag(Flag::Negative));
    assert!(!cpu.get_flag(Flag::Zero));
    assert!(!cpu.get_flag(Flag::Carry));
}

#[test]
fn test_bit_sets_zero_flag_if_result_is_zero() {
    let mut cpu = Cpu::default();
    cpu.a = 0b0000_0000; // Accumulator = 0
    cpu.memory[0x0042] = 0b1100_0000; // Memory = non-zero value with bit 7 and bit 6 set
    cpu.pc = 0x8000;
    cpu.memory[0x8000] = 0x42;

    cpu.zp0(); // addr_abs = 0x0042
    cpu.bit();

    assert!(cpu.get_flag(Flag::Zero)); // A & M == 0
    assert!(cpu.get_flag(Flag::Overflow)); // Bit 6 is 1
    assert!(cpu.get_flag(Flag::Negative)); // Bit 7 is 1
}

#[test]
fn test_bit_clears_zero_flag_if_result_nonzero() {
    let mut cpu = Cpu::default();
    cpu.a = 0b0000_0100;
    cpu.memory[0x1234] = 0b0100_0100; // shares with register A

    cpu.memory[0x8001] = 0x34;
    cpu.memory[0x8002] = 0x12;
    cpu.pc = 0x8000;

    cpu.abs(); // addr_abs = 0x1234
    cpu.bit();

    assert!(!cpu.get_flag(Flag::Zero)); // A & M != 0
    assert!(cpu.get_flag(Flag::Overflow)); // Bit 6 is 1
    assert!(!cpu.get_flag(Flag::Negative)); // Bit 7 is 0
}

#[test]
fn test_bit_negative_and_overflow_flags_from_memory() {
    let mut cpu = Cpu::default();
    cpu.a = 0b1111_1111;
    cpu.memory[0x0042] = 0b1100_0000; // Bits 7 and 6 set

    cpu.pc = 0x8000;
    cpu.memory[0x8000] = 0x42;

    cpu.zp0(); // addr_abs = 0x0042
    cpu.bit();

    assert!(!cpu.get_flag(Flag::Zero)); // A & M != 0
    assert!(cpu.get_flag(Flag::Overflow)); // Bit 6 is 1
    assert!(cpu.get_flag(Flag::Negative)); // Bit 7 is 1
}

#[test]
fn test_bit_no_flags_set_when_all_clear() {
    let mut cpu = Cpu::default();
    cpu.a = 0b0000_0100;
    cpu.memory[0x1234] = 0b0000_0000; // No bits set

    cpu.memory[0x8001] = 0x34;
    cpu.memory[0x8002] = 0x12;
    cpu.pc = 0x8000;

    cpu.abs(); // addr_abs = 0x1234
    cpu.bit();

    assert!(cpu.get_flag(Flag::Zero)); // A & M == 0
    assert!(!cpu.get_flag(Flag::Overflow)); // Bit 6 = 0
    assert!(!cpu.get_flag(Flag::Negative)); // Bit 7 = 0
}

#[test]
fn test_sty_zero_page() {
    let mut cpu = Cpu::default();
    cpu.y = 0xAB;

    cpu.memory[0x8000] = 0x42; // Operand (zero page address)
    cpu.pc = 0x8000;

    cpu.zp0(); // addr_abs = 0x0042
    cpu.sty(); // memory[addr_abs] = y

    assert_eq!(cpu.memory[0x0042], 0xAB);
}

#[test]
fn test_sty_zero_page_x_wraps() {
    let mut cpu = Cpu::default();
    cpu.y = 0xCD;
    cpu.x = 0x10;

    cpu.memory[0x8000] = 0xF0; // base addr = 0xF0
    cpu.pc = 0x8000;

    cpu.zpx(); // addr_abs = (0xF0 + 0x10) & 0xFF = 0x00
    cpu.sty();

    assert_eq!(cpu.memory[0x0000], 0xCD);
}

#[test]
fn test_sty_absolute() {
    let mut cpu = Cpu::default();
    cpu.y = 0x77;

    cpu.pc = 0x8000;
    cpu.memory[0x8001] = 0x34; // lo
    cpu.memory[0x8002] = 0x12; // hi

    cpu.abs(); // addr_abs = 0x1234
    cpu.sty();

    assert_eq!(cpu.memory[0x1234], 0x77);
}

#[test]
fn test_php_pushes_correct_flags() {
    let mut cpu = Cpu::default();

    // Set some status flags
    cpu.set_flag(Flag::Carry, true);
    cpu.set_flag(Flag::InterruptDisable, true);
    cpu.set_flag(Flag::Negative, true);

    let sp_before = cpu.sp;

    cpu.php(); // Pushes status to stack

    // Stack should be decremented by 1
    assert_eq!(cpu.sp, sp_before.wrapping_sub(1));

    // Read back the pushed status byte
    // Stack grows down, so actual pushed byte is at SP + 1
    let pushed_flags = cpu.memory[0x0100 + cpu.sp.wrapping_add(1) as usize];

    // expected pushed flags
    let expected = cpu.status | (1 << Flag::Break as u8) | (1 << Flag::Unused as u8);

    assert_eq!(pushed_flags, expected);
    assert_eq!(pushed_flags & 0b00110000, 0b00110000); // Break + Unused set
}

#[test]
fn test_plp_restores_flags() {
    let mut cpu = Cpu::default();

    // Create a byte with all flags flipped on
    let flags: u8 = 0b1111_1111;

    // Push this byte to the stack manually
    cpu.push(flags);

    // Execute PLP to pull status
    cpu.plp();

    // Check relevant flags are restored
    // PLP ignores the Break flag (bit 4), so we do not test it.
    assert!(cpu.get_flag(Flag::Carry));
    assert!(cpu.get_flag(Flag::Zero));
    assert!(cpu.get_flag(Flag::InterruptDisable));
    assert!(cpu.get_flag(Flag::Decimal));
    assert!(cpu.get_flag(Flag::Overflow));
    assert!(cpu.get_flag(Flag::Negative));

    // Ensure bit 5 (Unused) is set
    assert_eq!(
        cpu.status & (1 << Flag::Unused as u8),
        1 << Flag::Unused as u8
    );
}

#[test]
fn test_pha_pushes_accumulator_to_stack() {
    let mut cpu = Cpu::default();

    cpu.a = 0x42; // Set A to some value
    let sp_before = cpu.sp; // Record current stack pointer

    cpu.pha(); // Push A onto the stack

    // Stack pointer should be decremented by 1
    assert_eq!(cpu.sp, sp_before.wrapping_sub(1));

    // Value pushed should match A
    let pushed = cpu.memory[0x0100 + cpu.sp.wrapping_add(1) as usize];
    assert_eq!(pushed, 0x42);
}

#[test]
fn test_pla_sets_accumulator_and_flags() {
    let mut cpu = Cpu::default();

    // Simulate value 0x80 (bit 7 set) pushed to the stack
    cpu.push(0x80);
    cpu.a = 0; // clear A

    cpu.pla(); // Pull into A

    // Accumulator should now be 0x80
    assert_eq!(cpu.a, 0x80);

    // Zero flag should be false (A != 0)
    assert!(!cpu.get_flag(Flag::Zero));

    // Negative flag should be set (bit 7 of A = 1)
    assert!(cpu.get_flag(Flag::Negative));
}

#[test]
fn test_pla_sets_zero_flag_if_result_zero() {
    let mut cpu = Cpu::default();

    cpu.push(0x00);
    cpu.a = 0xFF;

    cpu.pla();

    assert_eq!(cpu.a, 0x00);
    assert!(cpu.get_flag(Flag::Zero));
    assert!(!cpu.get_flag(Flag::Negative));
}

#[test]
fn test_dey_decrements_y_and_sets_flags() {
    let mut cpu = Cpu::default();
    cpu.y = 0x01;

    cpu.dey();

    // Y should now be 0x00
    assert_eq!(cpu.y, 0x00);

    // Zero flag should be set (Y == 0)
    assert!(cpu.get_flag(Flag::Zero));

    // Negative flag should be clear (bit 7 == 0)
    assert!(!cpu.get_flag(Flag::Negative));
}

#[test]
fn test_tay_transfers_a_to_y_and_sets_flags() {
    let mut cpu = Cpu::default();

    cpu.a = 0x00;
    cpu.y = 0xFF; // ensure Y is different

    cpu.tay();

    // Y should now equal A
    assert_eq!(cpu.y, 0x00);

    // Zero flag should be set (Y == 0)
    assert!(cpu.get_flag(Flag::Zero));

    // Negative flag should be clear (bit 7 == 0)
    assert!(!cpu.get_flag(Flag::Negative));
}

#[test]
fn test_tay_sets_negative_flag() {
    let mut cpu = Cpu::default();

    cpu.a = 0x80; // bit 7 set
    cpu.tay();

    assert_eq!(cpu.y, 0x80);
    assert!(!cpu.get_flag(Flag::Zero));
    assert!(cpu.get_flag(Flag::Negative));
}

#[test]
fn test_iny_increments_y_and_sets_zero_and_negative_flags() {
    let mut cpu = Cpu::default();
    cpu.y = 0x00;

    cpu.iny();

    // Y should now be 0x01
    assert_eq!(cpu.y, 0x01);

    // Flags: Z = false, N = false
    assert!(!cpu.get_flag(Flag::Zero));
    assert!(!cpu.get_flag(Flag::Negative));
}

#[test]
fn test_inx_increments_x_and_sets_zero_and_negative_flags() {
    let mut cpu = Cpu::default();
    cpu.x = 0x00;

    cpu.inx();

    // Y should now be 0x01
    assert_eq!(cpu.x, 0x01);

    // Flags: Z = false, N = false
    assert!(!cpu.get_flag(Flag::Zero));
    assert!(!cpu.get_flag(Flag::Negative));
}

#[test]
fn test_jmp_absolute_sets_pc_correctly() {
    let mut cpu = Cpu::default();

    cpu.pc = 0x8000;
    cpu.memory[0x8001] = 0x34; // low byte
    cpu.memory[0x8002] = 0x12; // high byte

    cpu.abs();
    cpu.jmp();

    assert_eq!(cpu.pc, 0x1234);
}

#[test]
fn test_jmp_indirect_sets_pc_correctly() {
    let mut cpu = Cpu::default();

    cpu.pc = 0x8000;
    cpu.memory[0x8001] = 0x00; // indirect ptr lo
    cpu.memory[0x8002] = 0x30; // indirect ptr hi

    // Set value at $3000 = $5678
    cpu.memory[0x3000] = 0x78; // low byte
    cpu.memory[0x3001] = 0x56; // high byte

    cpu.ind();
    cpu.jmp();

    assert_eq!(cpu.pc, 0x5678);
}

#[test]
fn test_jmp_indirect_page_boundary_bug() {
    let mut cpu = Cpu::default();

    cpu.pc = 0x8000;
    cpu.memory[0x8001] = 0xFF; // pointer lo
    cpu.memory[0x8002] = 0x30; // pointer hi => pointer = $30FF

    cpu.memory[0x30FF] = 0xCD; // low byte of target
    cpu.memory[0x3000] = 0xAB; // high byte (should be at $3100 but bug uses $3000)

    cpu.ind();
    cpu.jmp();

    assert_eq!(cpu.pc, 0xABCD); // bug causes wraparound
}
