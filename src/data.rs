use std::{cell::{Cell, RefCell}, rc::Rc};

use crate::cpu::Six502;


const CPU_RAM_MIN: u16 = 0x0000;
const CPU_RAM_MAX: u16 = 0x1FFF;
pub const RAM_SIZE: usize = 1024*64;

pub trait DataBus {
    fn load(&mut self, addr: u16, data: &[u8]);
    fn write(&mut self, addr: u16, data: u8); 
    fn read(&self, addr: u16) -> u8; 
    fn size(&self) -> usize;
    fn clear(&mut self);
}

pub struct System<D: DataBus> {
    bus: Rc<RefCell<D>>,
    cpu: Six502<D>,
}

impl<D: DataBus> System<D> {
    pub fn new(bus: Rc<RefCell<D>>) -> Self {
        let mut cpu = Six502::new();
        cpu.connect_bus(Rc::clone(&bus));
        Self {
            bus,
            cpu,
        }
    }

    pub fn self_contained(&mut self) {
        self.cpu.execute();
    }
}

