use crate::instruction::Instruction;
use thiserror::Error;

pub fn decode(word: u32) -> Result<Instruction, DecodeError> {
    // instruction[6:0]
    let opcode = word & 0x7f;

    match opcode {
        0b0110011 => decode_op(word),
        0b0010011 => decode_op_imm(word),
        0b1100011 => decode_branch(word),
        0b0000011 => decode_load(word),
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
        0b0100000000 => Ok(Instruction::Sub { rd, rs1, rs2 }),
        0b0000000001 => Ok(Instruction::Sll { rd, rs1, rs2 }),
        0b0000000010 => Ok(Instruction::Slt { rd, rs1, rs2 }),
        0b0000000011 => Ok(Instruction::Sltu { rd, rs1, rs2 }),
        0b0000000100 => Ok(Instruction::Xor { rd, rs1, rs2 }),
        0b0000000101 => Ok(Instruction::Srl { rd, rs1, rs2 }),
        0b0100000101 => Ok(Instruction::Sra { rd, rs1, rs2 }),
        0b0000000110 => Ok(Instruction::Or { rd, rs1, rs2 }),
        0b0000000111 => Ok(Instruction::And { rd, rs1, rs2 }),
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
    let imm = (word >> 20) as i16;

    match funct3 {
        0b000 => Ok(Instruction::Addi { rd, rs1, imm }),
        _ => Err(DecodeError),
    }
}

fn decode_branch(word: u32) -> Result<Instruction, DecodeError> {
    // instruction[14:12]
    let funct3 = (word >> 12) & 0x7;

    // instruction[31:25] -> imm[12|10:5]
    let hi = word as i32 >> (32 - 13);
    let bit12 = (hi >> 12) << 12; // set low bits to zero but keep sign extension bits
    let bit10_5 = (hi >> 1) & 0x7e0;

    // instruction[11:7] -> imm[4:1|11]
    let lo = word as i32 >> 7;
    let bit4_1 = lo & 0x1e;
    let bit11 = (lo & 0x1) << 11;

    // bit0 is 0 implicity (jumps are 2 byte aligned)
    let imm = (bit12 | bit11 | bit10_5 | bit4_1) as i16;

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

fn decode_load(word: u32) -> Result<Instruction, DecodeError> {
    // instruction[14:12]
    let funct3 = (word >> 12) & 0x7;

    // instruction[11:7]
    let rd = (word >> 7) as u8 & 0x1f;
    // instruction[19:15]
    let rs1 = (word >> 15) as u8 & 0x1f;
    // instruction[31:20]
    let imm = (word as i32 >> 20) as i16;

    match funct3 {
        0b000 => Ok(Instruction::Lb { rd, rs1, imm }),
        0b001 => Ok(Instruction::Lh { rd, rs1, imm }),
        0b010 => Ok(Instruction::Lw { rd, rs1, imm }),
        0b100 => Ok(Instruction::Lbu { rd, rs1, imm }),
        0b101 => Ok(Instruction::Lhu { rd, rs1, imm }),
        _ => Err(DecodeError),
    }
}

#[derive(Error, Debug)]
#[error("Illigal instruction")]
pub struct DecodeError;
