#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Add { rd: u8, rs1: u8, rs2: u8 },
    Addi { rd: u8, rs1: u8, imm: u16 },
}
