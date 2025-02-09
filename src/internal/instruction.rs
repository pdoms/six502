use crate::{cpu::Six502, data::DataBus};

use super::modes::AddressingMode;

pub struct InstrCode<D: DataBus> {
    pub mnemonic: &'static str,
    pub opcode: u8,
    pub mode: AddressingMode,
    pub cycles: u8,
    pub exec_fn: fn(&mut Six502<D>) -> bool 
}



///Takes: row (num), col (num), name (str literal), AddressingMode, cycles (num), and a function
///pointer
#[macro_export]
macro_rules! instr {
    ($r:expr, $c:expr, $n:expr, $m:expr, $cy:expr, $f:expr) => {
        {
            super::InstrCode {
                mnemonic: $n,
                opcode: $r << 4 | $c,
                mode: $m,
                cycles: $cy,
                exec_fn: $f
            }
        }
    };
}







