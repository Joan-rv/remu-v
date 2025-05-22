#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Add { rd: u8, rs1: u8, rs2: u8 },
}
