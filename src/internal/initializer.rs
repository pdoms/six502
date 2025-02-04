use crate::instr;

use super::ops::{adc, and, asl, bcc, bcs, lda};
use super::{modes::AddressingMode as am, instruction::InstrCode, TABLE_COLS};
use super::ops::common::nop;

#[inline]
pub fn row_0() -> [InstrCode; TABLE_COLS] {
    [
        instr!(0,0, "XXX", am::IMM, 1, nop), 
        instr!(0,1, "XXX", am::IMM, 1, nop),
        instr!(0,2, "XXX", am::IMM, 1, nop), //this is the current HALT op
        instr!(0,3, "XXX", am::IMM, 1, nop),
        instr!(0,4, "XXX", am::IMM, 1, nop),
        instr!(0,5, "XXX", am::IMM, 1, nop),
        instr!(0,6, "ASL", am::ZP0, 5, asl::zp0),
        instr!(0,7, "XXX", am::IMM, 1, nop),
        instr!(0,8, "XXX", am::IMM, 1, nop),
        instr!(0,9, "XXX", am::IMM, 1, nop),
        instr!(0,10, "ASL", am::ACC, 2, asl::acc),
        instr!(0,11, "XXX", am::IMM, 1, nop),
        instr!(0,12, "XXX", am::IMM, 1, nop),
        instr!(0,13, "XXX", am::IMM, 1, nop),
        instr!(0,14, "ASL", am::ABS, 6, asl::abs),
        instr!(0,15, "XXX", am::IMM, 1, nop),
    ]
}
#[inline]
pub fn row_1() -> [InstrCode; TABLE_COLS] {
    [
        instr!(1,0, "XXX", am::IMM, 1, nop), 
        instr!(1,1, "XXX", am::IMM, 5, nop), 
        instr!(1,2, "XXX", am::IMM, 1, nop),
        instr!(1,3, "XXX", am::IMM, 1, nop),
        instr!(1,4, "XXX", am::IMM, 1, nop),
        instr!(1,5, "XXX", am::IMM, 1, nop),
        instr!(1,6, "ASL", am::ZPX, 6, asl::zpx),
        instr!(1,7, "XXX", am::IMM, 1, nop),
        instr!(1,8, "XXX", am::IMM, 1, nop),
        instr!(1,9, "XXX", am::IMM, 1, nop),
        instr!(1,10, "XXX", am::IMM, 1, nop),
        instr!(1,11, "XXX", am::IMM, 1, nop),
        instr!(1,12, "XXX", am::IMM, 1, nop),
        instr!(1,13, "XXX", am::IMM, 1, nop),
        instr!(1,14, "ASL", am::ABX, 7, asl::abx),
        instr!(1,15, "XXX", am::IMM, 1, nop),
    ]
}

#[inline]
pub fn row_2() -> [InstrCode; TABLE_COLS] {
    [
        instr!(2,0, "XXX", am::IMM, 1, nop), 
        instr!(2,1, "AND", am::IZX, 6, and::izx), 
        instr!(2,2, "XXX", am::IMM, 1, nop),
        instr!(2,3, "XXX", am::IMM, 1, nop),
        instr!(2,4, "XXX", am::IMM, 1, nop),
        instr!(2,5, "AND", am::ZP0, 3, and::zp0),
        instr!(2,6, "XXX", am::IMM, 1, nop),
        instr!(2,7, "XXX", am::IMM, 1, nop),
        instr!(2,8, "XXX", am::IMM, 1, nop),
        instr!(2,9, "AND", am::IMM, 2, and::imm),
        instr!(2,10, "XXX", am::IMM, 1, nop),
        instr!(2,11, "XXX", am::IMM, 1, nop),
        instr!(2,12, "XXX", am::IMM, 1, nop),
        instr!(2,13, "AND", am::ABS, 4, and::abs),
        instr!(2,14, "XXX", am::IMM, 1, nop),
        instr!(2,15, "XXX", am::IMM, 1, nop),
    ]
}

