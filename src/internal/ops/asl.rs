//######################################################
//###############     ###    #####   #  ################ 
//###############    #  #   #       #   ################  
//###############   #####   #####  #    ################ 
//###############  #    #       # ####  ################   
//######################## ##### #######################                   
//######################################################

use crate::{cpu::{Six502, Word}, internal::ops::common::{and, asl}};

pub fn acc(cpu: &mut Six502) -> bool {
    let a = cpu.a();
    *cpu.a_mut() = asl(cpu, a);
    assert!(cpu.cycles_at(0));
    return true;
}

pub fn zp0(cpu: &mut Six502) -> bool {
    let addr = cpu.zp0() as Word;
    let operand = cpu.read_byte(addr);
    let b = asl(cpu, operand);
    cpu.write_byte(addr, b);
    assert!(cpu.cycles_at(0));
    return true;
}


pub fn zpx(cpu: &mut Six502) -> bool { 
    let addr = cpu.zpx() as Word;
    let operand = cpu.read_byte(addr);
    let b = asl(cpu, operand);
    cpu.write_byte(addr, b);
    assert!(cpu.cycles_at(0));
    return true;
}

pub fn abs(cpu: &mut Six502) -> bool {
    let addr = cpu.abs() as Word;
    let operand = cpu.read_byte(addr);
    let b = asl(cpu, operand);
    cpu.write_byte(addr, b);
    assert!(cpu.cycles_at(0));
    return true;
} 
pub fn abx(cpu: &mut Six502) -> bool {
    let mut abs_addr_x = 0;
    cpu.abx(&mut abs_addr_x);
    let operand = cpu.read_byte(abs_addr_x);
    let b = asl(cpu, operand);
    cpu.write_byte(abs_addr_x, b);
    assert!(cpu.cycles_at(0));
    return true;
} 
