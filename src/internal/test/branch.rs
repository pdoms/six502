use crate::{cpu::{Six502, Word, SP_INIT}, flags::{set_flag, Flag, DEFAULT_STATUS}, internal::{modes::AddressingMode, opcodes::OpCode, Instructions}, mem::Mem};

use super::prelude;

const PC_START: Word = 0xFFF;

#[test]
fn bcc_branch_forward() {
    let label = 0x16;  // 18
    let mem = &[
        OpCode::BccRel.into(), 
        label, 
        OpCode::Nop.into()
    ];
    let mut cpu = prelude(PC_START, mem);
    cpu.set_flag(Flag::C, 0);
    cpu.execute();
    let pc = 0x1015 + 2;
    let sp = SP_INIT;
    let a = 0;
    let x = 0;
    let y = 0;
    let cycles = 0;
    cpu.assert_state(pc, sp, a, x, y, cycles, DEFAULT_STATUS);
}

#[test]
fn bcc_branch_backward() {
    let label = 0xEE;  // -18
    let mem = &[
        OpCode::BccRel.into(), 
        label, 
        OpCode::Nop.into()
    ];
    let mut cpu = prelude(PC_START, mem);
    cpu.set_flag(Flag::C, 0);
    cpu.execute();
    let pc = 0xFFF + 2 - 18;
    let sp = SP_INIT;
    let a = 0;
    let a = 0;
    let x = 0;
    let y = 0;
    let cycles = 0;
    cpu.assert_state(pc, sp, a, x, y, cycles, DEFAULT_STATUS);
}
#[test]
fn bcc_no_branch() {
    let label = 0x16;  // 18
    let mem = &[
        OpCode::BccRel.into(), 
        label, 
        OpCode::Nop.into()
    ];
    let mut cpu = prelude(PC_START, mem);
    cpu.set_flag(Flag::C, 1);
    cpu.execute();
    let pc = 0xFFF+2;
    let sp = SP_INIT;
    let a = 0;
    let x = 0;
    let y = 0;
    let cycles = 0;
    let mut status = DEFAULT_STATUS;
    set_flag(&mut status, Flag::C, 1);
    cpu.assert_state(pc, sp, a, x, y, cycles, status);
}


#[test]
fn bcs_branch() {
    let label = 0x16;  // 18
    let mem = &[
        OpCode::BcsRel.into(), 
        label, 
        OpCode::Nop.into()
    ];
    let mut cpu = prelude(PC_START, mem);
    cpu.set_flag(Flag::C, 1);

    cpu.execute();
    let pc = 0x1015 + 2;
    let sp = SP_INIT;
    let a = 0;
    let x = 0;
    let y = 0;
    let cycles = 0;
    let mut status = DEFAULT_STATUS;
    set_flag(&mut status, Flag::C, 1);
    cpu.assert_state(pc, sp, a, x, y, cycles, status);
}

#[test]
fn bcs_no_branch() {
    let label = 0x16;  // 18
    let mem = &[
        OpCode::BcsRel.into(), 
        label, 
        OpCode::Nop.into()
    ];
    let mut cpu = prelude(PC_START, mem);
    cpu.set_flag(Flag::C, 0);
    cpu.execute();
    let pc = 0xFFF+2;
    let sp = SP_INIT;
    let a = 0;
    let x = 0;
    let y = 0;
    let cycles = 0;
    cpu.assert_state(pc, sp, a, x, y, cycles, DEFAULT_STATUS);
}

#[test]
fn instr_correct_spots() {
    let instructions: Instructions<Mem> = Instructions::init();
    assert_eq!(instructions[0x90].mnemonic, "BCC"); 
    assert_eq!(instructions[0x90].mode, AddressingMode::REL);
    assert_eq!(instructions[0xB0].mnemonic, "BCS"); 
    assert_eq!(instructions[0xB0].mode, AddressingMode::REL);
}
