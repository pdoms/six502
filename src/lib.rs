pub mod cpu;
mod common;
mod instructions;
mod instr_init;
mod executors;



#[cfg(test)]
mod test {
    use crate::{cpu::{Flags, Six502}, instructions::OpCode};

    
    #[test]
    fn just_test() {
        let mem = &[OpCode::LdaImm as u8, 0x22];
        let mut cpu = Six502::new();
        cpu.load_to_pc(mem);
        cpu.execute();
        let pc = 65534;
        let sp = 0;
        let a = 34;
        let x = 0;
        let y = 0;
        let cycles = 0;
        let flags = Flags::fix_state(&[]);
        cpu.assert_state(pc, sp, a, x, y, cycles, flags);
    }
}
