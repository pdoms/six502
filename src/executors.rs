use crate::cpu::Six502;



pub fn nop(_cpu: &mut Six502) -> bool {
    false
}

pub fn lda_imm(cpu: &mut Six502) -> bool {
    let b = cpu.fetch_byte();
    cpu.set_a(b);
    //set flags
    let a = cpu.a();
    cpu.set_flag_value(crate::cpu::FlagIndex::Z, a);
    cpu.set_flag_value(crate::cpu::FlagIndex::N, a);
    // cycles match?
    assert!(cpu.cycles_at(0));
    return true;
}
