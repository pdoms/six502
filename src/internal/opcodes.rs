use crate::cpu::Byte;


#[repr(u8)]
pub enum OpCode {
    Nop = 0x02,

    AdcIzx = 0x61,
    AdcZp0 = 0x65,
    AdcImm = 0x69,
    AdcAbs = 0x6D,
    AdcIzy = 0x71,
    AdcAby = 0x79,
    AdcZpx = 0x75,
    AdcAbx = 0x7D,

    AndIzx = 0x21,
    AndZp0 = 0x25,
    AndImm = 0x29,
    AndAbs = 0x2D,
    AndIzy = 0x31,
    AndAby = 0x39,
    AndZpx = 0x35,
    AndAbx = 0x3D,

    AslZp0 = 0x06,
    AslAcc = 0x0A,
    AslAbs = 0x0E,
    AslZpx = 0x16,
    AslAbx = 0x1E,

    BccRel = 0x90,
    BcsRel = 0xB0,

    BitZp0 = 0x24,
    BitAbs = 0x2C,

    BrkImp = 0x00,

    LdaIzx = 0xA1,
    LdaZp0 = 0xA5,
    LdaImm = 0xA9,
    LdaAbs = 0xAD,
    LdaIzy = 0xB1,
    LdaZpx = 0xB5,
    LdaAbx = 0xBD,
    LdaAby = 0xB9
}

impl std::fmt::Debug for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::Nop    => f.write_str("Nop"),

            OpCode::AdcIzx => f.write_str("AdcAIzx") ,
            OpCode::AdcZp0 => f.write_str("AdcZp0"),
            OpCode::AdcImm => f.write_str("AdcImm"),
            OpCode::AdcZpx => f.write_str("AdcZpx"),
            OpCode::AdcAbs => f.write_str("AdcAbs") ,
            OpCode::AdcIzy => f.write_str("AdcAIzy") ,
            OpCode::AdcAby => f.write_str("AdcAby") ,
            OpCode::AdcAbx => f.write_str("AdcAbx") ,

            OpCode::AndIzx => f.write_str("AndIzx") ,
            OpCode::AndZp0 => f.write_str("AndZp0"),
            OpCode::AndImm => f.write_str("AndImm"),
            OpCode::AndZpx => f.write_str("AndZpx"),
            OpCode::AndAbs => f.write_str("AndAbs") ,
            OpCode::AndIzy => f.write_str("AndIzy") ,
            OpCode::AndAby => f.write_str("AndAby") ,
            OpCode::AndAbx => f.write_str("AndAbx") ,

            OpCode::AslZp0 => f.write_str("AslZp0"),
            OpCode::AslAcc => f.write_str("AslAcc"),
            OpCode::AslAbs => f.write_str("AslAbs"),
            OpCode::AslZpx => f.write_str("AslZpx"),
            OpCode::AslAbx => f.write_str("AslAbx"),

            OpCode::BccRel => f.write_str("BccRel"),
            OpCode::BcsRel => f.write_str("BcsRel"),

            OpCode::BitZp0 => f.write_str("BitZp0"),
            OpCode::BitAbs => f.write_str("BitAbs"),

            OpCode::BrkImp => f.write_str("BrkImp"),

            OpCode::LdaAbs => f.write_str("LdaAbs"),
            OpCode::LdaAbx => f.write_str("LdaAbs"),
            OpCode::LdaAby => f.write_str("LdaAby"),
            OpCode::LdaImm => f.write_str("LdaImm"),
            OpCode::LdaIzx => f.write_str("LdaIzx"),
            OpCode::LdaIzy => f.write_str("LdaIzy"),
            OpCode::LdaZp0 => f.write_str("LdaZp0"),
            OpCode::LdaZpx => f.write_str("LdaZpx"),
        }
    }
}

impl From<Byte> for OpCode {
    fn from(value: Byte) -> Self {
        match value {
        0x02 => OpCode::Nop,

        0x61 => OpCode::AdcIzx,
        0x65 => OpCode::AdcZp0,
        0x69 => OpCode::AdcImm,
        0x6D => OpCode::AdcAbs,
        0x71 => OpCode::AdcIzy,
        0x79 => OpCode::AdcAby,
        0x7D => OpCode::AdcAbx,
        0x75 => OpCode::AdcZpx,

        0x21 => OpCode::AndIzx,
        0x25 => OpCode::AndZp0,
        0x29 => OpCode::AndImm,
        0x2D => OpCode::AndAbs,
        0x31 => OpCode::AndIzy,
        0x39 => OpCode::AndAby,
        0x3D => OpCode::AndAbx,
        0x35 => OpCode::AndZpx,

        0x06 => OpCode::AslZp0,
        0x0A => OpCode::AslAcc,
        0x0E => OpCode::AslAbs,
        0x16 => OpCode::AslZpx,
        0x1E => OpCode::AslAbx,

        0x90 => OpCode::BccRel,
        0xB0 => OpCode::BcsRel,

        0x24 => OpCode::BitZp0,
        0x2C => OpCode::BitAbs,

        0x00 => OpCode::BrkImp,

        0xA1 => OpCode::LdaIzx,
        0xA5 => OpCode::LdaZp0,
        0xA9 => OpCode::LdaImm,
        0xAD => OpCode::LdaAbs,
        0xB1 => OpCode::LdaIzy,
        0xB5 => OpCode::LdaZpx,
        0xBD => OpCode::LdaAbs,
        0xB9 => OpCode::LdaAby,
        _ => panic!("Unimplemented Opcode: {value:#02x}")
        }
    }
}

impl Into<u8> for OpCode {
    fn into(self) -> u8 {
        match self {
            OpCode::Nop    => 0x02,

            OpCode::AndIzx => 0x21,
            OpCode::AndZp0 => 0x25,
            OpCode::AndImm => 0x29,
            OpCode::AndAbs => 0x2D,
            OpCode::AndIzy => 0x31,
            OpCode::AndAby => 0x39,
            OpCode::AndZpx => 0x35,
            OpCode::AndAbx => 0x3D,
            
            OpCode::AdcIzx => 0x61,
            OpCode::AdcZp0 => 0x65,
            OpCode::AdcImm => 0x69,
            OpCode::AdcAbs => 0x6D,
            OpCode::AdcIzy => 0x71,
            OpCode::AdcAby => 0x79,
            OpCode::AdcZpx => 0x75,
            OpCode::AdcAbx => 0x7D,

            OpCode::AslZp0 => 0x06,
            OpCode::AslAcc => 0x0A,
            OpCode::AslAbs => 0x0E,
            OpCode::AslZpx => 0x16,
            OpCode::AslAbx => 0x1E,

            OpCode::BccRel => 0x90,
            OpCode::BcsRel => 0xB0,

            OpCode::BitZp0 => 0x24,
            OpCode::BitAbs => 0x2C,

            OpCode::BrkImp => 0x00,

            OpCode::LdaIzx => 0xA1,
            OpCode::LdaZp0 => 0xA5,
            OpCode::LdaImm => 0xA9,
            OpCode::LdaAbs => 0xAD,
            OpCode::LdaIzy => 0xB1,
            OpCode::LdaZpx => 0xB5,
            OpCode::LdaAbx => 0xBD,
            OpCode::LdaAby => 0xB9
        }
    }
}
