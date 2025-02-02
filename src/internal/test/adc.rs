use crate::{cpu::{Six502, Word}, flags::{FlagIndex, Flags}, internal::{modes::AddressingMode, opcodes::OpCode, Instructions}};

#[test]
fn imm() {
    let bytes = 2+2;
    let mem = &[OpCode::LdaImm.into(), 0x02, OpCode::AdcImm.into(), 0x02, OpCode::Nop.into()];
    let mut cpu = Six502::new();
    cpu.set_pc(0xFFF);
    cpu.load_to_pc(mem);
    cpu.execute();
    let pc = 0xFFF+bytes+1;
    let sp = 0;
    let a = 0x04;
    let x = 0;
    let y = 0;
    let cycles = 0;
    let flags = Flags::fix_state(&[]);
    cpu.assert_state(pc, sp, a, x, y, cycles, flags);

    //ending with carry bit set
    let mem = &[OpCode::LdaImm.into(), 0x02, OpCode::AdcImm.into(), 0xFF, OpCode::Nop.into()];
    let mut cpu = Six502::new();
    cpu.set_pc(0xFFF);
    cpu.load_to_pc(mem);
    cpu.execute();
    let pc = 0xFFF+bytes+1;
    let sp = 0;
    let a = 0x01;
    let x = 0;
    let y = 0;
    let cycles = 0;
    let flags = Flags::fix_state(&[FlagIndex::C]);
    cpu.assert_state(pc, sp, a, x, y, cycles, flags);

    let mem = &[OpCode::LdaImm.into(), 0x40, OpCode::AdcImm.into(), 0x40, OpCode::Nop.into()];
    let mut cpu = Six502::new();
    cpu.set_pc(0xFFF);
    cpu.load_to_pc(mem);
    cpu.execute();
    let pc = 0xFFF+bytes+1;
    let sp = 0;
    let a = 0x80;
    let x = 0;
    let y = 0;
    let cycles = 0;
    let flags = Flags::fix_state(&[FlagIndex::N, FlagIndex::O]);
    cpu.assert_state(pc, sp, a, x, y, cycles, flags);
}

#[test]
fn zp0() {
    let zp_addr = 0x01;
    let bytes = 2+2;
    let mem = &[OpCode::LdaImm.into(), 0x02, OpCode::AdcZp0.into(), zp_addr, OpCode::Nop.into()];
    let mut cpu = Six502::new();
    cpu.set_pc(0xFFF);
    cpu.load_to_pc(mem);
    cpu.set_byte_at(zp_addr as Word, 0x40);
    cpu.execute();
    let pc = 0xFFF+bytes+1;
    let sp = 0;
    let a = 0x42;
    let x = 0;
    let y = 0;
    let cycles = 0;
    let flags = Flags::fix_state(&[]);
    cpu.assert_state(pc, sp, a, x, y, cycles, flags);
}


#[test]
fn zpx() {
    let bytes = 0;
    let start = 0xFFF;

    let mem = &[OpCode::LdaImm.into(), 0x02, 0, 0, OpCode::Nop.into()];

    let mut cpu = Six502::new();
    cpu.set_pc(start);
    cpu.load_to_pc(mem);

    cpu.execute();
    let pc = 0xFFF+bytes+1;
    let sp = 0;
    let a = 0;
    let x = 0;
    let y = 0;
    let cycles = 0;
    let flags = Flags::fix_state(&[]);
    cpu.assert_state(pc, sp, a, x, y, cycles, flags);
}


