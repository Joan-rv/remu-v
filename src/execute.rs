use crate::instruction::Instruction;
use crate::memory::Memory;
use crate::state::State;
use crate::syscall::syscall;

pub fn execute(
    instruction: Instruction,
    state: &mut State,
    memory: &mut Memory,
) -> anyhow::Result<()> {
    match instruction {
        Instruction::Lui { rd, imm } => {
            state.set(rd, imm);
        }
        Instruction::Auipc { rd, imm } => {
            let v = imm + state.pc;
            state.set(rd, v);
        }
        Instruction::Jal { rd, imm } => {
            state.set(rd, state.pc);
            state.pc = state.pc.wrapping_add_signed(imm - 4);
        }
        Instruction::Jalr { rd, rs1, imm } => {
            let mut new_pc = state.gets(rs1) + imm as i32;
            new_pc &= !0b1i32; // set last bit to 0
            state.set(rd, state.pc);
            state.pc = new_pc as u32;
        }
        Instruction::Beq { imm, rs1, rs2 } => {
            if state.get(rs1) == state.get(rs2) {
                state.pc = state.pc.wrapping_add_signed(imm as i32 - 4);
            }
        }
        Instruction::Bne { imm, rs1, rs2 } => {
            if state.get(rs1) != state.get(rs2) {
                state.pc = state.pc.wrapping_add_signed(imm as i32 - 4);
            }
        }
        Instruction::Blt { imm, rs1, rs2 } => {
            if state.gets(rs1) < state.gets(rs2) {
                state.pc = state.pc.wrapping_add_signed(imm as i32 - 4);
            }
        }
        Instruction::Bge { imm, rs1, rs2 } => {
            if state.gets(rs1) >= state.gets(rs2) {
                state.pc = state.pc.wrapping_add_signed(imm as i32 - 4);
            }
        }
        Instruction::Bltu { imm, rs1, rs2 } => {
            if state.get(rs1) < state.get(rs2) {
                state.pc = state.pc.wrapping_add_signed(imm as i32 - 4);
            }
        }
        Instruction::Bgeu { imm, rs1, rs2 } => {
            if state.get(rs1) >= state.get(rs2) {
                state.pc = state.pc.wrapping_add_signed(imm as i32 - 4);
            }
        }
        Instruction::Lb { rd, rs1, imm } => {
            let addr = state.get(rs1).wrapping_add_signed(imm as i32);
            let v = memory.lb(addr) as u32;
            let v = (v << 24) >> 24;
            state.set(rd, v);
        }
        Instruction::Lh { rd, rs1, imm } => {
            let addr = state.get(rs1).wrapping_add_signed(imm as i32);
            let v = memory.lh(addr) as u32;
            let v = (v << 16) >> 16;
            state.set(rd, v);
        }
        Instruction::Lw { rd, rs1, imm } => {
            let addr = state.get(rs1).wrapping_add_signed(imm as i32);
            let v = memory.lw(addr);
            state.set(rd, v);
        }
        Instruction::Lbu { rd, rs1, imm } => {
            let addr = state.get(rs1).wrapping_add_signed(imm as i32);
            let v = memory.lb(addr) as u32;
            state.set(rd, v);
        }
        Instruction::Lhu { rd, rs1, imm } => {
            let addr = state.get(rs1).wrapping_add_signed(imm as i32);
            let v = memory.lh(addr) as u32;
            state.set(rd, v);
        }
        Instruction::Sb { rs1, rs2, imm } => {
            let addr = state.get(rs1).wrapping_add_signed(imm as i32);
            memory.sb(addr, state.get(rs2) as u8);
        }
        Instruction::Sh { rs1, rs2, imm } => {
            let addr = state.get(rs1).wrapping_add_signed(imm as i32);
            memory.sh(addr, state.get(rs2) as u16);
        }
        Instruction::Sw { rs1, rs2, imm } => {
            let addr = state.get(rs1).wrapping_add_signed(imm as i32);
            memory.sw(addr, state.get(rs2));
        }
        Instruction::Addi { rd, rs1, imm } => {
            let v = state.get(rs1).wrapping_add_signed(imm as i32);
            state.set(rd, v);
        }
        Instruction::Slti { rd, rs1, imm } => {
            let v = state.gets(rs1) < imm as i32;
            state.set(rd, v as u32);
        }
        Instruction::Sltiu { rd, rs1, imm } => {
            let v = state.get(rs1) < imm as u32;
            state.set(rd, v as u32);
        }
        Instruction::Xori { rd, rs1, imm } => {
            let v = state.get(rs1) ^ imm as u32;
            state.set(rd, v);
        }
        Instruction::Ori { rd, rs1, imm } => {
            let v = state.get(rs1) | imm as u32;
            state.set(rd, v);
        }
        Instruction::Andi { rd, rs1, imm } => {
            let v = state.get(rs1) & imm as u32;
            state.set(rd, v);
        }
        Instruction::Slli { rd, rs1, shamt } => {
            let v = state.get(rs1) << shamt;
            state.set(rd, v);
        }
        Instruction::Srli { rd, rs1, shamt } => {
            let v = state.get(rs1) >> shamt;
            state.set(rd, v);
        }
        Instruction::Srai { rd, rs1, shamt } => {
            let v = state.gets(rs1) >> shamt;
            state.set(rd, v as u32);
        }
        Instruction::Add { rd, rs1, rs2 } => {
            let v = state.get(rs1).wrapping_add(state.get(rs2));
            state.set(rd, v);
        }
        Instruction::Sub { rd, rs1, rs2 } => {
            let v = state.get(rs1).wrapping_sub(state.get(rs2));
            state.set(rd, v);
        }
        Instruction::Sll { rd, rs1, rs2 } => {
            let v = state.get(rs1) << state.get(rs2);
            state.set(rd, v);
        }
        Instruction::Slt { rd, rs1, rs2 } => {
            let v = state.gets(rs1) < state.gets(rs2);
            state.set(rd, v as u32);
        }
        Instruction::Sltu { rd, rs1, rs2 } => {
            let v = state.get(rs1) < state.get(rs2);
            state.set(rd, v as u32);
        }
        Instruction::Xor { rd, rs1, rs2 } => {
            let v = state.get(rs1) ^ state.get(rs2);
            state.set(rd, v);
        }
        Instruction::Srl { rd, rs1, rs2 } => {
            let v = state.get(rs1) >> state.get(rs2);
            state.set(rd, v);
        }
        Instruction::Sra { rd, rs1, rs2 } => {
            let v = state.gets(rs1) >> state.get(rs2);
            state.set(rd, v as u32);
        }
        Instruction::Or { rd, rs1, rs2 } => {
            let v = state.get(rs1) | state.get(rs2);
            state.set(rd, v);
        }
        Instruction::And { rd, rs1, rs2 } => {
            let v = state.get(rs1) & state.get(rs2);
            state.set(rd, v);
        }
        Instruction::Fence => {}
        Instruction::Ecall => syscall(state, memory)?,
        Instruction::Ebreak => todo!(), // maybe implement as nop or std::instrinsics::breakpoint
    };
    Ok(())
}
