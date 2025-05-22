use crate::instruction::Instruction;

pub fn decode(word: u32) -> Result<Instruction, DecodeError> {
    // instruction[6:0]
    let opcode = word & 0x7f;

    match opcode {
        0b0110011 => decode_op(word),
        _ => Err(DecodeError),
    }
}

fn decode_op(word: u32) -> Result<Instruction, DecodeError> {
    // instruction[14:12]
    let funct3 = (word >> 12) & 0x7;
    // instruction[31:25]
    let funct7 = (word >> 25) & 0x7f;
    let funct = (funct7 << 3) | funct3;

    // instruction[11:7]
    let rd = (word >> 7) as u8 & 0x1f;
    // instruction[19:15]
    let rs1 = (word >> 15) as u8 & 0x1f;
    // instruction[24:20]
    let rs2 = (word >> 20) as u8 & 0x1f;

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
