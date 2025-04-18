use crate::{cpu::Cpu, instructions::Instruction};
use std::array::from_fn;

// Build table function
// Probably shoulduse oncecell maybe?
pub fn build_instruction_table() -> [Instruction; 256] {
    let table = from_fn(|opcode| match opcode {
        0x00 => Instruction {
            name: "brk",
            op: Cpu::brk,
            addr_mode: Cpu::imm,
            cycles: 7,
        },

        0x01 => Instruction {
            name: "ora",
            op: Cpu::ora,
            addr_mode: Cpu::indx,
            cycles: 6,
        },

        0x02 => Instruction {
            name: "ora",
            op: Cpu::ora,
            addr_mode: Cpu::indx,
            cycles: 6,
        },

        0x05 => Instruction {
            name: "ora",
            op: Cpu::ora,
            addr_mode: Cpu::zp0,
            cycles: 3,
        },

        0x06 => Instruction {
            name: "asl",
            op: Cpu::asl_mem,
            addr_mode: Cpu::zp0,
            cycles: 5,
        },

        0x08 => Instruction {
            name: "php",
            op: Cpu::php,
            addr_mode: |_cpu| 0,
            cycles: 3,
        },

        0x09 => Instruction {
            name: "ora",
            op: Cpu::ora,
            addr_mode: Cpu::imm,
            cycles: 2,
        },

        0x0A => Instruction {
            name: "asl",
            op: Cpu::asl_acc,
            addr_mode: |_cpu| 0,
            cycles: 2,
        },

        0x0D => Instruction {
            name: "ora",
            op: Cpu::ora,
            addr_mode: Cpu::abs,
            cycles: 4,
        },

        0x0E => Instruction {
            name: "asl",
            op: Cpu::asl_mem,
            addr_mode: Cpu::abs,
            cycles: 6,
        },

        0x10 => Instruction {
            name: "bpl",
            op: Cpu::bpl,
            addr_mode: Cpu::rel,
            cycles: 2,
        },

        0x11 => Instruction {
            name: "ora",
            op: Cpu::ora,
            addr_mode: Cpu::indy,
            cycles: 5,
        },

        0x15 => Instruction {
            name: "ora",
            op: Cpu::ora,
            addr_mode: Cpu::zpx,
            cycles: 4,
        },

        0x16 => Instruction {
            name: "asl",
            op: Cpu::asl_mem,
            addr_mode: Cpu::zpx,
            cycles: 6,
        },

        0x18 => Instruction {
            name: "clc",
            op: Cpu::clc,
            addr_mode: |_cpu| 0,
            cycles: 2,
        },

        0x19 => Instruction {
            name: "ora",
            op: Cpu::ora,
            addr_mode: Cpu::absy,
            cycles: 4,
        },

        0x1D => Instruction {
            name: "ora",
            op: Cpu::ora,
            addr_mode: Cpu::absx,
            cycles: 4,
        },

        0x1E => Instruction {
            name: "asl",
            op: Cpu::asl_mem,
            addr_mode: Cpu::absx,
            cycles: 6,
        },

        _ => Instruction::default(),
    });

    table
}
