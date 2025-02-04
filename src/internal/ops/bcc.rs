use crate::cpu::Six502;

use super::common::branch_if;

pub fn rel(cpu: &mut Six502) -> bool {
    let c_flag = cpu.carry_b();
    let remaining_cycles = branch_if(cpu, c_flag, 0);
    assert!(cpu.cycles_at(remaining_cycles));
    cpu.set_cycle(0);
    return false;
}
