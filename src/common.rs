use crate::{cpu::Word, flags::Flag::N};

pub fn eq_sign_bits(i: u8, j: u8) -> bool {
    return !((i ^ j) & N > 0) 
}

pub fn check_overflow_pre(i: u8, j: u8, pre: bool) -> bool {
    pre && ((i ^ j) & N) > 0
}

pub fn page_x_word(new: Word, old: Word) -> bool {
    (new >> 8) != (old >> 8)
}


#[cfg(test)]
mod test {
    use crate::common::eq_sign_bits;

    #[test]
    fn bit_eq_sign() {
        let negative:u8 = 0xF1; 
        let positive:u8 = 0x0A;
        assert!(!eq_sign_bits(negative, positive), "- +");
        assert!(eq_sign_bits(negative, negative), "- -");
        assert!(eq_sign_bits(positive, positive), "+ +");
    }
}
