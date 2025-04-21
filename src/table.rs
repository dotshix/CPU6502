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

        0x20 => Instruction {
            name: "jsr",
            op: Cpu::jsr,
            addr_mode: Cpu::abs,
            cycles: 6,
        },

        0x21 => Instruction {
            name: "and",
            op: Cpu::and,
            addr_mode: Cpu::indy,
            cycles: 6,
        },

        0x24 => Instruction {
            name: "bit",
            op: Cpu::bit,
            addr_mode: Cpu::zp0,
            cycles: 3,
        },

        0x25 => Instruction {
            name: "and",
            op: Cpu::and,
            addr_mode: Cpu::zp0,
            cycles: 2,
        },

        0x26 => Instruction {
            name: "rol",
            op: Cpu::rol_mem,
            addr_mode: Cpu::zp0,
            cycles: 5,
        },

        0x28 => Instruction {
            name: "plp",
            op: Cpu::plp,
            addr_mode: |_cpu| 0,
            cycles: 4,
        },

        0x29 => Instruction {
            name: "and",
            op: Cpu::and,
            addr_mode: Cpu::imm,
            cycles: 2,
        },

        0x2A => Instruction {
            name: "rol",
            op: Cpu::rol_acc,
            addr_mode: |_cpu| 0,
            cycles: 2,
        },

        0x2C => Instruction {
            name: "bit",
            op: Cpu::bit,
            addr_mode: Cpu::abs,
            cycles: 4,
        },

        0x2D => Instruction {
            name: "and",
            op: Cpu::and,
            addr_mode: Cpu::abs,
            cycles: 4,
        },

        0x2E => Instruction {
            name: "rol",
            op: Cpu::rol_mem,
            addr_mode: Cpu::abs,
            cycles: 6,
        },

        0x30 => Instruction {
            name: "bmi",
            op: Cpu::bmi,
            addr_mode: Cpu::rel,
            cycles: 2,
        },

        0x31 => Instruction {
            name: "and",
            op: Cpu::and,
            addr_mode: Cpu::indy,
            cycles: 5,
        },

        0x35 => Instruction {
            name: "and",
            op: Cpu::and,
            addr_mode: Cpu::zpx,
            cycles: 4,
        },

        0x36 => Instruction {
            name: "rol",
            op: Cpu::rol_mem,
            addr_mode: Cpu::zpx,
            cycles: 6,
        },

        0x38 => Instruction {
            name: "sec",
            op: Cpu::sec,
            addr_mode: |_cpu| 0,
            cycles: 2,
        },

        0x39 => Instruction {
            name: "and",
            op: Cpu::and,
            addr_mode: Cpu::absy,
            cycles: 4,
        },

        0x3D => Instruction {
            name: "and",
            op: Cpu::and,
            addr_mode: Cpu::absx,
            cycles: 4,
        },

        0x3E => Instruction {
            name: "rol",
            op: Cpu::rol_mem,
            addr_mode: Cpu::absx,
            cycles: 7,
        },

        0x40 => Instruction {
            name: "rti",
            op: Cpu::rti,
            addr_mode: |_cpu| 0,
            cycles: 6,
        },

        0x41 => Instruction {
            name: "eor",
            op: Cpu::eor,
            addr_mode: Cpu::indx,
            cycles: 6,
        },

        0x45 => Instruction {
            name: "eor",
            op: Cpu::eor,
            addr_mode: Cpu::zp0,
            cycles: 3,
        },

        0x46 => Instruction {
            name: "lsr",
            op: Cpu::lsr_mem,
            addr_mode: Cpu::zp0,
            cycles: 5,
        },

        0x48 => Instruction {
            name: "pha",
            op: Cpu::pha,
            addr_mode: |_cpu| 0,
            cycles: 3,
        },

        0x49 => Instruction {
            name: "eor",
            op: Cpu::eor,
            addr_mode: |_cpu| 0,
            cycles: 2,
        },

        0x4A => Instruction {
            name: "lsr",
            op: Cpu::lsr_acc,
            addr_mode: |_cpu| 0,
            cycles: 2,
        },

        0x4C => Instruction {
            name: "jmp",
            op: Cpu::jmp,
            addr_mode: Cpu::abs,
            cycles: 3,
        },

        0x4D => Instruction {
            name: "eor",
            op: Cpu::eor,
            addr_mode: Cpu::abs,
            cycles: 4,
        },

        0x4E => Instruction {
            name: "lsr",
            op: Cpu::lsr_mem,
            addr_mode: Cpu::abs,
            cycles: 6,
        },

        0x50 => Instruction {
            name: "bvc",
            op: Cpu::bvc,
            addr_mode: Cpu::rel,
            cycles: 2,
        },

        0x51 => Instruction {
            name: "eor",
            op: Cpu::eor,
            addr_mode: Cpu::indy,
            cycles: 5,
        },

        0x55 => Instruction {
            name: "eor",
            op: Cpu::eor,
            addr_mode: Cpu::zpx,
            cycles: 4,
        },

        0x56 => Instruction {
            name: "lsr",
            op: Cpu::lsr_mem,
            addr_mode: Cpu::zpx,
            cycles: 6,
        },

        0x58 => Instruction {
            name: "cli",
            op: Cpu::cli,
            addr_mode: |_cpu| 0,
            cycles: 2,
        },

        0x59 => Instruction {
            name: "eor",
            op: Cpu::eor,
            addr_mode: Cpu::absy,
            cycles: 4,
        },

        0x5D => Instruction {
            name: "eor",
            op: Cpu::eor,
            addr_mode: Cpu::absx,
            cycles: 4,
        },

        0x5E => Instruction {
            name: "lsr",
            op: Cpu::lsr_mem,
            addr_mode: Cpu::absx,
            cycles: 7,
        },

        _ => Instruction::default(),
    });

    table
}
