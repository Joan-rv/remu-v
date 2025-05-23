#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Beq { imm: i16, rs1: u8, rs2: u8 },
    Bne { imm: i16, rs1: u8, rs2: u8 },
    Blt { imm: i16, rs1: u8, rs2: u8 },
    Bge { imm: i16, rs1: u8, rs2: u8 },
    Bltu { imm: i16, rs1: u8, rs2: u8 },
    Bgeu { imm: i16, rs1: u8, rs2: u8 },
    Lb { rd: u8, rs1: u8, imm: i16 },
    Lh { rd: u8, rs1: u8, imm: i16 },
    Lw { rd: u8, rs1: u8, imm: i16 },
    Lbu { rd: u8, rs1: u8, imm: i16 },
    Lhu { rd: u8, rs1: u8, imm: i16 },
    Addi { rd: u8, rs1: u8, imm: u16 },
    Add { rd: u8, rs1: u8, rs2: u8 },
}
