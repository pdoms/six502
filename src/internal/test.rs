use crate::{cpu::Six502, flags::{FlagIndex, Flags}, internal::{executors::nop, modes::AddressingMode, Instructions}};

use super::opcodes::OpCode;




#[test]
fn lda_imm_no_flags() {

    //TODO compress to one test for all flag outcomes!!!
    let mem = &[OpCode::LdaImm as u8, 0x22, OpCode::Nop as u8];
    let mut cpu = Six502::new();
    cpu.load_to_pc(mem);
    cpu.execute();
    let pc = 65535;
    let sp = 0;
    let a = 34;
    let x = 0;
    let y = 0;
    let cycles = 0;
    let flags = Flags::fix_state(&[]);
    cpu.assert_state(pc, sp, a, x, y, cycles, flags);
}

#[test]
fn lda_imm_zero_flag() {
    let mem = &[OpCode::LdaImm as u8, 0x0, OpCode::Nop as u8];
    let mut cpu = Six502::new();
    cpu.load_to_pc(mem);
    cpu.execute();
    let pc = 65535;
    let sp = 0;
    let a = 0;
    let x = 0;
    let y = 0;
    let cycles = 0;
    let flags = Flags::fix_state(&[FlagIndex::Z]);
    cpu.assert_state(pc, sp, a, x, y, cycles, flags);
}
#[test]
fn lda_imm_negative_flag() {
    let mem = &[OpCode::LdaImm as u8, 0xF1, OpCode::Nop as u8];
    let mut cpu = Six502::new();
    cpu.load_to_pc(mem);
    cpu.execute();
    let pc = 65535;
    let sp = 0;
    let a = 241;
    let x = 0;
    let y = 0;
    let cycles = 0;
    let flags = Flags::fix_state(&[FlagIndex::N]);
    cpu.assert_state(pc, sp, a, x, y, cycles, flags);
}
#[test]
fn instr_correct_spots() {
    let instructions = Instructions::init();
    assert_eq!(instructions[0xA5].mnemonic, "LDA");
    assert_eq!(instructions[0xA5].mode, AddressingMode::ZP0);
    assert_eq!(instructions[0xA9].mnemonic, "LDA");
    assert_eq!(instructions[0xA9].mode, AddressingMode::IMM);



}
