use crate::{flags::{FlagIndex, Flags}, internal::{Instructions, opcodes::OpCode}, log::Log, mem::Mem
};

pub type Byte = u8;
pub type Word = u16;

#[cfg(test)]
pub enum Register {
    A,
    X,
    Y,
    SP
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
    instructions: Instructions,
    log: Log,

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
            instructions: Instructions::init(),
            log: Log::init(),
        }
    }

    pub fn load_to_pc(&mut self, data: &[u8]) {
        self.log.info(format!("Loading data to mem[{:#04x}]", self.pc).as_str());
        self.mem.load(self.pc as isize, data);
    }

    pub fn reset(&mut self) {
        self.log.info(format!("Running Reset Sequence").as_str());
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

    ///the decreases the cycle counter
    pub (crate) fn clock(&mut self) {
        self.cycles -= 1;
    }

    pub (crate) fn cycles_at(&self, c: i64) -> bool {
        self.cycles == c
    }
    pub(crate) fn cycles(&self) -> i64 {
        self.cycles
    }

    pub (crate) fn set_cycle(&mut self, c: i64) {
        self.cycles = c;
    }

    pub(crate) fn set_flag_value(&mut self, flag: FlagIndex, value: Byte) {
        match flag {
            FlagIndex::Z => self.flags.set_zero(value),
            FlagIndex::N => self.flags.set_neg(value),
            FlagIndex::C => self.flags.set_carry(value),
            FlagIndex::O => self.flags.set_overflow(value),
            _ => {
                panic!("Unhandled flag: set_flag_value(): {flag:?}")
            }
        }
    }

    
    
    pub(crate) fn carry_b(&self) -> u8 {
        self.flags.carry_b()
    }
    
    #[inline]
    pub(crate) fn set_a(&mut self, b: Byte) {
        self.log.info(format!("Setting register: A to {b:#02x}").as_str());
        self.a = b;
    }

    #[inline]
    pub(crate) fn load_a(&mut self, addr: Word) {
        self.log.info(format!("Loading from {addr:#02x} into register: A").as_str());
        self.a = self.read_byte(addr);
    }

    #[inline]
    pub(crate) fn a(&mut self) -> Byte {
        self.a
    }

    #[inline]
    pub(crate) fn x(&mut self) -> Byte {
        self.x
    }

    #[inline]
    pub(crate) fn y(&mut self) -> Byte {
        self.y
    }
    
    pub fn execute(&mut self) {
        while self.instruction() {}
    }

    pub fn instruction(&mut self) -> bool {
        let opcode = self.mem[self.pc];
        self.log.info(format!("Read instruciton '{:?}' from mem[{:#02x}]", OpCode::from(opcode), self.pc).as_str());

        self.pc += 1;
        let instr = &self.instructions[opcode];
        self.cycles = instr.cycles as i64;
        self.cycles -= 1;
        return (instr.exec_fn)(self)
    }

    /// Returns byte at pc and increases pc
    pub(crate) fn fetch_byte(&mut self) -> Byte {
        let b = self.mem[self.pc];
        self.log.info(format!("Fetched byte {b:#02x} from mem[{:#02x}]", self.pc).as_str());
        self.pc +=1;
        self.clock();
        return b;
    }
    //reads one byte at specified addr.
    pub(crate) fn read_byte(&mut self, addr: Word) -> Byte {
 
        let b = self.mem[addr];
        self.log.info(format!("Read byte {b:#02x} from mem[{addr:#02x}]").as_str());
        self.clock();
        return b;
    }

    //reads two bytes at PC, increases PC (2x)
    pub(crate) fn fetch_word(&mut self) -> Word {
        let mut word: Word = self.mem[self.pc] as Word;
        self.pc+=1;
        self.clock();
        word |= (self.mem[self.pc] as Word) << 8;
        self.pc+=1;
        self.clock();
        word
    }

    pub(crate) fn read_word(&mut self, addr: Word) -> Word {
        let lo = self.read_byte(addr) as Word;
        let hi = self.read_byte(addr+1) as Word;
        return lo | (hi << 8);
    }

    pub(crate) fn imm(&mut self) -> Byte {
        self.fetch_byte()
    }

    pub(crate) fn zp0(&mut self) -> Byte {
        self.fetch_byte()
    }

    pub(crate) fn zpx(&mut self) -> Word {
        self.clock();
        self.fetch_byte().wrapping_add(self.x()) as Word
    } 

    pub(crate) fn abs(&mut self) -> Word {
        self.fetch_word()
    }

    pub(crate) fn abx(&mut self, addr: &mut Word) -> bool {
        self.clock();
        let abs_addr = self.fetch_word();
        *addr = abs_addr.wrapping_add(self.x() as Word);
        (abs_addr ^ *addr) >> 8 > 0
    }


    pub(crate) fn aby(&mut self, addr: &mut Word) -> bool {
        let abs_addr = self.fetch_word();
        *addr = abs_addr.wrapping_add(self.y() as Word);
        (abs_addr ^ *addr) >> 8 > 0
    }

    pub(crate) fn izx(&mut self) -> Word {
        let addr = self.fetch_byte().wrapping_add(self.x());
        self.clock();
        self.read_word(addr as Word)
    }

    pub(crate) fn izy(&mut self, addr: &mut Word) -> bool {
        let i_addr = self.fetch_byte();
        let effective_addr = self.read_word(i_addr as Word);
        self.clock();
        *addr = effective_addr.wrapping_add(self.y() as Word);
        ((effective_addr ^ *addr) >> 8) > 0
    }



    #[cfg(test)]
    pub(crate) fn mem_section(&self, start: usize, len: usize) {
        let mut mem = vec![0u8; len];
        self.mem.get_mem_section(start, &mut mem);
        println!("mem at: {start}, {mem:?}");
    }

    #[cfg(test)]
    pub (crate) fn set_byte_at(&mut self, addr: Word, b: Byte) {
        self.mem[addr] = b;
    }
    #[cfg(test)]
    pub(crate) fn set_reg_byte(&mut self, reg: Register, v: Byte) {
        match reg {
            Register::A => self.a = v,
            Register::X => self.x = v,
            Register::Y => self.y = v,
            Register::SP => self.sp = v,
        }
    }

    #[cfg(test)]
    pub(crate) fn set_pc(&mut self, pc: Word) {
        self.pc = pc;
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



