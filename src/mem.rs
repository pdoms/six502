use std::ops::{Index, IndexMut};

use crate::cpu::{Byte, Word};


pub const MEM_CAP: usize = 1024*64;

pub struct Mem {
    inner: [u8; MEM_CAP]
}

impl Mem {
    pub fn init() -> Self {
        Self {inner: [0; MEM_CAP]}
    }

    pub fn clear(&mut self) {
        //apparently this gets turned into memset call:
        //https://rust.godbolt.org/
        self.inner.iter_mut().for_each(|m| *m = 0);
    }

    pub fn load(&mut self, at: isize, data: &[u8]) {
        if MEM_CAP < (at as usize) + data.len() {
            panic!("Provided data is too big for cpu memory: {}kb", MEM_CAP/1024);
        } 
        unsafe {
            let ptr = self.inner.as_mut_ptr().offset(at);
            ptr.copy_from(data.as_ptr(), data.len());
        }
    }
    #[cfg(test)]
    pub fn get_mem_section(&self, start: usize, buf: &mut [Byte]) {
        buf.copy_from_slice(&self.inner[start..buf.len()]);
    }
}

impl Index<u8> for Mem {
    type Output=Byte;

    fn index(&self, index: u8) -> &Self::Output {
        &self.inner[index as usize]
    }
}

impl Index<u16> for Mem {
    type Output=Byte;

    fn index(&self, index: u16) -> &Self::Output {
        &self.inner[index as usize]
    }
}

impl IndexMut<u8> for Mem {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.inner[index as usize]
    }
}

impl IndexMut<u16> for Mem {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.inner[index as usize]
    }
}
