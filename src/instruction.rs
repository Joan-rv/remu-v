#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Beq { imm: i16, rs1: u8, rs2: u8 },
    Addi { rd: u8, rs1: u8, imm: u16 },
    Add { rd: u8, rs1: u8, rs2: u8 },
}
