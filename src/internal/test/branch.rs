use crate::{cpu::Six502, flags::{FlagIndex, Flags}, internal::opcodes::OpCode};



#[test]
fn bcc_branch_forward() {
    let label = 0x16;  // 18
    let mem = &[
        OpCode::BccRel.into(), 
        label, 
        OpCode::Nop.into()
    ];
    let mut cpu = Six502::new();
    cpu.set_pc(0xFFF);
    cpu.load_to_pc(mem);
    cpu.set_flag_value(FlagIndex::C, 0);
    cpu.execute();
    let pc = 0x1015 + 2;
    let sp = 0;
    let a = 0;
    let x = 0;
    let y = 0;
    let cycles = 0;
    let flags = Flags::fix_state(&[]);
    cpu.assert_state(pc, sp, a, x, y, cycles, flags);
}

#[test]
fn bcc_branch_backward() {
    let label = 0xEE;  // -18
    let mem = &[
        OpCode::BccRel.into(), 
        label, 
        OpCode::Nop.into()
    ];
    let mut cpu = Six502::new();
    cpu.set_pc(0xFFF);
    cpu.load_to_pc(mem);
    cpu.set_flag_value(FlagIndex::C, 0);
    cpu.execute();
    let pc = 0xFFF + 2 - 18;
    let sp = 0;
    let a = 0;
    let x = 0;
    let y = 0;
    let cycles = 0;
    let flags = Flags::fix_state(&[]);
    cpu.assert_state(pc, sp, a, x, y, cycles, flags);
}
#[test]
fn bcc_no_branch() {
    let label = 0x16;  // 18
    let mem = &[
        OpCode::BccRel.into(), 
        label, 
        OpCode::Nop.into()
    ];
    let mut cpu = Six502::new();
    cpu.set_pc(0xFFF);
    cpu.load_to_pc(mem);
    cpu.set_flag_value(FlagIndex::C, 1);
    cpu.execute();
    let pc = 0xFFF+2;
    let sp = 0;
    let a = 0;
    let x = 0;
    let y = 0;
    let cycles = 0;
    let flags = Flags::fix_state(&[FlagIndex::C]);
    cpu.assert_state(pc, sp, a, x, y, cycles, flags);
}


#[test]
fn bcs_branch() {
    let label = 0x16;  // 18
    let mem = &[
        OpCode::BcsRel.into(), 
        label, 
        OpCode::Nop.into()
    ];
    let mut cpu = Six502::new();
    cpu.set_pc(0xFFF);
    cpu.load_to_pc(mem);
    cpu.set_flag_value(FlagIndex::C, 1);
    cpu.execute();
    let pc = 0x1015 + 2;
    let sp = 0;
    let a = 0;
    let x = 0;
    let y = 0;
    let cycles = 0;
    let flags = Flags::fix_state(&[FlagIndex::C]);
    cpu.assert_state(pc, sp, a, x, y, cycles, flags);
}

#[test]
fn bcs_no_branch() {
    let label = 0x16;  // 18
    let mem = &[
        OpCode::BcsRel.into(), 
        label, 
        OpCode::Nop.into()
    ];
    let mut cpu = Six502::new();
    cpu.set_pc(0xFFF);
    cpu.load_to_pc(mem);
    cpu.set_flag_value(FlagIndex::C, 0);
    cpu.execute();
    let pc = 0xFFF+2;
    let sp = 0;
    let a = 0;
    let x = 0;
    let y = 0;
    let cycles = 0;
    let flags = Flags::fix_state(&[]);
    cpu.assert_state(pc, sp, a, x, y, cycles, flags);
}
