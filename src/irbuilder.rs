#[repr(u16)]
#[derive(Debug, Copy, Clone)]
pub enum OpCode {
    NOP = 0x0000,
    imm,
    imm_u64,
    add,
    sub,
    mov,
    extend,
    memcpy,
}

#[derive(Debug, Clone)]
pub enum CodeBody {
    V1,
    V2(String, String),
    V3(String, String, String),
}

#[derive(Debug, Clone)]
pub struct HighCode(pub OpCode, pub CodeBody);
// fn parseFrom
