use crate::{cpu::Word, flags::FlagBits};



pub fn eq_sign_bits(i: u8, j: u8) -> bool {
    return !((i ^ j) & (FlagBits::N as u8) > 0) 
}

pub fn check_overflow_pre(i: u8, j: u8, pre: bool) -> bool {
    pre && ((i ^ j) & (FlagBits::N as u8)) > 0
}

pub fn page_x_word(new: Word, old: Word) -> bool {
    (new >> 8) != (old >> 8)
}




#[macro_export]
macro_rules! setbit{
    ($num:expr,$b:expr)=>{
        $num | (1 << ($b as u8))
    };
    ($num:expr,$b:expr,$t:ty)=>{
        ($num as $t) | ((1 as $t) << ($b as $t))
    };
}

#[macro_export]
macro_rules! clearbit{
    ($num:expr,$b:expr)=>{
        $num & !(1 << ($b as u8))
    };
    ($num:expr,$b:expr,$t:ty)=>{
        ($num as $t) & ~((1 as $t) << ($b  as $t))
    };
}

#[macro_export]
macro_rules! bitset{
    ($num:expr,$b:expr)=>{
        ($num >> ($b as u8) & 1)
    };
}

#[cfg(test)]
mod test {
    use crate::{common::eq_sign_bits, flags::{FlagBits, FlagIndex}};


    #[test]
    fn bit_negative() {
        //set
        let mut flag = 0;
        flag = setbit!(flag, FlagIndex::N as u8);
        assert_eq!(flag, FlagBits::N as u8);
        //set again, should not change
        flag = setbit!(flag, FlagIndex::N);
        assert_eq!(bitset!(flag, FlagIndex::N), 1);
        //clear
        flag = clearbit!(flag, FlagIndex::N as u8);
        assert_eq!(bitset!(flag, FlagIndex::N), 0);
    }

    #[test]
    fn bit_overflow() {
        //set
        let mut flag = 0;
        flag = setbit!(flag, FlagIndex::O as u8);
        assert_eq!(flag, FlagBits::O as u8);
        //set again, should not change
        flag = setbit!(flag, FlagIndex::O);
        assert_eq!(bitset!(flag, FlagIndex::O), 1);
        //clear
        flag = clearbit!(flag, FlagIndex::O as u8);
        assert_eq!(bitset!(flag, FlagIndex::O), 0);
    }

    #[test]
    fn bit_one() {
        //set
        let mut flag = 0;
        flag = setbit!(flag, FlagIndex::U as u8);
        assert_eq!(flag, FlagBits::U as u8);
        //set again, should not change
        flag = setbit!(flag, FlagIndex::U);
        assert_eq!(bitset!(flag, FlagIndex::U), 1);
        //clear
        flag = clearbit!(flag, FlagIndex::U as u8);
        assert_eq!(bitset!(flag, FlagIndex::U), 0);
    }

    #[test]
    fn bit_break() {
        //set
        let mut flag = 0;
        flag = setbit!(flag, FlagIndex::B as u8);
        assert_eq!(flag, FlagBits::B as u8);
        //set again, should not change
        flag = setbit!(flag, FlagIndex::B);
        assert_eq!(bitset!(flag, FlagIndex::B), 1);
        //clear
        flag = clearbit!(flag, FlagIndex::B as u8);
        assert_eq!(bitset!(flag, FlagIndex::B), 0);
    }

    #[test]
    fn bit_dec() {
        //set
        let mut flag = 0;
        flag = setbit!(flag, FlagIndex::D as u8);
        assert_eq!(flag, FlagBits::D as u8);
        //set again, should not change
        flag = setbit!(flag, FlagIndex::D);
        assert_eq!(bitset!(flag, FlagIndex::D), 1);
        //clear
        flag = clearbit!(flag, FlagIndex::D as u8);
        assert_eq!(bitset!(flag, FlagIndex::D), 0);
    }

    #[test]
    fn bit_interupt() {
        //set
        let mut flag = 0;
        flag = setbit!(flag, FlagIndex::I as u8);
        assert_eq!(flag, FlagBits::I as u8);
        //set again, should not change
        flag = setbit!(flag, FlagIndex::I);
        assert_eq!(bitset!(flag, FlagIndex::I), 1);
        //clear
        flag = clearbit!(flag, FlagIndex::I as u8);
        assert_eq!(bitset!(flag, FlagIndex::I), 0);
    }

    #[test]
    fn bit_zero() {
        //set
        let mut flag = 0;
        flag = setbit!(flag, FlagIndex::Z as u8);
        assert_eq!(flag, FlagBits::Z as u8);
        //set again, should not change
        flag = setbit!(flag, FlagIndex::Z);
        assert_eq!(bitset!(flag, FlagIndex::Z), 1);
        //clear
        flag = clearbit!(flag, FlagIndex::Z as u8);
        assert_eq!(bitset!(flag, FlagIndex::Z), 0);
    }

    #[test]
    fn bit_carry() {
        //set
        let mut flag = 0;
        flag = setbit!(flag, FlagIndex::C as u8);
        assert_eq!(flag, FlagBits::C as u8);
        //set again, should not change
        flag = setbit!(flag, FlagIndex::C);
        assert_eq!(bitset!(flag, FlagIndex::C), 1);
        //clear
        flag = clearbit!(flag, FlagIndex::C as u8);
        assert_eq!(bitset!(flag, FlagIndex::C), 0);
    }

    #[test]
    fn bit_eq_sign() {
        let negative:u8 = 0xF1; 
        let positive:u8 = 0x0A;
        assert!(!eq_sign_bits(negative, positive), "- +");
        assert!(eq_sign_bits(negative, negative), "- -");
        assert!(eq_sign_bits(positive, positive), "+ +");
    }
}
