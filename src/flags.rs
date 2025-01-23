use std::fmt;
use crate::{bitset, clearbit, cpu::Byte, setbit};


#[derive(Clone, Copy)]
#[repr(u8)]
pub enum FlagIndex {
    C, //Carry
    Z, //Zero
    I, //Interrut
    D, //Decimal
    B, //Break -> no cpu effect
    One, // no cpu effect, always pushed as 1
    O, //Overflow
    N, //Negative Flag
}


impl From<u8> for FlagIndex {
    fn from(value: u8) -> Self {
        match value {
            0 => FlagIndex::C,
            1 => FlagIndex::Z,
            2 => FlagIndex::I,
            3 => FlagIndex::D,
            4 => FlagIndex::B,
            5 => FlagIndex::One,
            6 => FlagIndex::O,
            7 => FlagIndex::N,
            _ => panic!("u8 out of FlagIndex Range")
        }
    }
}

impl Into<u8> for FlagIndex {
    fn into(self) -> u8 {
        self as u8
    }
}

impl std::fmt::Debug for FlagIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::C => write!(f, "C (0)"),
            Self::Z => write!(f, "Z (1)"),
            Self::I => write!(f, "I (2)"),
            Self::D => write!(f, "D (3)"),
            Self::B => write!(f, "B (4)"),
            Self::One => write!(f, "One (5)"),
            Self::O => write!(f, "O (6)"),
            Self::N => write!(f, "N (7)"),
        }
    }
}

#[derive(PartialEq)]
pub struct Flags {
    inner: Byte
}

impl fmt::Debug for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = &mut [0u8; 8];
        for i in 0..8 {
            if bitset!(self.inner, i) != 0 {
                data[i] = 1;
            }
        };
        data.reverse();
        write!(f, "{data:?} ({:#04x}|{})", self.inner, self.inner)
    }
}

impl Flags {
    pub fn new() -> Self {
        let inner = setbit!(0, FlagIndex::One);
        Self {inner}
    }

    #[cfg(test)]
    pub fn fix_state(set_bits: &[FlagIndex]) -> Self {
        let mut s = Self::new();
        for idx in set_bits.into_iter() {
            s.inner = setbit!(s.inner, *idx);
        }
        s
    }

    pub fn reset(&mut self) {
        self.inner = 0;
    }

    /// Sets Zero Flag if provided value = 0
    pub fn set_zero(&mut self, v: u8) {
        if v == 0 {
            self.inner = setbit!(self.inner, FlagIndex::Z)
        } else {
            self.inner = clearbit!(self.inner, FlagIndex::Z)
        }  
    }
    pub fn zero(&self) -> bool {
        bitset!(self.inner, FlagIndex::Z) != 0
    }
    /// Sets Zero Flag if bit at index 7 of provided v is set
    pub fn set_neg(&mut self, v: u8) {
        if bitset!(v, 7) != 0 {
            self.inner = setbit!(self.inner, FlagIndex::N)
        } else {
            self.inner = clearbit!(self.inner, FlagIndex::N)
        }  
    }
    pub fn neg(&self) -> bool {
        bitset!(self.inner, FlagIndex::N) != 0
    }
}

#[cfg(test)]
mod test {
    use super::Flags;

    #[test]
    fn set_z_flag() {
        let mut flag = Flags::new();
        flag.set_zero(0);
        assert!(flag.zero());
    }
    #[test]
    fn not_set_z_flag() {
        let mut flag = Flags::new();
        flag.set_zero(1);
        assert!(!flag.zero());
    }

    #[test]
    fn set_n_flag() {
        let mut flag = Flags::new();
        flag.set_neg(0xF3);
        assert!(flag.neg());
    }
    #[test]
    fn not_set_n_flag() {
        let mut flag = Flags::new();
        flag.set_neg(1);
        assert!(!flag.neg());
    }
}
