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
