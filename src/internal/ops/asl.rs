//######################################################
//###############     ###    #####   #  ################ 
//###############    #  #   #       #   ################  
//###############   #####   #####  #    ################ 
//###############  #    #       # ####  ################   
//######################## ##### #######################                   
//######################################################

use crate::{cpu::{Six502, Word}, data::DataBus, internal::ops::common::asl};

pub fn acc<D: DataBus>(cpu: &mut Six502<D>) -> bool {
    let a = cpu.a();
    *cpu.a_mut() = asl(cpu, a);
    assert!(cpu.cycles_at(0));
    return true;
}

pub fn zp0<D: DataBus>(cpu: &mut Six502<D>) -> bool {
    let addr = cpu.zp0() as Word;
    let operand = cpu.read_byte(addr);
    let b = asl(cpu, operand);
    cpu.write_byte(addr, b);
    assert!(cpu.cycles_at(0));
    return true;
}


pub fn zpx<D: DataBus>(cpu: &mut Six502<D>) -> bool { 
    let addr = cpu.zpx() as Word;
    let operand = cpu.read_byte(addr);
    let b = asl(cpu, operand);
    cpu.write_byte(addr, b);
    assert!(cpu.cycles_at(0));
    return true;
}

pub fn abs<D: DataBus>(cpu: &mut Six502<D>) -> bool {
    let addr = cpu.abs() as Word;
    let operand = cpu.read_byte(addr);
    let b = asl(cpu, operand);
    cpu.write_byte(addr, b);
    assert!(cpu.cycles_at(0));
    return true;
} 
pub fn abx<D: DataBus>(cpu: &mut Six502<D>) -> bool {
    let mut abs_addr_x = 0;
    cpu.abx(&mut abs_addr_x);
    let operand = cpu.read_byte(abs_addr_x);
    let b = asl(cpu, operand);
    cpu.write_byte(abs_addr_x, b);
    assert!(cpu.cycles_at(0));
    return true;
} 
