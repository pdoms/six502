use crate::{instr, instructions::{Instruction, TABLE_COLS}};



pub fn row_0() -> [Instruction; TABLE_COLS] {
    use crate::executors::*;
    use crate::instructions::AddressingMode::*;
    [
        instr!(0,0, "XXX", IMM, 1, nop), 
        instr!(0,1, "XXX", IMM, 1, nop),
        instr!(0,2, "XXX", IMM, 1, nop),
        instr!(0,3, "XXX", IMM, 1, nop),
        instr!(0,4, "XXX", IMM, 1, nop),
        instr!(0,5, "XXX", IMM, 1, nop),
        instr!(0,6, "XXX", IMM, 1, nop),
        instr!(0,7, "XXX", IMM, 1, nop),
        instr!(0,8, "XXX", IMM, 1, nop),
        instr!(0,9, "XXX", IMM, 1, nop),
        instr!(0,10, "XXX", IMM, 1, nop),
        instr!(0,11, "XXX", IMM, 1, nop),
        instr!(0,12, "XXX", IMM, 1, nop),
        instr!(0,13, "XXX", IMM, 1, nop),
        instr!(0,14, "XXX", IMM, 1, nop),
        instr!(0,15, "XXX", IMM, 1, nop),
    ]
}
pub fn row_1() -> [Instruction; TABLE_COLS] {
    use crate::executors::*;
    use crate::instructions::AddressingMode::*;
    [
        instr!(1,0, "XXX", IMM, 1, nop), 
        instr!(1,1, "XXX", IMM, 5, nop), 
        instr!(1,2, "XXX", IMM, 1, nop),
        instr!(1,3, "XXX", IMM, 1, nop),
        instr!(1,4, "XXX", IMM, 1, nop),
        instr!(1,5, "XXX", IMM, 1, nop),
        instr!(1,6, "XXX", IMM, 1, nop),
        instr!(1,7, "XXX", IMM, 1, nop),
        instr!(1,8, "XXX", IMM, 1, nop),
        instr!(1,9, "XXX", IMM, 1, nop),
        instr!(1,10, "XXX", IMM, 1, nop),
        instr!(1,11, "XXX", IMM, 1, nop),
        instr!(1,12, "XXX", IMM, 1, nop),
        instr!(1,13, "XXX", IMM, 1, nop),
        instr!(1,14, "XXX", IMM, 1, nop),
        instr!(1,15, "XXX", IMM, 1, nop),
    ]
}

pub fn row_2() -> [Instruction; TABLE_COLS] {
    use crate::executors::*;
    use crate::instructions::AddressingMode::*;
    [
        instr!(2,0, "XXX", IMM, 1, nop), 
        instr!(2,1, "AND", IZX, 6, nop), 
        instr!(2,2, "XXX", IMM, 1, nop),
        instr!(2,3, "XXX", IMM, 1, nop),
        instr!(2,4, "XXX", IMM, 1, nop),
        instr!(2,5, "XXX", IMM, 1, nop),
        instr!(2,6, "XXX", IMM, 1, nop),
        instr!(2,7, "XXX", IMM, 1, nop),
        instr!(2,8, "XXX", IMM, 1, nop),
        instr!(2,9, "XXX", IMM, 1, nop),
        instr!(2,10, "XXX", IMM, 1, nop),
        instr!(2,11, "XXX", IMM, 1, nop),
        instr!(2,12, "XXX", IMM, 1, nop),
        instr!(2,13, "XXX", IMM, 1, nop),
        instr!(2,14, "XXX", IMM, 1, nop),
        instr!(2,15, "XXX", IMM, 1, nop),
    ]
}

pub fn row_3() -> [Instruction; TABLE_COLS] {
    use crate::executors::*;
    use crate::instructions::AddressingMode::*;
    [
        instr!(3,0, "XXX", IMM, 1, nop), 
        instr!(3,1, "XXX", IMM, 1, nop), 
        instr!(3,2, "XXX", IMM, 1, nop),
        instr!(3,3, "XXX", IMM, 1, nop),
        instr!(3,4, "XXX", IMM, 1, nop),
        instr!(3,5, "XXX", IMM, 1, nop),
        instr!(3,6, "XXX", IMM, 1, nop),
        instr!(3,7, "XXX", IMM, 1, nop),
        instr!(3,8, "XXX", IMM, 1, nop),
        instr!(3,9, "XXX", IMM, 1, nop),
        instr!(3,10, "XXX", IMM, 1, nop),
        instr!(3,11, "XXX", IMM, 1, nop),
        instr!(3,12, "XXX", IMM, 1, nop),
        instr!(3,13, "XXX", IMM, 1, nop),
        instr!(3,14, "XXX", IMM, 1, nop),
        instr!(3,15, "XXX", IMM, 1, nop),
    ]
}


