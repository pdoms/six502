use crate::{cpu::{Six502, Word, SP_INIT}, flags::{set_flag, Flag::{N, V, Z}, DEFAULT_STATUS}, internal::{modes::AddressingMode, opcodes::OpCode, Instructions}, mem::Mem};

use super::prelude;

const PC_START: Word = 0xFFF;

#[test]
fn zp0() {
    let bytes = 2+2;
    let mem = &[OpCode::LdaImm.into(), 0x55, OpCode::BitZp0.into(), 0x01, OpCode::Nop.into()];
    let mut cpu = prelude(PC_START, mem);
    cpu.set_byte_at(0x01, 0xEE);
    cpu.execute();
    let pc = 0xFFF+bytes+1;
    let sp = SP_INIT;
    let a = 0x55;
    let x = 0;
    let y = 0;
    let cycles = 0;
    let mut status = DEFAULT_STATUS;
    set_flag(&mut status, Z, 1);
    set_flag(&mut status, N, 1);
    set_flag(&mut status, V, 1);
    cpu.assert_state(pc, sp, a, x, y, cycles, status);
}

#[test]
fn abs() {
    let bytes = 3+2;
    let mem = &[OpCode::LdaImm.into(), 0x01, OpCode::BitAbs.into(), 0x11, 0x12, OpCode::Nop.into()];
    let mut cpu = prelude(PC_START, mem);
    cpu.set_byte_at(0x1211, 0x05);
    cpu.execute();
    let pc = 0xFFF+bytes+1;
    let sp = SP_INIT;
    let a = 0x01;
    let x = 0;
    let y = 0;
    let cycles = 0;
    let mut status = DEFAULT_STATUS;
    set_flag(&mut status, Z, 1);
    cpu.assert_state(pc, sp, a, x, y, cycles, status);
}


#[test]
fn instr_correct_spots() {
    let instructions: Instructions<Mem> = Instructions::init();
    assert_eq!(instructions[0x24].mnemonic, "BIT"); 
    assert_eq!(instructions[0x24].mode, AddressingMode::ZP0);
    assert_eq!(instructions[0x2c].mnemonic, "BIT"); 
    assert_eq!(instructions[0x2c].mode, AddressingMode::ABS);
}