#[test]
fn abs() {

    let bytes = 2+3;
    let start = 0xFFF;

    let mem = &[OpCode::LdaImm.into(), 0x02, OpCode::AdcAbs.into(), 0x11, 0x11, OpCode::Nop.into()];

    let mut cpu = Six502::new();
    cpu.set_pc(start);
    cpu.load_to_pc(mem);
    cpu.set_byte_at(0x1111, 0x02);

    cpu.execute();
    let pc = 0xFFF+bytes+1;
    let sp = 0;
    let a = 0x4;
    let x = 0;
    let y = 0;
    let cycles = 0;
    let flags = Flags::fix_state(&[]);
    cpu.assert_state(pc, sp, a, x, y, cycles, flags);


}
#[test]
fn abx() {

    let bytes = 2+3;
    let start = 0xFFF;

    let mem = &[OpCode::LdaImm.into(), 0x02, OpCode::AdcAbx.into(), 0x00, 0x30, OpCode::Nop.into()];

    let mut cpu = Six502::new();
    cpu.set_pc(start);
    cpu.load_to_pc(mem);
    cpu.set_reg_byte(crate::cpu::Register::X, 0x92);
    cpu.set_byte_at(0x3092, 0xFF);
    cpu.execute();
    let pc = 0xFFF+bytes+1;
    let sp = 0;
    let a = 0x01;
    let x = 0x92;
    let y = 0;
    let cycles = 0;
    let flags = Flags::fix_state(&[FlagIndex::C]);
    cpu.assert_state(pc, sp, a, x, y, cycles, flags);

}
#[test]
fn aby() {

    let bytes = 2+3;
    let start = 0xFFF;
    let mem = &[OpCode::LdaImm.into(), 0x02, OpCode::AdcAby.into(), 0x00, 0x30, OpCode::Nop.into()];

    let mut cpu = Six502::new();
    cpu.set_pc(start);
    cpu.load_to_pc(mem);
    cpu.set_reg_byte(crate::cpu::Register::Y, 0x92);
    cpu.set_byte_at(0x3092, 0xFF);
    cpu.execute();
    let pc = 0xFFF+bytes+1;
    let sp = 0;
    let a = 0x01;
    let x = 0;
    let y = 0x92;
    let cycles = 0;
    let flags = Flags::fix_state(&[FlagIndex::C]);
    cpu.assert_state(pc, sp, a, x, y, cycles, flags);

}
#[test]
fn izx() {

    let bytes = 2+2;
    let start = 0xFFF;

    let mem = &[OpCode::LdaImm.into(), 0x02, OpCode::AdcIzx.into(), 0x40, OpCode::Nop.into()];

    let mut cpu = Six502::new();
    cpu.set_pc(start);
    cpu.load_to_pc(mem);
    cpu.set_reg_byte(crate::cpu::Register::X, 0x04);
    cpu.set_byte_at(0x44, 0x01);
    cpu.set_byte_at(0x45, 0x02);
    cpu.set_byte_at(0x0201, 0x22);

    cpu.execute();
    let pc = 0xFFF+bytes+1;
    let sp = 0;
    let a = 0x24;
    let x = 0x04;
    let y = 0;
    let cycles = 0;
    let flags = Flags::fix_state(&[]);
    cpu.assert_state(pc, sp, a, x, y, cycles, flags);

}
#[test]
fn izy() {
    let bytes = 2+2;
    let start = 0xFFF;

    let mem = &[OpCode::LdaImm.into(), 0x02, OpCode::AdcIzy.into(), 0x40, OpCode::Nop.into()];

    let mut cpu = Six502::new();
    cpu.set_pc(start);
    cpu.load_to_pc(mem);
    cpu.set_reg_byte(crate::cpu::Register::Y, 0x04);
    cpu.set_byte_at(0x40, 0x01);
    cpu.set_byte_at(0x41, 0x02);
    cpu.set_byte_at(0x0205, 0x22);

    cpu.execute();
    let pc = 0xFFF+bytes+1;
    let sp = 0;
    let a = 0x24;
    let x = 0;
    let y = 0x04;
    let cycles = 0;
    let flags = Flags::fix_state(&[]);
    cpu.assert_state(pc, sp, a, x, y, cycles, flags);

}



#[test]
fn instr_correct_spots() {
    let instructions = Instructions::init();
    assert_eq!(instructions[0x61].mnemonic, "ADC");
    assert_eq!(instructions[0x61].mode, AddressingMode::IZX);
    assert_eq!(instructions[0x65].mnemonic, "ADC");
    assert_eq!(instructions[0x65].mode, AddressingMode::ZP0);
    assert_eq!(instructions[0x69].mnemonic, "ADC");
    assert_eq!(instructions[0x69].mode, AddressingMode::IMM);
    assert_eq!(instructions[0x6D].mnemonic, "ADC");
    assert_eq!(instructions[0x6D].mode, AddressingMode::ABS);
    assert_eq!(instructions[0x71].mnemonic, "ADC");
    assert_eq!(instructions[0x71].mode, AddressingMode::IZY);
    assert_eq!(instructions[0x75].mnemonic, "ADC");
    assert_eq!(instructions[0x75].mode, AddressingMode::ZPX);
    assert_eq!(instructions[0x69].mnemonic, "ADC");
    assert_eq!(instructions[0x69].mode, AddressingMode::ABY);
    assert_eq!(instructions[0x7D].mnemonic, "ADC");
    assert_eq!(instructions[0x7D].mode, AddressingMode::ABX);
}