pub fn row_4() -> [Instruction; TABLE_COLS] {
    use crate::executors::*;
    use crate::instructions::AddressingMode::*;
    [
        instr!(4,0, "XXX", IMM, 1, nop), 
        instr!(4,1, "XXX", IMM, 1, nop), 
        instr!(4,2, "XXX", IMM, 1, nop),
        instr!(4,3, "XXX", IMM, 1, nop),
        instr!(4,4, "XXX", IMM, 1, nop),
        instr!(4,5, "XXX", IMM, 1, nop),
        instr!(4,6, "XXX", IMM, 1, nop),
        instr!(4,7, "XXX", IMM, 1, nop),
        instr!(4,8, "XXX", IMM, 1, nop),
        instr!(4,9, "XXX", IMM, 1, nop),
        instr!(4,10, "XXX", IMM, 1, nop),
        instr!(4,11, "XXX", IMM, 1, nop),
        instr!(4,12, "XXX", IMM, 1, nop),
        instr!(4,13, "XXX", IMM, 1, nop),
        instr!(4,14, "XXX", IMM, 1, nop),
        instr!(4,15, "XXX", IMM, 1, nop),
    ]
}


pub fn row_5() -> [Instruction; TABLE_COLS] {
    use crate::executors::*;
    use crate::instructions::AddressingMode::*;
    [
        instr!(5,0, "XXX", IMM, 1, nop), 
        instr!(5,1, "XXX", IMM, 1, nop), 
        instr!(5,2, "XXX", IMM, 1, nop),
        instr!(5,3, "XXX", IMM, 1, nop),
        instr!(5,4, "XXX", IMM, 1, nop),
        instr!(5,5, "XXX", IMM, 1, nop),
        instr!(5,6, "XXX", IMM, 1, nop),
        instr!(5,7, "XXX", IMM, 1, nop),
        instr!(5,8, "XXX", IMM, 1, nop),
        instr!(5,9, "XXX", IMM, 1, nop),
        instr!(5,10, "XXX", IMM, 1, nop),
        instr!(5,11, "XXX", IMM, 1, nop),
        instr!(5,12, "XXX", IMM, 1, nop),
        instr!(5,13, "XXX", IMM, 1, nop),
        instr!(5,14, "XXX", IMM, 1, nop),
        instr!(5,15, "XXX", IMM, 1, nop),
    ]
}

pub fn row_6() -> [Instruction; TABLE_COLS] {
    use crate::executors::*;
    use crate::instructions::AddressingMode::*;
    [
        instr!(6,0, "XXX", IMM, 1, nop), 
        instr!(6,1, "XXX", IMM, 1, nop), 
        instr!(6,2, "XXX", IMM, 1, nop),
        instr!(6,3, "XXX", IMM, 1, nop),
        instr!(6,4, "XXX", IMM, 1, nop),
        instr!(6,5, "XXX", IMM, 1, nop),
        instr!(6,6, "XXX", IMM, 1, nop),
        instr!(6,7, "XXX", IMM, 1, nop),
        instr!(6,8, "XXX", IMM, 1, nop),
        instr!(6,9, "XXX", IMM, 1, nop),
        instr!(6,10, "XXX", IMM, 1, nop),
        instr!(6,11, "XXX", IMM, 1, nop),
        instr!(6,12, "XXX", IMM, 1, nop),
        instr!(6,13, "XXX", IMM, 1, nop),
        instr!(6,14, "XXX", IMM, 1, nop),
        instr!(6,15, "XXX", IMM, 1, nop),
    ]
}

pub fn row_7() -> [Instruction; TABLE_COLS] {
    use crate::executors::*;
    use crate::instructions::AddressingMode::*;
    [
        instr!(7,0, "XXX", IMM, 1, nop), 
        instr!(7,1, "XXX", IMM, 1, nop), 
        instr!(7,2, "XXX", IMM, 1, nop),
        instr!(7,3, "XXX", IMM, 1, nop),
        instr!(7,4, "XXX", IMM, 1, nop),
        instr!(7,5, "XXX", IMM, 1, nop),
        instr!(7,6, "XXX", IMM, 1, nop),
        instr!(7,7, "XXX", IMM, 1, nop),
        instr!(7,8, "XXX", IMM, 1, nop),
        instr!(7,9, "XXX", IMM, 1, nop),
        instr!(7,10, "XXX", IMM, 1, nop),
        instr!(7,11, "XXX", IMM, 1, nop),
        instr!(7,12, "XXX", IMM, 1, nop),
        instr!(7,13, "XXX", IMM, 1, nop),
        instr!(7,14, "XXX", IMM, 1, nop),
        instr!(7,15, "XXX", IMM, 1, nop),
    ]
}

