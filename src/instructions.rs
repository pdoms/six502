
//See: https://github.com/OneLoneCoder/olcNES/blob/master/Part%232%20-%20CPU/olc6502.cpp

use std::ops::{Index, IndexMut};

use crate::{cpu::Six502, instr_init::*};

pub const TABLE_ROWS: usize = 16;
pub const TABLE_COLS: usize = 16;


#[derive(Debug, PartialEq)]
pub enum AddressingMode {
    IMP,
    IMM,
    ZP0,
    ZPX,
    ZPY,
    REL,
    ABS,
    ABX,
    ABY,
    IND,
    IZX,
    IZY
}

///Takes: row (num), col (num), name (str literal), AddressingMode, cycles (num), and a function
///pointer
#[macro_export]
macro_rules! instr {
    ($r:expr, $c:expr, $n:expr, $m:expr, $cy:expr, $f:expr) => {
        {
            crate::instructions::Instruction {
                mnemonic: $n,
                opcode: $r << 4 | $c,
                mode: $m,
                cycles: $cy,
                exec_fn: $f
            }
        }
    };
}

pub struct Instruction {
    pub mnemonic: &'static str,
    pub opcode: u8,
    pub mode: AddressingMode,
    pub cycles: u8,
    pub exec_fn: fn(&mut Six502) -> bool 
}


pub struct Instructions {
    //COLS;ROWS
    inner: [[Instruction; TABLE_COLS]; TABLE_ROWS],
}

impl Index<u8> for Instructions {
    type Output = Instruction;

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
                row_0(),
                row_1(),
                row_2(),
                row_3(),
                row_4(),
                row_5(),
                row_6(),
                row_7(),
                row_8(),
                row_9(),
                row_10(),
                row_11(),
                row_12(),
                row_13(),
                row_14(),
                row_15(),
            ]
        }
    }
}

#[repr(u8)]
pub enum OpCode {
    LdaImm = 0xA9
}

#[cfg(test)]
mod test {
    use crate::{executors::nop, instructions::{AddressingMode, Instructions}};

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
}


