use crate::{common::{check_overflow_pre, eq_sign_bits}, cpu::{Byte, Six502, Word}, flags::FlagIndex};


/// It was deliberately decided tp keep
/// the addressing mode operations inside the 
/// instruction executors. Yes, this
/// will duplicate a lot of code, but is a) arguably 
/// easier to read the code as it keeps it all together and b) likely to get optimised 
/// out as well.

pub fn nop(_cpu: &mut Six502) -> bool {
    false
}

pub fn flags_z_n(cpu: &mut Six502) {
    let a = cpu.a();
    cpu.set_flag_value(FlagIndex::Z, a);
    cpu.set_flag_value(FlagIndex::N, a);
}


//Arithmetic Chip Functions
pub fn adc(cpu: &mut Six502, operand: Byte) {
    let eq_signed_bits: bool = eq_sign_bits(cpu.a(), operand); 
    let mut sum = cpu.a() as Word;
    sum += operand as Word;
    sum += cpu.carry_b() as Word;
    cpu.set_a(sum as u8 & 0xFF);
    flags_z_n(cpu);
    //set carry
    cpu.set_flag_value(FlagIndex::C, (sum  > 0xFF) as u8);
    //set overvlow
    let v = check_overflow_pre(cpu.a(), operand, eq_signed_bits);
    cpu.set_flag_value(FlagIndex::O, v as u8);
}