pub fn row_8() -> [Instruction; TABLE_COLS] {
    use crate::executors::*;
    use crate::instructions::AddressingMode::*;
    [
        instr!(8,0, "XXX", IMM, 1, nop), 
        instr!(8,1, "XXX", IMM, 1, nop), 
        instr!(8,2, "XXX", IMM, 1, nop),
        instr!(8,3, "XXX", IMM, 1, nop),
        instr!(8,4, "XXX", IMM, 1, nop),
        instr!(8,5, "XXX", IMM, 1, nop),
        instr!(8,6, "XXX", IMM, 1, nop),
        instr!(8,7, "XXX", IMM, 1, nop),
        instr!(8,8, "XXX", IMM, 1, nop),
        instr!(8,9, "XXX", IMM, 1, nop),
        instr!(8,10, "XXX", IMM, 1, nop),
        instr!(8,11, "XXX", IMM, 1, nop),
        instr!(8,12, "XXX", IMM, 1, nop),
        instr!(8,13, "XXX", IMM, 1, nop),
        instr!(8,14, "XXX", IMM, 1, nop),
        instr!(8,15, "XXX", IMM, 1, nop),
    ]
}

pub fn row_9() -> [Instruction; TABLE_COLS] {
    use crate::executors::*;
    use crate::instructions::AddressingMode::*;
    [
        instr!(9,0, "XXX", IMP, 1, nop), 
        instr!(9,1, "XXX", IMP, 1, nop), 
        instr!(9,2, "XXX", IMM, 1, nop),
        instr!(9,3, "XXX", IMM, 1, nop),
        instr!(9,4, "XXX", IMM, 1, nop),
        instr!(9,5, "XXX", IMM, 1, nop),
        instr!(9,6, "XXX", IMM, 1, nop),
        instr!(9,7, "XXX", IMM, 1, nop),
        instr!(9,8, "XXX", IMM, 1, nop),
        instr!(9,9, "XXX", IMM, 1, nop),
        instr!(9,10, "XXX", IMM, 1, nop),
        instr!(9,11, "XXX", IMM, 1, nop),
        instr!(9,12, "XXX", IMM, 1, nop),
        instr!(9,13, "XXX", IMM, 1, nop),
        instr!(9,14, "XXX", IMM, 1, nop),
        instr!(9,15, "XXX", IMM, 1, nop),
    ]
}

pub fn row_10() -> [Instruction; TABLE_COLS] {
    use crate::executors::*;
    use crate::instructions::AddressingMode::*;
    [
        instr!(10,0, "XXX", IMM, 1, nop), 
        instr!(10,1, "XXX", IMM, 1, nop), 
        instr!(10,2, "XXX", IMM, 1, nop),
        instr!(10,3, "XXX", IMM, 1, nop),
        instr!(10,4, "XXX", IMM, 1, nop),
        instr!(10,5, "XXX", IMM, 1, nop),
        instr!(10,6, "XXX", IMM, 1, nop),
        instr!(10,7, "XXX", IMM, 1, nop),
        instr!(10,8, "XXX", IMM, 1, nop),
        instr!(10,9, "LDA", IMM, 2, lda_imm),
        instr!(10,10, "XXX", IMM, 1, nop),
        instr!(10,11, "XXX", IMM, 1, nop),
        instr!(10,12, "XXX", IMM, 1, nop),
        instr!(10,13, "XXX", IMM, 1, nop),
        instr!(10,14, "XXX", IMM, 1, nop),
        instr!(10,15, "XXX", IMM, 1, nop),
    ]
}

pub fn row_11() -> [Instruction; TABLE_COLS] {
    use crate::executors::*;
    use crate::instructions::AddressingMode::*;
    [
        instr!(11,0, "XXX", IMM, 1, nop), 
        instr!(11,1, "XXX", IMM, 1, nop), 
        instr!(11,2, "XXX", IMM, 1, nop),
        instr!(11,3, "XXX", IMM, 1, nop),
        instr!(11,4, "XXX", IMM, 1, nop),
        instr!(11,5, "XXX", IMM, 1, nop),
        instr!(11,6, "XXX", IMM, 1, nop),
        instr!(11,7, "XXX", IMM, 1, nop),
        instr!(11,8, "XXX", IMM, 1, nop),
        instr!(11,9, "XXX", IMM, 1, nop),
        instr!(11,10, "XXX", IMM, 1, nop),
        instr!(11,11, "XXX", IMM, 1, nop),
        instr!(11,12, "XXX", IMM, 1, nop),
        instr!(11,13, "XXX", IMM, 1, nop),
        instr!(11,14, "XXX", IMM, 1, nop),
        instr!(11,15, "XXX", IMM, 1, nop),
    ]
}

