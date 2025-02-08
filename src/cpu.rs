use crate::{flags::{flag_debug, Flag::{self}, Flag6502, DEFAULT_STATUS}, internal::{opcodes::OpCode, Instructions}, log::{self, Log}, mem::Mem
};

pub type Byte = u8;
pub type SByte = i8;
pub type Word = u16;

pub const ST_ADDR: Word = 0x0100;
pub const ST_SIZE: Word = 0x00FF;
const SP_INIT: Byte = 0xFF;

pub const INERRUPT_VECTOR: Word = 0xFFFE;

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
    status: Byte,
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
            sp: SP_INIT,
            a: 0,
            x: 0,
            y: 0,
            status: DEFAULT_STATUS,
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
        self.sp = SP_INIT;
        //    all flags to zero
        self.status = DEFAULT_STATUS;
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

    #[inline]
    pub(crate) fn set_flag(&mut self, flag: Flag6502, v: u8) {
        
        if v > 0 {
            self.log.info(format!("Setting {:?} {:08b}", flag_debug(flag), self.status).as_str());
            self.status |= flag as Byte;
        } else {
            self.log.info(format!("Clearing {:?} {:08b}", flag_debug(flag), self.status).as_str());
            self.status &= !(flag as Byte)
        }
    }

    #[inline]
    pub(crate) fn get_flag(&self, flag: Flag6502) -> Byte {
        if (self.status & flag as u8) > 0 {1} else {0} 
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
    pub(crate) fn a_mut(&mut self) -> &mut Byte {
        &mut self.a
    }

    #[inline]
    pub(crate) fn x(&mut self) -> Byte {
        self.x
    }

    #[inline]
    pub(crate) fn y(&mut self) -> Byte {
        self.y
    }

    #[inline]
    pub(crate) fn pc(&self) -> Word {
        self.pc
    }
    
    #[inline]
    pub(crate) fn status(&self) -> Byte {
        self.status
    }

    #[inline]
    pub (crate) fn pc_mut(&mut self) -> &mut Word {
        &mut self.pc
    }
    
    pub fn execute(&mut self) {
        while self.instruction() {}
    }

    pub fn instruction(&mut self) -> bool {
        let opcode = self.mem[self.pc];
        self.log.info(format!("Read instruction '{:?}' from mem[{:#02x}]", OpCode::from(opcode), self.pc).as_str());

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

    /// Returns byte at pc and increases pc
    pub(crate) fn fetch_signed_byte(&mut self) -> SByte {
        let b = self.mem[self.pc];
        self.log.info(format!("Fetched byte {b:#02x} from mem[{:#02x}]", self.pc).as_str());
        self.pc +=1;
        self.clock();
        return b as i8;
    }
    pub(crate) fn write_byte(&mut self, addr: Word, b: Byte) {
        self.mem[addr] = b;
        self.clock();
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

    pub(crate) fn push_stack(&mut self, data: Byte) {
        let addr = ST_ADDR + self.sp as Word;
        self.mem[addr] = data;
        self.log.info(format!("Pushing {data:#02x} to stack at {:#02x}", self.sp).as_str());
        self.sp -= 1;
        self.clock();
    }

    pub(crate) fn pop_stack(&mut self) -> Byte {
        self.sp += 1;
        let addr = ST_ADDR + self.sp as Word;
        let data = self.mem[addr];
        self.clock();
        self.log.info(format!("Popping {data:#02x} from stack at {:#02x}", self.sp).as_str());
        data
    }

    #[cfg(test)]
    pub(crate) fn read_stack(&self) -> Byte {
        let addr = ST_ADDR + (self.sp + 0x1) as Word;
        self.mem[addr]
    }


    #[cfg(test)]
    pub(crate) fn byte_at(&self, at: Word) -> Byte {
        self.mem[at]
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
        status: Byte) {
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
        if self.status != status {
            panic!("[Six502.flags] assertion failed. Is {:08b}, but expected: {:08b}", self.status, status);
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
           f.write_fmt(format_args!("   C: {}\n", self.get_flag(Flag::C)))?;
           f.write_fmt(format_args!("   Z: {}\n", self.get_flag(Flag::Z)))?;
           f.write_fmt(format_args!("   I: {}\n", self.get_flag(Flag::I)))?;
           f.write_fmt(format_args!("   D: {}\n", self.get_flag(Flag::D)))?;
           f.write_fmt(format_args!("   B: {}\n", self.get_flag(Flag::B)))?;
           f.write_fmt(format_args!("   V: {}\n", self.get_flag(Flag::V)))?;
           f.write_fmt(format_args!("   N: {}\n", self.get_flag(Flag::N)))?;
           f.write_str("<<\n")
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::SP_INIT;

    use super::Six502;

    #[test]
    fn cpu_stack() {
        let mut cpu = Six502::new();
        assert_eq!(cpu.sp, SP_INIT);
        cpu.push_stack(0x01);
        assert_eq!(cpu.read_stack(), 0x01);
        cpu.push_stack(0x02);
        assert_eq!(cpu.pop_stack(), 0x02);
        assert_eq!(cpu.read_stack(), 0x01);
    }
}

