#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Add { rd: u8, rs1: u8, rs2: u8 },
}

pub fn decode(instruction: u32) -> Result<Instruction, DecodeError> {
    // instruction[6:0]
    let opcode = instruction & 0x7f;

    match opcode {
        0b0110011 => decode_op(instruction),
        _ => Err(DecodeError),
    }
}

fn decode_op(instruction: u32) -> Result<Instruction, DecodeError> {
    // instruction[14:12]
    let funct3 = (instruction >> 12) & 0x7;
    // instruction[31:25]
    let funct7 = (instruction >> 25) & 0x7f;
    let funct = (funct7 << 3) | funct3;

    // instruction[11:7]
    let rd = (instruction >> 7) as u8 & 0x1f;
    // instruction[19:15]
    let rs1 = (instruction >> 15) as u8 & 0x1f;
    // instruction[24:20]
    let rs2 = (instruction >> 20) as u8 & 0x1f;

    match funct {
        0b0000000000 => Ok(Instruction::Add { rd, rs1, rs2 }),
        _ => Err(DecodeError),
    }
}

#[derive(Debug)]
pub struct DecodeError;
impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Illigal instruction")
    }
}
impl std::error::Error for DecodeError {}