pub fn row_12() -> [Instruction; TABLE_COLS] {
    use crate::executors::*;
    use crate::instructions::AddressingMode::*;
    [
        instr!(12,0, "XXX", IMM, 1, nop), 
        instr!(12,1, "XXX", IMM, 1, nop), 
        instr!(12,2, "XXX", IMM, 1, nop),
        instr!(12,3, "XXX", IMM, 1, nop),
        instr!(12,4, "XXX", IMM, 1, nop),
        instr!(12,5, "XXX", IMM, 1, nop),
        instr!(12,6, "XXX", IMM, 1, nop),
        instr!(12,7, "XXX", IMM, 1, nop),
        instr!(12,8, "XXX", IMM, 1, nop),
        instr!(12,9, "XXX", IMM, 1, nop),
        instr!(12,10, "XXX", IMM, 1, nop),
        instr!(12,11, "XXX", IMM, 1, nop),
        instr!(12,12, "XXX", IMM, 1, nop),
        instr!(12,13, "XXX", IMM, 1, nop),
        instr!(12,14, "XXX", IMM, 1, nop),
        instr!(12,15, "XXX", IMM, 1, nop),
    ]
}


pub fn row_13() -> [Instruction; TABLE_COLS] {
    use crate::executors::*;
    use crate::instructions::AddressingMode::*;
    [
        instr!(13,0, "XXX", IMM, 1, nop), 
        instr!(13,1, "XXX", IMM, 1, nop), 
        instr!(13,2, "XXX", IMM, 1, nop),
        instr!(13,3, "XXX", IMM, 1, nop),
        instr!(13,4, "XXX", IMM, 1, nop),
        instr!(13,5, "XXX", IMM, 1, nop),
        instr!(13,6, "XXX", IMM, 1, nop),
        instr!(13,7, "XXX", IMM, 1, nop),
        instr!(13,8, "XXX", IMM, 1, nop),
        instr!(13,9, "XXX", IMM, 1, nop),
        instr!(13,10, "XXX", IMM, 1, nop),
        instr!(13,11, "XXX", IMM, 1, nop),
        instr!(13,12, "XXX", IMM, 1, nop),
        instr!(13,13, "XXX", IMM, 1, nop),
        instr!(13,14, "XXX", IMM, 1, nop),
        instr!(13,15, "XXX", IMM, 1, nop),
    ]
}


pub fn row_14() -> [Instruction; TABLE_COLS] {
    use crate::executors::*;
    use crate::instructions::AddressingMode::*;
    [
        instr!(14,0, "XXX", IMM, 1, nop), 
        instr!(14,1, "XXX", IMM, 1, nop), 
        instr!(14,2, "XXX", IMM, 1, nop),
        instr!(14,3, "XXX", IMM, 1, nop),
        instr!(14,4, "XXX", IMM, 1, nop),
        instr!(14,5, "XXX", IMM, 1, nop),
        instr!(14,6, "XXX", IMM, 1, nop),
        instr!(14,7, "XXX", IMM, 1, nop),
        instr!(14,8, "XXX", IMM, 1, nop),
        instr!(14,9, "XXX", IMM, 1, nop),
        instr!(14,10, "XXX", IMM, 1, nop),
        instr!(14,11, "XXX", IMM, 1, nop),
        instr!(14,12, "XXX", IMM, 1, nop),
        instr!(14,13, "XXX", IMM, 1, nop),
        instr!(14,14, "XXX", IMM, 1, nop),
        instr!(14,15, "XXX", IMM, 1, nop),
    ]
}

pub fn row_15() -> [Instruction; TABLE_COLS] {
    use crate::executors::*;
    use crate::instructions::AddressingMode::*;
    [
        instr!(15,0, "XXX", IMM, 1, nop), 
        instr!(15,1, "XXX", IMM, 1, nop), 
        instr!(15,2, "XXX", IMM, 1, nop),
        instr!(15,3, "XXX", IMM, 1, nop),
        instr!(15,4, "XXX", IMM, 1, nop),
        instr!(15,5, "XXX", IMM, 1, nop),
        instr!(15,6, "XXX", IMM, 1, nop),
        instr!(15,7, "XXX", IMM, 1, nop),
        instr!(15,8, "XXX", IMM, 1, nop),
        instr!(15,9, "XXX", IMM, 1, nop),
        instr!(15,10, "XXX", IMM, 1, nop),
        instr!(15,11, "XXX", IMM, 1, nop),
        instr!(15,12, "XXX", IMM, 1, nop),
        instr!(15,13, "XXX", IMM, 1, nop),
        instr!(15,14, "XXX", IMM, 1, nop),
        instr!(15,15, "XXX", IMM, 1, nop),
    ]
}
