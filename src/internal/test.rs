use crate::internal::{executors::nop, modes::AddressingMode, Instructions};

//#[test]
//fn just_test() {
//    let mem = &[OpCode::LdaImm as u8, 0x22];
//    let mut cpu = Six502::new();
//    cpu.load_to_pc(mem);
//    cpu.execute();
//    let pc = 65534;
//    let sp = 0;
//    let a = 34;
//    let x = 0;
//    let y = 0;
//    let cycles = 0;
//    let flags = Flags::fix_state(&[]);
//    cpu.assert_state(pc, sp, a, x, y, cycles, flags);
//}
#[test]
fn instructions_base() {

    let instruction = instr!(2, 1, "AND", AddressingMode::IZX, 6, nop);
    assert_eq!(instruction.opcode, 0x21);
    assert_eq!(instruction.mnemonic, "AND");
    assert_eq!(instruction.mode, AddressingMode::IZX);
    
    let instructions = Instructions::init();

    let opcode = 0xA9;
    assert_eq!(instructions[opcode].mnemonic, "LDA");
    assert_eq!(instructions[opcode].mode, AddressingMode::IMM);
    assert_eq!(instructions[opcode].cycles, 2);
}
