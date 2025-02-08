use crate::{
    cpu::{Register, Six502, Word, SP_INIT}, 
    flags::{set_flag, Flag, DEFAULT_STATUS}, 
    internal::{modes::AddressingMode, opcodes::OpCode, Instructions}};

#[test]
fn imm() {
    let bytes = 2+2;
    let mem = &[OpCode::LdaImm.into(), 0x03, OpCode::AndImm.into(), 0x05, OpCode::Nop.into()];
    let mut cpu = Six502::new();
    cpu.set_pc(0xFFF);
    cpu.load_to_pc(mem);
    cpu.execute();
    let pc = 0xFFF+bytes+1;
    let sp = SP_INIT;
    let a = 0x01;
    let x = 0;
    let y = 0;
    let cycles = 0;
    cpu.assert_state(pc, sp, a, x, y, cycles, DEFAULT_STATUS);
}

#[test] 
fn zp0() { 
    let zp_addr = 0x01; 
    let bytes = 2+2; 
    let mem = &[OpCode::LdaImm.into(), 0xEE, OpCode::AndZp0.into(), zp_addr, OpCode::Nop.into()]; 
    let mut cpu = Six502::new(); 
    cpu.set_pc(0xFFF); 
    cpu.load_to_pc(mem); 
    cpu.set_byte_at(zp_addr as Word, 0xDD); 
    cpu.execute(); 
    let pc = 0xFFF+bytes+1;
    let sp = SP_INIT;
    let a = 0xCC;
    let x = 0;
    let y = 0;
    let cycles = 0;
    let mut status = DEFAULT_STATUS;
    set_flag(&mut status, Flag::N, 1);
    cpu.assert_state(pc, sp, a, x, y, cycles, status);
}


#[test]
fn zpx() {
    let bytes_per_instr = 2+2;
    let mem = &[OpCode::LdaImm.into(), 0x03, OpCode::AndZpx.into(), 0x80, OpCode::Nop.into()];
    let mut cpu = Six502::new();
    cpu.set_pc(0xFFF);
    cpu.load_to_pc(mem);
    cpu.set_reg_byte(Register::X, 0x0F);
    cpu.set_byte_at(0x8F, 0x05);
    cpu.execute();
    let pc = 0xFFF+bytes_per_instr+1;
    let sp = SP_INIT;
    let a = 0x01;
    let x = 0x0F;
    let y = 0;
    let cycles = 0;
    cpu.assert_state(pc, sp, a, x, y, cycles, DEFAULT_STATUS);
}


#[test]
fn abs() {

    let bytes = 2+3;
    let start = 0xFFF;

    let mem = &[OpCode::LdaImm.into(), 0x03, OpCode::AndAbs.into(), 0x11, 0x11, OpCode::Nop.into()];

    let mut cpu = Six502::new();
    cpu.set_pc(start);
    cpu.load_to_pc(mem);
    cpu.set_byte_at(0x1111, 0x05);

    cpu.execute();
    let pc = 0xFFF+bytes+1;
    let sp = SP_INIT;
    let a = 0x1;
    let x = 0;
    let y = 0;
    let cycles = 0;
    cpu.assert_state(pc, sp, a, x, y, cycles, DEFAULT_STATUS);
}

#[test]
fn abx() {

    let bytes = 2+3;
    let start = 0xFFF;

    let mem = &[OpCode::LdaImm.into(), 0xDD, OpCode::AndAbx.into(), 0x00, 0x30, OpCode::Nop.into()];

    let mut cpu = Six502::new();
    cpu.set_pc(start);
    cpu.load_to_pc(mem);
    cpu.set_reg_byte(crate::cpu::Register::X, 0x92);
    cpu.set_byte_at(0x3092, 0xEE);
    cpu.execute();
    let pc = 0xFFF+bytes+1;
    let sp = SP_INIT;
    let a = 0xCC;
    let x = 0x92;
    let y = 0;
    let cycles = 0;
    let mut status = DEFAULT_STATUS;
    set_flag(&mut status, Flag::N, 1);
    cpu.assert_state(pc, sp, a, x, y, cycles, status);

}