#[inline]
pub fn row_3() -> [InstrCode; TABLE_COLS] {
    [
        instr!(3,0, "XXX", am::IMM, 1, nop), 
        instr!(3,1, "AND", am::IZY, 6, and::izy), 
        instr!(3,2, "XXX", am::IMM, 1, nop),
        instr!(3,3, "XXX", am::IMM, 1, nop),
        instr!(3,4, "XXX", am::IMM, 1, nop),
        instr!(3,5, "AND", am::ZPX, 4, and::zpx),
        instr!(3,6, "XXX", am::IMM, 1, nop),
        instr!(3,7, "XXX", am::IMM, 1, nop),
        instr!(3,8, "XXX", am::IMM, 1, nop),
        instr!(3,9, "AND", am::ABY, 5, and::aby),
        instr!(3,10, "XXX", am::IMM, 1, nop),
        instr!(3,11, "XXX", am::IMM, 1, nop),
        instr!(3,12, "XXX", am::IMM, 1, nop),
        instr!(3,13, "AND", am::ABX, 6, and::abx),
        instr!(3,14, "XXX", am::IMM, 1, nop),
        instr!(3,15, "XXX", am::IMM, 1, nop),
    ]
}


#[inline]
pub fn row_4() -> [InstrCode; TABLE_COLS] {
    [
        instr!(4,0, "XXX", am::IMM, 1, nop), 
        instr!(4,1, "XXX", am::IMM, 1, nop), 
        instr!(4,2, "XXX", am::IMM, 1, nop),
        instr!(4,3, "XXX", am::IMM, 1, nop),
        instr!(4,4, "XXX", am::IMM, 1, nop),
        instr!(4,5, "XXX", am::IMM, 1, nop),
        instr!(4,6, "XXX", am::IMM, 1, nop),
        instr!(4,7, "XXX", am::IMM, 1, nop),
        instr!(4,8, "XXX", am::IMM, 1, nop),
        instr!(4,9, "XXX", am::IMM, 1, nop),
        instr!(4,10, "XXX", am::IMM, 1, nop),
        instr!(4,11, "XXX", am::IMM, 1, nop),
        instr!(4,12, "XXX", am::IMM, 1, nop),
        instr!(4,13, "XXX", am::IMM, 1, nop),
        instr!(4,14, "XXX", am::IMM, 1, nop),
        instr!(4,15, "XXX", am::IMM, 1, nop),
    ]
}


#[inline]
pub fn row_5() -> [InstrCode; TABLE_COLS] {
    [
        instr!(5,0, "XXX", am::IMM, 1, nop), 
        instr!(5,1, "XXX", am::IMM, 1, nop), 
        instr!(5,2, "XXX", am::IMM, 1, nop),
        instr!(5,3, "XXX", am::IMM, 1, nop),
        instr!(5,4, "XXX", am::IMM, 1, nop),
        instr!(5,5, "XXX", am::IMM, 1, nop),
        instr!(5,6, "XXX", am::IMM, 1, nop),
        instr!(5,7, "XXX", am::IMM, 1, nop),
        instr!(5,8, "XXX", am::IMM, 1, nop),
        instr!(5,9, "XXX", am::IMM, 1, nop),
        instr!(5,10, "XXX", am::IMM, 1, nop),
        instr!(5,11, "XXX", am::IMM, 1, nop),
        instr!(5,12, "XXX", am::IMM, 1, nop),
        instr!(5,13, "XXX", am::IMM, 1, nop),
        instr!(5,14, "XXX", am::IMM, 1, nop),
        instr!(5,15, "XXX", am::IMM, 1, nop),
    ]
}

