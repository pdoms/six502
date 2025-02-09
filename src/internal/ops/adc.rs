//######################################################
//###############    ###     ####    ####  #############
//###############   #  #    #   #   #      #############
//###############  #####   #   #   #       #############
//############### #    #  ####    ####     #############
//######################################################

use crate::{cpu::{Six502, Word}, data::DataBus, internal::ops::common::adc};

pub fn imm<D: DataBus>(cpu: &mut Six502<D>) -> bool {
    let operand = cpu.imm();
    adc(cpu, operand);
    assert!(cpu.cycles_at(0));
    return true;
}

pub fn zp0<D: DataBus>(cpu: &mut Six502<D>) -> bool {
    let addr = cpu.zp0();
    let operand = cpu.read_byte(addr as Word);
    adc(cpu, operand);
    assert!(cpu.cycles_at(0));
    return true;
}


pub fn zpx<D: DataBus>(cpu: &mut Six502<D>) -> bool { 
    let addr = cpu.zpx();
    let operand = cpu.read_byte(addr);
    adc(cpu, operand);
    assert!(cpu.cycles_at(0));
    return true;
}

pub fn abs<D: DataBus>(cpu: &mut Six502<D>) -> bool {
    let addr = cpu.abs();
    let operand = cpu.read_byte(addr);
    adc(cpu, operand);
    assert!(cpu.cycles_at(0));
    return true;
} 
pub fn abx<D: DataBus>(cpu: &mut Six502<D>) -> bool {
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
pub fn aby<D: DataBus>(cpu: &mut Six502<D>) -> bool {
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
pub fn izx<D: DataBus>(cpu: &mut Six502<D>) -> bool {
    let effective_addr = cpu.izx();
    let operand = cpu.read_byte(effective_addr);
    adc(cpu, operand);
    assert!(cpu.cycles_at(0));
    return true;
}


pub fn izy<D: DataBus>(cpu: &mut Six502<D>) -> bool {
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
