use std::{cell::RefCell, rc::Rc};

use crate::{cpu::{Six502, Word}, data::DataBus, mem::Mem};

mod adc;
mod brk;
mod and;
mod asl;
mod bit;
mod branch;
mod lda;

pub fn prelude(pc: Word, data: &[u8]) -> Six502<Mem> {
    let mem = Rc::new(RefCell::new(Mem::init()));
    mem.borrow_mut().load(pc, data);
    let mut cpu = Six502::new();
    cpu.connect_bus(mem);
    cpu.set_pc(pc);
    cpu
}

