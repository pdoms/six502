#[cfg(test)]
use crate::cpu::Byte;

pub type Flag6502 = u8;
pub const DEFAULT_STATUS: u8 = 0x20;

pub mod Flag {
    pub const C: u8 = 1 << 0;   // Carry                                 
    pub const Z: u8 = 1 << 1;   // Zero
    pub const I: u8 = 1 << 2;   // Interrut
    pub const D: u8 = 1 << 3;   // Decimal
    pub const B: u8 = 1 << 4;   // Break -> no cpu effect
    pub const U: u8 = 1 << 5;   // no cpu effect, always pushed as 1
    pub const V: u8 = 1 << 6;   // Overflow
    pub const N: u8 = 1 << 7;   // Negative Flag                        
}

pub fn flag_debug(f: Flag6502) -> String {
    match f {
        Flag::C => String::from("C"),
        Flag::Z => String::from("Z"),
        Flag::I => String::from("I"),
        Flag::D => String::from("D"),
        Flag::B => String::from("B"),
        Flag::U => String::from("U"),
        Flag::V => String::from("V"),
        Flag::N => String::from("N"),
        _ => unreachable!()
    }
}

#[cfg(test)]
pub(crate) fn set_flag(status: &mut Byte, flag: Flag6502, v: u8) {
    if v > 0 {
        *status |= flag as Byte 
    } else {
        *status &= !(flag as Byte)
    }
}

