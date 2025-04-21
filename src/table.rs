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

        0x60 => Instruction {
            name: "rts",
            op: Cpu::rts,
            addr_mode: |_cpu| 0,
            cycles: 6,
        },

        0x61 => Instruction {
            name: "adc",
            op: Cpu::adc,
            addr_mode: Cpu::indx,
            cycles: 6,
        },

        0x65 => Instruction {
            name: "adc",
            op: Cpu::adc,
            addr_mode: Cpu::zpx,
            cycles: 3,
        },

        0x66 => Instruction {
            name: "ror",
            op: Cpu::ror_mem,
            addr_mode: Cpu::zpx,
            cycles: 5,
        },

        0x68 => Instruction {
            name: "pla",
            op: Cpu::pla,
            addr_mode: |_cpu| 0,
            cycles: 4,
        },

        0x69 => Instruction {
            name: "adc",
            op: Cpu::adc,
            addr_mode: Cpu::imm,
            cycles: 2,
        },

        0x6A => Instruction {
            name: "ror",
            op: Cpu::ror_acc,
            addr_mode: |_cpu| 0,
            cycles: 2,
        },

        0x6C => Instruction {
            name: "jmp",
            op: Cpu::jmp,
            addr_mode: Cpu::ind,
            cycles: 5,
        },

        0x6D => Instruction {
            name: "adc",
            op: Cpu::adc,
            addr_mode: Cpu::abs,
            cycles: 4,
        },

        0x6E => Instruction {
            name: "ror",
            op: Cpu::ror_mem,
            addr_mode: Cpu::abs,
            cycles: 6,
        },

        0x70 => Instruction {
            name: "bvs",
            op: Cpu::bvs,
            addr_mode: Cpu::rel,
            cycles: 2,
        },

        0x71 => Instruction {
            name: "adc",
            op: Cpu::adc,
            addr_mode: Cpu::indy,
            cycles: 5,
        },

        0x75 => Instruction {
            name: "adc",
            op: Cpu::adc,
            addr_mode: Cpu::zpx,
            cycles: 4,
        },

        0x76 => Instruction {
            name: "ror",
            op: Cpu::ror_mem,
            addr_mode: Cpu::zpx,
            cycles: 6,
        },

        0x78 => Instruction {
            name: "sei",
            op: Cpu::sei,
            addr_mode: |_cpu| 0,
            cycles: 2,
        },

        0x79 => Instruction {
            name: "adc",
            op: Cpu::adc,
            addr_mode: Cpu::absy,
            cycles: 4,
        },

        0x7D => Instruction {
            name: "adc",
            op: Cpu::adc,
            addr_mode: Cpu::absx,
            cycles: 4,
        },

        0x7E => Instruction {
            name: "ror",
            op: Cpu::ror_mem,
            addr_mode: Cpu::absx,
            cycles: 7,
        },

        0x81 => Instruction {
            name: "sta",
            op: Cpu::sta,
            addr_mode: Cpu::indx,
            cycles: 6,
        },

        0x84 => Instruction {
            name: "sty",
            op: Cpu::sty,
            addr_mode: Cpu::zp0,
            cycles: 3,
        },

        0x85 => Instruction {
            name: "sta",
            op: Cpu::sta,
            addr_mode: Cpu::zp0,
            cycles: 3,
        },

        0x86 => Instruction {
            name: "stx",
            op: Cpu::stx,
            addr_mode: Cpu::zp0,
            cycles: 3,
        },

        0x88 => Instruction {
            name: "dey",
            op: Cpu::dey,
            addr_mode: |_cpu| 0,
            cycles: 2,
        },

        0x8A => Instruction {
            name: "txa",
            op: Cpu::txa,
            addr_mode: |_cpu| 0,
            cycles: 2,
        },

        0x8C => Instruction {
            name: "sty",
            op: Cpu::sty,
            addr_mode: Cpu::abx,
            cycles: 4,
        },

        0x8D => Instruction {
            name: "sta",
            op: Cpu::sta,
            addr_mode: Cpu::abs,
            cycles: 4,
        },

        0x8E => Instruction {
            name: "stx",
            op: Cpu::stx,
            addr_mode: Cpu::abs,
            cycles: 4,
        },

        0x90 => Instruction {
            name: "bcc",
            op: Cpu::bcc,
            addr_mode: Cpu::rel,
            cycles: 2,
        },

        0x91 => Instruction {
            name: "sta",
            op: Cpu::sta,
            addr_mode: Cpu::indy,
            cycles: 6,
        },

        0x94 => Instruction {
            name: "sty",
            op: Cpu::sty,
            addr_mode: Cpu::zpx,
            cycles: 4,
        },

        0x95 => Instruction {
            name: "sta",
            op: Cpu::sta,
            addr_mode: Cpu::zpx,
            cycles: 4,
        },

        0x96 => Instruction {
            name: "stx",
            op: Cpu::stx,
            addr_mode: Cpu::zpy,
            cycles: 4,
        },

        0x98 => Instruction {
            name: "tya",
            op: Cpu::tya,
            addr_mode: |_cpu| 0,
            cycles: 2,
        },

        0x99 => Instruction {
            name: "sta",
            op: Cpu::sta,
            addr_mode: Cpu::absy,
            cycles: 5,
        },

        0x9A => Instruction {
            name: "txs",
            op: Cpu::txs,
            addr_mode: |_cpu| 0,
            cycles: 2,
        },

        0x9D => Instruction {
            name: "sta",
            op: Cpu::sta,
            addr_mode: Cpu::absx,
            cycles: 5,
        },

        0xA0 => Instruction {
            name: "ldy",
            op: Cpu::ldy,
            addr_mode: Cpu::imm,
            cycles: 2,
        },

        0xA1 => Instruction {
            name: "lda",
            op: Cpu::lda,
            addr_mode: Cpu::indx,
            cycles: 6,
        },

        0xA2 => Instruction {
            name: "ldx",
            op: Cpu::ldx,
            addr_mode: Cpu::imm,
            cycles: 2,
        },

        0xA4 => Instruction {
            name: "ldy",
            op: Cpu::ldy,
            addr_mode: Cpu::zp0,
            cycles: 3,
        },

        0xA5 => Instruction {
            name: "lda",
            op: Cpu::lda,
            addr_mode: Cpu::zp0,
            cycles: 3,
        },

        0xA6 => Instruction {
            name: "ldx",
            op: Cpu::ldx,
            addr_mode: Cpu::zp0,
            cycles: 3,
        },

        0xA8 => Instruction {
            name: "tay",
            op: Cpu::tay,
            addr_mode: |_cpu| 0,
            cycles: 2,
        },

        0xA9 => Instruction {
            name: "lda",
            op: Cpu::lda,
            addr_mode: Cpu::imm,
            cycles: 2,
        },

        0xAA => Instruction {
            name: "tax",
            op: Cpu::tax,
            addr_mode: |_cpu| 0,
            cycles: 2,
        },

        0xAC => Instruction {
            name: "ldy",
            op: Cpu::ldy,
            addr_mode: Cpu::abs,
            cycles: 4,
        },

        0xAD => Instruction {
            name: "lda",
            op: Cpu::lda,
            addr_mode: Cpu::abs,
            cycles: 4,
        },

        0xAE => Instruction {
            name: "ldx",
            op: Cpu::ldx,
            addr_mode: Cpu::abs,
            cycles: 4,
        },

        0xB0 => Instruction {
            name: "bcs",
            op: Cpu::bcs,
            addr_mode: Cpu::rel,
            cycles: 2,
        },

        0xB1 => Instruction {
            name: "lda",
            op: Cpu::lda,
            addr_mode: Cpu::indy,
            cycles: 5,
        },

        0xB4 => Instruction {
            name: "ldy",
            op: Cpu::ldy,
            addr_mode: Cpu::zpx,
            cycles: 4,
        },

        0xB5 => Instruction {
            name: "lda",
            op: Cpu::lda,
            addr_mode: Cpu::zpx,
            cycles: 4,
        },

        0xB6 => Instruction {
            name: "ldx",
            op: Cpu::ldx,
            addr_mode: Cpu::zpy,
            cycles: 4,
        },

        0xB8 => Instruction {
            name: "clv",
            op: Cpu::clv,
            addr_mode: |_cpu| 0,
            cycles: 2,
        },

        0xB9 => Instruction {
            name: "lda",
            op: Cpu::lda,
            addr_mode: Cpu::absy,
            cycles: 4,
        },

        0xBA => Instruction {
            name: "tsx",
            op: Cpu::tsx,
            addr_mode: |_cpu| 0,
            cycles: 2,
        },

        0xBC => Instruction {
            name: "ldy",
            op: Cpu::ldy,
            addr_mode: Cpu::absx,
            cycles: 4,
        },

        0xBD => Instruction {
            name: "lda",
            op: Cpu::lda,
            addr_mode: Cpu::absx,
            cycles: 4,
        },

        0xBE => Instruction {
            name: "ldx",
            op: Cpu::ldx,
            addr_mode: Cpu::absy,
            cycles: 4,
        },
        _ => Instruction::default(),
    });

    table
}
