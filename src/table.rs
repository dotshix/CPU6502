use crate::{cpu::Cpu, instructions::Instruction};
use std::array::from_fn;

// Build table function
// Probably shoulduse oncecell maybe?
pub fn build_instruction_table() -> [Instruction; 256] {
    let table = from_fn(|opcode| match opcode {
        // BRK - Break (software IRQ)
        0x00 => Instruction {
            name: "brk",
            op: Cpu::brk,
            addr_mode: Cpu::imm,
            cycles: 7,
        },
        _ => Instruction::default(),
    });

    table
}
