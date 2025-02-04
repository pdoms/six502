use crate::{cpu::{Register, Six502, Word}, flags::{FlagIndex, Flags}, internal::{modes::AddressingMode, opcodes::OpCode, Instructions}};

#[test]
fn acc() {
    let bytes = 2+1;
    let mem = &[OpCode::LdaImm.into(), 0x03, OpCode::AslAcc.into(), OpCode::Nop.into()];
    let mut cpu = Six502::new();
    cpu.set_pc(0xFFF);
    cpu.load_to_pc(mem);
    cpu.execute();
    let pc = 0xFFF+bytes+1;
    let sp = 0;
    let a = 0x6;
    let x = 0;
    let y = 0;
    let cycles = 0;
    let flags = Flags::fix_state(&[]);
    cpu.assert_state(pc, sp, a, x, y, cycles, flags);
}

#[test] 
fn zp0() { 
    let zp_addr = 0x01; 
    let bytes = 2; 
    let mem = &[OpCode::AslZp0.into(), zp_addr, OpCode::Nop.into()]; 
    let mut cpu = Six502::new(); 
    cpu.set_pc(0xFFF); 
    cpu.load_to_pc(mem); 
    cpu.set_byte_at(zp_addr as Word, 0xE); 
    cpu.execute(); 
    let pc = 0xFFF+bytes+1;
    let sp = 0;
    let a = 0;
    let x = 0;
    let y = 0;
    let cycles = 0;
    let flags = Flags::fix_state(&[]);
    cpu.assert_state(pc, sp, a, x, y, cycles, flags);
    assert_eq!(cpu.byte_at(0x01), 0x1C);
}


#[test]
fn zpx() {
    let bytes_per_instr = 2;
    let mem = &[OpCode::AslZpx.into(), 0x80, OpCode::Nop.into()];
    let mut cpu = Six502::new();
    cpu.set_pc(0xFFF);
    cpu.load_to_pc(mem);
    cpu.set_reg_byte(Register::X, 0x0F);
    cpu.set_byte_at(0x8F, 0x05);
    cpu.execute();
    let pc = 0xFFF+bytes_per_instr+1;
    let sp = 0;
    let a = 0x0;
    let x = 0x0F;
    let y = 0;
    let cycles = 0;
    let flags = Flags::fix_state(&[]);
    cpu.assert_state(pc, sp, a, x, y, cycles, flags);
    assert_eq!(cpu.byte_at(0x8F), 0xA);
}


#[test]
fn abs() {
    let bytes = 3;
    let start = 0xFFF;

    let mem = &[OpCode::AslAbs.into(), 0x11, 0x11, OpCode::Nop.into()];

    let mut cpu = Six502::new();
    cpu.set_pc(start);
    cpu.load_to_pc(mem);
    cpu.set_byte_at(0x1111, 0x05);

    cpu.execute();
    let pc = 0xFFF+bytes+1;
    let sp = 0;
    let a = 0;
    let x = 0;
    let y = 0;
    let cycles = 0;
    let flags = Flags::fix_state(&[]);
    cpu.assert_state(pc, sp, a, x, y, cycles, flags);
    assert_eq!(cpu.byte_at(0x1111), 0xA);
}

#[test]
fn abx() {

    let bytes = 3;
    let start = 0xFFF;

    let mem = &[OpCode::AslAbx.into(), 0x00, 0x30, OpCode::Nop.into()];

    let mut cpu = Six502::new();
    cpu.set_pc(start);
    cpu.load_to_pc(mem);
    cpu.set_reg_byte(crate::cpu::Register::X, 0x92);
    cpu.set_byte_at(0x3092, 0xE);
    cpu.execute();
    let pc = 0xFFF+bytes+1;
    let sp = 0;
    let a = 0;
    let x = 0x92;
    let y = 0;
    let cycles = 0;
    let flags = Flags::fix_state(&[]);
    cpu.assert_state(pc, sp, a, x, y, cycles, flags);
    assert_eq!(cpu.byte_at(0x3092), 0x1C);
}


#[test]
fn instr_correct_spots() {
    let instructions = Instructions::init();
    assert_eq!(instructions[0x0A].mnemonic, "ASL"); 
    assert_eq!(instructions[0x0A].mode, AddressingMode::ACC);
    assert_eq!(instructions[0x06].mnemonic, "ASL"); 
    assert_eq!(instructions[0x06].mode, AddressingMode::ZP0);
    assert_eq!(instructions[0x16].mnemonic, "ASL"); 
    assert_eq!(instructions[0x16].mode, AddressingMode::ZPX);
    assert_eq!(instructions[0x0E].mnemonic, "ASL"); 
    assert_eq!(instructions[0x0E].mode, AddressingMode::ABS);
    assert_eq!(instructions[0x1E].mnemonic, "ASL"); 
    assert_eq!(instructions[0x1E].mode, AddressingMode::ABX);
}
