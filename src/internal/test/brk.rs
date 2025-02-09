use crate::{cpu::{Six502, SP_INIT}, flags::DEFAULT_STATUS, internal::{modes::AddressingMode, opcodes::OpCode, Instructions}, mem::Mem};

use super::prelude;

#[test]
fn imp() {
    let mem = &[OpCode::BrkImp.into(), 0x01];
    let mut cpu = prelude(0x1111, mem);
    cpu.set_pc(0x1111);
    cpu.set_byte_at(0xFFFE, 0x11);
    cpu.set_byte_at(0xFFFF, 0x12);
    cpu.set_byte_at(0x1211, OpCode::Nop.into());
    cpu.execute();
    let pc = 0x1211+1;
    let sp = SP_INIT - 3;
    let a = 0x0;
    let x = 0;
    let y = 0;
    let cycles = 0;
    let status = DEFAULT_STATUS;
    cpu.assert_state(pc, sp, a, x, y, cycles, status);
}

#[test]
fn instr_correct_spots() {
    let instructions: Instructions<Mem> = Instructions::init();
    assert_eq!(instructions[0x00].mnemonic, "BRK"); 
    assert_eq!(instructions[0x00].mode, AddressingMode::IMP);
}

