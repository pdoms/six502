//######################################################
//###############    #       ####   ###  ###############
//###############   #       #   #  #  #  ###############
//###############  #       #   #  #####  ###############
//############### ######  ####   #    #  ###############
//######################################################

use crate::{cpu::{Six502, Word}, data::DataBus, internal::ops::common::a_flags_z_n};


pub fn imm<D: DataBus>(cpu: &mut Six502<D>) -> bool {
    let b = cpu.imm();
    cpu.set_a(b);
    a_flags_z_n(cpu);
    assert!(cpu.cycles_at(0));
    return true;
}

pub fn zp0<D: DataBus>(cpu: &mut Six502<D>) -> bool {
    let addr = cpu.zp0();
    cpu.load_a(addr as Word);
    a_flags_z_n(cpu);
    assert!(cpu.cycles_at(0));
    return true;
}

pub fn zpx<D: DataBus>(cpu: &mut Six502<D>) -> bool {
    let addr = cpu.zpx();
    cpu.load_a(addr);
    a_flags_z_n(cpu);
    assert!(cpu.cycles_at(0));
    return true;
}

pub fn abs<D: DataBus>(cpu: &mut Six502<D>) -> bool {
    let addr = cpu.abs();
    cpu.load_a(addr);
    a_flags_z_n(cpu);
    assert!(cpu.cycles_at(0));
    return true;
} 

pub fn abx<D: DataBus>(cpu: &mut Six502<D>) -> bool {
    let mut abs_addr_x = 0;
    let cross = cpu.abx(&mut abs_addr_x);
    cpu.load_a(abs_addr_x);
    a_flags_z_n(cpu);
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
    cpu.load_a(abs_addr_y);
    a_flags_z_n(cpu);
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
    cpu.load_a(effective_addr);
    a_flags_z_n(cpu);
    assert!(cpu.cycles_at(0));
    return true;

}
pub fn izy<D: DataBus>(cpu: &mut Six502<D>) -> bool {
    let mut effective_addr_y = 0;
    let cross = cpu.izy(&mut effective_addr_y);
    cpu.load_a(effective_addr_y);
    a_flags_z_n(cpu);
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
