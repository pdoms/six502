//######################################################
//###############    ###     ##   #   ####  ############
//###############   #  #    # #  #   #   #  ############
//###############  #####   #  # #   #   #   ############
//############### #    #  #   ##   ####     ############
//######################################################

use crate::{cpu::{Six502, Word}, internal::ops::common::and};

pub fn imm(cpu: &mut Six502) -> bool {
    let b = cpu.imm();
    *cpu.a_mut() &= b;
    assert!(cpu.cycles_at(0));
    return true;
}

pub fn zp0(cpu: &mut Six502) -> bool {
    let addr = cpu.zp0();
    and(cpu, addr as Word);
    assert!(cpu.cycles_at(0));
    return true;
}


pub fn zpx(cpu: &mut Six502) -> bool { 
    let addr = cpu.zpx();
    and(cpu, addr as Word);
    assert!(cpu.cycles_at(0));
    return true;
}

pub fn abs(cpu: &mut Six502) -> bool {
    let addr = cpu.abs();
    and(cpu, addr);
    assert!(cpu.cycles_at(0));
    return true;
} 
pub fn abx(cpu: &mut Six502) -> bool {
    let mut abs_addr_x = 0;
    let cross = cpu.abx(&mut abs_addr_x);
    and(cpu, abs_addr_x);
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
    and(cpu, abs_addr_y);
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
    and(cpu, effective_addr);
    assert!(cpu.cycles_at(0));
    return true;
}

pub fn izy(cpu: &mut Six502) -> bool {
    let mut effective_addr = 0;
    let cross = cpu.izy(&mut effective_addr);
    and(cpu, effective_addr);
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
