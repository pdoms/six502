use crate::{data::DataBus, cpu::{Six502, Word}, flags::Flag};

pub fn zp0<D: DataBus>(cpu: &mut Six502<D>) -> bool {
    let addr = cpu.zp0();
    let byte = cpu.read_byte(addr as Word);
    let tmp = byte & cpu.a();
    cpu.set_flag(Flag::Z, tmp & 0x00FF);
    cpu.set_flag(Flag::N, byte & (1 << 7));
    cpu.set_flag(Flag::V, byte & (1 << 6));
    assert!(cpu.cycles_at(0));
    return true;
}

pub fn abs<D: DataBus>(cpu: &mut Six502<D>) -> bool {
    let addr = cpu.abs();
    let byte = cpu.read_byte(addr);
    let tmp = byte & cpu.a();
    cpu.set_flag(Flag::Z, tmp & 0x00FF);
    cpu.set_flag(Flag::N, byte & (1 << 7));
    cpu.set_flag(Flag::V, byte & (1 << 6));
    assert!(cpu.cycles_at(0));
    return true;
}
