use crate::{flags::{FlagIndex, Flags}, 
    internal::{instruction, Instructions}, 
    mem::Mem,
};

pub type Byte = u8;
pub type Word = u16;

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
   // fn op_code(&mut self) -> &InstrCode {
   //     let opcode = self.mem[self.pc];
   //     self.pc += 1;
   //     let instruction = &self.instructions[opcode];
   //     self.cycles = instruction.cycles as i64;
   //     self.cycles -= 1;
   //     return instruction
   // }
   //
    pub fn execute(&mut self) {
        while self.instruction() {}
    }

    pub fn instruction(&mut self) -> bool {
        let opcode = self.mem[self.pc];
        self.pc += 1;
        let instr = &self.instructions[opcode];
        self.cycles = instr.cycles as i64;
        self.cycles -= 1;
        return (instr.exec_fn)(self)
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



