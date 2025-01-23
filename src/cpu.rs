use core::fmt;
use std::{ops::Index};

use crate::{bitset, clearbit, instructions::{Instruction, Instructions}, setbit};

const MEM_CAP: usize = 1024*64;
pub type Byte = u8;
pub type Word = u16;



#[derive(Clone, Copy)]
#[repr(u8)]
pub(crate) enum FlagIndex {
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
pub(crate) struct Flags {
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
    fn new() -> Self {
        let inner = setbit!(0, FlagIndex::One);
        Self {inner}
    }

    #[cfg(test)]
    pub(crate) fn fix_state(set_bits: &[FlagIndex]) -> Self {
        let mut s = Self::new();
        for idx in set_bits.into_iter() {
            s.inner = setbit!(s.inner, *idx);
        }
        s
    }

    fn reset(&mut self) {
        self.inner = 0;
    }

    /// Sets Zero Flag if provided value = 0
    fn set_zero(&mut self, v: u8) {
        if v == 0 {
            self.inner = setbit!(self.inner, FlagIndex::Z)
        } else {
            self.inner = clearbit!(self.inner, FlagIndex::Z)
        }  
    }
    fn zero(&self) -> bool {
        bitset!(self.inner, FlagIndex::Z) != 0
    }
    /// Sets Zero Flag if bit at index 7 of provided v is set
    fn set_neg(&mut self, v: u8) {
        if bitset!(v, 7) != 0 {
            self.inner = setbit!(self.inner, FlagIndex::N)
        } else {
            self.inner = clearbit!(self.inner, FlagIndex::N)
        }  
    }
    fn neg(&self) -> bool {
        bitset!(self.inner, FlagIndex::N) != 0
    }
}

struct Mem {
    inner: [u8; MEM_CAP]
}

impl Mem {
    fn init() -> Self {
        Self {inner: [0; MEM_CAP]}
    }

    fn clear(&mut self) {
        //apparently this gets turned into memset call:
        //https://rust.godbolt.org/
        self.inner.iter_mut().for_each(|m| *m = 0);
    }

    pub fn load(&mut self, at: isize, data: &[u8]) {
        if MEM_CAP < (at as usize) + data.len() {
            panic!("Provided data is too big for cpu memory: {}kb", MEM_CAP/1024);
        } 
        unsafe {
            let ptr = self.inner.as_mut_ptr().offset(at);
            ptr.copy_from(data.as_ptr(), data.len());
        }
    }
}

impl Index<u8> for Mem {
    type Output=Byte;

    fn index(&self, index: u8) -> &Self::Output {
        &self.inner[index as usize]
    }
}

impl Index<u16> for Mem {
    type Output=Byte;

    fn index(&self, index: u16) -> &Self::Output {
        &self.inner[index as usize]
    }
}


pub struct Six502 {
    pc: Word,
    sp: Byte,
    a: Byte,
    x: Byte,
    y: Byte,
    flags: Flags,
    mem: Mem,
    //this will hold the current state of cycles
    cycles: i64,
    instructions: Instructions
}

impl Six502 {
    pub fn new() -> Self {
        Self {
            pc: 0xFFFC,
            sp: 0,
            a: 0,
            x: 0,
            y: 0,
            flags: Flags::new(),
            mem: Mem::init(),
            cycles: 0,
            instructions: Instructions::init()
        }
    }

    pub fn load_to_pc(&mut self, data: &[u8]) {
        self.mem.load(self.pc as isize, data);
    }

    pub fn reset(&mut self) {
        //for now:
        //    PC to 0xFFFC
        self.pc = 0xFFFC;
        //    SP to 0xFF
        self.sp = 0xFF;
        //    all flags to zero
        self.flags.reset();
        //    all registers to zero
        self.a = 0;
        self.x = 0;
        self.y = 0;
        //    memory to zero
        self.mem.clear();
        self.cycles = 0;
    }

    pub (crate) fn cycles_at(&self, c: i64) -> bool {
        self.cycles == c
    }

    pub(crate) fn set_flag_value(&mut self, flag: FlagIndex, value: Byte) {
        match flag {
            FlagIndex::Z => self.flags.set_zero(value),
            FlagIndex::N => self.flags.set_neg(value),
            _ => {
                panic!("Unhandled flag: set_flag_value(): {flag:?}")
            }
        }
    }
    
    #[inline]
    pub(crate) fn set_a(&mut self, b: Byte) {
        self.a = b;
        self.cycles -= 1;
    }

    #[inline]
    pub(crate) fn a(&mut self) -> Byte {
        self.a
    }
    
    ///This reads one byte, at PC and then increases the PC; 
    ///Then sets the `self.cycles` to instruction.cycles
    ///Decreasing its count by one;
    ///Finally, it returns a reference to the instruction 
    ///for evaluation and execution;
    fn op_code(&mut self) -> &Instruction {
        let opcode = self.mem[self.pc];
        self.pc += 1;
        let instruction = &self.instructions[opcode];
        self.cycles = instruction.cycles as i64;
        self.cycles -= 1;
        return instruction
    }

    pub fn execute(&mut self) -> bool {
        let opcode = self.mem[self.pc];
        self.pc += 1;
        let instr = &self.instructions[opcode];
        self.cycles = instr.cycles as i64;
        self.cycles -= 1;
        //TODO WHAT if illegal instruction?
        (instr.exec_fn)(self);
        return false;
    }

    /// Returns byte at pc and increases pc
    pub(crate) fn fetch_byte(&mut self) -> Byte {
        let b = self.mem[self.pc];
        self.pc += 1;
        return b;
    }

    #[cfg(test)]
    pub(crate) fn assert_state(&self, 
        pc: Word, 
        sp: Byte, 
        a: Byte, 
        x: Byte, 
        y: Byte, 
        cycles: i64,
        flags: Flags) {
        if self.pc != pc {
            panic!("[Six502.pc] assertion failed. Is {}, but expected: {}", self.pc, pc);
        };
        if self.sp != sp {
            panic!("[Six502.sp] assertion failed. Is {}, but expected: {}", self.sp, sp);

        };
        if self.a != a {
            panic!("[Six502.a] assertion failed. Is {}, but expected: {}", self.a, a);
        };
        if self.x != x {
            panic!("[Six502.x] assertion failed. Is {}, but expected: {}", self.x, x);
        };
        if self.y != y {
            panic!("[Six502.y] assertion failed. Is {}, but expected: {}", self.x, x);
        }
        if self.cycles != cycles {
            panic!("[Six502.cycles] assertion failed. Is {}, but expected: {}", self.cycles, cycles);
        }
        if self.flags != flags {
            panic!("[Six502.flags] assertion failed. Is {:?}, but expected: {:?}", self.flags, flags);
        }
    }
}

impl std::fmt::Debug for Six502 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(">>\n")?;
           f.write_fmt(format_args!("   pc: {}\n", self.pc))?;
           f.write_fmt(format_args!("   sp: {}\n", self.sp))?;
           f.write_fmt(format_args!("   a: {}\n", self.a))?;
           f.write_fmt(format_args!("   x: {}\n", self.x))?;
           f.write_fmt(format_args!("   y: {}\n", self.y))?;
           f.write_fmt(format_args!("   cycles: {}\n", self.cycles))?;
           f.write_fmt(format_args!("   flags: {:?}\n", self.flags))?;
           f.write_str("<<\n")
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
