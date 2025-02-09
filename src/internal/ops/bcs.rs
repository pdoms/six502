use crate::{data::DataBus, cpu::Six502, flags::Flag, internal::ops::common::branch_if};

pub fn rel<D: DataBus>(cpu: &mut Six502<D>) -> bool {
    let c_flag = cpu.get_flag(Flag::C);
    println!("c flag: {c_flag}");
    let remaining_cycles = branch_if(cpu, c_flag, 1);
    assert!(cpu.cycles_at(remaining_cycles));
    cpu.set_cycle(0);
    return false;
}
