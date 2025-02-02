mod ops;
#[macro_use]
pub mod instruction;
mod modes;
mod initializer;
pub mod opcodes;
#[cfg(test)]
mod test;

use std::ops::{Index, IndexMut};

use instruction::InstrCode;

pub(crate) const TABLE_COLS: usize = 16;
pub(crate) const TABLE_ROWS: usize = 16;

pub struct Instructions {
    //COLS;ROWS
    inner: [[InstrCode; TABLE_COLS]; TABLE_ROWS],
}

impl Index<u8> for Instructions {
    type Output = InstrCode;

    fn index(&self, index: u8) -> &Self::Output {
        let row = index >> 4; 
        let col = index  & 0xF; 
        &self.inner[row as usize][col as usize]
    }
}

impl IndexMut<u8> for Instructions {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        let row = index >> 4; 
        let col = index  & 0xF; 
        &mut self.inner[row as usize][col as usize]
    }
}


impl Instructions {
    pub fn init() -> Self {
        Self {
            inner: [
                initializer::row_0(),
                initializer::row_1(),
                initializer::row_2(),
                initializer::row_3(),
                initializer::row_4(),
                initializer::row_5(),
                initializer::row_6(),
                initializer::row_7(),
                initializer::row_8(),
                initializer::row_9(),
                initializer::row_10(),
                initializer::row_11(),
                initializer::row_12(),
                initializer::row_13(),
                initializer::row_14(),
                initializer::row_15(),
            ]
        }
    }
}
