
#[repr(u8)]
pub enum OpCode {
    Nop = 0x02,
    LdaZp0 = 0xA5,
    LdaImm = 0xA9
}
