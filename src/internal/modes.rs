use crate::cpu::{Six502, Word};


#[allow(unused)]
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

