use crate::cpu::Six502;
use crate::flags::FlagIndex;

pub fn nop(_cpu: &mut Six502) -> bool {
    false
}

pub fn lda_imm(cpu: &mut Six502) -> bool {
    let b = cpu.fetch_byte();
    cpu.set_a(b);
    //set flags
    let a = cpu.a();
    cpu.set_flag_value(FlagIndex::Z, a);
    cpu.set_flag_value(FlagIndex::N, a);
    // cycles match?
    assert!(cpu.cycles_at(0));
    return true;
}