#[inline]
pub fn row_6() -> [InstrCode; TABLE_COLS] {
    [
        instr!(6,0, "XXX", am::IMM, 1, nop), 
        instr!(6,1, "ADC", am::IZX, 6, adc::izx), 
        instr!(6,2, "XXX", am::IMM, 1, nop),
        instr!(6,3, "XXX", am::IMM, 1, nop),
        instr!(6,4, "XXX", am::IMM, 1, nop),
        instr!(6,5, "ADC", am::ZP0, 3, adc::zp0),
        instr!(6,6, "XXX", am::IMM, 1, nop),
        instr!(6,7, "XXX", am::IMM, 1, nop),
        instr!(6,8, "XXX", am::IMM, 1, nop),
        instr!(6,9, "ADC", am::IMM, 2, adc::imm),
        instr!(6,10, "XXX", am::IMM, 1, nop),
        instr!(6,11, "XXX", am::IMM, 1, nop),
        instr!(6,12, "XXX", am::IMM, 1, nop),
        instr!(6,13, "ADC", am::ABS, 4, adc::abs),
        instr!(6,14, "XXX", am::IMM, 1, nop),
        instr!(6,15, "XXX", am::IMM, 1, nop),
    ]
}

#[inline]
pub fn row_7() -> [InstrCode; TABLE_COLS] {
    [
        instr!(7,0, "XXX", am::IMM, 1, nop), 
        instr!(7,1, "ADC", am::IZY, 6, adc::izy), 
        instr!(7,2, "XXX", am::IMM, 1, nop),
        instr!(7,3, "XXX", am::IMM, 1, nop),
        instr!(7,4, "XXX", am::IMM, 1, nop),
        instr!(7,5, "ADC", am::ZPX, 4, adc::zpx),
        instr!(7,6, "XXX", am::IMM, 1, nop),
        instr!(7,7, "XXX", am::IMM, 1, nop),
        instr!(7,8, "XXX", am::IMM, 1, nop),
        instr!(7,9, "ADC", am::ABY,  5, adc::aby),
        instr!(7,10, "XXX", am::IMM, 1, nop),
        instr!(7,11, "XXX", am::IMM, 1, nop),
        instr!(7,12, "XXX", am::IMM, 1, nop),
        instr!(7,13, "ADC", am::ABX, 5, adc::abx),
        instr!(7,14, "XXX", am::IMM, 1, nop),
        instr!(7,15, "XXX", am::IMM, 1, nop),
    ]
}

#[inline]
pub fn row_8() -> [InstrCode; TABLE_COLS] {
    [
        instr!(8,0, "XXX", am::IMM, 1, nop), 
        instr!(8,1, "XXX", am::IMM, 1, nop), 
        instr!(8,2, "XXX", am::IMM, 1, nop),
        instr!(8,3, "XXX", am::IMM, 1, nop),
        instr!(8,4, "XXX", am::IMM, 1, nop),
        instr!(8,5, "XXX", am::IMM, 1, nop),
        instr!(8,6, "XXX", am::IMM, 1, nop),
        instr!(8,7, "XXX", am::IMM, 1, nop),
        instr!(8,8, "XXX", am::IMM, 1, nop),
        instr!(8,9, "XXX", am::IMM, 1, nop),
        instr!(8,10, "XXX", am::IMM, 1, nop),
        instr!(8,11, "XXX", am::IMM, 1, nop),
        instr!(8,12, "XXX", am::IMM, 1, nop),
        instr!(8,13, "XXX", am::IMM, 1, nop),
        instr!(8,14, "XXX", am::IMM, 1, nop),
        instr!(8,15, "XXX", am::IMM, 1, nop),
    ]
}

