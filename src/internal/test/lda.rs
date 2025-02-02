use crate::{
    cpu::{Register, Six502, Word}, 
    flags::{FlagIndex, Flags}, 
    internal::{modes::AddressingMode, opcodes::OpCode, Instructions}};


#[test]
fn imm() {
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
fn zp0() {
    let zp_addr = 0x01;
    let mem = &[OpCode::LdaZp0 as u8, zp_addr, OpCode::Nop as u8];
    let mut cpu = Six502::new();
    cpu.load_to_pc(mem);
    cpu.set_byte_at(zp_addr as Word, 0xF1);
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
fn zpx() {
    let bytes_per_instr = 2;
    let mem = &[OpCode::LdaZpx as u8, 0x80, OpCode::Nop as u8];
    let mut cpu = Six502::new();
    cpu.set_pc(0xFFF);
    cpu.load_to_pc(mem);
    cpu.set_reg_byte(Register::X, 0x0F);
    cpu.set_byte_at(0x8F, 0x22);
    cpu.execute();
    let pc = 0xFFF+bytes_per_instr+1;
    let sp = 0;
    let a = 34;
    let x = 0x0F;
    let y = 0;
    let cycles = 0;
    let flags = Flags::fix_state(&[]);
    cpu.assert_state(pc, sp, a, x, y, cycles, flags);

    let mem2 = &[OpCode::LdaZpx as u8, 0x80, OpCode::Nop as u8];
    let mut cpu2 = Six502::new();
    cpu2.set_pc(0xFFF);
    cpu2.load_to_pc(mem2);
    cpu2.set_reg_byte(Register::X, 0xFF);
    cpu2.set_byte_at(0x7F, 0x22);
    cpu2.execute();
    let pc = 0xFFF+bytes_per_instr+1;
    let sp = 0;
    let a = 34;
    let x = 0xFF;
    let y = 0;
    let cycles = 0;
    let flags = Flags::fix_state(&[]);
    cpu2.assert_state(pc, sp, a, x, y, cycles, flags);
}

#[test]
fn abs() {
    let bytes_per_instr = 3;
    let mem = &[OpCode::LdaAbs as u8, 0x11, 0xAA, OpCode::Nop as u8];
    let mut cpu = Six502::new();
    cpu.set_pc(0xFFF);
    cpu.load_to_pc(mem);
    cpu.set_byte_at(0xAA11, 0x01);
    cpu.execute();
    let pc = 0xFFF+bytes_per_instr+1;
    let sp = 0;
    let a = 1;
    let x = 0;
    let y = 0;
    let cycles = 0;
    let flags = Flags::fix_state(&[]);
    cpu.assert_state(pc, sp, a, x, y, cycles, flags);
}

#[test]
fn abx() {
    let bytes_per_instr = 3;
    let mem = &[OpCode::LdaAbx as u8, 0x00, 0x20, OpCode::Nop as u8];
    let mut cpu = Six502::new();
    cpu.set_pc(0xFFF);
    cpu.load_to_pc(mem);
    cpu.set_reg_byte(Register::X, 0x92);
    cpu.set_byte_at(0x2092, 0x22);
    cpu.execute();
    let pc = 0xFFF+bytes_per_instr+1;
    let sp = 0;
    let a = 34;
    let x = 0x92;
    let y = 0;
    let cycles = 0;
    let flags = Flags::fix_state(&[]);
    cpu.assert_state(pc, sp, a, x, y, cycles, flags);
}

#[test]
fn aby() {
    let bytes_per_instr = 3;
    let mem = &[OpCode::LdaAby as u8, 0x00, 0x20, OpCode::Nop as u8];
    let mut cpu = Six502::new();
    cpu.set_pc(0xFFF);
    cpu.load_to_pc(mem);
    cpu.set_reg_byte(Register::Y, 0x92);
    cpu.set_byte_at(0x2092, 0xF1);
    cpu.execute();
    let pc = 0xFFF+bytes_per_instr+1;
    let sp = 0;
    let a = 241;
    let x = 0;
    let y = 0x92;
    let cycles = 0;
    let flags = Flags::fix_state(&[FlagIndex::N]);
    cpu.assert_state(pc, sp, a, x, y, cycles, flags);
}

#[test]
fn izx() {
    let bytes_per_instr = 2;
    let mem = &[OpCode::LdaIzx as u8, 0x40, OpCode::Nop as u8];
    let mut cpu = Six502::new();
    cpu.set_pc(0xFFF);
    cpu.load_to_pc(mem);
    cpu.set_reg_byte(Register::X, 0x04);
    cpu.set_byte_at(0x44, 0x01);
    cpu.set_byte_at(0x45, 0x02);
    cpu.set_byte_at(0x0201, 0x22);
    cpu.execute();
    let pc = 0xFFF+bytes_per_instr+1;
    let sp = 0;
    let a = 34;
    let x = 0x04;
    let y = 0;
    let cycles = 0;
    let flags = Flags::fix_state(&[]);
    cpu.assert_state(pc, sp, a, x, y, cycles, flags);
}

#[test]
fn izy() {
    let bytes_per_instr = 2;
    let mem = &[OpCode::LdaIzy as u8, 0x40, OpCode::Nop as u8];
    let mut cpu = Six502::new();
    cpu.set_pc(0xFFF);
    cpu.load_to_pc(mem);
    cpu.set_reg_byte(Register::Y, 0x04);
    cpu.set_byte_at(0x40, 0x01);
    cpu.set_byte_at(0x41, 0x02);
    cpu.set_byte_at(0x0205, 0x22);
    cpu.execute();
    let pc = 0xFFF+bytes_per_instr+1;
    let sp = 0;
    let a = 34;
    let x = 0;
    let y = 0x04;
    let cycles = 0;
    let flags = Flags::fix_state(&[]);
    cpu.assert_state(pc, sp, a, x, y, cycles, flags);
}

#[test]
fn instr_correct_spots() {
    let instructions = Instructions::init();
    assert_eq!(instructions[0xA5].mnemonic, "LDA");
    assert_eq!(instructions[0xA5].mode, AddressingMode::ZP0);
    assert_eq!(instructions[0xA9].mnemonic, "LDA");
    assert_eq!(instructions[0xA9].mode, AddressingMode::IMM);
    assert_eq!(instructions[0xAD].mnemonic, "LDA");
    assert_eq!(instructions[0xAD].mode, AddressingMode::ABS);
    assert_eq!(instructions[0xB5].mnemonic, "LDA");
    assert_eq!(instructions[0xB5].mode, AddressingMode::ZPX);
    assert_eq!(instructions[0xB9].mnemonic, "LDA");
    assert_eq!(instructions[0xB9].mode, AddressingMode::ABY);
    assert_eq!(instructions[0xB9].cycles, 5);
    assert_eq!(instructions[0xBD].mnemonic, "LDA");
    assert_eq!(instructions[0xBD].mode, AddressingMode::ABX);
}
