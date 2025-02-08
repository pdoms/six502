use crate::{
    common::{
        check_overflow_pre, eq_sign_bits, page_x_word}, 
    cpu::{
        Byte, Six502, Word}, 
    flags::Flag
};

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
    cpu.set_flag(Flag::Z, (a == 0) as u8);
    cpu.set_flag(Flag::N, a & 0x80);
}

pub fn flags_z_n(cpu: &mut Six502, result: Byte) {
    cpu.set_flag(Flag::Z, (result == 0) as u8);
    cpu.set_flag(Flag::N, result & 0x80);
}


//Arithmetic Chip Functions
pub fn adc(cpu: &mut Six502, operand: Byte) {
    let eq_signed_bits: bool = eq_sign_bits(cpu.a(), operand); 
    let mut sum = cpu.a() as Word;
    sum += operand as Word;
    sum += cpu.get_flag(Flag::C) as Word;
    cpu.set_a(sum as u8 & 0xFF);
    a_flags_z_n(cpu);
    //set carry
    cpu.set_flag(Flag::C, (sum  > 0xFF) as u8);
    //set overvlow
    let v = check_overflow_pre(cpu.a(), operand, eq_signed_bits);
    cpu.set_flag(Flag::V, v as u8);
}


pub fn and(cpu: &mut Six502, addr: Word) {
    let b = cpu.read_byte(addr);
    let a = cpu.a_mut();
    *a &= b;
    a_flags_z_n(cpu);
}

pub fn asl(cpu: &mut Six502, operand: Byte) -> Byte {
    cpu.set_flag(Flag::C, operand & Flag::N);
    let result = operand << 1;
    cpu.clock();
    flags_z_n(cpu, result);
    result
}

/// returns how many cycles should be left
pub fn branch_if(cpu: &mut Six502, test: u8, exp: u8) -> i64 {
    let mut offset = cpu.fetch_byte() as Word;
    if (offset & 0x80) > 0 {
        offset |= 0xFF00;
    }
    if test == exp {
        let prev_pc = cpu.pc();
        let new_pc = prev_pc.wrapping_add(offset);
        *cpu.pc_mut() = new_pc;
        cpu.clock(); 
        if page_x_word(cpu.pc(), prev_pc) {
            cpu.clock();
            return 0;
        }
        return 1;
    }
    2
}


