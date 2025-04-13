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
