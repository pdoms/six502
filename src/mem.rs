use std::ops::{Index, IndexMut};

use crate::{cpu::Byte, data::{DataBus, RAM_SIZE}};

pub struct Mem {
    inner: [u8; RAM_SIZE]
}

impl DataBus for Mem {
    fn load(&mut self, addr: u16, data: &[u8]) {
        if RAM_SIZE < (addr as usize) + data.len() {
            panic!("Provided data is too big for cpu memory: {}kb", RAM_SIZE/1024);
        } 
        unsafe {
            let ptr = self.inner.as_mut_ptr().offset(addr as isize);
            ptr.copy_from(data.as_ptr(), data.len());
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        self[addr] = data;
    }

    fn read(&self, addr: u16) -> u8 {
        self[addr]
    }

    fn size(&self) -> usize {
        self.inner.len()
    }

    fn clear(&mut self) {
        //apparently this gets turned into memset call:
        //https://rust.godbolt.org/
        self.inner.iter_mut().for_each(|m| *m = 0);
    }
}

impl Mem {
    pub fn init() -> Self {
        Self {inner: [0; RAM_SIZE]}
    }

    #[cfg(test)]
    pub fn get_mem_section(&self, start: usize, buf: &mut [Byte]) {
        buf.copy_from_slice(&self.inner[start..buf.len()]);
    }
}


impl Index<u16> for Mem {
    type Output=Byte;

    fn index(&self, index: u16) -> &Self::Output {
        &self.inner[index as usize]
    }
}

impl IndexMut<u16> for Mem {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.inner[index as usize]
    }
}