#[inline]
pub fn row_9() -> [InstrCode; TABLE_COLS] {
    [
        instr!(9,0, "BCC", am::REL, 4, bcc::rel), 
        instr!(9,1, "XXX", am::IMP, 1, nop), 
        instr!(9,2, "XXX", am::IMM, 1, nop),
        instr!(9,3, "XXX", am::IMM, 1, nop),
        instr!(9,4, "XXX", am::IMM, 1, nop),
        instr!(9,5, "XXX", am::IMM, 1, nop),
        instr!(9,6, "XXX", am::IMM, 1, nop),
        instr!(9,7, "XXX", am::IMM, 1, nop),
        instr!(9,8, "XXX", am::IMM, 1, nop),
        instr!(9,9, "XXX", am::IMM, 1, nop),
        instr!(9,10, "XXX", am::IMM, 1, nop),
        instr!(9,11, "XXX", am::IMM, 1, nop),
        instr!(9,12, "XXX", am::IMM, 1, nop),
        instr!(9,13, "XXX", am::IMM, 1, nop),
        instr!(9,14, "XXX", am::IMM, 1, nop),
        instr!(9,15, "XXX", am::IMM, 1, nop),
    ]
}

#[inline]
pub fn row_10() -> [InstrCode; TABLE_COLS] {
    [
        instr!(10,0, "XXX", am::IMM, 1, nop), 
        instr!(10,1, "LDA", am::IZX, 6, lda::izx), 
        instr!(10,2, "XXX", am::IMM, 1, nop),
        instr!(10,3, "XXX", am::IMM, 1, nop),
        instr!(10,4, "XXX", am::IMM, 1, nop),
        instr!(10,5, "LDA", am::ZP0, 3, lda::zp0),
        instr!(10,6, "XXX", am::IMM, 1, nop),
        instr!(10,7, "XXX", am::IMM, 1, nop),
        instr!(10,8, "XXX", am::IMM, 1, nop),
        instr!(10,9, "LDA", am::IMM, 2, lda::imm),
        instr!(10,10, "XXX", am::IMM, 1, nop),
        instr!(10,11, "XXX", am::IMM, 1, nop),
        instr!(10,12, "XXX", am::IMM, 1, nop),
        instr!(10,13, "LDA", am::ABS, 4, lda::abs),
        instr!(10,14, "XXX", am::IMM, 1, nop),
        instr!(10,15, "XXX", am::IMM, 1, nop),
    ]
}

#[inline]
pub fn row_11() -> [InstrCode; TABLE_COLS] {
    [
        instr!(11,0, "BCS", am::REL, 4, bcs::rel), 
        instr!(11,1, "LDA", am::IZY, 6, lda::izy), 
        instr!(11,2, "XXX", am::IMM, 1, nop),
        instr!(11,3, "XXX", am::IMM, 1, nop),
        instr!(11,4, "XXX", am::IMM, 1, nop),
        instr!(11,5, "LDA", am::ZPX, 4, lda::zpx),
        instr!(11,6, "XXX", am::IMM, 1, nop),
        instr!(11,7, "XXX", am::IMM, 1, nop),
        instr!(11,8, "XXX", am::IMM, 1, nop),
        instr!(11,9, "LDA", am::ABY, 5, lda::aby),
        instr!(11,10, "XXX", am::IMM, 1, nop),
        instr!(11,11, "XXX", am::IMM, 1, nop),
        instr!(11,12, "XXX", am::IMM, 1, nop),
        instr!(11,13, "LDA", am::ABX, 5, lda::abx),
        instr!(11,14, "XXX", am::IMM, 1, nop),
        instr!(11,15, "XXX", am::IMM, 1, nop),
    ]
}

#[inline]
pub fn row_12() -> [InstrCode; TABLE_COLS] {
    [
        instr!(12,0, "XXX", am::IMM, 1, nop), 
        instr!(12,1, "XXX", am::IMM, 1, nop), 
        instr!(12,2, "XXX", am::IMM, 1, nop),
        instr!(12,3, "XXX", am::IMM, 1, nop),
        instr!(12,4, "XXX", am::IMM, 1, nop),
        instr!(12,5, "XXX", am::IMM, 1, nop),
        instr!(12,6, "XXX", am::IMM, 1, nop),
        instr!(12,7, "XXX", am::IMM, 1, nop),
        instr!(12,8, "XXX", am::IMM, 1, nop),
        instr!(12,9, "XXX", am::IMM, 1, nop),
        instr!(12,10, "XXX", am::IMM, 1, nop),
        instr!(12,11, "XXX", am::IMM, 1, nop),
        instr!(12,12, "XXX", am::IMM, 1, nop),
        instr!(12,13, "XXX", am::IMM, 1, nop),
        instr!(12,14, "XXX", am::IMM, 1, nop),
        instr!(12,15, "XXX", am::IMM, 1, nop),
    ]
}

