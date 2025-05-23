use crate::instruction::Instruction;
use thiserror::Error;

pub fn decode(word: u32) -> Result<Instruction, DecodeError> {
    // instruction[6:0]
    let opcode = word & 0x7f;

    match opcode {
        0b0110011 => decode_op(word),
        0b0010011 => decode_op_imm(word),
        0b1100011 => decode_branch(word),
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

fn decode_op_imm(word: u32) -> Result<Instruction, DecodeError> {
    // instruction[14:12]
    let funct3 = (word >> 12) & 0x7;

    // instruction[11:7]
    let rd = (word >> 7) as u8 & 0x1f;
    // instruction[19:15]
    let rs1 = (word >> 15) as u8 & 0x1f;
    // instruction[31:20]
    let imm = (word >> 20) as u16 & 0xfff;

    match funct3 {
        0b000 => Ok(Instruction::Addi { rd, rs1, imm }),
        _ => Err(DecodeError),
    }
}

fn decode_branch(word: u32) -> Result<Instruction, DecodeError> {
    // instruction[14:12]
    let funct3 = (word >> 12) & 0x7;

    // instruction[31:25] -> imm[12|10:5]
    let hi = (word >> 25) & 0x7f;
    let bit12 = (hi & 0x40) << (12 - 6);
    let bit10_5 = (hi & 0x3f) << (10 - 5);

    // instruction[11:7] -> imm[4:1|11]
    let lo = (word >> 7) & 0x1f;
    let bit4_1 = lo & 0x1e;
    let bit11 = (lo & 0x1) << 11;

    // bit0 is 0
    let u_imm = bit12 | bit11 | bit10_5 | bit4_1;
    // sign extend
    let imm = ((u_imm as i16) << 3) >> 3;

    // instruction[19:15]
    let rs1 = (word >> 15) as u8 & 0x1f;
    // instruction[24:20]
    let rs2 = (word >> 20) as u8 & 0x1f;

    match funct3 {
        0b000 => Ok(Instruction::Beq { imm, rs1, rs2 }),
        0b001 => Ok(Instruction::Bne { imm, rs1, rs2 }),
        0b100 => Ok(Instruction::Blt { imm, rs1, rs2 }),
        0b101 => Ok(Instruction::Bge { imm, rs1, rs2 }),
        0b110 => Ok(Instruction::Bltu { imm, rs1, rs2 }),
        0b111 => Ok(Instruction::Bgeu { imm, rs1, rs2 }),
        _ => Err(DecodeError),
    }
}

#[derive(Error, Debug)]
#[error("Illigal instruction")]
pub struct DecodeError;
