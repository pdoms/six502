use crate::{data::DataBus, cpu::{Byte, Six502, INERRUPT_VECTOR}, flags::Flag};

pub fn imp<D: DataBus>(cpu: &mut Six502<D>) -> bool {
    
    *cpu.pc_mut() += 1;
    println!("{}", cpu.pc());
    cpu.clock();
    
    let pc = cpu.pc();
    //cpu.set_flag(Flag::I, 1);

    // push pc to stack
    cpu.push_stack((pc >> 8 & 0x00FF) as Byte);
    cpu.push_stack((pc & 0x00FF) as Byte);

    cpu.set_flag(Flag::B, 1);
    cpu.push_stack(cpu.status());

    cpu.set_flag(Flag::B, 0);

    *cpu.pc_mut() = cpu.read_word(INERRUPT_VECTOR);
    assert!(cpu.cycles_at(0));
    return true;
}
