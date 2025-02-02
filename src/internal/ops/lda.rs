//######################################################
//###############    #       ####   ###  ###############
//###############   #       #   #  #  #  ###############
//###############  #       #   #  #####  ###############
//############### ######  ####   #    #  ###############
//######################################################

use crate::{cpu::{Six502, Word}, internal::ops::common::flags_z_n};


pub fn imm(cpu: &mut Six502) -> bool {
    let b = cpu.imm();
    cpu.set_a(b);
    flags_z_n(cpu);
    assert!(cpu.cycles_at(0));
    return true;
}

pub fn zp0(cpu: &mut Six502) -> bool {
    let addr = cpu.zp0();
    cpu.load_a(addr as Word);
    flags_z_n(cpu);
    assert!(cpu.cycles_at(0));
    return true;
}

pub fn zpx(cpu: &mut Six502) -> bool {
    let addr = cpu.zpx();
    cpu.load_a(addr);
    flags_z_n(cpu);
    assert!(cpu.cycles_at(0));
    return true;
}

pub fn abs(cpu: &mut Six502) -> bool {
    let addr = cpu.abs();
    cpu.load_a(addr);
    flags_z_n(cpu);
    assert!(cpu.cycles_at(0));
    return true;
} 

pub fn abx(cpu: &mut Six502) -> bool {
    let mut abs_addr_x = 0;
    let cross = cpu.abx(&mut abs_addr_x);
    cpu.load_a(abs_addr_x);
    flags_z_n(cpu);
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
    cpu.load_a(abs_addr_y);
    flags_z_n(cpu);
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
    cpu.load_a(effective_addr);
    flags_z_n(cpu);
    assert!(cpu.cycles_at(0));
    return true;

}
pub fn izy(cpu: &mut Six502) -> bool {
    let mut effective_addr_y = 0;
    let cross = cpu.izy(&mut effective_addr_y);
    cpu.load_a(effective_addr_y);
    flags_z_n(cpu);
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
