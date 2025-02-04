use crate::{common::{check_overflow_pre, eq_sign_bits}, cpu::{Byte, Six502, Word}, flags::{FlagBits, FlagIndex}};


/// It was deliberately decided tp keep
/// the addressing mode operations inside the 
/// instruction executors. Yes, this
/// will duplicate a lot of code, but is a) arguably 
/// easier to read the code as it keeps it all together and b) likely to get optimised 
/// out as well.

pub fn nop(_cpu: &mut Six502) -> bool {
    false
}

pub fn a_flags_z_n(cpu: &mut Six502) {
    let a = cpu.a();
    cpu.set_flag_value(FlagIndex::Z, a);
    cpu.set_flag_value(FlagIndex::N, a);
}

pub fn flags_z_n(cpu: &mut Six502, result: Byte) {
    cpu.set_flag_value(FlagIndex::Z, result);
    cpu.set_flag_value(FlagIndex::N, result);
}


//Arithmetic Chip Functions
pub fn adc(cpu: &mut Six502, operand: Byte) {
    let eq_signed_bits: bool = eq_sign_bits(cpu.a(), operand); 
    let mut sum = cpu.a() as Word;
    sum += operand as Word;
    sum += cpu.carry_b() as Word;
    cpu.set_a(sum as u8 & 0xFF);
    a_flags_z_n(cpu);
    //set carry
    cpu.set_flag_value(FlagIndex::C, (sum  > 0xFF) as u8);
    //set overvlow
    let v = check_overflow_pre(cpu.a(), operand, eq_signed_bits);
    cpu.set_flag_value(FlagIndex::O, v as u8);
}


pub fn and(cpu: &mut Six502, addr: Word) {
    let b = cpu.read_byte(addr);
    let a = cpu.a_mut();
    *a &= b;
    a_flags_z_n(cpu);
}

pub fn asl(cpu: &mut Six502, operand: Byte) -> Byte {
    cpu.set_flag_value(FlagIndex::C, operand & FlagBits::N as u8);
    let result = operand << 1;
    cpu.clock();
    flags_z_n(cpu, result);
    result
}


