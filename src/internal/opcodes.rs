use crate::cpu::Byte;


#[repr(u8)]
pub enum OpCode {
    Nop = 0x02,
    LdaZp0 = 0xA5,
    LdaImm = 0xA9,
    LdaAbs = 0xAD,
    LdaZpx = 0xB5,
    LdaAbx = 0xBD,
    LdaAby = 0xB9
}

impl std::fmt::Debug for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::Nop    => f.write_str("Nop"),
            OpCode::LdaZp0 => f.write_str("LdaZp0"),
            OpCode::LdaImm => f.write_str("LdaImm"),
            OpCode::LdaAbs => f.write_str("LdaAbs"),
            OpCode::LdaZpx => f.write_str("LdaZpx"),
            OpCode::LdaAbx => f.write_str("LdaAbs"),
            OpCode::LdaAby => f.write_str("LdaAby")
        }
    }
}

impl From<Byte> for OpCode {
    fn from(value: Byte) -> Self {
        match value {
        0x02 => OpCode::Nop,
        0xA5 => OpCode::LdaZp0,
        0xA9 => OpCode::LdaImm,
        0xAD => OpCode::LdaAbs,
        0xB5 => OpCode::LdaZpx,
        0xBD => OpCode::LdaAbs,
        0xB9 => OpCode::LdaAby,
        _ => panic!("Unimplemented Opcode: {value:#02x}")
        }

    }
}