#[test]
fn aby() {

    let bytes = 2+3;
    let start = 0xFFF;
    let mem = &[OpCode::LdaImm.into(), 0x03, OpCode::AndAby.into(), 0x00, 0x30, OpCode::Nop.into()];

    let mut cpu = Six502::new();
    cpu.set_pc(start);
    cpu.load_to_pc(mem);
    cpu.set_reg_byte(crate::cpu::Register::Y, 0x92);
    cpu.set_byte_at(0x3092, 0x05);
    cpu.execute();
    let pc = 0xFFF+bytes+1;
    let sp = SP_INIT;
    let a = 0x01;
    let x = 0;
    let y = 0x92;
    let cycles = 0;
    cpu.assert_state(pc, sp, a, x, y, cycles, DEFAULT_STATUS);

}

#[test]
fn izx() {

    let bytes = 2+2;
    let start = 0xFFF;

    let mem = &[OpCode::LdaImm.into(), 0x03, OpCode::AndIzx.into(), 0x40, OpCode::Nop.into()];

    let mut cpu = Six502::new();
    cpu.set_pc(start);
    cpu.load_to_pc(mem);
    cpu.set_reg_byte(crate::cpu::Register::X, 0x04);
    cpu.set_byte_at(0x44, 0x01);
    cpu.set_byte_at(0x45, 0x02);
    cpu.set_byte_at(0x0201, 0x05);

    cpu.execute();
    let pc = 0xFFF+bytes+1;
    let sp = SP_INIT;
    let a = 0x01;
    let x = 0x04;
    let y = 0;
    let cycles = 0;
    cpu.assert_state(pc, sp, a, x, y, cycles, DEFAULT_STATUS);

}
#[test]
fn izy() {
    let bytes = 2+2;
    let start = 0xFFF;

    let mem = &[OpCode::LdaImm.into(), 0x03, OpCode::AndIzy.into(), 0x40, OpCode::Nop.into()];

    let mut cpu = Six502::new();
    cpu.set_pc(start);
    cpu.load_to_pc(mem);
    cpu.set_reg_byte(crate::cpu::Register::Y, 0x04);
    cpu.set_byte_at(0x40, 0x01);
    cpu.set_byte_at(0x41, 0x02);
    cpu.set_byte_at(0x0205, 0x05);

    cpu.execute();
    let pc = 0xFFF+bytes+1;
    let sp = SP_INIT;
    let a = 0x01;
    let x = 0;
    let y = 0x04;
    let cycles = 0;
    cpu.assert_state(pc, sp, a, x, y, cycles, DEFAULT_STATUS);
}



#[test]
fn instr_correct_spots() {
    let instructions = Instructions::init();
    assert_eq!(instructions[0x21].mnemonic, "AND"); 
    assert_eq!(instructions[0x21].mode, AddressingMode::IZX);
    assert_eq!(instructions[0x25].mnemonic, "AND"); 
    assert_eq!(instructions[0x25].mode, AddressingMode::ZP0);
    assert_eq!(instructions[0x29].mnemonic, "AND"); 
    assert_eq!(instructions[0x29].mode, AddressingMode::IMM);
    assert_eq!(instructions[0x2D].mnemonic, "AND"); 
    assert_eq!(instructions[0x2D].mode, AddressingMode::ABS);
    assert_eq!(instructions[0x31].mnemonic, "AND"); 
    assert_eq!(instructions[0x31].mode, AddressingMode::IZY);
    assert_eq!(instructions[0x35].mnemonic, "AND"); 
    assert_eq!(instructions[0x35].mode, AddressingMode::ZPX);
    assert_eq!(instructions[0x39].mnemonic, "AND"); 
    assert_eq!(instructions[0x39].mode, AddressingMode::ABY);
    assert_eq!(instructions[0x3D].mnemonic, "AND"); 
    assert_eq!(instructions[0x3D].mode, AddressingMode::ABX);
}
