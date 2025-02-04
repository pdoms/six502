

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
    IND, // Indirect
    IZX, // aka Indexed Indirect
    IZY, // aka Indirect Indexed
    ACC, // accumulator direct
}