#[inline]
pub fn row_13() -> [InstrCode; TABLE_COLS] {
    [
        instr!(13,0, "XXX", am::IMM, 1, nop), 
        instr!(13,1, "XXX", am::IMM, 1, nop), 
        instr!(13,2, "XXX", am::IMM, 1, nop),
        instr!(13,3, "XXX", am::IMM, 1, nop),
        instr!(13,4, "XXX", am::IMM, 1, nop),
        instr!(13,5, "XXX", am::IMM, 1, nop),
        instr!(13,6, "XXX", am::IMM, 1, nop),
        instr!(13,7, "XXX", am::IMM, 1, nop),
        instr!(13,8, "XXX", am::IMM, 1, nop),
        instr!(13,9, "XXX", am::IMM, 1, nop),
        instr!(13,10, "XXX", am::IMM, 1, nop),
        instr!(13,11, "XXX", am::IMM, 1, nop),
        instr!(13,12, "XXX", am::IMM, 1, nop),
        instr!(13,13, "XXX", am::IMM, 1, nop),
        instr!(13,14, "XXX", am::IMM, 1, nop),
        instr!(13,15, "XXX", am::IMM, 1, nop),
    ]
}

#[inline]
pub fn row_14() -> [InstrCode; TABLE_COLS] {
    [
        instr!(14,0, "XXX", am::IMM, 1, nop), 
        instr!(14,1, "XXX", am::IMM, 1, nop), 
        instr!(14,2, "XXX", am::IMM, 1, nop),
        instr!(14,3, "XXX", am::IMM, 1, nop),
        instr!(14,4, "XXX", am::IMM, 1, nop),
        instr!(14,5, "XXX", am::IMM, 1, nop),
        instr!(14,6, "XXX", am::IMM, 1, nop),
        instr!(14,7, "XXX", am::IMM, 1, nop),
        instr!(14,8, "XXX", am::IMM, 1, nop),
        instr!(14,9, "XXX", am::IMM, 1, nop),
        instr!(14,10, "XXX", am::IMM, 1, nop),
        instr!(14,11, "XXX", am::IMM, 1, nop),
        instr!(14,12, "XXX", am::IMM, 1, nop),
        instr!(14,13, "XXX", am::IMM, 1, nop),
        instr!(14,14, "XXX", am::IMM, 1, nop),
        instr!(14,15, "XXX", am::IMM, 1, nop),
    ]
}

#[inline]
pub fn row_15() -> [InstrCode; TABLE_COLS] {
    [
        instr!(15,0, "XXX", am::IMM, 1, nop), 
        instr!(15,1, "XXX", am::IMM, 1, nop), 
        instr!(15,2, "XXX", am::IMM, 1, nop),
        instr!(15,3, "XXX", am::IMM, 1, nop),
        instr!(15,4, "XXX", am::IMM, 1, nop),
        instr!(15,5, "XXX", am::IMM, 1, nop),
        instr!(15,6, "XXX", am::IMM, 1, nop),
        instr!(15,7, "XXX", am::IMM, 1, nop),
        instr!(15,8, "XXX", am::IMM, 1, nop),
        instr!(15,9, "XXX", am::IMM, 1, nop),
        instr!(15,10, "XXX", am::IMM, 1, nop),
        instr!(15,11, "XXX", am::IMM, 1, nop),
        instr!(15,12, "XXX", am::IMM, 1, nop),
        instr!(15,13, "XXX", am::IMM, 1, nop),
        instr!(15,14, "XXX", am::IMM, 1, nop),
        instr!(15,15, "XXX", am::IMM, 1, nop),
    ]
}
