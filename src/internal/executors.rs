use std::clone;

use crate::cpu::{Six502, Word};
use crate::flags::FlagIndex;

/// It was deliberately decided tp keep
/// the addressing mode operations inside the 
/// instruction executors. Yes, this
/// will duplicate a lot of code, but is a) arguably 
/// easier to read the code as it keeps it all
/// together and b) likely to get optimised 
/// out as well.

pub fn nop(_cpu: &mut Six502) -> bool {
    false
}

//######################################################
//###############    #       ####   ###  ###############
//###############   #       #   #  #  #  ###############
//###############  #       #   #  #####  ###############
//############### ######  ####   #    #  ###############
//######################################################

pub fn lda_set_flags(cpu: &mut Six502) {
    let a = cpu.a();
    cpu.set_flag_value(FlagIndex::Z, a);
    cpu.set_flag_value(FlagIndex::N, a);
}

pub fn lda_imm(cpu: &mut Six502) -> bool {
    let b = cpu.fetch_byte();
    cpu.set_a(b);
    lda_set_flags(cpu);
    assert!(cpu.cycles_at(0));
    return true;
}

pub fn lda_zp0(cpu: &mut Six502) -> bool {
    let b = cpu.fetch_byte();
    cpu.load_a(b as Word);
    lda_set_flags(cpu);
    assert!(cpu.cycles_at(0));
    return true;
}

pub fn lda_zpx(cpu: &mut Six502) -> bool {
    let b = cpu.fetch_byte().wrapping_add(cpu.x());
    cpu.clock();
    cpu.load_a(b as Word);
    lda_set_flags(cpu);
    assert!(cpu.cycles_at(0));
    return true;
}

pub fn lda_abs(cpu: &mut Six502) -> bool {
    let addr = cpu.fetch_word();
    cpu.load_a(addr);
    lda_set_flags(cpu);
    assert!(cpu.cycles_at(0));
    return true;
} 

pub fn lda_abx(cpu: &mut Six502) -> bool {
    let abs_addr = cpu.fetch_word();
    let abs_addr_x = abs_addr.wrapping_add(cpu.x() as Word);
    cpu.clock();
    cpu.load_a(abs_addr_x);
    lda_set_flags(cpu);
    let cross =(abs_addr ^ abs_addr_x) >> 8 > 0;
    if cross {
        cpu.clock();
    } else {
        if cpu.cycles_at(1) {
            cpu.set_cycle(0);
        }
    }
    assert!(cpu.cycles_at(0));
    return true;
}

pub fn lda_aby(cpu: &mut Six502) -> bool {
    let abs_addr = cpu.fetch_word();
    let abs_addr_y = abs_addr.wrapping_add(cpu.y() as Word);
    cpu.clock();
    cpu.load_a(abs_addr_y);
    lda_set_flags(cpu);
    let cross =(abs_addr ^ abs_addr_y) >> 8 > 0;
    if cross {
        cpu.clock();
    } else {
        if cpu.cycles_at(1) {
            cpu.set_cycle(0);
        }
    }
    let cycles = cpu.cycles();
    assert!(cpu.cycles_at(0), "recieved: {cycles}");
    return true;
}



