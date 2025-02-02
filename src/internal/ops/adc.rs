//######################################################
//###############    ###     ####    ####  #############
//###############   #  #    #   #   #      #############
//###############  #####   #   #   #       #############
//############### #    #  ####    ####     #############
//######################################################

use crate::{cpu::{Six502, Word}, internal::ops::common::adc};

pub fn imm(cpu: &mut Six502) -> bool {
    let operand = cpu.imm();
    adc(cpu, operand);
    assert!(cpu.cycles_at(0));
    return true;
}

pub fn zp0(cpu: &mut Six502) -> bool {
    let addr = cpu.zp0();
    let operand = cpu.read_byte(addr as Word);
    adc(cpu, operand);
    assert!(cpu.cycles_at(0));
    return true;
}


pub fn zpx(cpu: &mut Six502) -> bool { 
    let addr = cpu.zpx();
    let operand = cpu.read_byte(addr);
    adc(cpu, operand);
    assert!(cpu.cycles_at(0));
    return true;
}

pub fn abs(cpu: &mut Six502) -> bool {
    let addr = cpu.abs();
    let operand = cpu.read_byte(addr);
    adc(cpu, operand);
    assert!(cpu.cycles_at(0));
    return true;
} 
pub fn abx(cpu: &mut Six502) -> bool {
    let mut abs_addr_x = 0;
    let cross = cpu.abx(&mut abs_addr_x);
    let operand = cpu.read_byte(abs_addr_x);
    adc(cpu, operand);
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
pub fn aby(cpu: &mut Six502) -> bool {
    let mut abs_addr_y = 0;
    let cross = cpu.aby(&mut abs_addr_y);
    let operand = cpu.read_byte(abs_addr_y);
    adc(cpu, operand);
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
pub fn izx(cpu: &mut Six502) -> bool {
    let effective_addr = cpu.izx();
    let operand = cpu.read_byte(effective_addr);
    adc(cpu, operand);
    assert!(cpu.cycles_at(0));
    return true;
}


pub fn izy(cpu: &mut Six502) -> bool {
    let mut effective_addr = 0;
    let cross = cpu.izy(&mut effective_addr);
    let operand = cpu.read_byte(effective_addr);
    adc(cpu, operand);
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
