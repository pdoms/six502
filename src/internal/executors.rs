use crate::cpu::Six502;
use crate::flags::FlagIndex;

pub fn nop(_cpu: &mut Six502) -> bool {
    false
}

//######################################################
//###############    #       ####   ###  ###############
//###############   #       #   #  #  #  ###############
//###############  #       #   #  #####  ###############
//############### ######  ####   #    #  ###############
//######################################################

pub fn lda_set_flags(cpu: &mut Six502) {
    let a = cpu.a();
    cpu.set_flag_value(FlagIndex::Z, a);
    cpu.set_flag_value(FlagIndex::N, a);
}

pub fn lda_imm(cpu: &mut Six502) -> bool {
    let b = cpu.fetch_byte();
    cpu.set_a(b);
    lda_set_flags(cpu);
    assert!(cpu.cycles_at(0));
    return true;
}

pub fn lda_zp0(cpu: &mut Six502) -> bool {
    todo!()
}


